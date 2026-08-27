// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jani Päijänen

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

const SCOPE_EXPRESSION: &[u8] =
    concat!("CC-BY-SA-4.0", " OR LicenseRef-AgentsGovern-Commercial").as_bytes();
const CL3_DECISION: &str = "20260818T054511Z-governor-reviewer-1gaj";
const LICENCE_SCOPE_DECISION: &str = "20260818T090855Z-governor-reviewer-7b2r";
const LICENCE_TEXT_PATH: &str = "LICENSES/LicenseRef-AgentsGovern-Commercial.txt";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommercialCorpusManifest {
    pub schema_version: u64,
    pub corpus_version: u64,
    pub corpus_content_revision: String,
    pub provenance: Provenance,
    pub source_inventory: SourceInventory,
    pub included: Vec<IncludedEntry>,
    pub excluded: Vec<ExcludedEntry>,
    pub corpus_set_digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Provenance {
    pub source_inventory_commit: String,
    pub cl3_decision: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceInventory {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IncludedEntry {
    pub path: String,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExcludedEntry {
    pub path: String,
    pub reason: String,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CurrentPointer {
    schema_version: u64,
    current_manifest: String,
}

pub struct GenerateOptions {
    pub repo_root: PathBuf,
    pub inventory_ref: String,
    pub inventory_path: PathBuf,
    pub corpus_content_revision: String,
    pub output: PathBuf,
    pub pointer: PathBuf,
    pub schema: PathBuf,
}

pub struct ValidateOptions {
    pub repo_root: PathBuf,
    pub pointer: PathBuf,
    pub schema: PathBuf,
    pub base_ref: Option<String>,
}

pub struct VerifyOptions {
    pub repo_root: PathBuf,
    pub manifest: PathBuf,
    pub schema: PathBuf,
}

#[derive(Debug)]
pub struct CommandReport {
    pub lines: Vec<String>,
}

pub fn generate(options: GenerateOptions) -> Result<CommandReport, Vec<String>> {
    validate_exact_commit(&options.inventory_ref, "inventory ref")?;
    validate_exact_commit(&options.corpus_content_revision, "corpus content revision")?;
    let inventory_path = repository_path_string(&options.inventory_path)?;
    let inventory_bytes = git_output(
        &options.repo_root,
        &[
            "show",
            &format!("{}:{inventory_path}", options.inventory_ref),
        ],
    )?;
    let (manifest, candidates) = build_manifest(
        &options.repo_root,
        &inventory_bytes,
        &options.inventory_ref,
        &inventory_path,
        &options.corpus_content_revision,
    )?;
    validate_against_schema(&options.repo_root, &options.schema, &manifest)?;

    let output_path = resolve(&options.repo_root, &options.output);
    let pointer_path = resolve(&options.repo_root, &options.pointer);
    write_yaml(&output_path, &manifest)?;
    let pointer = CurrentPointer {
        schema_version: 1,
        current_manifest: repository_path_string(&options.output)?,
    };
    write_yaml(&pointer_path, &pointer)?;

    Ok(CommandReport {
        lines: vec![format!(
            "OK generated {} included={} excluded={} candidates={} corpus_set_digest={}",
            options.output.display(),
            manifest.included.len(),
            manifest.excluded.len(),
            candidates.len(),
            manifest.corpus_set_digest
        )],
    })
}

fn build_manifest(
    repo_root: &Path,
    inventory_bytes: &[u8],
    inventory_ref: &str,
    inventory_path: &str,
    corpus_content_revision: &str,
) -> Result<(CommercialCorpusManifest, BTreeSet<String>), Vec<String>> {
    validate_exact_commit(inventory_ref, "inventory ref")?;
    validate_exact_commit(corpus_content_revision, "corpus content revision")?;
    let inventory = parse_inventory(inventory_bytes)?;

    let mut included = Vec::new();
    let mut excluded = Vec::new();
    for row in inventory {
        match row.classification.as_str() {
            "Licensor-authored" => included.push(IncludedEntry {
                sha256: sha256_git_path(repo_root, corpus_content_revision, &row.path)?,
                path: row.path,
            }),
            "generated/vendor" => excluded.push(ExcludedEntry {
                path: row.path,
                reason: format!(
                    "generated/vendor -- excluded pending qualified-counsel clearance ({CL3_DECISION})"
                ),
                source: "inventory".to_string(),
                decision: None,
            }),
            other => {
                return Err(vec![format!(
                    "inventory classification {other:?} is not governed for commercial corpus v1"
                )])
            }
        }
    }
    excluded.push(ExcludedEntry {
        path: LICENCE_TEXT_PATH.to_string(),
        reason: format!(
            "licensing instrument, not Framework Content -- explicit post-inventory exclusion per governor decision {LICENCE_SCOPE_DECISION}"
        ),
        source: "post-inventory-governor-decision".to_string(),
        decision: Some(LICENCE_SCOPE_DECISION.to_string()),
    });

    let mut manifest = CommercialCorpusManifest {
        schema_version: 1,
        corpus_version: 1,
        corpus_content_revision: corpus_content_revision.to_string(),
        provenance: Provenance {
            source_inventory_commit: inventory_ref.to_string(),
            cl3_decision: CL3_DECISION.to_string(),
        },
        source_inventory: SourceInventory {
            path: inventory_path.to_string(),
        },
        included,
        excluded,
        corpus_set_digest: String::new(),
    };
    structural_errors(&manifest)?;

    let candidates = candidate_paths_at_revision(repo_root, corpus_content_revision)?;
    let accounted = accounted_paths(&manifest);
    let unaccounted: Vec<_> = candidates.difference(&accounted).cloned().collect();
    let non_candidates: Vec<_> = accounted.difference(&candidates).cloned().collect();
    if !unaccounted.is_empty() || !non_candidates.is_empty() {
        let mut errors = Vec::new();
        if !unaccounted.is_empty() {
            errors.push(format!(
                "generation stopped: {} candidate path(s) lack an included/excluded disposition: {}",
                unaccounted.len(),
                unaccounted.join(", ")
            ));
        }
        if !non_candidates.is_empty() {
            errors.push(format!(
                "generation stopped: {} manifest path(s) do not satisfy the candidate rule at {}: {}",
                non_candidates.len(),
                corpus_content_revision,
                non_candidates.join(", ")
            ));
        }
        return Err(errors);
    }

    manifest.corpus_set_digest = corpus_set_digest(&manifest)?;
    Ok((manifest, candidates))
}

pub fn validate_current(options: ValidateOptions) -> Result<CommandReport, Vec<String>> {
    let pointer_path = resolve(&options.repo_root, &options.pointer);
    let pointer_text = fs::read_to_string(&pointer_path)
        .map_err(|error| vec![format!("{}: {error}", pointer_path.display())])?;
    let pointer: CurrentPointer = serde_norway::from_str(&pointer_text)
        .map_err(|error| vec![format!("{}: {error}", pointer_path.display())])?;
    if pointer.schema_version != 1 {
        return Err(vec![format!(
            "{}: unsupported pointer schema_version {}",
            pointer_path.display(),
            pointer.schema_version
        )]);
    }
    let manifest_relative = PathBuf::from(&pointer.current_manifest);
    repository_path_string(&manifest_relative)?;
    let manifest = load_manifest(&options.repo_root, &manifest_relative, &options.schema)?;

    let mut lines = vec![format!("OK schema {}", manifest_relative.display())];
    let mut advisories = Vec::new();
    let expected_digest = corpus_set_digest(&manifest)?;
    if expected_digest != manifest.corpus_set_digest {
        advisories.push(format!(
            "corpus_set_digest mismatch: recorded={} computed={expected_digest}",
            manifest.corpus_set_digest
        ));
    }

    for entry in &manifest.included {
        let path = resolve(&options.repo_root, Path::new(&entry.path));
        match fs::read(&path) {
            Ok(bytes) => {
                let actual = sha256(&bytes);
                if actual != entry.sha256 {
                    advisories.push(format!(
                        "included path digest mismatch: {} recorded={} actual={actual}",
                        entry.path, entry.sha256
                    ));
                }
            }
            Err(error) => advisories.push(format!(
                "included path missing or unreadable: {} ({error})",
                entry.path
            )),
        }
    }

    let candidates = candidate_paths_live(&options.repo_root)?;
    let accounted = accounted_paths(&manifest);
    for path in candidates.difference(&accounted) {
        advisories.push(format!(
            "candidate lacks included/excluded disposition: {path}"
        ));
    }
    if let Some(base_ref) = options.base_ref.as_deref() {
        advisories.extend(numbered_manifest_mutations(&options.repo_root, base_ref)?);
    }

    if advisories.is_empty() {
        lines.push(format!(
            "OK current-tree advisory check: included={} excluded={} candidates={}",
            manifest.included.len(),
            manifest.excluded.len(),
            candidates.len()
        ));
    } else {
        lines.extend(
            advisories
                .into_iter()
                .map(|finding| format!("ADVISORY {finding}")),
        );
    }
    Ok(CommandReport { lines })
}

pub fn verify_version(options: VerifyOptions) -> Result<CommandReport, Vec<String>> {
    let manifest = load_manifest(&options.repo_root, &options.manifest, &options.schema)?;
    validate_exact_commit(
        &manifest.corpus_content_revision,
        "manifest corpus_content_revision",
    )?;
    let mut errors = Vec::new();
    match corpus_set_digest(&manifest) {
        Ok(actual) if actual == manifest.corpus_set_digest => {}
        Ok(actual) => errors.push(format!(
            "corpus_set_digest mismatch: recorded={} computed={actual}",
            manifest.corpus_set_digest
        )),
        Err(mut digest_errors) => errors.append(&mut digest_errors),
    }
    for entry in &manifest.included {
        match sha256_git_path(
            &options.repo_root,
            &manifest.corpus_content_revision,
            &entry.path,
        ) {
            Ok(actual) if actual == entry.sha256 => {}
            Ok(actual) => errors.push(format!(
                "historical digest mismatch at {}: recorded={} actual={actual}",
                entry.path, entry.sha256
            )),
            Err(mut path_errors) => errors.append(&mut path_errors),
        }
    }
    match candidate_paths_at_revision(&options.repo_root, &manifest.corpus_content_revision) {
        Ok(candidates) => {
            let accounted = accounted_paths(&manifest);
            for path in candidates.difference(&accounted) {
                errors.push(format!(
                    "historical candidate lacks included/excluded disposition: {path}"
                ));
            }
        }
        Err(mut candidate_errors) => errors.append(&mut candidate_errors),
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(CommandReport {
        lines: vec![format!(
            "OK verified {} against corpus_content_revision {}",
            options.manifest.display(),
            manifest.corpus_content_revision
        )],
    })
}

#[derive(Debug)]
struct InventoryRow {
    path: String,
    classification: String,
}

fn parse_inventory(bytes: &[u8]) -> Result<Vec<InventoryRow>, Vec<String>> {
    let mut reader = csv::ReaderBuilder::new().flexible(false).from_reader(bytes);
    let headers = reader
        .headers()
        .map_err(|error| vec![format!("inventory CSV header: {error}")])?
        .clone();
    let path_index = headers
        .iter()
        .position(|field| field == "path_or_portion")
        .ok_or_else(|| vec!["inventory CSV lacks path_or_portion".to_string()])?;
    let classification_index = headers
        .iter()
        .position(|field| field == "classification")
        .ok_or_else(|| vec!["inventory CSV lacks classification".to_string()])?;
    let mut rows = Vec::new();
    let mut seen = BTreeSet::new();
    for (offset, record) in reader.records().enumerate() {
        let record =
            record.map_err(|error| vec![format!("inventory CSV row {}: {error}", offset + 2)])?;
        let path = record.get(path_index).unwrap_or_default().to_string();
        validate_repository_path(&path)?;
        if !seen.insert(path.clone()) {
            return Err(vec![format!("inventory path is duplicated: {path}")]);
        }
        rows.push(InventoryRow {
            path,
            classification: record
                .get(classification_index)
                .unwrap_or_default()
                .to_string(),
        });
    }
    if rows.is_empty() {
        return Err(vec!["inventory CSV has no data rows".to_string()]);
    }
    Ok(rows)
}

fn load_manifest(
    repo_root: &Path,
    manifest_path: &Path,
    schema_path: &Path,
) -> Result<CommercialCorpusManifest, Vec<String>> {
    repository_path_string(manifest_path)?;
    let path = resolve(repo_root, manifest_path);
    let content =
        fs::read_to_string(&path).map_err(|error| vec![format!("{}: {error}", path.display())])?;
    let value: Value = serde_norway::from_str(&content)
        .map_err(|error| vec![format!("{}: {error}", path.display())])?;
    validate_value_against_schema(repo_root, schema_path, &value)?;
    let manifest: CommercialCorpusManifest = serde_json::from_value(value)
        .map_err(|error| vec![format!("{}: {error}", path.display())])?;
    structural_errors(&manifest)?;
    Ok(manifest)
}

fn validate_against_schema(
    repo_root: &Path,
    schema_path: &Path,
    manifest: &CommercialCorpusManifest,
) -> Result<(), Vec<String>> {
    let value = serde_json::to_value(manifest)
        .map_err(|error| vec![format!("manifest serialization: {error}")])?;
    validate_value_against_schema(repo_root, schema_path, &value)
}

fn validate_value_against_schema(
    repo_root: &Path,
    schema_path: &Path,
    value: &Value,
) -> Result<(), Vec<String>> {
    let schema_file = resolve(repo_root, schema_path);
    let schema = agov_core::schema::load_and_compile(&schema_file)
        .map_err(|error| vec![format!("{}: {error}", schema_file.display())])?;
    let findings = agov_core::schema::validate(&schema, value);
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings
            .into_iter()
            .map(|finding| format!("schema {}: {}", finding.pointer, finding.message))
            .collect())
    }
}

fn structural_errors(manifest: &CommercialCorpusManifest) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    if manifest.schema_version != 1 {
        errors.push(format!(
            "unsupported manifest schema_version {}",
            manifest.schema_version
        ));
    }
    if manifest.corpus_version == 0 {
        errors.push("corpus_version must be at least 1".to_string());
    }
    if !manifest.corpus_set_digest.is_empty() && !is_lower_hex(&manifest.corpus_set_digest, 64) {
        errors.push("corpus_set_digest must be 64 lowercase hexadecimal characters".to_string());
    }
    let mut seen = BTreeSet::new();
    for path in manifest
        .included
        .iter()
        .map(|entry| entry.path.as_str())
        .chain(manifest.excluded.iter().map(|entry| entry.path.as_str()))
    {
        if let Err(mut path_errors) = validate_repository_path(path) {
            errors.append(&mut path_errors);
        }
        if !seen.insert(path.to_string()) {
            errors.push(format!("manifest path is duplicated: {path}"));
        }
    }
    for entry in &manifest.included {
        if !is_lower_hex(&entry.sha256, 64) {
            errors.push(format!("included sha256 is invalid: {}", entry.path));
        }
    }
    for entry in &manifest.excluded {
        match entry.source.as_str() {
            "inventory" => {
                if entry.decision.is_some() {
                    errors.push(format!(
                        "inventory exclusion must not carry a post-inventory decision: {}",
                        entry.path
                    ));
                }
            }
            "post-inventory-governor-decision" => {
                if entry.decision.as_deref().unwrap_or_default().is_empty() {
                    errors.push(format!(
                        "post-inventory exclusion lacks decision: {}",
                        entry.path
                    ));
                }
            }
            other => errors.push(format!(
                "excluded source {other:?} is unsupported: {}",
                entry.path
            )),
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn corpus_set_digest(manifest: &CommercialCorpusManifest) -> Result<String, Vec<String>> {
    let mut value = serde_json::to_value(manifest)
        .map_err(|error| vec![format!("manifest serialization: {error}")])?;
    value
        .as_object_mut()
        .ok_or_else(|| vec!["manifest projection is not an object".to_string()])?
        .remove("corpus_set_digest");
    let mut bytes = Vec::new();
    write_canonical_json(&value, &mut bytes)?;
    Ok(sha256(&bytes))
}

fn write_canonical_json(value: &Value, output: &mut Vec<u8>) -> Result<(), Vec<String>> {
    match value {
        Value::Null => output.extend_from_slice(b"null"),
        Value::Bool(true) => output.extend_from_slice(b"true"),
        Value::Bool(false) => output.extend_from_slice(b"false"),
        Value::Number(number) => output.extend_from_slice(number.to_string().as_bytes()),
        Value::String(string) => {
            let encoded = serde_json::to_string(string)
                .map_err(|error| vec![format!("canonical JSON string: {error}")])?;
            output.extend_from_slice(encoded.as_bytes());
        }
        Value::Array(values) => {
            output.push(b'[');
            for (index, item) in values.iter().enumerate() {
                if index > 0 {
                    output.push(b',');
                }
                write_canonical_json(item, output)?;
            }
            output.push(b']');
        }
        Value::Object(map) => {
            output.push(b'{');
            let ordered: BTreeMap<_, _> = map.iter().collect();
            for (index, (key, item)) in ordered.into_iter().enumerate() {
                if index > 0 {
                    output.push(b',');
                }
                let encoded = serde_json::to_string(key)
                    .map_err(|error| vec![format!("canonical JSON key: {error}")])?;
                output.extend_from_slice(encoded.as_bytes());
                output.push(b':');
                write_canonical_json(item, output)?;
            }
            output.push(b'}');
        }
    }
    Ok(())
}

fn candidate_paths_at_revision(
    repo_root: &Path,
    revision: &str,
) -> Result<BTreeSet<String>, Vec<String>> {
    validate_exact_commit(revision, "candidate revision")?;
    let tree = git_output(repo_root, &["ls-tree", "-r", "-z", "--full-tree", revision])?;
    let mut candidates = BTreeSet::new();
    for record in tree
        .split(|byte| *byte == 0)
        .filter(|record| !record.is_empty())
    {
        let Some(tab) = record.iter().position(|byte| *byte == b'\t') else {
            return Err(vec!["git ls-tree returned an unparsable record".to_string()]);
        };
        let metadata = String::from_utf8_lossy(&record[..tab]);
        let mut fields = metadata.split_whitespace();
        let mode = fields.next().unwrap_or_default();
        let kind = fields.next().unwrap_or_default();
        if mode == "120000" || kind != "blob" {
            continue;
        }
        let path = String::from_utf8(record[tab + 1..].to_vec())
            .map_err(|error| vec![format!("non-UTF-8 repository path: {error}")])?;
        validate_repository_path(&path)?;
        let bytes = git_output(repo_root, &["show", &format!("{revision}:{path}")])?;
        if first_fifteen_lines_contain_scope(&bytes) {
            candidates.insert(path);
        }
    }
    Ok(candidates)
}

fn candidate_paths_live(repo_root: &Path) -> Result<BTreeSet<String>, Vec<String>> {
    let index = git_output(repo_root, &["ls-files", "-s", "-z"])?;
    let mut candidates = BTreeSet::new();
    for record in index
        .split(|byte| *byte == 0)
        .filter(|record| !record.is_empty())
    {
        let Some(tab) = record.iter().position(|byte| *byte == b'\t') else {
            return Err(vec![
                "git ls-files returned an unparsable record".to_string()
            ]);
        };
        let metadata = String::from_utf8_lossy(&record[..tab]);
        let mode = metadata.split_whitespace().next().unwrap_or_default();
        if mode == "120000" {
            continue;
        }
        let path = String::from_utf8(record[tab + 1..].to_vec())
            .map_err(|error| vec![format!("non-UTF-8 repository path: {error}")])?;
        validate_repository_path(&path)?;
        let full_path = repo_root.join(&path);
        let bytes = match fs::read(&full_path) {
            Ok(bytes) => bytes,
            Err(_) => continue,
        };
        if first_fifteen_lines_contain_scope(&bytes) {
            candidates.insert(path);
        }
    }
    Ok(candidates)
}

fn first_fifteen_lines_contain_scope(bytes: &[u8]) -> bool {
    bytes.split(|byte| *byte == b'\n').take(15).any(|line| {
        line.windows(SCOPE_EXPRESSION.len())
            .any(|window| window == SCOPE_EXPRESSION)
    })
}

fn numbered_manifest_mutations(
    repo_root: &Path,
    base_ref: &str,
) -> Result<Vec<String>, Vec<String>> {
    if base_ref.trim().is_empty() {
        return Err(vec!["base ref must not be empty".to_string()]);
    }
    let range = format!("{base_ref}...HEAD");
    let diff = git_output(
        repo_root,
        &[
            "diff",
            "--name-status",
            "-z",
            &range,
            "--",
            "LICENSES/commercial-corpus-manifest-v*.yaml",
        ],
    )?;
    let fields: Vec<_> = diff
        .split(|byte| *byte == 0)
        .filter(|field| !field.is_empty())
        .map(|field| String::from_utf8_lossy(field).to_string())
        .collect();
    let mut findings = Vec::new();
    let mut index = 0;
    while index < fields.len() {
        let status = &fields[index];
        index += 1;
        let Some(first_path) = fields.get(index) else {
            return Err(vec![
                "git diff returned an incomplete name-status record".to_string()
            ]);
        };
        index += 1;
        let second_path = if status.starts_with('R') || status.starts_with('C') {
            let Some(path) = fields.get(index) else {
                return Err(vec![
                    "git diff returned an incomplete rename record".to_string()
                ]);
            };
            index += 1;
            Some(path)
        } else {
            None
        };
        if !status.starts_with('A')
            && (is_numbered_manifest(first_path)
                || second_path.is_some_and(|path| is_numbered_manifest(path)))
        {
            findings.push(format!(
                "numbered manifest path changed in {range}: status={status} path={first_path}{}",
                second_path
                    .map(|path| format!(" new_path={path}"))
                    .unwrap_or_default()
            ));
        }
    }
    Ok(findings)
}

fn is_numbered_manifest(path: &str) -> bool {
    let prefix = "LICENSES/commercial-corpus-manifest-v";
    path.strip_prefix(prefix)
        .and_then(|rest| rest.strip_suffix(".yaml"))
        .is_some_and(|version| {
            !version.is_empty() && version.bytes().all(|byte| byte.is_ascii_digit())
        })
}

fn accounted_paths(manifest: &CommercialCorpusManifest) -> BTreeSet<String> {
    manifest
        .included
        .iter()
        .map(|entry| entry.path.clone())
        .chain(manifest.excluded.iter().map(|entry| entry.path.clone()))
        .collect()
}

fn sha256_git_path(repo_root: &Path, revision: &str, path: &str) -> Result<String, Vec<String>> {
    validate_repository_path(path)?;
    let bytes = git_output(repo_root, &["show", &format!("{revision}:{path}")])?;
    Ok(sha256(&bytes))
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn git_output(repo_root: &Path, args: &[&str]) -> Result<Vec<u8>, Vec<String>> {
    let output = Command::new("git")
        .args(args)
        .current_dir(repo_root)
        .output()
        .map_err(|error| vec![format!("git {}: {error}", args.join(" "))])?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(vec![format!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        )])
    }
}

fn validate_exact_commit(value: &str, label: &str) -> Result<(), Vec<String>> {
    if is_lower_hex(value, 40) {
        Ok(())
    } else {
        Err(vec![format!(
            "{label} must be an exact 40-character lowercase hexadecimal commit SHA"
        )])
    }
}

fn is_lower_hex(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn validate_repository_path(path: &str) -> Result<(), Vec<String>> {
    if path.is_empty() || path.contains('\\') {
        return Err(vec![format!("invalid repository-relative path: {path:?}")]);
    }
    let parsed = Path::new(path);
    if parsed.is_absolute()
        || parsed.components().any(|component| {
            matches!(
                component,
                Component::ParentDir
                    | Component::CurDir
                    | Component::RootDir
                    | Component::Prefix(_)
            )
        })
    {
        return Err(vec![format!("invalid repository-relative path: {path:?}")]);
    }
    Ok(())
}

fn repository_path_string(path: &Path) -> Result<String, Vec<String>> {
    let value = path
        .to_str()
        .ok_or_else(|| vec![format!("non-UTF-8 repository path: {}", path.display())])?
        .to_string();
    validate_repository_path(&value)?;
    Ok(value)
}

fn resolve(repo_root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        repo_root.join(path)
    }
}

fn write_yaml<T: Serialize>(path: &Path, value: &T) -> Result<(), Vec<String>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| vec![format!("{}: {error}", parent.display())])?;
    }
    let yaml = serde_norway::to_string(value)
        .map_err(|error| vec![format!("{}: {error}", path.display())])?;
    fs::write(path, yaml).map_err(|error| vec![format!("{}: {error}", path.display())])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn schema_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../../../agf/schemas/commercial-corpus-manifest.schema.json")
            .canonicalize()
            .unwrap()
    }

    fn git(repo: &Path, args: &[&str]) -> String {
        let output = Command::new("git")
            .args(args)
            .current_dir(repo)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }

    fn init_repo(repo: &Path) {
        git(repo, &["init", "-q"]);
        git(repo, &["config", "user.name", "Test"]);
        git(repo, &["config", "user.email", "test@example.invalid"]);
    }

    fn commit_all(repo: &Path, message: &str) -> String {
        git(repo, &["add", "."]);
        git(repo, &["commit", "-q", "-m", message]);
        git(repo, &["rev-parse", "HEAD"])
    }

    fn write_test_manifest(repo: &Path, revision: &str, bytes: &[u8]) {
        fs::create_dir_all(repo.join("LICENSES")).unwrap();
        let mut manifest = CommercialCorpusManifest {
            schema_version: 1,
            corpus_version: 1,
            corpus_content_revision: revision.to_string(),
            provenance: Provenance {
                source_inventory_commit: "b".repeat(40),
                cl3_decision: CL3_DECISION.to_string(),
            },
            source_inventory: SourceInventory {
                path: "inventory.csv".to_string(),
            },
            included: vec![IncludedEntry {
                path: "agf/a.md".to_string(),
                sha256: sha256(bytes),
            }],
            excluded: Vec::new(),
            corpus_set_digest: String::new(),
        };
        manifest.corpus_set_digest = corpus_set_digest(&manifest).unwrap();
        write_yaml(
            &repo.join("LICENSES/commercial-corpus-manifest-v1.yaml"),
            &manifest,
        )
        .unwrap();
        write_yaml(
            &repo.join("LICENSES/commercial-corpus-manifest-current.yaml"),
            &CurrentPointer {
                schema_version: 1,
                current_manifest: "LICENSES/commercial-corpus-manifest-v1.yaml".to_string(),
            },
        )
        .unwrap();
    }

    #[test]
    fn scope_expression_must_be_within_first_fifteen_lines() {
        let mut line_15 = (1..15)
            .map(|number| format!("line {number}\n"))
            .collect::<String>();
        line_15.push_str("CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial\n");
        assert!(first_fifteen_lines_contain_scope(line_15.as_bytes()));

        let mut line_16 = (1..16)
            .map(|number| format!("line {number}\n"))
            .collect::<String>();
        line_16.push_str("CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial\n");
        assert!(!first_fifteen_lines_contain_scope(line_16.as_bytes()));
    }

    #[test]
    fn canonical_json_sorts_object_keys_but_preserves_array_order() {
        let value = serde_json::json!({"z": 1, "a": {"y": 2, "b": [3, 1]}});
        let mut bytes = Vec::new();
        write_canonical_json(&value, &mut bytes).unwrap();
        assert_eq!(
            String::from_utf8(bytes).unwrap(),
            r#"{"a":{"b":[3,1],"y":2},"z":1}"#
        );
    }

    #[test]
    fn numbered_manifest_pattern_excludes_mutable_pointer() {
        assert!(is_numbered_manifest(
            "LICENSES/commercial-corpus-manifest-v12.yaml"
        ));
        assert!(!is_numbered_manifest(
            "LICENSES/commercial-corpus-manifest-current.yaml"
        ));
    }

    #[test]
    fn structural_check_rejects_duplicate_paths_across_lists() {
        let manifest = CommercialCorpusManifest {
            schema_version: 1,
            corpus_version: 1,
            corpus_content_revision: "a".repeat(40),
            provenance: Provenance {
                source_inventory_commit: "b".repeat(40),
                cl3_decision: CL3_DECISION.to_string(),
            },
            source_inventory: SourceInventory {
                path: "inventory.csv".to_string(),
            },
            included: vec![IncludedEntry {
                path: "agf/a.md".to_string(),
                sha256: "c".repeat(64),
            }],
            excluded: vec![ExcludedEntry {
                path: "agf/a.md".to_string(),
                reason: "test".to_string(),
                source: "inventory".to_string(),
                decision: None,
            }],
            corpus_set_digest: "d".repeat(64),
        };
        let errors = structural_errors(&manifest).unwrap_err();
        assert!(errors.iter().any(|error| error.contains("duplicated")));
    }

    #[test]
    fn missing_required_schema_field_is_blocking() {
        let temp = tempfile::tempdir().unwrap();
        fs::create_dir_all(temp.path().join("LICENSES")).unwrap();
        fs::write(
            temp.path()
                .join("LICENSES/commercial-corpus-manifest-current.yaml"),
            "schema_version: 1\ncurrent_manifest: LICENSES/commercial-corpus-manifest-v1.yaml\n",
        )
        .unwrap();
        fs::write(
            temp.path()
                .join("LICENSES/commercial-corpus-manifest-v1.yaml"),
            "schema_version: 1\n",
        )
        .unwrap();
        let errors = validate_current(ValidateOptions {
            repo_root: temp.path().to_path_buf(),
            pointer: PathBuf::from("LICENSES/commercial-corpus-manifest-current.yaml"),
            schema: schema_path(),
            base_ref: None,
        })
        .unwrap_err();
        assert!(errors.iter().any(|error| error.contains("Required")));
    }

    #[test]
    fn current_tree_mismatch_and_unaccounted_candidate_are_advisory() {
        let temp = tempfile::tempdir().unwrap();
        init_repo(temp.path());
        fs::create_dir_all(temp.path().join("agf")).unwrap();
        let original = b"CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial\noriginal\n";
        fs::write(temp.path().join("agf/a.md"), original).unwrap();
        let revision = commit_all(temp.path(), "base");
        write_test_manifest(temp.path(), &revision, original);
        fs::write(
            temp.path().join("agf/a.md"),
            b"CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial\nchanged\n",
        )
        .unwrap();
        fs::write(
            temp.path().join("agf/new.md"),
            b"CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial\nnew\n",
        )
        .unwrap();
        git(temp.path(), &["add", "agf/new.md"]);

        let report = validate_current(ValidateOptions {
            repo_root: temp.path().to_path_buf(),
            pointer: PathBuf::from("LICENSES/commercial-corpus-manifest-current.yaml"),
            schema: schema_path(),
            base_ref: None,
        })
        .unwrap();
        assert!(report
            .lines
            .iter()
            .any(|line| line.contains("ADVISORY included path digest mismatch")));
        assert!(report
            .lines
            .iter()
            .any(|line| line.contains("ADVISORY candidate lacks") && line.contains("agf/new.md")));
    }

    #[test]
    fn missing_included_path_is_advisory() {
        let temp = tempfile::tempdir().unwrap();
        init_repo(temp.path());
        fs::create_dir_all(temp.path().join("agf")).unwrap();
        let original = b"CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial\noriginal\n";
        fs::write(temp.path().join("agf/a.md"), original).unwrap();
        let revision = commit_all(temp.path(), "base");
        write_test_manifest(temp.path(), &revision, original);
        fs::remove_file(temp.path().join("agf/a.md")).unwrap();

        let report = validate_current(ValidateOptions {
            repo_root: temp.path().to_path_buf(),
            pointer: PathBuf::from("LICENSES/commercial-corpus-manifest-current.yaml"),
            schema: schema_path(),
            base_ref: None,
        })
        .unwrap();
        assert!(report.lines.iter().any(|line| {
            line.contains("ADVISORY included path missing or unreadable")
                && line.contains("agf/a.md")
        }));
    }

    #[test]
    fn historical_verification_uses_recorded_revision_not_live_tree() {
        let temp = tempfile::tempdir().unwrap();
        init_repo(temp.path());
        fs::create_dir_all(temp.path().join("agf")).unwrap();
        let original = b"CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial\noriginal\n";
        fs::write(temp.path().join("agf/a.md"), original).unwrap();
        let revision = commit_all(temp.path(), "base");
        write_test_manifest(temp.path(), &revision, original);
        fs::write(
            temp.path().join("agf/a.md"),
            b"CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial\nlater\n",
        )
        .unwrap();
        commit_all(temp.path(), "later");

        let report = verify_version(VerifyOptions {
            repo_root: temp.path().to_path_buf(),
            manifest: PathBuf::from("LICENSES/commercial-corpus-manifest-v1.yaml"),
            schema: schema_path(),
        })
        .unwrap();
        assert!(report.lines[0].starts_with("OK verified"));
    }

    #[test]
    fn v1_remains_historically_valid_while_current_tree_validation_uses_v2() {
        let temp = tempfile::tempdir().unwrap();
        init_repo(temp.path());
        fs::create_dir_all(temp.path().join("agf")).unwrap();
        let v1_bytes = b"CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial\nv1\n";
        fs::write(temp.path().join("agf/a.md"), v1_bytes).unwrap();
        let v1_revision = commit_all(temp.path(), "v1");
        write_test_manifest(temp.path(), &v1_revision, v1_bytes);

        let v2_bytes = b"CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial\nv2\n";
        fs::write(temp.path().join("agf/a.md"), v2_bytes).unwrap();
        let v2_revision = commit_all(temp.path(), "v2");
        let mut v2_manifest = CommercialCorpusManifest {
            schema_version: 1,
            corpus_version: 2,
            corpus_content_revision: v2_revision,
            provenance: Provenance {
                source_inventory_commit: "b".repeat(40),
                cl3_decision: CL3_DECISION.to_string(),
            },
            source_inventory: SourceInventory {
                path: "inventory.csv".to_string(),
            },
            included: vec![IncludedEntry {
                path: "agf/a.md".to_string(),
                sha256: sha256(v2_bytes),
            }],
            excluded: Vec::new(),
            corpus_set_digest: String::new(),
        };
        v2_manifest.corpus_set_digest = corpus_set_digest(&v2_manifest).unwrap();
        write_yaml(
            &temp
                .path()
                .join("LICENSES/commercial-corpus-manifest-v2.yaml"),
            &v2_manifest,
        )
        .unwrap();
        write_yaml(
            &temp
                .path()
                .join("LICENSES/commercial-corpus-manifest-current.yaml"),
            &CurrentPointer {
                schema_version: 1,
                current_manifest: "LICENSES/commercial-corpus-manifest-v2.yaml".to_string(),
            },
        )
        .unwrap();

        let historical = verify_version(VerifyOptions {
            repo_root: temp.path().to_path_buf(),
            manifest: PathBuf::from("LICENSES/commercial-corpus-manifest-v1.yaml"),
            schema: schema_path(),
        })
        .unwrap();
        assert!(historical.lines[0].starts_with("OK verified"));

        let current = validate_current(ValidateOptions {
            repo_root: temp.path().to_path_buf(),
            pointer: PathBuf::from("LICENSES/commercial-corpus-manifest-current.yaml"),
            schema: schema_path(),
            base_ref: None,
        })
        .unwrap();
        assert!(current.lines.iter().any(
            |line| line == "OK current-tree advisory check: included=1 excluded=0 candidates=1"
        ));
        assert!(!current
            .lines
            .iter()
            .any(|line| line.starts_with("ADVISORY")));
    }

    #[test]
    fn real_mr1712_generation_acceptance_is_142_included_plus_50_excluded() {
        const CORPUS_REVISION: &str = "48049759dfd9c681863d510ac6f7971235c5b0fb";
        const INVENTORY_REVISION: &str = "d13bb9084fce9a9b1c7c08bb735e0fe6d08d7e98";
        const INVENTORY_PATH: &str = "docs/reviews/commercial-licensing/20260817T224900Z-rights-provenance-inventory/inventory.csv";
        const AGREEMENT_TEMPLATE: &str = "LICENSES/agf-commercial-agreement-template.md";
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../../..")
            .canonicalize()
            .unwrap();
        let inventory =
            include_bytes!("../tests/fixtures/commercial-corpus/inventory-path-classification.csv");

        let (manifest, candidates) = build_manifest(
            &repo_root,
            inventory,
            INVENTORY_REVISION,
            INVENTORY_PATH,
            CORPUS_REVISION,
        )
        .unwrap();

        assert_eq!(manifest.corpus_content_revision, CORPUS_REVISION);
        assert_eq!(manifest.included.len(), 142);
        assert_eq!(manifest.excluded.len(), 50);
        assert_eq!(
            manifest
                .excluded
                .iter()
                .filter(|entry| entry.source == "inventory")
                .count(),
            49
        );
        assert_eq!(
            manifest
                .excluded
                .iter()
                .filter(|entry| entry.source == "post-inventory-governor-decision")
                .count(),
            1
        );
        assert_eq!(candidates.len(), 192);
        assert_eq!(accounted_paths(&manifest), candidates);
        assert!(!candidates.contains(AGREEMENT_TEMPLATE));
    }

    #[test]
    fn numbered_manifest_mutation_is_advisory_evidence() {
        let temp = tempfile::tempdir().unwrap();
        init_repo(temp.path());
        fs::create_dir_all(temp.path().join("LICENSES")).unwrap();
        fs::write(
            temp.path()
                .join("LICENSES/commercial-corpus-manifest-v1.yaml"),
            "version: 1\n",
        )
        .unwrap();
        let base = commit_all(temp.path(), "base");
        fs::write(
            temp.path()
                .join("LICENSES/commercial-corpus-manifest-v1.yaml"),
            "version: changed\n",
        )
        .unwrap();
        commit_all(temp.path(), "mutate");

        let findings = numbered_manifest_mutations(temp.path(), &base).unwrap();
        assert_eq!(findings.len(), 1);
        assert!(findings[0].contains("status=M"));
    }
}
