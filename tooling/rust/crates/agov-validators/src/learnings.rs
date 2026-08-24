use crate::{split_results, validate_schema, yaml_files, CollectionResult};
use agov_core::Finding;
use regex::Regex;
use std::path::{Path, PathBuf};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

const CONFIDENCE_LEVELS: &[&str] = &[
    "single-adopter",
    "multi-adopter",
    "longitudinal",
    "industry-corroborated",
];
const CORROBORATED_LEVELS: &[&str] = &["multi-adopter", "longitudinal"];
const HISTORICAL_ID: &str = r#"^L\d{3}$"#;
const BARE_TIMESTAMP_ID: &str = r#"^\d{8}T\d{6}Z$"#;
const SUFFIXED_TIMESTAMP_ID: &str = r#"^\d{8}T\d{6}Z-[a-z0-9]{4}$"#;

/// Bound on how long an `upgrade_in_progress` marker suppresses the
/// version-drift check. Steps 1 and 2 of `agf/skills/upgrade/SKILL.md`
/// both have explicit user-confirmation gates that can pause indefinitely,
/// but 24 hours from Step 3's actual start is well past any single working
/// session -- see the plan's "Mid-upgrade transient window" design.
const UPGRADE_WINDOW_HOURS: i64 = 24;

/// Validate all learning record YAML files in `dir`.
///
/// Mirrors `Agov.ValidateLearnings.validate_all/1`. Each file is validated
/// against `learning.schema.json`. Also runs the corpus-wide
/// `checkout_role: live` version-drift check (see `corpus_errors`) --
/// `agf/VERSION` is read as `dir`'s sibling, i.e. `dir` is assumed to be
/// `<repo_root>/agf/learnings`, matching `agov_core::learnings_dir()`.
pub fn validate_all(dir: &Path) -> CollectionResult {
    let mut results = validate_directory_files(dir);
    apply_corpus_checks(&mut results, dir);
    split_results(results)
}

/// Validate learning records from explicit files or directories.
///
/// Corpus-wide checks (see `corpus_errors`) apply per directory argument --
/// a directory's full file set is corpus-aware (matching `validate_all`'s
/// behavior for that same directory), but an individual explicit file
/// argument is validated on its own, without corpus context, since the
/// caller named a narrow subset rather than "the whole learnings corpus."
pub fn validate_paths(paths: &[PathBuf]) -> CollectionResult {
    let mut results = Vec::new();
    for path in paths {
        if path.is_dir() {
            let mut dir_results = validate_directory_files(path);
            apply_corpus_checks(&mut dir_results, path);
            results.extend(dir_results);
        } else {
            let result = validate_file(path);
            results.push((path.clone(), result));
        }
    }

    split_results(results)
}

fn validate_directory_files(
    dir: &Path,
) -> Vec<(PathBuf, Result<serde_json::Value, Vec<Finding>>)> {
    learning_files(dir)
        .into_iter()
        .map(|file| {
            let result = validate_file(&file);
            (file, result)
        })
        .collect()
}

fn apply_corpus_checks(
    results: &mut Vec<(PathBuf, Result<serde_json::Value, Vec<Finding>>)>,
    dir: &Path,
) {
    // `validate_file` returns `Err` whenever a file has *any* finding, not
    // only when the file itself is unreadable -- a record with a purely
    // advisory finding elsewhere (e.g. a bare-timestamp learning id) still
    // returns `Err`, discarding the parsed data. The corpus check only
    // needs to distrust a file that genuinely failed to parse; re-read any
    // `Err` file directly for corpus purposes, using the re-read value only
    // if that succeeds.
    let corpus_data: Vec<(PathBuf, serde_json::Value)> = results
        .iter()
        .filter_map(|(path, result)| match result {
            Ok(data) => Some((path.clone(), data.clone())),
            Err(_) => crate::read_yaml_file(path)
                .ok()
                .map(|data| (path.clone(), data)),
        })
        .collect();

    let agf_version_path = dir.parent().map(|agf_dir| agf_dir.join("VERSION"));
    let corpus_findings = corpus_errors(&corpus_data, agf_version_path.as_deref());
    merge_corpus_findings(results, dir, corpus_findings);
}

