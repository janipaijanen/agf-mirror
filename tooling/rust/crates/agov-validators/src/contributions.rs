use crate::{path_label, yaml_files, CollectionResult, FileError};
use agov_core::Finding;
use regex::Regex;
use std::path::Path;

/// Validate contribution-auditor findings in learning records under `dir`.
///
/// Mirrors `Agov.ValidateContributions.validate_all/1`. Returns advisory
/// (non-blocking) findings — maps to the `:warnings` reporter variant.
pub fn validate_all(dir: &Path) -> CollectionResult {
    let mut ok_files = Vec::new();
    let mut file_errors = Vec::new();

    for file in yaml_files(dir) {
        match validate_file(&file) {
            Ok(warnings) => {
                ok_files.push(path_label(&file));
                if !warnings.is_empty() {
                    file_errors.push(FileError {
                        file: path_label(&file),
                        findings: warnings,
                    });
                }
            }
            Err(_errors) => {}
        }
    }

    CollectionResult {
        ok_files,
        file_errors,
    }
}

fn validate_file(path: &Path) -> Result<Vec<Finding>, Vec<Finding>> {
    let data = crate::read_yaml_file(path)?;
    Ok(warning_findings(&data))
}

fn warning_findings(data: &serde_json::Value) -> Vec<Finding> {
    let text = crate::freetext(data).join("\n");
    let absence = Regex::new(
        r"(?i)no\s+(issues?|problems?|failures?)|without\s+(any\s+)?(issues?|problems?)",
    )
    .unwrap();
    let relaxation = Regex::new(
        r"(?i)(remove|skip|bypass|weaken|relax).{0,40}(rule|gate|review|check|boundary|validation)",
    )
    .unwrap();

    let mut findings = Vec::new();
    if absence.is_match(&text) {
        findings.push(Finding::advisory(
            "absence-of-incident claim requires evidence",
        ));
        if let Some(last) = findings.last_mut() {
            last.location = Some("freetext".to_string());
        }
    }
    if relaxation.is_match(&text) {
        findings.push(Finding::advisory(
            "governance relaxation proposal requires evidence",
        ));
        if let Some(last) = findings.last_mut() {
            last.location = Some("freetext".to_string());
        }
    }
    findings.extend(evidence_quality_warnings(data));
    findings
}

fn evidence_quality_warnings(data: &serde_json::Value) -> Vec<Finding> {
    data.get("learnings")
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten()
        .enumerate()
        .filter_map(|(index, entry)| {
            let confidence = entry.get("confidence")?.as_object()?;
            let level = confidence.get("level")?.as_str()?;
            if !matches!(level, "multi-adopter" | "longitudinal") {
                return None;
            }

            let count_is_one = confidence
                .get("evidence_count")
                .and_then(serde_json::Value::as_i64)
                == Some(1);
            let refs_empty = confidence
                .get("evidence_refs")
                .and_then(serde_json::Value::as_array)
                .is_none_or(Vec::is_empty);

            if count_is_one || refs_empty {
                let mut finding =
                    Finding::advisory(format!("weak evidence for {level} confidence"));
                finding.location = Some(format!("learnings/{}/confidence", index + 1));
                Some(finding)
            } else {
                None
            }
        })
        .collect()
}
