use crate::{path_label, CollectionResult, FileError};
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
pub fn validate(root: &Path) -> Result<Vec<Finding>, Vec<Finding>> {
    let agents_dir = root.join("agents");
    let mut errors = Vec::new();

    if !agents_dir.is_dir() {
        errors.push(Finding::blocking_at(
            "agents/ directory is missing",
            "agents",
        ));
    }

    let agent_files = agent_files(&agents_dir);
    if agent_files.is_empty() {
        errors.push(Finding::blocking_at(
            "agents/ has no agent YAML files",
            "agents",
        ));
    }

    let present = present_agent_names(&agent_files);
    for mandatory in ["contribution-auditor", "infosec-sentinel"] {
        if !present.iter().any(|agent| agent == mandatory) {
            errors.push(Finding::blocking_at(
                format!("mandatory agent missing: {mandatory}"),
                "agents",
            ));
        }
    }

    if !(root.join("GOVERNANCE.md").exists() || root.join("framework.md").exists()) {
        errors.push(Finding::blocking_at(
            "GOVERNANCE.md or framework.md is missing",
            "GOVERNANCE.md",
        ));
    }

    if !agents_dir.join("MANIFEST.yaml").exists() {
        errors.push(Finding::blocking_at(
            "agents/MANIFEST.yaml is missing",
            "agents/MANIFEST.yaml",
        ));
    }

    if errors.is_empty() {
        Ok(Vec::new())
    } else {
        Err(errors)
    }
}

pub fn validate_all(root: &Path) -> CollectionResult {
    match validate(root) {
        Ok(_findings) => CollectionResult {
            ok_files: vec![path_label(root)],
            file_errors: vec![],
        },
        Err(findings) => CollectionResult {
            ok_files: vec![],
            file_errors: vec![FileError {
                file: path_label(root),
                findings,
            }],
        },
    }
}

fn agent_files(agents_dir: &Path) -> Vec<std::path::PathBuf> {
    crate::yaml_files(agents_dir)
        .into_iter()
        .filter(|path| path.file_name().and_then(std::ffi::OsStr::to_str) != Some("MANIFEST.yaml"))
        .collect()
}

fn present_agent_names(files: &[std::path::PathBuf]) -> Vec<String> {
    files
        .iter()
        .filter_map(|file| crate::read_yaml_file(file).ok())
        .flat_map(|data| {
            let name = data
                .pointer("/agent/name")
                .and_then(serde_json::Value::as_str)
                .map(ToOwned::to_owned);
            let id_segment = data
                .pointer("/agent/id")
                .and_then(serde_json::Value::as_str)
                .and_then(|id| id.split(':').nth(1))
                .map(ToOwned::to_owned);
            [name, id_segment].into_iter().flatten()
        })
        .collect()
}