/// Corpus-wide `checkout_role: live` version-drift check, per the plan's
/// "Deterministic live-record selection and migration" design:
///
/// - Zero `checkout_role: live` records: always-on advisory finding,
///   attached to the directory itself (never blocks).
/// - Two or more `checkout_role: live` records: blocking ambiguity
///   finding, naming every live record file, attached to the directory
///   itself.
/// - Exactly one `checkout_role: live` record: compare its
///   `adoption.framework_version` against `agf/VERSION`, honoring the
///   `upgrade_in_progress` suppression window (see `live_record_findings`).
///
/// Only files that could actually be read and parsed participate -- a
/// record with other, unrelated advisory findings (e.g. a bare-timestamp
/// learning id) still participates, since those findings say nothing about
/// whether its `adoption` block can be trusted. A file that fails to parse
/// at all is excluded; its own schema/parse errors are reported separately.
fn corpus_errors(
    corpus_data: &[(PathBuf, serde_json::Value)],
    agf_version_path: Option<&Path>,
) -> Vec<(Option<PathBuf>, Finding)> {
    let live: Vec<(&PathBuf, &serde_json::Value)> = corpus_data
        .iter()
        .map(|(path, data)| (path, data))
        .filter(|(_, data)| {
            data.pointer("/adoption/checkout_role")
                .and_then(serde_json::Value::as_str)
                == Some("live")
        })
        .collect();

    match live.as_slice() {
        [] => vec![(
            None,
            Finding::advisory(
                "no live-marked adoption record found; version-drift detection is inactive \
                 for this checkout -- add checkout_role: live to your own learning record \
                 (see agf/docs/site/docs/adopt/install-and-layouts.md, Step 7)",
            ),
        )],
        [(path, data)] => live_record_findings(data, agf_version_path)
            .into_iter()
            .map(|finding| (Some((*path).clone()), finding))
            .collect(),
        many => {
            let names = many
                .iter()
                .map(|(path, _)| path.display().to_string())
                .collect::<Vec<_>>()
                .join(", ");
            vec![(
                None,
                Finding::blocking(format!(
                    "multiple checkout_role: live records found ({names}); exactly one \
                     record may be live at a time"
                )),
            )]
        }
    }
}

/// Version-drift comparison for the single identified `checkout_role: live`
/// record, honoring the `upgrade_in_progress` suppression window.
fn live_record_findings(
    data: &serde_json::Value,
    agf_version_path: Option<&Path>,
) -> Vec<Finding> {
    let mut findings = Vec::new();

    let recorded = data
        .pointer("/adoption/framework_version")
        .and_then(serde_json::Value::as_str);
    let Some(recorded) = recorded else {
        findings.push(Finding::blocking_at(
            "adoption.framework_version is required when adoption.checkout_role is live",
            "adoption/framework_version",
        ));
        return findings;
    };

    let Some(agf_version_path) = agf_version_path else {
        findings.push(Finding::blocking_at(
            "could not determine agf/VERSION path for this checkout; version-drift \
             detection cannot run",
            "adoption/framework_version",
        ));
        return findings;
    };
    let Some(current) = read_agf_version(agf_version_path) else {
        findings.push(Finding::blocking_at(
            format!(
                "could not read {} to compare against adoption.framework_version; \
                 version-drift detection cannot run",
                agf_version_path.display()
            ),
            "adoption/framework_version",
        ));
        return findings;
    };

    let in_progress = data
        .pointer("/adoption/upgrade_in_progress")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let started_at_raw = data
        .pointer("/adoption/upgrade_started_at")
        .and_then(serde_json::Value::as_str);
    let started_at = started_at_raw.and_then(|value| OffsetDateTime::parse(value, &Rfc3339).ok());

    // Only a marker with a valid, non-future timestamp inside the bounded
    // window suppresses the mismatch. A missing/invalid/future timestamp on
    // an in-progress marker is malformed -- it must never suppress, and is
    // surfaced as its own advisory so it doesn't fail silently.
    let elapsed_hours = started_at.map(|started| (OffsetDateTime::now_utc() - started).whole_hours());
    let malformed_marker = in_progress && (started_at_raw.is_none() || started_at.is_none());
    let future_marker = in_progress && elapsed_hours.is_some_and(|hours| hours < 0);
    let within_window =
        in_progress && elapsed_hours.is_some_and(|hours| (0..UPGRADE_WINDOW_HOURS).contains(&hours));
    let stale_marker = in_progress && elapsed_hours.is_some_and(|hours| hours >= UPGRADE_WINDOW_HOURS);

    if recorded != current && !within_window {
        findings.push(Finding::blocking_at(
            format!(
                "adoption.framework_version {recorded:?} does not match agf/VERSION \
                 {current:?}"
            ),
            "adoption/framework_version",
        ));
    }

    if stale_marker {
        findings.push(Finding::advisory(
            "stale upgrade_in_progress marker -- upgrade did not complete Step 4 within the \
             expected window; verify manually and clear the marker",
        ));
    } else if future_marker {
        findings.push(Finding::advisory(
            "malformed upgrade_started_at marker -- timestamp is in the future and does not \
             suppress version-drift detection; verify manually and clear the marker",
        ));
    } else if malformed_marker {
        findings.push(Finding::advisory(
            "malformed or missing upgrade_started_at marker -- upgrade_in_progress is set but \
             upgrade_started_at is absent or not a valid RFC3339 timestamp, so it does not \
             suppress version-drift detection; verify manually and clear the marker",
        ));
    }

    findings
}

