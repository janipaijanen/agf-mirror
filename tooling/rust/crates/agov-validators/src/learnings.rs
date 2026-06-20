use crate::CollectionResult;
use std::path::Path;

/// Validate all learning record YAML files in `dir`.
///
/// Mirrors `Agov.ValidateLearnings.validate_all/1`. Each file is validated
/// against `learning.schema.json`.
pub fn validate_all(_dir: &Path) -> CollectionResult {
    todo!()
}
