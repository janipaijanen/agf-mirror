use agov_core::Finding;
use std::path::Path;

/// Validate a `framework.yaml` or equivalent config file.
///
/// Mirrors `Agov.ValidateFrameworkConfig.validate_file/1`. Validates against
/// the framework config schema.
pub fn validate_file(path: &Path) -> Result<Vec<Finding>, Vec<Finding>> {
    todo!()
}