fn read_agf_version(path: &Path) -> Option<String> {
    std::fs::read_to_string(path)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

/// Merge corpus-wide findings (see `corpus_errors`) into the per-file
/// `results` list. A finding tied to a specific file (`Some(path)`) is
/// appended to that file's entry, converting it from `Ok` to `Err` if
/// needed. A directory-level finding (`None`, e.g. the zero-live advisory
/// or the multi-live ambiguity finding) is attached to a synthetic entry
/// at `dir` itself, since it describes the corpus as a whole, not one file.
fn merge_corpus_findings(
    results: &mut Vec<(PathBuf, Result<serde_json::Value, Vec<Finding>>)>,
    dir: &Path,
    corpus_findings: Vec<(Option<PathBuf>, Finding)>,
) {
    for (target, finding) in corpus_findings {
        let target = target.unwrap_or_else(|| dir.to_path_buf());
        match results.iter().position(|(path, _)| *path == target) {
            Some(index) if results[index].1.is_ok() => {
                results[index].1 = Err(vec![finding]);
            }
            Some(index) => {
                if let Err(findings) = &mut results[index].1 {
                    findings.push(finding);
                }
            }
            None => results.push((target, Err(vec![finding]))),
        }
    }
}

fn learning_files(dir: &Path) -> Vec<std::path::PathBuf> {
    yaml_files(dir)
        .into_iter()
        .filter(|path| {
            path.file_name()
                .and_then(std::ffi::OsStr::to_str)
                .is_none_or(|name| !name.starts_with('_'))
        })
        .collect()
}

pub fn validate_file(path: &Path) -> Result<serde_json::Value, Vec<Finding>> {
    let data = crate::read_yaml_file(path)?;
    let mut errors = validate_schema(&data, "learning.schema.json")
        .into_iter()
        .filter(|finding| !is_learning_id_schema_finding(finding))
        .collect::<Vec<_>>();
    errors.extend(semantic_errors(&data));

    if errors.is_empty() {
        Ok(data)
    } else {
        Err(errors)
    }
}

fn is_learning_id_schema_finding(finding: &Finding) -> bool {
    finding
        .location
        .as_deref()
        .is_some_and(|location| location.ends_with("/id"))
}

fn semantic_errors(data: &serde_json::Value) -> Vec<Finding> {
    let mut errors = learning_id_errors(data);
    errors.extend(pii_errors(data));
    errors.extend(confidence_errors(data));
    errors.extend(supersedes_errors(data));
    errors
}

/// Validates `supersedes`: the referenced id must exist in the same file
/// and must precede the referencing entry's position. Forward-only-by-
/// position also rules out cycles, so no separate cycle check is needed.
fn supersedes_errors(data: &serde_json::Value) -> Vec<Finding> {
    let mut errors = Vec::new();
    let Some(learnings) = data.get("learnings").and_then(serde_json::Value::as_array) else {
        return errors;
    };

    let mut positions: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for (offset, entry) in learnings.iter().enumerate() {
        if let Some(id) = entry.get("id").and_then(serde_json::Value::as_str) {
            positions.entry(id.to_string()).or_insert(offset + 1);
        }
    }

    for (offset, entry) in learnings.iter().enumerate() {
        let index = offset + 1;
        let Some(target) = entry.get("supersedes").and_then(serde_json::Value::as_str) else {
            continue;
        };

        match positions.get(target) {
            None => errors.push(Finding::blocking_at(
                format!("supersedes references unknown id {target:?}"),
                format!("learnings/{index}/supersedes"),
            )),
            Some(&target_position) if target_position >= index => {
                errors.push(Finding::blocking_at(
                    format!(
                        "supersedes references a later-or-equal entry {target:?} (position {target_position} >= {index})"
                    ),
                    format!("learnings/{index}/supersedes"),
                ));
            }
            Some(_) => {}
        }
    }

    errors
}

fn learning_id_errors(data: &serde_json::Value) -> Vec<Finding> {
    let mut errors = Vec::new();
    let historical = Regex::new(HISTORICAL_ID).expect("valid historical learning id regex");
    let bare_timestamp = Regex::new(BARE_TIMESTAMP_ID).expect("valid bare timestamp id regex");
    let suffixed_timestamp =
        Regex::new(SUFFIXED_TIMESTAMP_ID).expect("valid suffixed timestamp id regex");
    let mut seen: std::collections::BTreeMap<String, Vec<usize>> =
        std::collections::BTreeMap::new();

    if let Some(learnings) = data.get("learnings").and_then(serde_json::Value::as_array) {
        for (offset, entry) in learnings.iter().enumerate() {
            let index = offset + 1;
            let Some(id) = entry.get("id") else {
                continue;
            };

            let Some(id) = id.as_str() else {
                errors.push(Finding::blocking_at(
                    format!("learning id {id:?} is invalid; expected string"),
                    format!("learnings/{index}/id"),
                ));
                continue;
            };

            if bare_timestamp.is_match(id) {
                let mut finding = Finding::advisory(
                    "bare timestamp learning id is transitional; new records require YYYYMMDDThhmmssZ-<4-char-random>",
                );
                finding.location = Some(format!("learnings/{index}/id"));
                errors.push(finding);
            } else if !historical.is_match(id) && !suffixed_timestamp.is_match(id) {
                errors.push(Finding::blocking_at(
                    format!(
                        "learning id {id:?} is invalid; use L### for historical records or YYYYMMDDThhmmssZ-<4-char-random> for new records"
                    ),
                    format!("learnings/{index}/id"),
                ));
            }

            seen.entry(id.to_string()).or_default().push(index);
        }
    }

    errors.extend(seen.into_iter().filter_map(|(id, indexes)| {
        (indexes.len() > 1).then(|| {
            Finding::blocking_at(
                format!(
                    "duplicate learning id {id:?} at entries {}",
                    indexes
                        .iter()
                        .map(usize::to_string)
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                "learnings/id",
            )
        })
    }));

    errors
}

fn pii_errors(data: &serde_json::Value) -> Vec<Finding> {
    let text = crate::freetext(data).join("\n");
    let patterns = [
        ("URL", r#"https?://[^\s"]+"#),
        (
            "email address",
            r#"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}"#,
        ),
        (
            "company suffix",
            r#"\b(Inc|Corp|Ltd|GmbH|LLC|Oy|Oyj|Pty|plc)\b"#,
        ),
        (
            "internal hostname",
            r#"(?i)\b[a-z0-9-]+\.(internal|local|corp|cloud)\b"#,
        ),
    ];

    patterns
        .into_iter()
        .filter(|(_label, pattern)| Regex::new(pattern).is_ok_and(|regex| regex.is_match(&text)))
        .map(|(label, _pattern)| Finding::blocking_at(format!("contains {label}"), "freetext"))
        .collect()
}

fn confidence_errors(data: &serde_json::Value) -> Vec<Finding> {
    data.get("learnings")
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten()
        .enumerate()
        .flat_map(|(index, entry)| {
            let index = index + 1;
            entry
                .get("confidence")
                .and_then(serde_json::Value::as_object)
                .map(|confidence| confidence_entry_errors(confidence, index))
                .unwrap_or_default()
        })
        .collect()
}

fn confidence_entry_errors(
    confidence: &serde_json::Map<String, serde_json::Value>,
    index: usize,
) -> Vec<Finding> {
    let level = confidence.get("level").and_then(serde_json::Value::as_str);
    let inspected = level.map_or("null".to_string(), |level| format!("\"{level}\""));
    let mut errors = Vec::new();

    if !level.is_some_and(|level| CONFIDENCE_LEVELS.contains(&level)) {
        errors.push(Finding::blocking_at(
            format!("confidence.level {inspected} is invalid"),
            format!("learnings/{index}/confidence/level"),
        ));
    }

    if level.is_some_and(|level| CORROBORATED_LEVELS.contains(&level)) && !corroborated(confidence)
    {
        let level = level.unwrap_or_default();
        errors.push(Finding::blocking_at(
            format!(
                "confidence.level {level} requires evidence_count >= 2 or at least two evidence_refs"
            ),
            format!("learnings/{index}/confidence"),
        ));
    }

    errors
}

fn corroborated(confidence: &serde_json::Map<String, serde_json::Value>) -> bool {
    let evidence_count = confidence
        .get("evidence_count")
        .and_then(serde_json::Value::as_i64)
        .is_some_and(|count| count >= 2);
    let evidence_refs = confidence
        .get("evidence_refs")
        .and_then(serde_json::Value::as_array)
        .is_some_and(|refs| refs.len() >= 2);

    evidence_count || evidence_refs
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn accepts_timestamp_and_historical_learning_ids() {
        let path = write_fixture(
            "valid-learning-ids",
            r#"
project:
  codename: foundry-fixture
learnings:
  - id: L021
    title: Historical id
    description: Generic finding.
  - id: 20260621T150000Z-a3fx
    title: Timestamp id with suffix
    description: Generic finding.
"#,
        );

        assert!(validate_file(&path).is_ok());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn warns_for_bare_timestamp_learning_ids() {
        let path = write_fixture(
            "bare-timestamp-learning-id",
            r#"
project:
  codename: foundry-fixture
learnings:
  - id: 20260621T150000Z
    title: Bare timestamp id
    description: Generic finding.
"#,
        );

        let findings = validate_file(&path).expect_err("bare timestamp id should warn");
        assert!(findings.iter().any(|finding| {
            finding.severity == agov_core::Severity::Advisory
                && finding.message.contains("bare timestamp learning id")
        }));
        assert!(findings
            .iter()
            .all(|finding| finding.severity == agov_core::Severity::Advisory));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn rejects_non_canonical_learning_ids() {
        let path = write_fixture(
            "bad-learning-id",
            r#"
project:
  codename: foundry-fixture
learnings:
  - id: learn-20260621T145130Z-os-background-timers-7rhn
    title: Bad id
    description: Generic finding.
"#,
        );

        let errors = validate_file(&path).expect_err("invalid id should fail");
        assert!(errors
            .iter()
            .any(|error| error.message.contains("learning id")));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn rejects_duplicate_learning_ids() {
        let path = write_fixture(
            "duplicate-learning-id",
            r#"
project:
  codename: foundry-fixture
learnings:
  - id: 20260621T150000Z
    title: First
    description: Generic finding.
  - id: 20260621T150000Z
    title: Second
    description: Generic finding.
"#,
        );

        let errors = validate_file(&path).expect_err("duplicate id should fail");
        assert!(errors
            .iter()
            .any(|error| error.message.contains("duplicate learning id")));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn accepts_valid_supersedes_chain() {
        let path = write_fixture(
            "valid-supersedes-chain",
            r#"
project:
  codename: foundry-fixture
learnings:
  - id: 20260621T150000Z-aaaa
    title: Original finding
    description: Generic finding.
  - id: 20260622T150000Z-bbbb
    title: Corrected finding
    description: Generic finding.
    supersedes: 20260621T150000Z-aaaa
"#,
        );

        assert!(validate_file(&path).is_ok());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn rejects_supersedes_unknown_id() {
        let path = write_fixture(
            "supersedes-unknown-id",
            r#"
project:
  codename: foundry-fixture
learnings:
  - id: 20260622T150000Z-bbbb
    title: Corrected finding
    description: Generic finding.
    supersedes: 20260621T150000Z-zzzz
"#,
        );

        let errors = validate_file(&path).expect_err("unknown supersedes target should fail");
        assert!(errors
            .iter()
            .any(|error| error.message.contains("supersedes references unknown id")));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn rejects_supersedes_self_reference() {
        let path = write_fixture(
            "supersedes-self-reference",
            r#"
project:
  codename: foundry-fixture
learnings:
  - id: 20260621T150000Z-aaaa
    title: Self-referencing finding
    description: Generic finding.
    supersedes: 20260621T150000Z-aaaa
"#,
        );

        let errors = validate_file(&path).expect_err("self-referencing supersedes should fail");
        assert!(errors.iter().any(|error| {
            error
                .message
                .contains("supersedes references a later-or-equal entry")
        }));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn rejects_supersedes_later_entry() {
        let path = write_fixture(
            "supersedes-later-entry",
            r#"
project:
  codename: foundry-fixture
learnings:
  - id: 20260621T150000Z-aaaa
    title: Earlier finding
    description: Generic finding.
    supersedes: 20260622T150000Z-bbbb
  - id: 20260622T150000Z-bbbb
    title: Later finding
    description: Generic finding.
"#,
        );

        let errors = validate_file(&path).expect_err("forward-referencing supersedes should fail");
        assert!(errors.iter().any(|error| {
            error
                .message
                .contains("supersedes references a later-or-equal entry")
        }));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn accepts_supersedes_targeting_a_bare_timestamp_id() {
        let path = write_fixture(
            "supersedes-bare-timestamp-target",
            r#"
project:
  codename: foundry-fixture
learnings:
  - id: 20260621T150000Z
    title: Historical bare-timestamp finding
    description: Generic finding.
  - id: 20260622T150000Z-bbbb
    title: Corrected finding
    description: Generic finding.
    supersedes: 20260621T150000Z
"#,
        );

        let result = validate_file(&path);
        assert!(
            result.is_err(),
            "the target's own bare-timestamp id still produces its existing advisory"
        );
        let errors = result.unwrap_err();
        assert!(
            errors
                .iter()
                .all(|error| error.severity == agov_core::Severity::Advisory),
            "supersedes referencing a bare-timestamp id must not itself be a blocking finding: {errors:?}"
        );
        assert!(errors
            .iter()
            .any(|error| error.message.contains("bare timestamp learning id")));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn rejects_invalid_status_value() {
        let path = write_fixture(
            "invalid-status-value",
            r#"
project:
  codename: foundry-fixture
learnings:
  - id: 20260621T150000Z-aaaa
    title: Finding with bad status
    description: Generic finding.
    status: superseded
"#,
        );

        assert!(validate_file(&path).is_err());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn accepts_proposed_and_implemented_status_values() {
        let path = write_fixture(
            "valid-status-values",
            r#"
project:
  codename: foundry-fixture
learnings:
  - id: 20260621T150000Z-aaaa
    title: Proposed finding
    description: Generic finding.
    status: proposed
  - id: 20260622T150000Z-bbbb
    title: Implemented finding
    description: Generic finding.
    status: implemented
"#,
        );

        assert!(validate_file(&path).is_ok());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn skips_template_learning_files() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let template = dir.path().join("_template.yaml");
        fs::write(
            &template,
            r#"
project:
  codename: ""
learnings: []
"#,
        )
        .expect("write template");

        let result = validate_all(dir.path());

        assert!(result.ok_files.is_empty());
        // The template file is still skipped as a real record, but a
        // learnings directory with zero checkout_role: live records
        // (including zero real records at all) now emits the corpus-wide
        // advisory finding -- this is the new version-drift check's
        // always-on zero-live signal, not a per-file finding.
        assert_eq!(result.file_errors.len(), 1);
        let findings = &result.file_errors[0].findings;
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].severity, agov_core::Severity::Advisory);
        assert!(findings[0].message.contains("no live-marked adoption record found"));
    }

    /// Build a fixture `<tmp>/agf/learnings/` directory (matching the real
    /// `agov_core::learnings_dir()` layout) plus `<tmp>/agf/VERSION`, and
    /// write each `(filename, content)` pair as a record inside it. Returns
    /// the `TempDir` (keep it alive for the test's duration) and the
    /// learnings directory path to pass to `validate_all`.
    fn write_corpus(version: &str, files: &[(&str, &str)]) -> (tempfile::TempDir, PathBuf) {
        let root = tempfile::tempdir().expect("create temp dir");
        let learnings_dir = root.path().join("agf").join("learnings");
        fs::create_dir_all(&learnings_dir).expect("create agf/learnings");
        fs::write(root.path().join("agf").join("VERSION"), version).expect("write agf/VERSION");
        for (name, content) in files {
            fs::write(learnings_dir.join(name), content).expect("write learning record");
        }
        (root, learnings_dir)
    }

    #[test]
    fn live_record_matching_version_passes() {
        let (_root, dir) = write_corpus(
            "2.6.1",
            &[(
                "foundry.yaml",
                r#"
project:
  codename: foundry-fixture
adoption:
  checkout_role: live
  framework_version: "2.6.1"
"#,
            )],
        );

        let result = validate_all(&dir);
        assert!(result.is_ok(), "expected no findings, got {:?}", result.file_errors.iter().flat_map(|e| e.findings.iter().map(|f| f.message.clone())).collect::<Vec<_>>());
    }

    #[test]
    fn live_record_with_unrelated_advisory_finding_is_still_recognized() {
        // Regression: validate_file() returns Err (discarding the parsed
        // Value) whenever a file has ANY finding, including one unrelated
        // to checkout_role/framework_version -- here, a bare-timestamp
        // learning id elsewhere in the same document. The corpus-wide
        // live-record scan must still see this record's checkout_role and
        // framework_version rather than silently excluding it.
        let (_root, dir) = write_corpus(
            "2.6.1",
            &[(
                "foundry.yaml",
                r#"
project:
  codename: foundry-fixture
adoption:
  checkout_role: live
  framework_version: "2.6.1"
learnings:
  - id: 20260621T150000Z
    title: Bare timestamp id
    description: Generic finding, unrelated to adoption version drift.
"#,
            )],
        );

        let result = validate_all(&dir);
        assert_eq!(result.file_errors.len(), 1, "expected only the bare-timestamp advisory to remain");
        let findings = &result.file_errors[0].findings;
        assert!(
            findings.iter().all(|finding| finding.severity == agov_core::Severity::Advisory),
            "expected no blocking findings, got {:?}",
            findings.iter().map(|f| f.message.clone()).collect::<Vec<_>>()
        );
        assert!(findings.iter().any(|finding| finding.message.contains("bare timestamp learning id")));
        assert!(
            !findings.iter().any(|finding| finding.message.contains("no live-marked adoption record found")),
            "live record should have been recognized despite the unrelated advisory finding"
        );
        assert!(
            !findings.iter().any(|finding| finding.message.contains("does not match agf/VERSION")),
            "matching version should not produce a mismatch finding"
        );
    }

    #[test]
    fn live_record_mismatched_version_blocks_naming_both_values() {
        let (_root, dir) = write_corpus(
            "2.6.1",
            &[(
                "foundry.yaml",
                r#"
project:
  codename: foundry-fixture
adoption:
  checkout_role: live
  framework_version: "2.0.0-provisional"
"#,
            )],
        );

        let result = validate_all(&dir);
        assert!(!result.is_ok());
        assert_eq!(result.file_errors.len(), 1);
        let findings = &result.file_errors[0].findings;
        assert!(findings
            .iter()
            .any(|f| f.severity == agov_core::Severity::Blocking
                && f.message.contains("2.0.0-provisional")
                && f.message.contains("2.6.1")));
    }

    #[test]
    fn imported_record_mismatched_version_is_skipped() {
        // Regression fixture: this repository's own harbor-workshop.yaml
        // shape (framework_version 2.5.0 vs agf/VERSION 2.6.1), no
        // checkout_role set -- proving the existing corpus is unaffected.
        let (_root, dir) = write_corpus(
            "2.6.1",
            &[(
                "harbor-workshop.yaml",
                r#"
project:
  codename: harbor-workshop
adoption:
  framework_version: "2.5.0"
"#,
            )],
        );

        let result = validate_all(&dir);
        // Zero live records -> the always-on advisory fires, but the
        // mismatched imported/unmarked record itself is never flagged.
        assert_eq!(result.file_errors.len(), 1);
        let findings = &result.file_errors[0].findings;
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].severity, agov_core::Severity::Advisory);
        assert!(findings[0].message.contains("no live-marked adoption record found"));
    }

    #[test]
    fn explicitly_imported_record_mismatched_version_is_skipped() {
        let (_root, dir) = write_corpus(
            "2.6.1",
            &[(
                "imported.yaml",
                r#"
project:
  codename: imported-fixture
adoption:
  checkout_role: imported
  framework_version: "1.0.0"
"#,
            )],
        );

        let result = validate_all(&dir);
        assert_eq!(result.file_errors.len(), 1);
        assert!(result.file_errors[0].findings[0]
            .message
            .contains("no live-marked adoption record found"));
    }

    #[test]
    fn live_record_with_fresh_upgrade_marker_suppresses_mismatch() {
        let started_at = (time::OffsetDateTime::now_utc() - time::Duration::hours(1))
            .format(&time::format_description::well_known::Rfc3339)
            .expect("format timestamp");
        let (_root, dir) = write_corpus(
            "2.6.1",
            &[(
                "foundry.yaml",
                &format!(
                    r#"
project:
  codename: foundry-fixture
adoption:
  checkout_role: live
  framework_version: "2.0.0-provisional"
  upgrade_in_progress: true
  upgrade_started_at: "{started_at}"
"#
                ),
            )],
        );

        let result = validate_all(&dir);
        assert!(result.is_ok(), "expected suppression, got {:?}", result.file_errors.iter().flat_map(|e| e.findings.iter().map(|f| f.message.clone())).collect::<Vec<_>>());
    }

    #[test]
    fn live_record_with_stale_upgrade_marker_fires_both_findings() {
        let started_at = (time::OffsetDateTime::now_utc() - time::Duration::hours(25))
            .format(&time::format_description::well_known::Rfc3339)
            .expect("format timestamp");
        let (_root, dir) = write_corpus(
            "2.6.1",
            &[(
                "foundry.yaml",
                &format!(
                    r#"
project:
  codename: foundry-fixture
adoption:
  checkout_role: live
  framework_version: "2.0.0-provisional"
  upgrade_in_progress: true
  upgrade_started_at: "{started_at}"
"#
                ),
            )],
        );

        let result = validate_all(&dir);
        assert!(!result.is_ok());
        let findings = &result.file_errors[0].findings;
        assert!(findings
            .iter()
            .any(|f| f.severity == agov_core::Severity::Blocking
                && f.message.contains("does not match")));
        assert!(findings
            .iter()
            .any(|f| f.severity == agov_core::Severity::Advisory
                && f.message.contains("stale upgrade_in_progress marker")));
    }

    #[test]
    fn live_record_at_zero_hours_elapsed_is_within_window() {
        // Boundary: elapsed_hours == 0 must still suppress (the window is
        // [0, UPGRADE_WINDOW_HOURS)).
        let started_at = time::OffsetDateTime::now_utc()
            .format(&time::format_description::well_known::Rfc3339)
            .expect("format timestamp");
        let (_root, dir) = write_corpus(
            "2.6.1",
            &[(
                "foundry.yaml",
                &format!(
                    r#"
project:
  codename: foundry-fixture
adoption:
  checkout_role: live
  framework_version: "2.0.0-provisional"
  upgrade_in_progress: true
  upgrade_started_at: "{started_at}"
"#
                ),
            )],
        );

        let result = validate_all(&dir);
        assert!(result.is_ok(), "expected suppression at zero elapsed hours, got {:?}", result.file_errors.iter().flat_map(|e| e.findings.iter().map(|f| f.message.clone())).collect::<Vec<_>>());
    }

    #[test]
    fn live_record_with_future_upgrade_marker_does_not_suppress_mismatch() {
        // Regression (MR !1965 round-1 audit CA-ADV-R1-1): a future
        // upgrade_started_at produced a negative elapsed-hours value that
        // satisfied `hours < UPGRADE_WINDOW_HOURS`, silently suppressing a
        // real version mismatch indefinitely.
        let started_at = (time::OffsetDateTime::now_utc() + time::Duration::hours(48))
            .format(&time::format_description::well_known::Rfc3339)
            .expect("format timestamp");
        let (_root, dir) = write_corpus(
            "2.6.1",
            &[(
                "foundry.yaml",
                &format!(
                    r#"
project:
  codename: foundry-fixture
adoption:
  checkout_role: live
  framework_version: "2.0.0-provisional"
  upgrade_in_progress: true
  upgrade_started_at: "{started_at}"
"#
                ),
            )],
        );

        let result = validate_all(&dir);
        assert!(!result.is_ok(), "future timestamp must not suppress a real mismatch");
        let findings = &result.file_errors[0].findings;
        assert!(
            findings
                .iter()
                .any(|f| f.severity == agov_core::Severity::Blocking
                    && f.message.contains("does not match")),
            "expected the mismatch to still block, got {:?}",
            findings.iter().map(|f| f.message.clone()).collect::<Vec<_>>()
        );
        assert!(
            findings
                .iter()
                .any(|f| f.severity == agov_core::Severity::Advisory
                    && f.message.contains("malformed upgrade_started_at marker")
                    && f.message.contains("future")),
            "expected a malformed-future-marker advisory, got {:?}",
            findings.iter().map(|f| f.message.clone()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn live_record_with_invalid_upgrade_started_at_does_not_suppress_mismatch() {
        let (_root, dir) = write_corpus(
            "2.6.1",
            &[(
                "foundry.yaml",
                r#"
project:
  codename: foundry-fixture
adoption:
  checkout_role: live
  framework_version: "2.0.0-provisional"
  upgrade_in_progress: true
  upgrade_started_at: "not-a-timestamp"
"#,
            )],
        );

        let result = validate_all(&dir);
        assert!(!result.is_ok(), "invalid timestamp must not suppress a real mismatch");
        let findings = &result.file_errors[0].findings;
        assert!(findings
            .iter()
            .any(|f| f.severity == agov_core::Severity::Blocking
                && f.message.contains("does not match")));
        assert!(findings
            .iter()
            .any(|f| f.severity == agov_core::Severity::Advisory
                && f.message.contains("malformed or missing upgrade_started_at marker")));
    }

    #[test]
    fn live_record_with_upgrade_in_progress_but_no_started_at_does_not_suppress_mismatch() {
        let (_root, dir) = write_corpus(
            "2.6.1",
            &[(
                "foundry.yaml",
                r#"
project:
  codename: foundry-fixture
adoption:
  checkout_role: live
  framework_version: "2.0.0-provisional"
  upgrade_in_progress: true
"#,
            )],
        );

        let result = validate_all(&dir);
        assert!(!result.is_ok(), "missing upgrade_started_at must not suppress a real mismatch");
        let findings = &result.file_errors[0].findings;
        assert!(findings
            .iter()
            .any(|f| f.severity == agov_core::Severity::Blocking
                && f.message.contains("does not match")));
        assert!(findings
            .iter()
            .any(|f| f.severity == agov_core::Severity::Advisory
                && f.message.contains("malformed or missing upgrade_started_at marker")));
    }

    #[test]
    fn live_record_missing_framework_version_blocks() {
        // Regression (MR !1965 round-1 audit CA-ADV-R1-2): a live record
        // with no framework_version at all previously activated neither
        // the zero-live advisory nor the drift comparison, and passed.
        let (_root, dir) = write_corpus(
            "2.6.1",
            &[(
                "foundry.yaml",
                r#"
project:
  codename: foundry-fixture
adoption:
  checkout_role: live
"#,
            )],
        );

        let result = validate_all(&dir);
        assert!(!result.is_ok(), "a live record with no framework_version must block");
        let findings = &result.file_errors[0].findings;
        assert!(
            findings.iter().any(|f| f.severity == agov_core::Severity::Blocking
                && f.message.contains("adoption.framework_version is required")),
            "expected a required-field blocking finding, got {:?}",
            findings.iter().map(|f| f.message.clone()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn zero_live_records_advisory_does_not_block() {
        let (_root, dir) = write_corpus(
            "2.6.1",
            &[
                (
                    "foundry.yaml",
                    r#"
project:
  codename: foundry-fixture
adoption:
  framework_version: "2.0.0-provisional"
"#,
                ),
                (
                    "harbor-workshop.yaml",
                    r#"
project:
  codename: harbor-workshop
adoption:
  checkout_role: imported
  framework_version: "2.5.0"
"#,
                ),
            ],
        );

        let result = validate_all(&dir);
        assert_eq!(result.file_errors.len(), 1);
        assert_eq!(
            result.file_errors[0].findings[0].severity,
            agov_core::Severity::Advisory
        );
    }

    #[test]
    fn two_live_records_is_a_blocking_ambiguity() {
        let (_root, dir) = write_corpus(
            "2.6.1",
            &[
                (
                    "first.yaml",
                    r#"
project:
  codename: first-fixture
adoption:
  checkout_role: live
  framework_version: "2.6.1"
"#,
                ),
                (
                    "second.yaml",
                    r#"
project:
  codename: second-fixture
adoption:
  checkout_role: live
  framework_version: "2.6.1"
"#,
                ),
            ],
        );

        let result = validate_all(&dir);
        assert!(!result.is_ok());
        assert_eq!(result.file_errors.len(), 1);
        let findings = &result.file_errors[0].findings;
        assert!(findings.iter().any(|f| f.severity == agov_core::Severity::Blocking
            && f.message.contains("multiple checkout_role: live records")
            && f.message.contains("first.yaml")
            && f.message.contains("second.yaml")));
    }

    #[test]
    fn exactly_one_live_record_skips_advisory_and_ambiguity() {
        let (_root, dir) = write_corpus(
            "2.6.1",
            &[
                (
                    "foundry.yaml",
                    r#"
project:
  codename: foundry-fixture
adoption:
  checkout_role: live
  framework_version: "2.6.1"
"#,
                ),
                (
                    "harbor-workshop.yaml",
                    r#"
project:
  codename: harbor-workshop
adoption:
  checkout_role: imported
  framework_version: "2.5.0"
"#,
                ),
            ],
        );

        let result = validate_all(&dir);
        assert!(result.is_ok(), "expected no findings, got {:?}", result.file_errors.iter().flat_map(|e| e.findings.iter().map(|f| f.message.clone())).collect::<Vec<_>>());
    }

    fn write_fixture(name: &str, content: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "agov-{name}-{}-{}.yaml",
            std::process::id(),
            std::thread::current().name().unwrap_or("test")
        ));
        fs::write(&path, content).expect("write fixture");
        path
    }
}
