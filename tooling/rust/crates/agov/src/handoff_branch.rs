// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jani Päijänen

//! Opt-in, advisory source inventory. Lifecycle resolution never crosses sources.
use super::*;
use std::io::Write;

#[derive(Clone)]
struct Snapshot {
    messages: Vec<HandoffMessage>,
    blobs: BTreeMap<String, String>,
    errors: Vec<String>,
}

struct Candidate {
    message: HandoffMessage,
    source: String,
    sha: String,
    blob: String,
    replies: Vec<ReplyEvidence>,
}

fn from_content(file: String, raw: serde_json::Value, body: &str) -> HandoffMessage {
    let mut refs = frontmatter_refs(&raw);
    refs.extend(
        body.split(|ch: char| {
            ch.is_whitespace() || matches!(ch, '`' | '"' | '\'' | '(' | ')' | ',')
        })
        .filter_map(clean_coord_filename),
    );
    refs.sort();
    refs.dedup();
    HandoffMessage {
        sort_key: (
            string_field(&raw, "timestamp").unwrap_or("").into(),
            file.clone(),
        ),
        file,
        refs,
        raw,
    }
}

fn raw_blob(content: &[u8]) -> Result<String, String> {
    // Hash exactly the validated snapshot, without worktree clean/eol filters.
    let mut child = std::process::Command::new("git")
        .args(["hash-object", "--no-filters", "--stdin"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    let write = child.stdin.take().unwrap().write_all(content);
    let result = child.wait_with_output().map_err(|e| e.to_string())?;
    write.map_err(|e| e.to_string())?;
    if !result.status.success() {
        return Err(String::from_utf8_lossy(&result.stderr).into_owned());
    }
    Ok(String::from_utf8_lossy(&result.stdout).trim().into())
}

fn filesystem(dir: &Path, role: &str) -> Result<Snapshot, String> {
    let schema = agov_core::schema::load_and_compile(
        &agov_core::schema_dir().join("coordination-message.schema.json"),
    )
    .map_err(|e| e.to_string())?;
    let mut snapshot = Snapshot {
        messages: vec![],
        blobs: BTreeMap::new(),
        errors: vec![],
    };
    let mut contents = BTreeMap::new();
    for entry in std::fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }
        let file = entry.file_name().to_string_lossy().into_owned();
        match std::fs::read_to_string(&path) {
            Ok(content) => {
                match agov_coord::coord_validator::validate_content_with_schema(&content, &schema) {
                    Ok(msg) => {
                        snapshot.messages.push(from_content(
                            file.clone(),
                            msg.raw,
                            content.splitn(3, "---").nth(2).unwrap_or(&content),
                        ));
                        contents.insert(file, content);
                    }
                    Err(errors) => snapshot.errors.push(format!(
                        "invalid candidate {file}: {}",
                        serde_json::to_string(&errors).unwrap()
                    )),
                }
            }
            Err(e) => snapshot
                .errors
                .push(format!("unreadable candidate {file}: {e}")),
        }
    }
    snapshot.messages.sort_by_key(|m| m.sort_key.clone());
    for message in pending_handoffs(&snapshot.messages)
        .iter()
        .filter(|m| string_field(&m.raw, "to") == Some(role))
    {
        match raw_blob(contents[&message.file].as_bytes()) {
            Ok(blob) => {
                snapshot.blobs.insert(message.file.clone(), blob);
            }
            Err(e) => snapshot
                .errors
                .push(format!("cannot hash {}: {e}", message.file)),
        }
    }
    Ok(snapshot)
}

type BlobCache = BTreeMap<(String, String), Result<HandoffMessage, String>>;

