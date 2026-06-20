use std::path::Path;

/// A parsed and schema-valid Gate 1 plan document (frontmatter only).
#[derive(Debug, Clone)]
pub struct PlanDocument {
    pub raw: serde_json::Value,
}

/// Validate a Gate 1 plan file.
///
/// Mirrors `Agov.PlanValidator.validate_file/1`:
///   - Read file → extract frontmatter → parse YAML → validate schema
///   - Validate semantics: required advisory roles present, no unknown roles,
///     no duplicate roles.
///
/// Reads allowed roles from `agf/agents/MANIFEST.yaml` via `$AGOV_MANIFEST_PATH`.
/// Falls back to the two required roles when the manifest is unreadable.
pub fn validate_file(path: &Path) -> Result<PlanDocument, Vec<(String, String)>> {
    todo!()
}
