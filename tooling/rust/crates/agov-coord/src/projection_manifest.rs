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
use std::path::{Component, Path};

#[derive(Debug, Clone)]
pub struct ProjectionManifest {
    pub raw: serde_json::Value,
}

/// Validate a rule-projection evidence manifest file against its schema.
/// `repo_root` is used to resolve every declared path for existence.
pub fn validate_file(
    path: &Path,
    repo_root: &Path,
    schema_path: &Path,
    allow_planned: bool,
) -> Result<ProjectionManifest, Vec<(String, String)>> {
    let raw = yaml::read_yaml_file_json(path).map_err(|err| match err {
        yaml::YamlError::Io(message) if message == "file not found" => {
            vec![("file not found".to_string(), String::new())]
        }
        err => vec![(format!("{err:?}"), String::new())],
    })?;

    let compiled = schema::load_and_compile(schema_path)
        .map_err(|err| vec![(format!("{err:?}"), schema_path.display().to_string())])?;
    let mut errors: Vec<(String, String)> = schema::validate(&compiled, &raw)
        .into_iter()
        .map(|err| (err.message, err.pointer))
        .collect();

    if errors.is_empty() {
        errors.extend(validate_paths_resolve(&raw, repo_root, allow_planned));
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
    schema_path: &Path,
    allow_planned: bool,
    changed_pages_or_runbooks: &BTreeSet<String>,
    ships_in_distribution: F,
) -> Result<ProjectionManifest, Vec<(String, String)>>
where
    F: Fn(&str) -> bool,
{
    let manifest = validate_file(path, repo_root, schema_path, allow_planned)?;
    let mut errors = validate_changed_files(&manifest.raw, changed_pages_or_runbooks);
    errors.extend(validate_distribution(&manifest.raw, &ships_in_distribution));
    errors.extend(validate_distribution_completeness(
        &manifest.raw,
        ships_in_distribution,
    ));
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

/// Every `changed_pages_or_runbooks` entry that actually ships via a
/// distribution glob must be reflected in `surfaces.distribution` -- either
/// listed in `paths` or covered by a `not_applicable` entry with a reason.
/// `validate_distribution` only checks the opposite direction (every
/// *declared* path really ships); this closes the completeness gap a
/// manifest author could otherwise leave silently open by omitting a
/// shipped changed file from `distribution.paths` altogether.
fn validate_distribution_completeness<F>(raw: &serde_json::Value, ships: F) -> Vec<(String, String)>
where
    F: Fn(&str) -> bool,
{
    let declared: BTreeSet<&str> = raw
        .pointer("/surfaces/distribution/paths")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .filter_map(|value| value.as_str())
        .collect();

    let not_applicable: BTreeSet<&str> = raw
        .pointer("/surfaces/distribution/not_applicable")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.get("path").and_then(|value| value.as_str()))
        .collect();

    raw.get("changed_pages_or_runbooks")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .filter_map(|value| value.as_str())
        .filter(|path| ships(path))
        .filter(|path| !declared.contains(path) && !not_applicable.contains(path))
        .map(|path| {
            (
                format!(
                    "changed file ships via a distribution glob but is absent from surfaces.distribution.paths and not_applicable: {path}"
                ),
                "/surfaces/distribution".to_string(),
            )
        })
        .collect()
}

fn validate_paths_resolve(
    raw: &serde_json::Value,
    repo_root: &Path,
    allow_planned: bool,
) -> Vec<(String, String)> {
    let mut errors = Vec::new();

    if let Some(changed) = raw
        .get("changed_pages_or_runbooks")
        .and_then(|value| value.as_array())
    {
        for (index, entry) in changed.iter().enumerate() {
            if let Some(relative) = entry.as_str() {
                match classify(repo_root, relative) {
                    PathClassification::ContainedExisting => {}
                    PathClassification::ContainedMissing => errors.push((
                        format!("changed_pages_or_runbooks[{index}] does not resolve to a real file: {relative}"),
                        format!("/changed_pages_or_runbooks/{index}"),
                    )),
                    PathClassification::Invalid => errors.push(invalid_path_error(
                        &format!("changed_pages_or_runbooks[{index}]"),
                        relative,
                        &format!("/changed_pages_or_runbooks/{index}"),
                    )),
                }
            }
        }
    }

    if let Some(surfaces) = raw.get("surfaces").and_then(|value| value.as_object()) {
        for (surface_name, entry) in surfaces {
            check_surface_paths(surface_name, entry, repo_root, allow_planned, &mut errors);
        }
    }

    errors
}

fn check_surface_paths(
    surface_name: &str,
    entry: &serde_json::Value,
    repo_root: &Path,
    allow_planned: bool,
    errors: &mut Vec<(String, String)>,
) {
    let planned = surface_name == "normative_owner"
        && entry.get("stage").and_then(serde_json::Value::as_str) == Some("planned");

    if let Some(path) = entry.get("path").and_then(|value| value.as_str()) {
        check_declared_path(
            surface_name,
            "path",
            path,
            planned,
            allow_planned,
            repo_root,
            errors,
        );
    }

    if let Some(paths) = entry.get("paths").and_then(|value| value.as_array()) {
        for (index, path) in paths.iter().enumerate() {
            if let Some(path) = path.as_str() {
                check_declared_path(
                    surface_name,
                    &format!("paths[{index}]"),
                    path,
                    planned,
                    allow_planned,
                    repo_root,
                    errors,
                );
            }
        }
    }

    // `not_applicable` sub-entries are explicitly allowed to name a path
    // that does not resolve -- that is often the entire point (e.g. a
    // loader file this project does not ship). Schema validation already
    // enforces `path`/`reason` presence; no existence check here.
}

fn check_declared_path(
    surface_name: &str,
    field: &str,
    path: &str,
    planned: bool,
    allow_planned: bool,
    repo_root: &Path,
    errors: &mut Vec<(String, String)>,
) {
    let pointer_field = field.replace('[', "/").replace(']', "");
    let pointer = format!("/surfaces/{surface_name}/{pointer_field}");
    match classify(repo_root, path) {
        PathClassification::ContainedExisting if planned => errors.push((
            format!(
                "normative_owner.stage is `planned` but `{path}` already exists -- update to `stage: existing` or remove the field before Gate 2"
            ),
            pointer,
        )),
        PathClassification::ContainedExisting => {}
        PathClassification::ContainedMissing if planned && allow_planned => {}
        PathClassification::ContainedMissing => errors.push((
            format!(
                "surfaces.{surface_name}.{field} does not resolve to a real file: {path}"
            ),
            pointer,
        )),
        PathClassification::Invalid => errors.push(invalid_path_error(
            &format!("surfaces.{surface_name}.{field}"),
            path,
            &pointer,
        )),
    }
}

fn invalid_path_error(label: &str, path: &str, pointer: &str) -> (String, String) {
    (
        format!(
            "{label} is not a valid repository-contained path: {path} (absolute paths, `..`, and paths whose existing components resolve outside the repository are forbidden)"
        ),
        pointer.to_string(),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PathClassification {
    ContainedExisting,
    ContainedMissing,
    Invalid,
}

fn classify(repo_root: &Path, relative: &str) -> PathClassification {
    let candidate = Path::new(relative);
    if candidate.is_absolute()
        || candidate
            .components()
            .any(|component| matches!(component, Component::ParentDir))
    {
        return PathClassification::Invalid;
    }

    let Ok(canonical_root) = repo_root.canonicalize() else {
        return PathClassification::Invalid;
    };
    let joined = repo_root.join(candidate);

    if joined.is_file() {
        return match joined.canonicalize() {
            Ok(real) if real.starts_with(&canonical_root) => PathClassification::ContainedExisting,
            _ => PathClassification::Invalid,
        };
    }

    let mut walked = repo_root.to_path_buf();
    for component in candidate.components() {
        walked.push(component);
        match std::fs::symlink_metadata(&walked) {
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => break,
            Err(_) => return PathClassification::Invalid,
        }
        match walked.canonicalize() {
            Ok(real) if real.starts_with(&canonical_root) => continue,
            _ => return PathClassification::Invalid,
        }
    }
    PathClassification::ContainedMissing
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

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

    fn schema_path() -> PathBuf {
        repo_root().join("agf/schemas/rule-projection-manifest.schema.json")
    }

    fn validate_manifest(manifest: &Path) -> Result<ProjectionManifest, Vec<(String, String)>> {
        validate_file(manifest, &repo_root(), &schema_path(), false)
    }

    fn validate_manifest_against<F>(
        manifest: &Path,
        actual: &BTreeSet<String>,
        ships: F,
    ) -> Result<ProjectionManifest, Vec<(String, String)>>
    where
        F: Fn(&str) -> bool,
    {
        validate_file_against(manifest, &repo_root(), &schema_path(), false, actual, ships)
    }

    fn surface(disposition: &str, path: &str) -> String {
        format!("disposition: {disposition}\n    path: {path}\n    reason: test reason")
    }

    fn manifest_with_owner(owner: &str) -> String {
        format!(
            r#"
plan_id: test-plan
changed_pages_or_runbooks:
  - AGENTS.md
surfaces:
  normative_owner:
{owner}
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
  validation_drift_evidence:
    disposition: implemented
    path: AGENTS.md
    reason: test reason
  distribution:
    disposition: confirmed
    paths:
      - AGENTS.md
    reason: test reason
"#
        )
    }

    fn fixture_repo() -> tempfile::TempDir {
        let root = tempfile::tempdir().unwrap();
        std::fs::write(root.path().join("AGENTS.md"), "fixture").unwrap();
        root
    }

    fn validate_fixture(
        manifest: &Path,
        fixture_root: &Path,
        allow_planned: bool,
    ) -> Result<ProjectionManifest, Vec<(String, String)>> {
        validate_file(manifest, fixture_root, &schema_path(), allow_planned)
    }

    #[test]
    fn planned_missing_normative_owner_passes_only_when_allowed() {
        let root = fixture_repo();
        let manifest = write_manifest(
            root.path(),
            &manifest_with_owner(
                "    disposition: updated\n    stage: planned\n    path: agf/rules/workflow/future-rule.md\n    reason: proposed by this plan",
            ),
        );

        assert!(validate_fixture(&manifest, root.path(), true).is_ok());
        let errors = validate_fixture(&manifest, root.path(), false)
            .expect_err("strict Gate 2 validation must reject an absent owner");
        assert!(errors
            .iter()
            .any(|(message, _)| message.contains("does not resolve to a real file")));
    }

    #[test]
    fn stale_planned_normative_owner_fails_with_or_without_relaxation() {
        let root = fixture_repo();
        let manifest = write_manifest(
            root.path(),
            &manifest_with_owner(
                "    disposition: updated\n    stage: planned\n    path: AGENTS.md\n    reason: stale marker",
            ),
        );

        for allow_planned in [false, true] {
            let errors = validate_fixture(&manifest, root.path(), allow_planned)
                .expect_err("an existing planned owner must be stale");
            assert!(errors
                .iter()
                .any(|(message, _)| message.contains("lifecycle")
                    || message.contains("already exists")));
        }
    }

    #[test]
    fn existing_stage_with_existing_owner_passes() {
        let root = fixture_repo();
        let manifest = write_manifest(
            root.path(),
            &manifest_with_owner(
                "    disposition: updated\n    stage: existing\n    path: AGENTS.md\n    reason: existing owner",
            ),
        );

        assert!(validate_fixture(&manifest, root.path(), false).is_ok());
        assert!(validate_fixture(&manifest, root.path(), true).is_ok());
    }

    #[test]
    fn planned_metadata_does_not_relax_another_surface() {
        let root = fixture_repo();
        let content = manifest_with_owner(
            "    disposition: updated\n    path: AGENTS.md\n    reason: existing owner",
        )
        .replace(
            "  ai_landing_loaders:\n    disposition: updated\n    path: AGENTS.md",
            "  ai_landing_loaders:\n    disposition: updated\n    stage: planned\n    path: missing-loader.md",
        );
        let manifest = write_manifest(root.path(), &content);

        let errors = validate_fixture(&manifest, root.path(), true)
            .expect_err("planned metadata must not relax non-owner surfaces");
        assert!(errors.iter().any(|(message, _)| {
            message.contains("ai_landing_loaders")
                && message.contains("does not resolve to a real file")
        }));
    }

    #[test]
    fn planned_paths_array_becomes_stale_when_any_member_exists() {
        let root = fixture_repo();
        let manifest = write_manifest(
            root.path(),
            &manifest_with_owner(
                "    disposition: updated\n    stage: planned\n    paths:\n      - missing-owner.md\n      - AGENTS.md\n    reason: partially stale marker",
            ),
        );

        let errors = validate_fixture(&manifest, root.path(), true)
            .expect_err("one existing member must make the whole planned entry stale");
        assert!(errors.iter().any(
            |(message, _)| message.contains("AGENTS.md") && message.contains("already exists")
        ));
    }

    #[test]
    fn planned_paths_array_passes_only_while_every_member_is_missing() {
        let root = fixture_repo();
        let missing_manifest = write_manifest(
            root.path(),
            &manifest_with_owner(
                "    disposition: updated\n    stage: planned\n    paths:\n      - first-missing.md\n      - second-missing.md\n    reason: both pending",
            ),
        );
        assert!(validate_fixture(&missing_manifest, root.path(), true).is_ok());

        std::fs::write(root.path().join("second-existing.md"), "existing").unwrap();
        let existing_manifest = write_manifest(
            root.path(),
            &manifest_with_owner(
                "    disposition: updated\n    stage: planned\n    paths:\n      - AGENTS.md\n      - second-existing.md\n    reason: both stale",
            ),
        );
        let errors = validate_fixture(&existing_manifest, root.path(), true)
            .expect_err("all-existing planned paths must be stale");
        assert_eq!(
            errors
                .iter()
                .filter(|(message, _)| message.contains("already exists"))
                .count(),
            2
        );
    }

    #[test]
    fn planned_relaxation_never_accepts_absolute_or_parent_paths() {
        let root = fixture_repo();
        for invalid in ["/etc/passwd", "../outside.md"] {
            let manifest = write_manifest(
                root.path(),
                &manifest_with_owner(&format!(
                    "    disposition: updated\n    stage: planned\n    path: {invalid}\n    reason: invalid path"
                )),
            );
            for allow_planned in [false, true] {
                let errors = validate_fixture(&manifest, root.path(), allow_planned)
                    .expect_err("repository escape must always fail");
                assert!(errors
                    .iter()
                    .any(|(message, _)| message.contains("repository-contained path")));
            }
        }
    }

    #[test]
    fn planned_owner_requires_a_non_empty_path_or_paths_array() {
        let root = fixture_repo();
        for owner in [
            "    disposition: updated\n    stage: planned\n    reason: missing path",
            "    disposition: updated\n    stage: planned\n    paths: []\n    reason: empty paths",
        ] {
            let manifest = write_manifest(root.path(), &manifest_with_owner(owner));
            let errors = validate_fixture(&manifest, root.path(), true)
                .expect_err("schema must require a concrete planned owner path");
            assert!(!errors.is_empty());
        }
    }

    #[cfg(unix)]
    #[test]
    fn planned_relaxation_rejects_outside_and_dangling_symlink_paths() {
        use std::os::unix::fs::symlink;

        let root = fixture_repo();
        let outside = tempfile::tempdir().unwrap();
        std::fs::write(outside.path().join("owner.md"), "outside").unwrap();
        symlink(
            outside.path().join("owner.md"),
            root.path().join("outside-owner"),
        )
        .unwrap();
        symlink(outside.path(), root.path().join("outside-dir")).unwrap();
        symlink(
            outside.path().join("missing.md"),
            root.path().join("dangling-outside"),
        )
        .unwrap();
        symlink("chain-target", root.path().join("chain-start")).unwrap();
        symlink(
            outside.path().join("missing.md"),
            root.path().join("chain-target"),
        )
        .unwrap();
        symlink("internal-missing", root.path().join("internal-dangling")).unwrap();

        for invalid in [
            "outside-owner",
            "outside-dir/future.md",
            "dangling-outside",
            "chain-start",
            "internal-dangling",
        ] {
            let manifest = write_manifest(
                root.path(),
                &manifest_with_owner(&format!(
                    "    disposition: updated\n    stage: planned\n    path: {invalid}\n    reason: invalid symlink path"
                )),
            );
            for allow_planned in [false, true] {
                let errors = validate_fixture(&manifest, root.path(), allow_planned)
                    .expect_err("symlink escape or dangling chain must always fail");
                assert!(errors
                    .iter()
                    .any(|(message, _)| message.contains("repository-contained path")));
            }
        }
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

        let result = validate_manifest(&manifest);
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

        let result = validate_manifest(&manifest);
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

        let result = validate_manifest(&manifest);
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

        let result = validate_manifest(&manifest);
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

        let result = validate_manifest(&manifest);
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
        let errors = validate_manifest_against(&manifest, &actual, |_| true)
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
        let errors = validate_manifest_against(&manifest, &BTreeSet::new(), |_| false)
            .expect_err("missing distribution member must fail");
        assert!(errors
            .iter()
            .any(|(message, _)| message.contains("absent from distribution")));
    }

    #[test]
    fn shipped_changed_file_omitted_from_distribution_fails() {
        let dir = tempfile::tempdir().unwrap();
        let manifest = write_manifest(
            dir.path(),
            &format!(
                r#"
plan_id: test-plan
changed_pages_or_runbooks:
  - agf/docs/site/docs/concepts/coordination-protocol.md
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
        let actual =
            BTreeSet::from(["agf/docs/site/docs/concepts/coordination-protocol.md".to_string()]);
        // Every path "ships" in this fixture -- simulates a real distribution
        // glob (e.g. agf/docs/site/**) covering the changed Pages file, which
        // the manifest above fails to declare or mark not_applicable.
        let errors = validate_manifest_against(&manifest, &actual, |_| true)
            .expect_err("shipped changed file omitted from distribution must fail");
        assert!(errors.iter().any(|(message, _)| message
            .contains("ships via a distribution glob but is absent from surfaces.distribution")));
    }

    #[test]
    fn shipped_changed_file_covered_by_not_applicable_passes() {
        let dir = tempfile::tempdir().unwrap();
        let manifest = write_manifest(
            dir.path(),
            &format!(
                r#"
plan_id: test-plan
changed_pages_or_runbooks:
  - agf/docs/site/docs/concepts/coordination-protocol.md
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
    not_applicable:
      - path: agf/docs/site/docs/concepts/coordination-protocol.md
        reason: reviewed and confirmed redundant with the already-declared AGENTS.md distribution entry; not independently listed to avoid a duplicate row
"#,
                surface("updated", "AGENTS.md"),
                surface("updated", "AGENTS.md"),
                surface("updated", "AGENTS.md"),
                surface("updated", "AGENTS.md"),
                surface("implemented", "AGENTS.md"),
            ),
        );
        let actual =
            BTreeSet::from(["agf/docs/site/docs/concepts/coordination-protocol.md".to_string()]);
        let result = validate_manifest_against(&manifest, &actual, |_| true);
        assert!(result.is_ok(), "expected pass, got: {:?}", result.err());
    }
}
