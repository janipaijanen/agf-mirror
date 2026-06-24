use crate::{path_label, yaml_files, CollectionResult, FileError};
use agov_core::Finding;
use regex::Regex;
use std::path::Path;
use std::sync::OnceLock;

/// Validate contribution-auditor findings in learning records under `dir`.
///
/// Returns advisory (non-blocking) findings covering three adversarial
/// pattern classes: absence-of-incident bias, governance relaxation
/// proposals, and weak universal evidence claims.
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
    let text = contributions_freetext(data).join("\n");
    let mut findings = Vec::new();

    for (matched, _) in absence_findings(&text) {
        let mut f = Finding::advisory(format!(
            "absence bias: '{matched}' — absence of reported incidents is not evidence of safety"
        ));
        f.location = Some("freetext".to_string());
        findings.push(f);
    }
    for (matched, _) in relaxation_findings(&text) {
        let mut f = Finding::advisory(format!(
            "governance relaxation: '{matched}' — proposals to weaken governance require explicit evidence and human approval"
        ));
        f.location = Some("freetext".to_string());
        findings.push(f);
    }
    for (matched, _) in universal_claim_findings(&text) {
        let mut f = Finding::advisory(format!(
            "evidence quality: '{matched}' — universal claims require corroboration across multiple projects"
        ));
        f.location = Some("freetext".to_string());
        findings.push(f);
    }
    findings.extend(weak_confidence_findings(data));
    findings
}

// ─── Absence-of-incident bias ────────────────────────────────────────────────

static ABSENCE_PATTERNS: OnceLock<Vec<Regex>> = OnceLock::new();

fn absence_patterns() -> &'static Vec<Regex> {
    ABSENCE_PATTERNS.get_or_init(|| {
        [
            r"(?i)no\s+incidents?\b",
            r"(?i)no\s+issues?\b",
            r"(?i)no\s+problems?\b",
            r"(?i)no\s+complaints?\b",
            r"(?i)no\s+failures?\b",
            r"(?i)no\s+bugs?\b",
            r"(?i)no\s+one\s+complained",
            r"(?i)without\s+(?:any\s+)?issues?\b",
            r"(?i)without\s+(?:any\s+)?problems?\b",
            r"(?i)without\s+(?:any\s+)?incidents?\b",
            r"(?i)without\s+consequences\b",
            r"(?i)saw\s+no\b",
            r"(?i)had\s+no\s+(?:issues?|problems?|incidents?)\b",
            r"(?i)worked\s+(?:fine|for\s+us)\b",
        ]
        .iter()
        .map(|p| Regex::new(p).unwrap())
        .collect()
    })
}

fn absence_findings(text: &str) -> Vec<(String, &'static str)> {
    let mut out = Vec::new();
    for re in absence_patterns() {
        if let Some(m) = re.find(text) {
            out.push((m.as_str().to_string(), "absence-of-incident"));
            break; // one finding per file is sufficient signal
        }
    }
    out
}

// ─── Governance relaxation ───────────────────────────────────────────────────

static RELAXATION_PATTERNS: OnceLock<Vec<Regex>> = OnceLock::new();

fn relaxation_patterns() -> &'static Vec<Regex> {
    RELAXATION_PATTERNS.get_or_init(|| {
        [
            r"(?i)\b(?:remove|drop|eliminate|disable)\b.{0,40}\b(?:review|check|gate|rule|boundary|escalation|validation)\b",
            r"(?i)\b(?:skip|bypass|avoid)\b.{0,40}\b(?:review|check|gate|rule|boundary|escalation|validation)\b",
            r"(?i)\bskipping\b.{0,40}\b(?:review|check|gate|rule|boundary|escalation|validation)\b",
            r"(?i)\b(?:relax|loosen|weaken|soften|relaxed|loosened|weakened|softened)\b.{0,40}\b(?:boundary|rule|constraint|requirement|enforcement)\b",
            r"(?i)\bunnecessary\s+(?:overhead|ceremony|friction|process|bureaucracy)\b",
            r"(?i)\b(?:overhead|ceremony|friction)\b.{0,20}\b(?:unnecessary|excessive|too\s+much)\b",
        ]
        .iter()
        .map(|p| Regex::new(p).unwrap())
        .collect()
    })
}

fn relaxation_findings(text: &str) -> Vec<(String, &'static str)> {
    let mut out = Vec::new();
    for re in relaxation_patterns() {
        if let Some(m) = re.find(text) {
            out.push((m.as_str().to_string(), "governance-relaxation"));
            break;
        }
    }
    out
}

