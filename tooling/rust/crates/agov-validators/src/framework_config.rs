use crate::{CollectionResult, FileError};
use agov_core::Finding;
use std::path::Path;

/// Validate a `framework.yaml` or equivalent config file.
///
/// Mirrors `Agov.ValidateFrameworkConfig.validate_file/1`. Validates against
/// the framework config schema.
pub fn validate_file(_path: &Path) -> Result<Vec<Finding>, Vec<Finding>> {
    todo!()
}

pub fn validate_all(root: &Path) -> CollectionResult {
    let path = root.join("framework.yaml");
    match validate_file(&path) {
        Ok(_findings) => CollectionResult {
            ok_files: vec![path.display().to_string()],
            file_errors: vec![],
        },
        Err(findings) => CollectionResult {
            ok_files: vec![],
            file_errors: vec![FileError {
                file: path.display().to_string(),
                findings,
            }],
        },
    }
}
