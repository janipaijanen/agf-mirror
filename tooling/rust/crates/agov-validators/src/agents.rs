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

/// Validate all agent YAML files in `dir` (excluding `MANIFEST.yaml` and
/// `MANIFEST.example.yaml`, neither of which is an agent definition file).
///
/// Mirrors `Agov.ValidateAgents.validate_all/1`. Each file is validated
/// against `agent.schema.json` and checked for a valid `agent.tier`.
pub fn validate_all(dir: &Path) -> CollectionResult {
    let results = yaml_files(dir)
        .into_iter()
        .filter(|path| {
            !matches!(
                path.file_name().and_then(std::ffi::OsStr::to_str),
                Some("MANIFEST.yaml" | "MANIFEST.example.yaml")
            )
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn fixture_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "agov-agents-validate-{name}-{}-{}",
            std::process::id(),
            std::thread::current().name().unwrap_or("test")
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create fixture dir");
        dir
    }

    /// Both MANIFEST.yaml (an adopter's own roster) and MANIFEST.example.yaml
    /// (the shipped template) are excluded from agent-definition validation
    /// -- neither is an `agent.schema.json` document. Includes a schema-
    /// invalid MANIFEST.example.yaml to prove the exclusion is by filename,
    /// not by accidentally happening to pass schema validation.
    #[test]
    fn manifest_and_manifest_example_are_excluded_from_agent_validation() {
        let dir = fixture_dir("exclusion");
        fs::write(dir.join("MANIFEST.yaml"), "agents:\n  coder-tester:\n    status: active\n")
            .expect("write MANIFEST.yaml");
        fs::write(
            dir.join("MANIFEST.example.yaml"),
            "agents:\n  coder-tester:\n    status: skipped\n",
        )
        .expect("write MANIFEST.example.yaml");

        let result = validate_all(&dir);

        assert!(
            result.ok_files.is_empty(),
            "expected zero ok files, got {:?}",
            result.ok_files
        );
        assert_eq!(
            result.file_errors.len(),
            0,
            "expected zero files considered (both MANIFEST files excluded)"
        );
    }

    /// Negative control: a real agent-definition-shaped file in the same
    /// directory is NOT excluded, proving the filter is scoped to the two
    /// MANIFEST filenames rather than accidentally skipping everything.
    #[test]
    fn non_manifest_file_in_same_directory_is_still_validated() {
        let dir = fixture_dir("negative-control");
        fs::write(dir.join("MANIFEST.example.yaml"), "agents:\n  coder-tester:\n    status: skipped\n")
            .expect("write MANIFEST.example.yaml");
        // Deliberately schema-invalid (missing required fields) -- this must
        // appear in file_errors, proving it was not silently excluded too.
        fs::write(dir.join("not-a-manifest.yaml"), "agent:\n  name: incomplete\n")
            .expect("write non-manifest file");

        let result = validate_all(&dir);

        assert!(result.ok_files.is_empty());
        assert_eq!(result.file_errors.len(), 1);
        assert!(result.file_errors[0].file.ends_with("not-a-manifest.yaml"));
    }
}
