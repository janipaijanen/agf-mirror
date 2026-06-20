use crate::CollectionResult;
use std::path::Path;

/// Validate all agent YAML files in `dir` (excluding `MANIFEST.yaml`).
///
/// Mirrors `Agov.ValidateAgents.validate_all/1`. Each file is validated
/// against `agent.schema.json` and checked for a valid `agent.tier`.
pub fn validate_all(dir: &Path) -> CollectionResult {
    todo!()
}
