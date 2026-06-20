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

    findings
}
