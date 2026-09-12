//! Opt-in artifact registry. Existing coordination config keys are preserved
//! by ignoring unrelated root fields, while artifact-set declarations are strict.
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct ArtifactConfig {
    #[serde(default)]
    pub artifact_sets: Vec<ArtifactSet>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactSet {
    pub name: String,
    pub paths: Vec<String>,
    pub schema: String,
    pub id_pointers: Vec<String>,
    #[serde(default)]
    pub reference_pointers: Vec<String>,
    /// Only these references create dependency edges. Ordinary references can
    /// point back without falsely making the dependency graph cyclic.
    #[serde(default)]
    pub dependency_pointers: Vec<String>,
    #[serde(default)]
    pub uniqueness_scope: UniquenessScope,
}
/// V1 validates IDs across the complete registry. Other namespace semantics
/// require a later contract rather than silently weakening global uniqueness.
#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UniquenessScope {
    #[default]
    Global,
}
impl ArtifactConfig {
    pub fn parse(bytes: &[u8]) -> Result<Self, Vec<String>> {
        let config: Self = serde_norway::from_slice(bytes).map_err(|e| vec![e.to_string()])?;
        let errors = config.validate();
        if errors.is_empty() {
            Ok(config)
        } else {
            Err(errors)
        }
    }
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();
        let mut names = BTreeSet::new();
        for (i, set) in self.artifact_sets.iter().enumerate() {
            let prefix = format!("artifact_sets/{i}");
            if set.name.trim().is_empty() || !names.insert(&set.name) {
                errors.push(format!("{prefix}/name: must be nonempty and unique"));
            }
            if set.paths.is_empty() {
                errors.push(format!("{prefix}/paths: must not be empty"));
            }
            for path in &set.paths {
                if !contained_relative_path(path) {
                    errors.push(format!(
                        "{prefix}/paths: invalid repository-relative glob {path:?}"
                    ));
                }
            }
            if !contained_relative_path(&set.schema) || set.schema.contains(['*', '?', '[', ']']) {
                errors.push(format!(
                    "{prefix}/schema: must name one repository-relative schema file"
                ));
            }
            if set.id_pointers.is_empty() {
                errors.push(format!("{prefix}/id_pointers: must not be empty"));
            }
            for (field, pointers) in [
                ("id_pointers", &set.id_pointers),
                ("reference_pointers", &set.reference_pointers),
                ("dependency_pointers", &set.dependency_pointers),
            ] {
                let mut seen = BTreeSet::new();
                for pointer in pointers {
                    if !json_pointer(pointer) || !seen.insert(pointer) {
                        errors.push(format!(
                            "{prefix}/{field}: invalid or duplicate JSON Pointer {pointer:?}"
                        ));
                    }
                }
            }
            for pointer in &set.dependency_pointers {
                if !set.reference_pointers.contains(pointer) {
                    errors.push(format!("{prefix}/dependency_pointers: {pointer:?} must also be a declared reference pointer"));
                }
            }
        }
        errors.sort();
        errors.dedup();
        errors
    }
}

/// Lexical containment is necessary but not sufficient: the filesystem reader
/// must additionally reject symlink escapes for matches and schema references.
pub fn contained_relative_path(value: &str) -> bool {
    !value.is_empty()
        && !value.contains(['\\', ':', '\0'])
        && !value.chars().any(char::is_control)
        && value
            .split('/')
            .all(|part| !part.is_empty() && part != "." && part != "..")
}
/// RFC 6901 syntax, including the root pointer and escaped literal tokens.
/// Wildcard expansion is not part of JSON Pointer syntax.
pub fn json_pointer(value: &str) -> bool {
    if value.is_empty() {
        return true;
    }
    if !value.starts_with('/') {
        return false;
    }
    let mut chars = value.chars();
    while let Some(ch) = chars.next() {
        if ch == '~' && !matches!(chars.next(), Some('0' | '1')) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    const VALID: &str = "artifact_sets:\n  - name: tasks\n    paths: ['tasks/*.yaml']\n    schema: schemas/task.json\n    id_pointers: ['/id']\n    reference_pointers: ['/depends_on']\n    dependency_pointers: ['/depends_on']\n";
    #[test]
    fn optional_registry_does_not_change_existing_coordination_configuration() {
        let config = ArtifactConfig::parse(
            b"from_aliases: {claude: planner-architect}\ncurrent_role: coder-tester\n",
        )
        .unwrap();
        assert!(config.artifact_sets.is_empty());
        let config = ArtifactConfig::parse(VALID.as_bytes()).unwrap();
        assert_eq!(
            config.artifact_sets[0].uniqueness_scope,
            UniquenessScope::Global
        );
    }
    #[test]
    fn declaration_typos_and_unknown_uniqueness_semantics_fail() {
        for input in [
            format!("{VALID}    unknown: value\n"),
            format!("{VALID}    uniqueness_scope: local\n"),
            VALID.replace("id_pointers:", "id_pointer:"),
        ] {
            assert!(ArtifactConfig::parse(input.as_bytes()).is_err(), "{input}");
        }
    }
    #[test]
    fn traversal_absolute_remote_and_nonportable_paths_fail_lexically() {
        for path in [
            "",
            "/tmp/a",
            "../a",
            "a/../b",
            "./a",
            "a//b",
            "a/",
            "C:/a",
            "a\\b",
            "https://example/schema.json",
            "a\0b",
            "a\nb",
        ] {
            assert!(!contained_relative_path(path), "{path:?}");
        }
        for path in ["tasks/*.yaml", "tasks/**/record.json", "schemas/task.json"] {
            assert!(contained_relative_path(path));
        }
    }
    #[test]
    fn pointers_use_exact_rfc6901_syntax() {
        for value in ["", "/id", "/items/0/id", "/a~1b/~0", "/"] {
            assert!(json_pointer(value), "{value}");
        }
        for value in ["id", "#/id", "/a~2b", "/a~", "/a~~0"] {
            assert!(!json_pointer(value), "{value}");
        }
    }
    #[test]
    fn malformed_registry_reports_all_structural_config_errors() {
        let mut config = ArtifactConfig::parse(VALID.as_bytes()).unwrap();
        config.artifact_sets.push(config.artifact_sets[0].clone());
        config.artifact_sets[0].paths = vec![];
        config.artifact_sets[0].schema = "../schema.json".into();
        config.artifact_sets[0].id_pointers = vec!["bad".into()];
        config.artifact_sets[0].dependency_pointers = vec!["/unregistered".into()];
        let errors = config.validate();
        assert_eq!(errors.len(), 5, "{errors:?}");
    }
}
