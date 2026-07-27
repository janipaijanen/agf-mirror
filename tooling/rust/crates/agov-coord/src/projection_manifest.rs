//! Validation for the structured six-surface evidence manifest required by
//! `agf/rules/workflow/documentation-rule-projection-rule.md`.
//!
//! Mechanically checks only the objective omissions the rule authorizes:
//! the manifest parses and matches its schema, every declared `changed_pages_or_runbooks`
//! entry names a real repository file, and every declared `path`/`paths` on a
//! surface entry (including `not_applicable` sub-entries) resolves to a real
//! repository file. It does not and cannot judge whether a disposition is
//! semantically correct, whether a pointer is sufficient, or whether
//! `not_applicable` is justified -- that stays human/role judgment per the
//! rule.

use agov_core::{schema, yaml};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ProjectionManifest {
    pub raw: serde_json::Value,
}

/// Validate a rule-projection evidence manifest file against its schema.
/// `repo_root` is used to resolve every declared path for existence.
pub fn validate_file(
    path: &Path,
    repo_root: &Path,
) -> Result<ProjectionManifest, Vec<(String, String)>> {
    let raw = yaml::read_yaml_file_json(path).map_err(|err| match err {
        yaml::YamlError::Io(message) if message == "file not found" => {
            vec![("file not found".to_string(), String::new())]
        }
        err => vec![(format!("{err:?}"), String::new())],
    })?;

    let schema_path = agov_core::schema_dir().join("rule-projection-manifest.schema.json");
    let compiled = schema::load_and_compile(&schema_path)
        .map_err(|err| vec![(format!("{err:?}"), schema_path.display().to_string())])?;
    let mut errors: Vec<(String, String)> = schema::validate(&compiled, &raw)
        .into_iter()
        .map(|err| (err.message, err.pointer))
        .collect();

    if errors.is_empty() {
        errors.extend(validate_paths_resolve(&raw, repo_root));
    }

    if errors.is_empty() {
        Ok(ProjectionManifest { raw })
    } else {
        Err(errors)
    }
}

/// Validate objective claims that require repository comparison context in
/// addition to schema/path validation.
pub fn validate_file_against<F>(
    path: &Path,
    repo_root: &Path,
    changed_pages_or_runbooks: &BTreeSet<String>,
    ships_in_distribution: F,
) -> Result<ProjectionManifest, Vec<(String, String)>>
where
    F: Fn(&str) -> bool,
{
    let manifest = validate_file(path, repo_root)?;
    let mut errors = validate_changed_files(&manifest.raw, changed_pages_or_runbooks);
    errors.extend(validate_distribution(&manifest.raw, ships_in_distribution));
    if errors.is_empty() {
        Ok(manifest)
    } else {
        Err(errors)
    }
}

fn validate_changed_files(
    raw: &serde_json::Value,
    actual: &BTreeSet<String>,
) -> Vec<(String, String)> {
    let declared = raw
        .get("changed_pages_or_runbooks")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .filter_map(|value| value.as_str())
        .collect::<BTreeSet<_>>();

    actual
        .iter()
        .filter(|path| !declared.contains(path.as_str()))
        .map(|path| {
            (
                format!(
                    "changed Pages/runbook file is missing from changed_pages_or_runbooks: {path}"
                ),
                "/changed_pages_or_runbooks".to_string(),
            )
        })
        .collect()
}

fn validate_distribution<F>(raw: &serde_json::Value, ships: F) -> Vec<(String, String)>
where
    F: Fn(&str) -> bool,
{
    raw.pointer("/surfaces/distribution/paths")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .enumerate()
        .filter_map(|(index, value)| value.as_str().map(|path| (index, path)))
        .filter(|(_, path)| !ships(path))
        .map(|(index, path)| {
            (
                format!("declared shipped surface is absent from distribution input: {path}"),
                format!("/surfaces/distribution/paths/{index}"),
            )
        })
        .collect()
}

fn validate_paths_resolve(raw: &serde_json::Value, repo_root: &Path) -> Vec<(String, String)> {
    let mut errors = Vec::new();

    if let Some(changed) = raw
        .get("changed_pages_or_runbooks")
        .and_then(|value| value.as_array())
    {
        for (index, entry) in changed.iter().enumerate() {
            if let Some(relative) = entry.as_str() {
                if !resolves(repo_root, relative) {
                    errors.push((
                        format!("changed_pages_or_runbooks[{index}] does not resolve to a real file: {relative}"),
                        format!("/changed_pages_or_runbooks/{index}"),
                    ));
                }
            }
        }
    }

    if let Some(surfaces) = raw.get("surfaces").and_then(|value| value.as_object()) {
        for (surface_name, entry) in surfaces {
            check_surface_paths(surface_name, entry, repo_root, &mut errors);
        }
    }

    errors
}

