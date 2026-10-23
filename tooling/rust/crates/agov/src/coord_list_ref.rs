//! Read the coordination channel from a fetched Git ref for `agov coord list --ref`.
//!
//! Local Git reads only: no fetch, no forge call. Reuses the visibility code's
//! `git_cat_file_batch` and the shared message validator; nothing here edits
//! `agov-coord`.

use super::{body_refs_from_content, frontmatter_refs, git_cat_file_batch, git_output, string_field, HandoffMessage};
use std::path::Path;

const MAX_ENTRIES: usize = 50_000;
const MAX_BLOB_BYTES: u64 = 1024 * 1024;
const MAX_TOTAL_BYTES: u64 = 128 * 1024 * 1024;
const MAX_SHOWN_CHARS: usize = 200;

pub(crate) struct RefChannel {
    pub source_ref: String,
    pub source_oid: String,
    pub coordination_dir: String,
    /// Symlinks, submodules and unsafe names in the channel directory that were not read.
    pub skipped: usize,
    pub messages: Vec<HandoffMessage>,
}

/// Escape control characters and cap the length before echoing caller text.
pub(crate) fn display_untrusted(raw: &str) -> String {
    let mut shown = String::new();
    for (count, ch) in raw.chars().enumerate() {
        if count == MAX_SHOWN_CHARS {
            shown.push_str("...[truncated]");
            break;
        }
        if ch.is_control() {
            shown.push_str(&format!("\\x{:02x}", ch as u32));
        } else {
            shown.push(ch);
        }
    }
    shown
}

/// A repository-relative directory: not absolute, no `..`, backslash, NUL or leading `-`.
fn normalize_dir(dir: &Path) -> Result<String, String> {
    let text = dir.to_str().ok_or("--dir is not valid UTF-8")?;
    let trimmed = text.trim_end_matches('/');
    if trimmed.is_empty()
        || dir.is_absolute()
        || trimmed.starts_with('-')
        || trimmed.contains(['\\', '\0'])
        || trimmed.split('/').any(|part| part.is_empty() || part == "." || part == "..")
    {
        return Err(format!("--dir must be a repository-relative path: {}", display_untrusted(text)));
    }
    Ok(trimmed.to_string())
}

pub(crate) fn read_channel_at_ref(reference: &str, dir: &Path) -> Result<RefChannel, String> {
    let shown = display_untrusted(reference);
    if reference.is_empty() || reference.contains('\0') {
        return Err(format!("--ref is empty or contains NUL: {shown}"));
    }
    let coordination_dir = normalize_dir(dir)?;
    let spec = format!("{reference}^{{commit}}");
    let source_oid = git_output(&["rev-parse", "--verify", "--end-of-options", &spec])
        .map_err(|err| format!("could not resolve --ref {shown}: {}", display_untrusted(&err)))?;

    let tree_spec = format!("{source_oid}:{coordination_dir}");
    let listing = git_output_raw(&["ls-tree", "-z", "-l", "--end-of-options", &tree_spec])
        .map_err(|err| format!("could not list {coordination_dir} at {source_oid}: {}", display_untrusted(&err)))?;

    let mut files = Vec::new();
    let mut skipped = 0usize;
    let mut total = 0u64;
    let mut entries = 0usize;
    for entry in listing.split(|b| *b == 0).filter(|b| !b.is_empty()) {
        entries += 1;
        if entries > MAX_ENTRIES {
            return Err(format!("{coordination_dir} has more than {MAX_ENTRIES} entries at {source_oid}"));
        }
        let text = std::str::from_utf8(entry).map_err(|_| "non-UTF8 Git tree entry".to_string())?;
        let (header, name) = text.split_once('\t').ok_or("malformed Git tree entry")?;
        let fields: Vec<&str> = header.split_whitespace().collect();
        if fields.len() != 4 {
            return Err("malformed Git tree header".into());
        }
        if name.len() <= 3 || !name.ends_with(".md") {
            continue;
        }
        let (mode, kind) = (fields[0], fields[1]);
        if kind == "tree" {
            continue;
        }
        if !(mode == "100644" || mode == "100755") || kind != "blob" || name.contains(|c: char| c.is_control()) {
            skipped += 1;
            continue;
        }
        let size: u64 = fields[3].parse().map_err(|_| "malformed Git blob size")?;
        if size > MAX_BLOB_BYTES {
            return Err(format!("{} is larger than {MAX_BLOB_BYTES} bytes", display_untrusted(name)));
        }
        total += size;
        if total > MAX_TOTAL_BYTES {
            return Err(format!("{coordination_dir} is larger than {MAX_TOTAL_BYTES} bytes at {source_oid}"));
        }
        files.push(name.to_string());
    }

    let paths: Vec<String> = files.iter().map(|name| format!("{coordination_dir}/{name}")).collect();
    let contents = git_cat_file_batch(&source_oid, &paths)?;
    if contents.len() != paths.len() {
        return Err("incomplete Git blob read".into());
    }
    let schema = agov_coord::coord_validator::compile_channel_schema()
        .map_err(|err| format!("channel schema unavailable: {err:?}"))?;

    let mut messages = Vec::new();
    for (name, (_, content)) in files.into_iter().zip(contents) {
        let content = content.ok_or_else(|| format!("{} vanished while reading", display_untrusted(&name)))?;
        let Ok(message) = agov_coord::coord_validator::validate_content_with_schema(&content, &schema) else {
            continue;
        };
        let timestamp = string_field(&message.raw, "timestamp").unwrap_or("").to_string();
        let mut refs = frontmatter_refs(&message.raw);
        refs.extend(body_refs_from_content(&content));
        refs.sort();
        refs.dedup();
        messages.push(HandoffMessage {
            sort_key: (timestamp, name.clone()),
            file: name,
            refs,
            raw: message.raw,
        });
    }
    messages.sort_by_key(|message| message.sort_key.clone());

    Ok(RefChannel {
        source_ref: reference.to_string(),
        source_oid,
        coordination_dir,
        skipped,
        messages,
    })
}

/// `git` stdout as raw bytes (NUL-framed output must not be trimmed or lossy-decoded).
fn git_output_raw(args: &[&str]) -> Result<Vec<u8>, String> {
    let output = std::process::Command::new("git")
        .args(args)
        .output()
        .map_err(|err| err.to_string())?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(if stderr.is_empty() { format!("git exited with {}", output.status) } else { stderr })
    }
}
