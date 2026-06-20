use crate::CollectionResult;
use std::path::Path;

/// Validate contribution-auditor findings in learning records under `dir`.
///
/// Mirrors `Agov.ValidateContributions.validate_all/1`. Returns advisory
/// (non-blocking) findings — maps to the `:warnings` reporter variant.
pub fn validate_all(_dir: &Path) -> CollectionResult {
    todo!()
}
