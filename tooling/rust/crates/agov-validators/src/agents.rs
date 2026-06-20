use crate::{split_results, validate_schema, yaml_files, CollectionResult};
use agov_core::Finding;
use std::path::Path;

const TIERS: &[&str] = &[
    "production",
    "governance",
    "cross-cutting",
    "publication-support",
    "meta",
];

/// Validate all agent YAML files in `dir` (excluding `MANIFEST.yaml`).
///
/// Mirrors `Agov.ValidateAgents.validate_all/1`. Each file is validated
/// against `agent.schema.json` and checked for a valid `agent.tier`.
pub fn validate_all(dir: &Path) -> CollectionResult {
    let results = yaml_files(dir)
        .into_iter()
        .filter(|path| path.file_name().and_then(std::ffi::OsStr::to_str) != Some("MANIFEST.yaml"))
        .map(|file| {
            let result = validate_file(&file);
            (file, result)
        })
        .collect();

    split_results(results)
}

fn validate_file(path: &Path) -> Result<serde_json::Value, Vec<Finding>> {
    let data = crate::read_yaml_file(path)?;
    let tier = data
        .pointer("/agent/tier")
        .and_then(serde_json::Value::as_str);

    let mut errors = validate_schema(&data, "agent.schema.json");
    if !tier.is_some_and(|tier| TIERS.contains(&tier)) {
        errors.push(Finding::blocking_at(
            "agent.tier is missing or invalid",
            "agent/tier",
        ));
    }

    if errors.is_empty() {
        Ok(data)
    } else {
        Err(errors)
    }
}