fn branch(sha: &str, dir: &str, cache: &mut BlobCache) -> Result<Snapshot, String> {
    let inventory = git_candidate_inventory(sha, dir)?
        .into_iter()
        .filter(|(path, _)| {
            agov_coord::channel_reader::is_channel_path(Path::new(path), Path::new(dir))
        })
        .collect::<BTreeMap<_, _>>();
    let missing = inventory
        .iter()
        .filter(|(path, blob)| !cache.contains_key(&((*path).clone(), (*blob).clone())))
        .map(|(p, b)| (p.clone(), b.clone()))
        .collect::<BTreeMap<_, _>>();
    if !missing.is_empty() {
        let mut invalid = vec![];
        let parsed = load_visibility_snapshot(
            sha,
            dir,
            &missing,
            &BTreeMap::new(),
            true,
            &mut vec![],
            &mut invalid,
            &mut vec![],
        )?;
        for m in parsed {
            let body = string_field(&m.raw, "_body").unwrap_or("").to_string();
            let file = Path::new(&m.file)
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned();
            let key = (m.file.clone(), inventory[&m.file].clone());
            cache.insert(key, Ok(from_content(file, m.raw, &body)));
        }
        for diagnostic in invalid {
            let file = diagnostic["path"]
                .as_str()
                .ok_or("invalid diagnostic lacks path")?;
            cache.insert(
                (file.into(), inventory[file].clone()),
                Err(format!(
                    "invalid candidate {file}: {}",
                    diagnostic["errors"]
                )),
            );
        }
    }
    // Reassemble the COMPLETE tree before resolution; the cache shares parsing,
    // never lifecycle state, even when only one changed blob was loaded above.
    let mut snapshot = Snapshot {
        messages: vec![],
        blobs: BTreeMap::new(),
        errors: vec![],
    };
    for (path, blob) in inventory {
        match cache
            .get(&(path.clone(), blob.clone()))
            .ok_or_else(|| format!("unreadable candidate {sha}:{path}"))?
        {
            Ok(message) => {
                snapshot.blobs.insert(message.file.clone(), blob);
                snapshot.messages.push(message.clone());
            }
            Err(error) => snapshot.errors.push(error.clone()),
        }
    }
    snapshot.messages.sort_by_key(|m| m.sort_key.clone());
    Ok(snapshot)
}

fn collect(
    snapshot: &Snapshot,
    role: &str,
    source: &str,
    sha: &str,
    candidates: &mut BTreeMap<(String, String), Candidate>,
) {
    // Called in attribution priority order, only AFTER each source resolves itself.
    for message in pending_handoffs(&snapshot.messages)
        .into_iter()
        .filter(|m| string_field(&m.raw, "to") == Some(role))
    {
        let Some(blob) = snapshot.blobs.get(&message.file) else {
            continue;
        };
        candidates
            .entry((message.file.clone(), blob.clone()))
            .or_insert_with(|| Candidate {
                replies: reply_evidence(&message, &snapshot.messages),
                message,
                source: source.into(),
                sha: sha.into(),
                blob: blob.clone(),
            });
    }
}

