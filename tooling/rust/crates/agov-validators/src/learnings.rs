use crate::{split_results, validate_schema, yaml_files, CollectionResult};
use agov_core::Finding;
use regex::Regex;
use std::path::{Path, PathBuf};

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

/// Validate all learning record YAML files in `dir`.
///
/// Mirrors `Agov.ValidateLearnings.validate_all/1`. Each file is validated
/// against `learning.schema.json`.
pub fn validate_all(dir: &Path) -> CollectionResult {
    validate_paths(&[dir.to_path_buf()])
}

/// Validate learning records from explicit files or directories.
pub fn validate_paths(paths: &[PathBuf]) -> CollectionResult {
    let results = paths
        .iter()
        .flat_map(|path| {
            if path.is_dir() {
                learning_files(path)
            } else {
                vec![path.to_path_buf()]
            }
        })
        .map(|file| {
            let result = validate_file(&file);
            (file, result)
        })
        .collect();

    split_results(results)
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

        assert!(result.is_ok());
        assert!(result.ok_files.is_empty());
        assert!(result.file_errors.is_empty());
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
