use crate::readable_id::{
    is_readable_id, parse_readable_id, readable_timestamp_from_iso, role_slug,
};
use agov_core::{schema, yaml};
use std::path::Path;

/// A parsed and schema-valid coordination message (frontmatter only).
///
/// Fields are `serde_json::Value` to avoid coupling the struct to schema evolution.
/// Codex may replace with a typed struct once the schema is stable.
#[derive(Debug, Clone)]
pub struct CoordMessage {
    pub raw: serde_json::Value,
}

impl CoordMessage {
    pub fn id(&self) -> Option<&str> {
        self.raw.get("id").and_then(|v| v.as_str())
    }

    pub fn kind(&self) -> Option<&str> {
        self.raw.get("kind").and_then(|v| v.as_str())
    }

    pub fn from(&self) -> Option<&str> {
        self.raw.get("from").and_then(|v| v.as_str())
    }

    pub fn to(&self) -> Option<&str> {
        self.raw.get("to").and_then(|v| v.as_str())
    }

    pub fn topic(&self) -> Option<&str> {
        self.raw.get("topic").and_then(|v| v.as_str())
    }

    pub fn timestamp(&self) -> Option<&str> {
        self.raw.get("timestamp").and_then(|v| v.as_str())
    }

    pub fn requires_ack(&self) -> bool {
        self.raw
            .get("requires-ack")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    /// Normalised `references-to` filenames (bare IDs resolved to `<id>.md`).
    pub fn references_to(&self) -> Vec<String> {
        self.raw
            .get("references-to")
            .and_then(|v| v.as_array())
            .into_iter()
            .flatten()
            .filter_map(|v| v.as_str())
            .filter_map(normalize_coord_reference)
            .collect()
    }
}

/// Normalise a single `references-to` entry to a coordination filename.
///
/// Strips path components; appends `.md` to bare IDs; rejects strings that
/// don't look like coordination filenames.
pub fn normalize_coord_reference(reference: &str) -> Option<String> {
    let filename = std::path::Path::new(reference).file_name()?.to_str()?;

    if filename.ends_with(".md") {
        if looks_like_coord_filename(filename) {
            Some(filename.to_string())
        } else {
            None
        }
    } else {
        let candidate = format!("{filename}.md");
        if looks_like_coord_filename(&candidate) {
            Some(candidate)
        } else {
            None
        }
    }
}

/// Returns true when a filename looks like a coordination message file.
///
/// Includes a legacy-compatibility branch for the 20-character
/// Crockford-alphabet ID form briefly produced by a broken generator in
/// June 2026 (e.g. `01KVK7J9N2AW8DJZQAG2`). That form is recognized here
/// **only** so references to those immutable historical files resolve
/// correctly — it is never generated and must never be treated as a
/// currently valid ID format elsewhere.
pub fn looks_like_coord_filename(filename: &str) -> bool {
    let stem = filename.strip_suffix(".md").unwrap_or(filename);
    let first = stem.split('-').next().unwrap_or("");

    // Sequence-numbered: `00001-role.md`
    first.len() == 5 && first.chars().all(|ch| ch.is_ascii_digit())
        // ULID-prefixed: 26-char Crockford base32
        || first.len() == 26
            && first.chars().all(|ch| {
                matches!(ch, '0'..='9' | 'A'..='H' | 'J'..='K' | 'M'..='N' | 'P'..='T' | 'V'..='Z')
            })
        // Legacy 20-char Crockford form: reference compatibility only
        || first.len() == 20
            && first.chars().all(|ch| {
                matches!(ch, '0'..='9' | 'A'..='H' | 'J'..='K' | 'M'..='N' | 'P'..='T' | 'V'..='Z')
            })
        // Readable timestamp: `20260623T181212Z-…`
        || stem.len() >= 17
            && stem.as_bytes().get(8) == Some(&b'T')
            && stem.as_bytes().get(15) == Some(&b'Z')
}

/// Validate a coordination message file.
///
/// Mirrors `Agov.CoordValidator.validate_file/1`:
///   - Read file
///   - Extract YAML frontmatter
///   - Parse YAML → JSON
///   - Validate against `coordination-message.schema.json`
///
/// Returns `Ok(CoordMessage)` on pass, `Err(errors)` on any failure.
/// Errors are `(message, json_pointer)` pairs matching Elixir output.
pub fn validate_file(path: &Path) -> Result<CoordMessage, Vec<(String, String)>> {
    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return Err(vec![("file not found".to_string(), String::new())]);
        }
        Err(err) => return Err(vec![(format!("{err:?}"), String::new())]),
    };

    validate_content(&content)
}

/// Validate Git blob or file contents through the same channel schema.
pub fn validate_content(content: &str) -> Result<CoordMessage, Vec<(String, String)>> {
    let schema_path = agov_core::schema_dir().join("coordination-message.schema.json");
    let compiled = schema::load_and_compile(&schema_path)
        .map_err(|err| vec![(format!("{err:?}"), schema_path.display().to_string())])?;
    validate_content_with_schema(content, &compiled)
}