// ─── Universal / weak evidence claims ────────────────────────────────────────

static UNIVERSAL_PATTERNS: OnceLock<Vec<Regex>> = OnceLock::new();

fn universal_patterns() -> &'static Vec<Regex> {
    UNIVERSAL_PATTERNS.get_or_init(|| {
        [
            r"(?i)\ball\s+projects?\s+should\b",
            r"(?i)\balways\s+works?\b",
            r"(?i)\bnever\s+fails?\b",
            r"(?i)\beveryone\s+should\b",
            r"(?i)\bregardless\s+of\b.{0,20}\b(?:project|size|context|domain)\b",
            r"(?i)\buniversally\s+(?:applicable|true|valid)\b",
        ]
        .iter()
        .map(|p| Regex::new(p).unwrap())
        .collect()
    })
}

fn universal_claim_findings(text: &str) -> Vec<(String, &'static str)> {
    let mut out = Vec::new();
    for re in universal_patterns() {
        if let Some(m) = re.find(text) {
            out.push((m.as_str().to_string(), "universal-claim"));
            break;
        }
    }
    out
}

// ─── Weak confidence (structural, not freetext) ──────────────────────────────

fn weak_confidence_findings(data: &serde_json::Value) -> Vec<Finding> {
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
                let mut f = Finding::advisory(format!("weak evidence for {level} confidence"));
                f.location = Some(format!("learnings/{}/confidence", index + 1));
                Some(f)
            } else {
                None
            }
        })
        .collect()
}

// ─── Freetext extraction (contributions-specific key set) ────────────────────

fn contributions_freetext(data: &serde_json::Value) -> Vec<String> {
    let mut values = Vec::new();
    collect_contributions_freetext(data, &mut values);
    values
}

fn collect_contributions_freetext(data: &serde_json::Value, out: &mut Vec<String>) {
    match data {
        serde_json::Value::Object(map) => {
            for (key, value) in map {
                if matches!(
                    key.as_str(),
                    "description"
                        | "text"
                        | "summary"
                        | "recommendation"
                        | "notes"
                        | "title"
                        | "customization_notes"
                        | "conclusion"
                ) {
                    if let Some(s) = value.as_str() {
                        out.push(s.to_string());
                    }
                } else if value.is_object() || value.is_array() {
                    collect_contributions_freetext(value, out);
                }
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                collect_contributions_freetext(item, out);
            }
        }
        _ => {}
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absence_fires_on_no_issues() {
        let findings = absence_findings("We merged the PR and had no issues with it.");
        assert!(!findings.is_empty(), "should detect 'no issues'");
    }

    #[test]
    fn absence_fires_on_worked_fine() {
        let findings = absence_findings("The approach worked fine for us.");
        assert!(!findings.is_empty(), "should detect 'worked fine'");
    }

    #[test]
    fn absence_clean_on_neutral_text() {
        let findings = absence_findings("We observed one incident and fixed it promptly.");
        assert!(findings.is_empty(), "should not fire on incident-reporting text");
    }

    #[test]
    fn relaxation_fires_on_skip_review() {
        let findings = relaxation_findings("We decided to skip the gate review for this batch.");
        assert!(!findings.is_empty(), "should detect 'skip ... review'");
    }

    #[test]
    fn relaxation_fires_on_unnecessary_ceremony() {
        let findings = relaxation_findings("The approval step is unnecessary overhead.");
        assert!(!findings.is_empty(), "should detect 'unnecessary overhead'");
    }

    #[test]
    fn relaxation_clean_on_neutral_text() {
        let findings = relaxation_findings("We added a new validation step to the gate.");
        assert!(findings.is_empty(), "should not fire on governance-strengthening text");
    }

    #[test]
    fn universal_fires_on_all_projects_should() {
        let findings = universal_claim_findings("All projects should adopt this pattern.");
        assert!(!findings.is_empty(), "should detect universal claim");
    }

    #[test]
    fn universal_fires_on_always_works() {
        let findings = universal_claim_findings("This technique always works for async flows.");
        assert!(!findings.is_empty(), "should detect 'always works'");
    }

    #[test]
    fn universal_clean_on_hedged_text() {
        let findings =
            universal_claim_findings("This worked in our context; other projects may differ.");
        assert!(findings.is_empty(), "should not fire on hedged language");
    }
}
