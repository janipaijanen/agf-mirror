use crate::{split_results, validate_schema, yaml_files, CollectionResult};
use agov_core::Finding;
use regex::Regex;
use std::path::Path;

const CONFIDENCE_LEVELS: &[&str] = &[
    "single-adopter",
    "multi-adopter",
    "longitudinal",
    "industry-corroborated",
];
const CORROBORATED_LEVELS: &[&str] = &["multi-adopter", "longitudinal"];

/// Validate all learning record YAML files in `dir`.
///
/// Mirrors `Agov.ValidateLearnings.validate_all/1`. Each file is validated
/// against `learning.schema.json`.
pub fn validate_all(dir: &Path) -> CollectionResult {
    let results = yaml_files(dir)
        .into_iter()
        .map(|file| {
            let result = validate_file(&file);
            (file, result)
        })
        .collect();

    split_results(results)
}

fn validate_file(path: &Path) -> Result<serde_json::Value, Vec<Finding>> {
    let data = crate::read_yaml_file(path)?;
    let mut errors = validate_schema(&data, "learning.schema.json");
    errors.extend(semantic_errors(&data));

    if errors.is_empty() {
        Ok(data)
    } else {
        Err(errors)
    }
}

fn semantic_errors(data: &serde_json::Value) -> Vec<Finding> {
    let mut errors = pii_errors(data);
    errors.extend(confidence_errors(data));
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