/// Reuse a compiled channel schema for a bounded Git snapshot scan.
pub fn validate_content_with_schema(
    content: &str,
    compiled: &schema::CompiledSchema,
) -> Result<CoordMessage, Vec<(String, String)>> {
    let raw = match yaml::parse_frontmatter_json(content) {
        Ok(value) => value,
        Err(yaml::YamlError::NoFrontmatter) => {
            return Err(vec![(
                "no YAML front matter found".to_string(),
                String::new(),
            )]);
        }
        Err(err) => return Err(vec![(format!("{err:?}"), String::new())]),
    };

    let errors = schema::validate(compiled, &raw);

    if errors.is_empty() {
        crate::publication_hold::validate_fields(&raw)?;
        crate::challenge::validate_fields(&raw)?;
        Ok(CoordMessage { raw })
    } else {
        Err(errors
            .into_iter()
            .map(|err| (err.message, err.pointer))
            .collect())
    }
}

/// Return advisory warning strings for a valid coordination message.
///
/// Mirrors `Agov.CoordValidator.advisory_findings/1`. Advisory findings do not
/// affect exit codes.
pub fn advisory_findings(msg: &CoordMessage) -> Vec<String> {
    advisory_findings_for_path(msg, None)
}

pub fn advisory_findings_for_path(msg: &CoordMessage, path: Option<&Path>) -> Vec<String> {
    let mut findings = Vec::new();

    if msg.raw.get("model").is_none() {
        findings.push(
            "recommended field 'model' not set; provenance tracking is weaker without it"
                .to_string(),
        );
    }

    if msg.raw.get("seq").is_some() {
        findings.push(
            "'seq' is deprecated; remove it — causal ordering belongs in references-to and timestamp"
                .to_string(),
        );
    }

    add_agent_manual_id_finding(&mut findings, msg);

    if let Some(id) = msg.id().filter(|id| is_readable_id(id)) {
        add_readable_drift_findings(&mut findings, msg, id, path);
    }

    findings
}

fn is_ulid(id: &str) -> bool {
    id.len() == 26
        && id
            .chars()
            .all(|ch| matches!(ch, '0'..='9' | 'A'..='H' | 'J'..='K' | 'M'..='N' | 'P'..='T' | 'V'..='Z'))
}

fn add_readable_drift_findings(
    findings: &mut Vec<String>,
    msg: &CoordMessage,
    id: &str,
    path: Option<&Path>,
) {
    let Some(parts) = parse_readable_id(id) else {
        return;
    };

    if let Some(from) = msg.raw.get("from").and_then(|value| value.as_str()) {
        let expected = role_slug(from);
        if parts.role != expected {
            findings.push(format!(
                "readable id role '{}' differs from from '{from}' (expected slug '{expected}')",
                parts.role
            ));
        }
    }

    if let Some(timestamp) = msg.raw.get("timestamp").and_then(|value| value.as_str()) {
        if let Some(frontmatter_timestamp) = readable_timestamp_from_iso(timestamp) {
            if parts.timestamp != frontmatter_timestamp {
                findings.push(format!(
                    "readable id timestamp '{}' differs from frontmatter timestamp '{frontmatter_timestamp}'",
                    parts.timestamp
                ));
            }
        }
    }

    let Some(path) = path else {
        return;
    };

    let Some(basename) = path.file_name().and_then(|value| value.to_str()) else {
        return;
    };

    if !basename.ends_with(".md") {
        return;
    }

    let expected = format!("{id}.md");
    if basename == expected {
        return;
    }

    if basename.starts_with(&format!("{id}-")) {
        findings.push(format!(
            "readable filename '{basename}' repeats sender suffix; expected '{expected}'"
        ));
        return;
    }

    findings.push(format!(
        "readable filename '{basename}' does not match expected '{expected}'"
    ));
}

fn add_agent_manual_id_finding(findings: &mut Vec<String>, msg: &CoordMessage) {
    let Some(id) = msg.id() else {
        return;
    };
    if is_readable_id(id) || is_ulid(id) {
        return;
    }

    let Some(from) = msg.raw.get("from").and_then(|value| value.as_str()) else {
        return;
    };

    if ai_agent_role(from) {
        findings.push(format!(
            "AI agent role '{from}' used manual coordination id '{id}'; use agov coord brief --id-format readable unless tooling is unavailable"
        ));
    }
}