pub(super) fn run(role: &str, dir: PathBuf, session: Option<&str>, args: &[String]) -> i32 {
    let mut output = OutputCounter::default();
    let dir = repo_relative_path(dir);
    let fs = filesystem(&dir, role);
    let mut errors = vec![];
    let mut candidates = BTreeMap::new();
    output.stdout("Filesystem next handoff (branch evidence below is advisory):");
    match &fs {
        Ok(snapshot) => {
            if let Some(message) = pending_handoffs(&snapshot.messages)
                .into_iter()
                .filter(|m| string_field(&m.raw, "to") == Some(role))
                .max_by_key(|m| m.sort_key.clone())
            {
                let replies = reply_evidence(&message, &snapshot.messages);
                render_handoff_summary(&mut output, &message, !replies.is_empty());
                render_reply_evidence(&mut output, &message, &snapshot.messages, "  ");
                if let Some(recipient) = addressed_elsewhere(&message.raw, session) {
                    output.stdout(format!("Addressed elsewhere: {recipient}"));
                }
            } else {
                output.stdout(format!("No filesystem handoff found for role: {role}"));
            }
            errors.extend(snapshot.errors.iter().map(|e| format!("working tree: {e}")));
            collect(
                snapshot,
                role,
                "working tree",
                "working tree",
                &mut candidates,
            );
        }
        Err(e) => errors.push(format!("working tree: {e}")),
    }

    let mut blob_cache = BlobCache::new();
    let mut diagnostic_sets: BTreeMap<Vec<String>, Vec<String>> = BTreeMap::new();
    match git_coord_dir(&dir).and_then(|path| git_branch_refs().map(|refs| (path, refs))) {
        Ok((path, mut refs)) => {
            refs.sort_by_key(|r| (branch_ref_class(&r.full_name), r.full_name.clone()));
            let mut groups: Vec<Vec<BranchRef>> = vec![];
            let mut positions = BTreeMap::new();
            for reference in refs {
                let index = *positions.entry(reference.oid.clone()).or_insert_with(|| {
                    groups.push(vec![]);
                    groups.len() - 1
                });
                groups[index].push(reference);
            }
            for group in groups {
                // Immutable equal tips have identical lifecycle sets. Share the read,
                // but preserve every failed ref label. Final dedup is still path/blob.
                let reference = &group[0];
                match branch(&reference.oid, &path, &mut blob_cache) {
                    Ok(snapshot) => {
                        if !snapshot.errors.is_empty() {
                            diagnostic_sets
                                .entry(snapshot.errors.clone())
                                .or_default()
                                .extend(
                                    group.iter().map(|r| format!("{} @ {}", r.full_name, r.oid)),
                                );
                        }
                        collect(
                            &snapshot,
                            role,
                            &reference.full_name,
                            &reference.oid,
                            &mut candidates,
                        );
                    }
                    Err(e) => errors.extend(
                        group
                            .iter()
                            .map(|r| format!("{} @ {}: {e}", r.full_name, r.oid)),
                    ),
                }
            }
        }
        Err(e) => errors.push(format!("ref inventory: {e}")),
    }
    output.stdout(format!("Advisory pending source inventory for {role}:"));
    let mut counts = BTreeMap::new();
    for (file, _) in candidates.keys() {
        *counts.entry(file.clone()).or_insert(0usize) += 1;
    }
    let mut ordered: Vec<_> = candidates.values().collect();
    // Preserve BTreeMap (file, blob) ordering and conflict attribution within each tier.
    ordered.sort_by_key(|candidate| !is_urgent(&candidate.message));
    for candidate in ordered {
        if counts[&candidate.message.file] > 1 {
            output.stdout(format!("CONFLICT: {}", candidate.message.file));
        }
        output.stdout(format!(
            "Candidate: {} | source: {} | commit: {} | blob: {}",
            candidate.message.file, candidate.source, candidate.sha, candidate.blob
        ));
        render_handoff_summary(&mut output, &candidate.message, !candidate.replies.is_empty());
        if !candidate.replies.is_empty() {
            output.stdout(format!(
                "  reply present — acknowledgement unconfirmed; reply_evidence: {}",
                serde_json::to_string(&candidate.replies).unwrap()
            ));
        }
        if let Some(recipient) = addressed_elsewhere(&candidate.message.raw, session) {
            output.stdout(format!("Addressed elsewhere: {recipient}"));
        }
    }
    let partial = !errors.is_empty() || !diagnostic_sets.is_empty();
    output.stdout(format!("partial_coverage: {partial}"));
    let dictionary = diagnostic_sets
        .keys()
        .flatten()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .enumerate()
        .map(|(i, text)| (text, i + 1))
        .collect::<BTreeMap<_, _>>();
    for (diagnostic, id) in &dictionary {
        output.stdout(format!("Scan error D{id}: {diagnostic}"));
    }
    for (diagnostics, sources) in &diagnostic_sets {
        let mut ids = diagnostics
            .iter()
            .map(|d| dictionary[d])
            .collect::<Vec<_>>();
        ids.sort_unstable();
        ids.dedup();
        let mut ranges = vec![];
        let mut cursor = 0;
        while cursor < ids.len() {
            let start = ids[cursor];
            let mut end = start;
            cursor += 1;
            while cursor < ids.len() && ids[cursor] == end + 1 {
                end = ids[cursor];
                cursor += 1;
            }
            ranges.push(if start == end {
                format!("D{start}")
            } else {
                format!("D{start}-D{end}")
            });
        }
        output.stdout(format!(
            "Scan errors {} shared by {} sources:",
            ranges.join(","),
            sources.len()
        ));
        for source in sources {
            output.stdout(format!("  source: {source}"));
        }
    }
    for error in &errors {
        output.stdout(format!("Scan error: {error}"));
    }
    agov_core::roi::record_tool_output(
        "agov.handoff.next",
        args,
        &format!("next-handoff:{role}"),
        output.chars,
    );
    if partial {
        1
    } else if candidates.is_empty() {
        2
    } else {
        0
    }
}
