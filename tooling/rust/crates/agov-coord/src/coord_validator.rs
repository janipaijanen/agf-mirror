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
    todo!()
}

/// Return advisory warning strings for a valid coordination message.
///
/// Mirrors `Agov.CoordValidator.advisory_findings/1`. Advisory findings do not
/// affect exit codes.
pub fn advisory_findings(msg: &CoordMessage) -> Vec<String> {
    todo!()
}