fn ai_agent_role(role: &str) -> bool {
    let manifest = match std::fs::read_to_string(agov_core::manifest_path()) {
        Ok(content) => content,
        Err(_) => return false,
    };
    let data: serde_json::Value = match serde_norway::from_str::<serde_json::Value>(&manifest) {
        Ok(data) => data,
        Err(_) => return false,
    };

    data.get("agents")
        .and_then(|value| value.get(role))
        .and_then(|value| value.get("played_by"))
        .and_then(|value| value.as_str())
        .map(|played_by| {
            let played_by = played_by.to_ascii_lowercase();
            played_by.contains("ai") || played_by.contains("claude")
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn message(id: &str, from: &str) -> CoordMessage {
        CoordMessage {
            raw: serde_json::json!({
                "id": id,
                "from": from,
                "timestamp": "2026-06-20T18:39:14Z",
                "model": "codex-gpt-5"
            }),
        }
    }

    #[test]
    fn readable_filename_without_sender_suffix_has_no_filename_warning() {
        let msg = message("20260620T183914Z-coder-tester-r7k2", "coder-tester");
        let findings = advisory_findings_for_path(
            &msg,
            Some(Path::new("20260620T183914Z-coder-tester-r7k2.md")),
        );

        assert!(!findings.iter().any(|finding| finding.contains("filename")));
    }

    #[test]
    fn readable_filename_with_sender_suffix_warns() {
        let msg = message("20260620T183914Z-coder-tester-r7k2", "coder-tester");
        let findings = advisory_findings_for_path(
            &msg,
            Some(Path::new(
                "20260620T183914Z-coder-tester-r7k2-coder-tester.md",
            )),
        );

        assert!(findings
            .iter()
            .any(|finding| finding.contains("repeats sender suffix")));
        assert!(findings
            .iter()
            .any(|finding| finding.contains("20260620T183914Z-coder-tester-r7k2.md")));
    }

    #[test]
    fn ai_agent_manual_id_warns() {
        let msg = message("manual-recovery-1", "coder-tester");
        let findings = advisory_findings_for_path(&msg, Some(Path::new("manual-recovery-1.md")));

        assert!(findings
            .iter()
            .any(|finding| finding
                .contains("AI agent role 'coder-tester' used manual coordination id")));
    }

    #[test]
    fn human_governor_manual_id_does_not_warn() {
        let msg = message("manual-governor-decision", "governor-reviewer");
        let findings =
            advisory_findings_for_path(&msg, Some(Path::new("manual-governor-decision.md")));

        assert!(!findings
            .iter()
            .any(|finding| finding.contains("manual coordination id")));
    }

    #[test]
    fn legacy_20_char_crockford_filename_is_recognized() {
        // Real historical filename from the June 2026 broken-generator window.
        assert!(looks_like_coord_filename(
            "01KVK7J9N2AW8DJZQAG2-planner-architect.md"
        ));
        assert!(looks_like_coord_filename(
            "01KVK7DYM21GGQ7ZZ5RR-planner-architect.md"
        ));
    }

    #[test]
    fn legacy_20_char_reference_normalizes_with_and_without_md_suffix() {
        assert_eq!(
            normalize_coord_reference("01KVK7J9N2AW8DJZQAG2-planner-architect"),
            Some("01KVK7J9N2AW8DJZQAG2-planner-architect.md".to_string())
        );
        assert_eq!(
            normalize_coord_reference("01KVK7J9N2AW8DJZQAG2-planner-architect.md"),
            Some("01KVK7J9N2AW8DJZQAG2-planner-architect.md".to_string())
        );
        assert_eq!(
            normalize_coord_reference(
                "docs/coordination/01KVK7J9N2AW8DJZQAG2-planner-architect.md"
            ),
            Some("01KVK7J9N2AW8DJZQAG2-planner-architect.md".to_string())
        );
    }

    #[test]
    fn near_length_20_char_variants_are_rejected() {
        // 19 chars — one short.
        assert!(!looks_like_coord_filename(
            "1KVK7J9N2AW8DJZQAG2-planner-architect.md"
        ));
        // 21 chars — one long.
        assert!(!looks_like_coord_filename(
            "001KVK7J9N2AW8DJZQAG2-planner-architect.md"
        ));
    }

    #[test]
    fn non_crockford_20_char_first_segment_is_rejected() {
        // 'I', 'L', 'O', 'U' are excluded from the Crockford base32 alphabet.
        assert!(!looks_like_coord_filename(
            "0IKVK7J9N2AW8DJZQAG2-planner-architect.md"
        ));
        assert!(!looks_like_coord_filename(
            "0LKVK7J9N2AW8DJZQAG2-planner-architect.md"
        ));
        assert!(!looks_like_coord_filename(
            "0OKVK7J9N2AW8DJZQAG2-planner-architect.md"
        ));
        assert!(!looks_like_coord_filename(
            "0UKVK7J9N2AW8DJZQAG2-planner-architect.md"
        ));
    }

    #[test]
    fn current_formats_remain_accepted_after_legacy_addition() {
        // Sequence-numbered.
        assert!(looks_like_coord_filename("00001-coder-tester.md"));
        // Valid 26-char ULID.
        assert!(looks_like_coord_filename(
            "01ARZ3NDEKTSV4RRFFQ69G5FAV-coder-tester.md"
        ));
        // Readable timestamp ID.
        assert!(looks_like_coord_filename(
            "20260623T181212Z-coder-tester-r7k2.md"
        ));
    }
}