fn check_surface_paths(
    surface_name: &str,
    entry: &serde_json::Value,
    repo_root: &Path,
    errors: &mut Vec<(String, String)>,
) {
    if let Some(path) = entry.get("path").and_then(|value| value.as_str()) {
        if !resolves(repo_root, path) {
            errors.push((
                format!("surfaces.{surface_name}.path does not resolve to a real file: {path}"),
                format!("/surfaces/{surface_name}/path"),
            ));
        }
    }

    if let Some(paths) = entry.get("paths").and_then(|value| value.as_array()) {
        for (index, path) in paths.iter().enumerate() {
            if let Some(path) = path.as_str() {
                if !resolves(repo_root, path) {
                    errors.push((
                        format!(
                            "surfaces.{surface_name}.paths[{index}] does not resolve to a real file: {path}"
                        ),
                        format!("/surfaces/{surface_name}/paths/{index}"),
                    ));
                }
            }
        }
    }

    // `not_applicable` sub-entries are explicitly allowed to name a path
    // that does not resolve -- that is often the entire point (e.g. a
    // loader file this project does not ship). Schema validation already
    // enforces `path`/`reason` presence; no existence check here.
}

fn resolves(repo_root: &Path, relative: &str) -> bool {
    let candidate: PathBuf = repo_root.join(relative);
    candidate.is_file()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_root() -> PathBuf {
        // 4 levels up from this crate's manifest dir:
        // tooling/rust/crates/agov-coord -> repo root.
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../../../")
            .canonicalize()
            .unwrap()
    }

    fn write_manifest(dir: &Path, content: &str) -> PathBuf {
        let path = dir.join("manifest.yaml");
        std::fs::write(&path, content).unwrap();
        path
    }

    fn surface(disposition: &str, path: &str) -> String {
        format!("disposition: {disposition}\n    path: {path}\n    reason: test reason")
    }

    #[test]
    fn valid_manifest_with_real_paths_passes() {
        let dir = tempfile::tempdir().unwrap();
        let manifest = write_manifest(
            dir.path(),
            &format!(
                r#"
plan_id: test-plan
changed_pages_or_runbooks:
  - AGENTS.md
surfaces:
  normative_owner:
    {}
  adopter_facing_explanation:
    {}
  runbook_situation_map:
    {}
  ai_landing_loaders:
    {}
  validation_drift_evidence:
    {}
  distribution:
    disposition: confirmed
    paths:
      - AGENTS.md
    reason: test reason
"#,
                surface("updated", "AGENTS.md"),
                surface("updated", "AGENTS.md"),
                surface("updated", "AGENTS.md"),
                surface("updated", "AGENTS.md"),
                surface("implemented", "AGENTS.md"),
            ),
        );

        let result = validate_file(&manifest, &repo_root());
        assert!(result.is_ok(), "expected pass, got: {:?}", result.err());
    }

    #[test]
    fn missing_surface_row_fails() {
        let dir = tempfile::tempdir().unwrap();
        let manifest = write_manifest(
            dir.path(),
            r#"
plan_id: test-plan
changed_pages_or_runbooks:
  - AGENTS.md
surfaces:
  normative_owner:
    disposition: updated
    path: AGENTS.md
    reason: test reason
  adopter_facing_explanation:
    disposition: updated
    path: AGENTS.md
    reason: test reason
  distribution:
    disposition: confirmed
    paths:
      - AGENTS.md
    reason: test reason
"#,
        );

        let result = validate_file(&manifest, &repo_root());
        assert!(result.is_err());
    }

    #[test]
    fn unresolvable_declared_path_fails() {
        let dir = tempfile::tempdir().unwrap();
        let manifest = write_manifest(
            dir.path(),
            &format!(
                r#"
plan_id: test-plan
changed_pages_or_runbooks:
  - AGENTS.md
surfaces:
  normative_owner:
    {}
  adopter_facing_explanation:
    {}
  runbook_situation_map:
    {}
  ai_landing_loaders:
    {}
  validation_drift_evidence:
    {}
  distribution:
    disposition: confirmed
    paths:
      - AGENTS.md
    reason: test reason
"#,
                surface("updated", "docs/does/not/exist.md"),
                surface("updated", "AGENTS.md"),
                surface("updated", "AGENTS.md"),
                surface("updated", "AGENTS.md"),
                surface("implemented", "AGENTS.md"),
            ),
        );

        let result = validate_file(&manifest, &repo_root());
        let errors = result.expect_err("expected an unresolvable-path error");
        assert!(errors
            .iter()
            .any(|(message, _)| message.contains("does not resolve to a real file")));
    }

    #[test]
    fn unresolvable_changed_file_entry_fails() {
        let dir = tempfile::tempdir().unwrap();
        let manifest = write_manifest(
            dir.path(),
            &format!(
                r#"
plan_id: test-plan
changed_pages_or_runbooks:
  - agf/docs/site/docs/does-not-exist.md
surfaces:
  normative_owner:
    {}
  adopter_facing_explanation:
    {}
  runbook_situation_map:
    {}
  ai_landing_loaders:
    {}
  validation_drift_evidence:
    {}
  distribution:
    disposition: confirmed
    paths:
      - AGENTS.md
    reason: test reason
"#,
                surface("updated", "AGENTS.md"),
                surface("updated", "AGENTS.md"),
                surface("updated", "AGENTS.md"),
                surface("updated", "AGENTS.md"),
                surface("implemented", "AGENTS.md"),
            ),
        );

        let result = validate_file(&manifest, &repo_root());
        let errors = result.expect_err("expected an unresolvable changed-file error");
        assert!(errors
            .iter()
            .any(|(message, _)| message.contains("changed_pages_or_runbooks")));
    }

    #[test]
    fn not_applicable_path_is_allowed_to_be_unresolvable() {
        let dir = tempfile::tempdir().unwrap();
        let manifest = write_manifest(
            dir.path(),
            r#"
plan_id: test-plan
changed_pages_or_runbooks:
  - AGENTS.md
surfaces:
  normative_owner:
    disposition: updated
    path: AGENTS.md
    reason: test reason
  adopter_facing_explanation:
    disposition: updated
    path: AGENTS.md
    reason: test reason
  runbook_situation_map:
    disposition: updated
    path: AGENTS.md
    reason: test reason
  ai_landing_loaders:
    disposition: updated
    path: AGENTS.md
    reason: test reason
    not_applicable:
      - path: .claude/CLAUDE.md
        reason: not shipped by this project
  validation_drift_evidence:
    disposition: implemented
    path: AGENTS.md
    reason: test reason
  distribution:
    disposition: confirmed
    paths:
      - AGENTS.md
    reason: test reason
"#,
        );

        let result = validate_file(&manifest, &repo_root());
        assert!(result.is_ok(), "expected pass, got: {:?}", result.err());
    }

    #[test]
    fn omitted_changed_page_fails() {
        let dir = tempfile::tempdir().unwrap();
        let manifest = write_manifest(
            dir.path(),
            &format!(
                r#"
plan_id: test-plan
changed_pages_or_runbooks:
  - AGENTS.md
surfaces:
  normative_owner:
    {}
  adopter_facing_explanation:
    {}
  runbook_situation_map:
    {}
  ai_landing_loaders:
    {}
  validation_drift_evidence:
    {}
  distribution:
    disposition: confirmed
    paths:
      - AGENTS.md
    reason: test reason
"#,
                surface("updated", "AGENTS.md"),
                surface("updated", "AGENTS.md"),
                surface("updated", "AGENTS.md"),
                surface("updated", "AGENTS.md"),
                surface("implemented", "AGENTS.md"),
            ),
        );
        let actual = BTreeSet::from(["agf/docs/site/docs/omitted.md".to_string()]);
        let errors = validate_file_against(&manifest, &repo_root(), &actual, |_| true)
            .expect_err("omitted changed page must fail");
        assert!(errors
            .iter()
            .any(|(message, _)| message.contains("omitted.md")));
    }

    #[test]
    fn declared_surface_absent_from_distribution_fails() {
        let dir = tempfile::tempdir().unwrap();
        let manifest = write_manifest(
            dir.path(),
            &format!(
                r#"
plan_id: test-plan
changed_pages_or_runbooks:
  - AGENTS.md
surfaces:
  normative_owner:
    {}
  adopter_facing_explanation:
    {}
  runbook_situation_map:
    {}
  ai_landing_loaders:
    {}
  validation_drift_evidence:
    {}
  distribution:
    disposition: confirmed
    paths:
      - AGENTS.md
    reason: test reason
"#,
                surface("updated", "AGENTS.md"),
                surface("updated", "AGENTS.md"),
                surface("updated", "AGENTS.md"),
                surface("updated", "AGENTS.md"),
                surface("implemented", "AGENTS.md"),
            ),
        );
        let errors = validate_file_against(&manifest, &repo_root(), &BTreeSet::new(), |_| false)
            .expect_err("missing distribution member must fail");
        assert!(errors
            .iter()
            .any(|(message, _)| message.contains("absent from distribution")));
    }
}
