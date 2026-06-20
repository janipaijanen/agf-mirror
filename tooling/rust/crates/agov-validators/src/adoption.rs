use crate::{CollectionResult, FileError};
use agov_core::Finding;
use std::path::Path;

/// Validate the adoption structure under `root`.
///
/// Mirrors `Agov.ValidateAdoption.validate/1`. Checks:
///   - `agents/` directory exists
///   - `agents/` contains at least one YAML file
///   - mandatory agents (infosec-sentinel, contribution-auditor) are present
///   - `agents/MANIFEST.yaml` exists
///   - `GOVERNANCE.md` or `framework.md` exists
pub fn validate(_root: &Path) -> Result<Vec<Finding>, Vec<Finding>> {
    todo!()
}

pub fn validate_all(root: &Path) -> CollectionResult {
    match validate(root) {
        Ok(_findings) => CollectionResult {
            ok_files: vec![root.display().to_string()],
            file_errors: vec![],
        },
        Err(findings) => CollectionResult {
            ok_files: vec![],
            file_errors: vec![FileError {
                file: root.display().to_string(),
                findings,
            }],
        },
    }
}
