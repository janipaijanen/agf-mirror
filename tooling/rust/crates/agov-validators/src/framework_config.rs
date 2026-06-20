use crate::{CollectionResult, FileError};
use agov_core::Finding;
use std::path::Path;

/// Validate a `framework.yaml` or equivalent config file.
///
/// Mirrors `Agov.ValidateFrameworkConfig.validate_file/1`. Validates against
/// the framework config schema.
pub fn validate_file(path: &Path) -> Result<Vec<Finding>, Vec<Finding>> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let data = crate::read_yaml_file(path)?;
    let mut errors = crate::validate_schema(&data, "framework.schema.json");
    let root = path.parent().unwrap_or_else(|| Path::new("."));
    let repo_version = std::fs::read_to_string(root.join("VERSION"))
        .ok()
        .map(|version| version.trim().to_string());
    let declared = data
        .get("contract_version")
        .and_then(serde_json::Value::as_str);

    if declared.is_some() && repo_version.is_some() && declared != repo_version.as_deref() {
        errors.push(Finding::blocking_at(
            "contract_version does not match VERSION",
            "contract_version",
        ));
    }

    if errors.is_empty() {
        Ok(Vec::new())
    } else {
        Err(errors)
    }
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
