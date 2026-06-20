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
        self.raw.get("id").and_then(|value| value.as_str())
    }
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

    let raw = match yaml::parse_frontmatter_json(&content) {
        Ok(value) => value,
        Err(yaml::YamlError::NoFrontmatter) => {
            return Err(vec![(
                "no YAML front matter found".to_string(),
                String::new(),
            )]);
        }
        Err(err) => return Err(vec![(format!("{err:?}"), String::new())]),
    };

    let schema_path = agov_core::schema_dir().join("coordination-message.schema.json");
    let compiled = schema::load_and_compile(&schema_path)
        .map_err(|err| vec![(format!("{err:?}"), schema_path.display().to_string())])?;
    let errors = schema::validate(&compiled, &raw);

    if errors.is_empty() {
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

    if let Some(id) = msg.id().filter(|id| is_readable_id(id)) {
        add_readable_drift_findings(&mut findings, msg, id, path);
    }

    findings
}

fn is_readable_id(id: &str) -> bool {
    readable_parts(id).is_some()
}

fn readable_parts(id: &str) -> Option<(&str, &str, &str)> {
    let (timestamp, rest) = id.split_once('-')?;
    if timestamp.len() != 16
        || !timestamp.ends_with('Z')
        || !timestamp[..8].chars().all(|ch| ch.is_ascii_digit())
        || &timestamp[8..9] != "T"
        || !timestamp[9..15].chars().all(|ch| ch.is_ascii_digit())
    {
        return None;
    }

    let (role, suffix) = rest.rsplit_once('-')?;
    if role.is_empty()
        || !role
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')
        || !role
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit())
        || !(4..=6).contains(&suffix.len())
        || !suffix
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit())
    {
        return None;
    }

    Some((timestamp, role, suffix))
}

fn add_readable_drift_findings(
    findings: &mut Vec<String>,
    msg: &CoordMessage,
    id: &str,
    path: Option<&Path>,
) {
    let Some((id_timestamp, id_role, _suffix)) = readable_parts(id) else {
        return;
    };

    if let Some(from) = msg.raw.get("from").and_then(|value| value.as_str()) {
        let expected = role_slug(from);
        if id_role != expected {
            findings.push(format!(
                "readable id role '{id_role}' differs from from '{from}' (expected slug '{expected}')"
            ));
        }
    }

    if let Some(timestamp) = msg.raw.get("timestamp").and_then(|value| value.as_str()) {
        if let Some(frontmatter_timestamp) = basic_utc_timestamp(timestamp) {
            if id_timestamp != frontmatter_timestamp {
                findings.push(format!(
                    "readable id timestamp '{id_timestamp}' differs from frontmatter timestamp '{frontmatter_timestamp}'"
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

    let prefix = format!("{id}-");
    if !basename.starts_with(&prefix) {
        findings.push(format!(
            "readable filename does not start with frontmatter id '{id}'"
        ));
        return;
    }

    let filename_sender = basename
        .trim_end_matches(".md")
        .strip_prefix(&prefix)
        .unwrap_or_default();

    if let Some(from) = msg.raw.get("from").and_then(|value| value.as_str()) {
        if filename_sender != from {
            findings.push(format!(
                "filename sender '{filename_sender}' differs from frontmatter from '{from}'"
            ));
        }
    }
}

fn role_slug(value: &str) -> String {
    let mut slug = String::new();
    let mut previous_dash = false;

    for ch in value.chars().flat_map(|ch| ch.to_lowercase()) {
        if ch.is_ascii_lowercase() || ch.is_ascii_digit() {
            slug.push(ch);
            previous_dash = false;
        } else if !previous_dash {
            slug.push('-');
            previous_dash = true;
        }
    }

    let trimmed = slug.trim_matches('-');
    if trimmed.is_empty() {
        "unknown".to_string()
    } else {
        trimmed.to_string()
    }
}

fn basic_utc_timestamp(timestamp: &str) -> Option<String> {
    if timestamp.len() >= 20
        && timestamp.as_bytes().get(4) == Some(&b'-')
        && timestamp.as_bytes().get(7) == Some(&b'-')
        && timestamp.as_bytes().get(10) == Some(&b'T')
        && timestamp.as_bytes().get(13) == Some(&b':')
        && timestamp.as_bytes().get(16) == Some(&b':')
        && timestamp.ends_with('Z')
    {
        Some(format!(
            "{}{}{}T{}{}{}Z",
            &timestamp[0..4],
            &timestamp[5..7],
            &timestamp[8..10],
            &timestamp[11..13],
            &timestamp[14..16],
            &timestamp[17..19]
        ))
    } else {
        None
    }
}
