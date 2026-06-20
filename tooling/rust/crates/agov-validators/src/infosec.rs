use crate::CollectionResult;
use std::path::Path;

/// Scan learning record files in `dir` for InfoSec findings.
///
/// Mirrors `Agov.ValidateInfosec.validate_all/1`. Scans freetext fields
/// (description, text, summary, recommendation) for secret patterns.
pub fn validate_all(dir: &Path) -> CollectionResult {
    todo!()
}
