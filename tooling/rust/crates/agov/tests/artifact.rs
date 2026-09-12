use std::fs;
use std::path::Path;
use std::process::{Command, Output};
fn fixture() -> tempfile::TempDir {
    let d = tempfile::tempdir().unwrap();
    fs::create_dir(d.path().join("items")).unwrap();
    for (path, bytes) in [
        (
            "agov-config.yaml",
            include_str!("fixtures/artifacts/agov-config.yaml"),
        ),
        (
            "task.schema.json",
            include_str!("fixtures/artifacts/task.schema.json"),
        ),
        (
            "items/a.json",
            include_str!("fixtures/artifacts/items/a.json"),
        ),
        (
            "items/b.json",
            include_str!("fixtures/artifacts/items/b.json"),
        ),
    ] {
        fs::write(d.path().join(path), bytes).unwrap();
    }
    d
}
fn run(root: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_agov"))
        .current_dir(root)
        .args(args)
        .output()
        .unwrap()
}
#[test]
fn artifact_cli_matches_independent_golden_and_repeated_bytes() {
    let d = fixture();
    let a = run(d.path(), &["artifact", "index"]);
    let b = run(d.path(), &["artifact", "index"]);
    assert!(a.status.success(), "{}", String::from_utf8_lossy(&a.stderr));
    assert_eq!(a.stdout, b.stdout);
    let expected: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/artifacts/expected.json")).unwrap();
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&a.stdout).unwrap(),
        expected
    );
    let validated = run(
        d.path(),
        &["artifact", "validate", "--format", "json", "items/a.json"],
    );
    assert_eq!(validated.stdout, a.stdout);
    assert!(validated.status.success());
}
#[test]
fn selected_file_cannot_hide_cross_file_duplicate_or_unregistered_path() {
    let d = fixture();
    fs::copy(
        d.path().join("items/a.json"),
        d.path().join("items/copy.json"),
    )
    .unwrap();
    let output = run(
        d.path(),
        &["artifact", "validate", "--format", "json", "items/b.json"],
    );
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stdout).contains("duplicate-id"));
    let output = run(
        d.path(),
        &[
            "artifact",
            "validate",
            "--format",
            "json",
            "../outside.json",
        ],
    );
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stdout).contains("unregistered-path"));
}
#[test]
fn missing_config_is_unavailable_and_invalid_config_is_a_validation_failure() {
    let d = tempfile::tempdir().unwrap();
    let output = run(d.path(), &["artifact", "index"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stdout).contains("INCOMPLETE"));
    fs::write(
        d.path().join("agov-config.yaml"),
        "artifact_sets: [{name: broken}]\n",
    )
    .unwrap();
    let output = run(d.path(), &["artifact", "index"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stdout).contains("invalid-config"));
}
#[cfg(unix)]
#[test]
fn symlinked_config_schema_and_artifact_are_rejected_through_cli() {
    use std::os::unix::fs::symlink;
    for target in ["agov-config.yaml", "task.schema.json", "items/a.json"] {
        let d = fixture();
        let outside = tempfile::tempdir().unwrap();
        let source = outside.path().join("source");
        fs::rename(d.path().join(target), &source).unwrap();
        symlink(source, d.path().join(target)).unwrap();
        let output = run(d.path(), &["artifact", "index"]);
        assert_eq!(output.status.code(), Some(2), "{target}");
        assert!(String::from_utf8_lossy(&output.stdout).contains("symlink"));
    }
}

#[test]
fn checked_in_negative_fixtures_exercise_each_structural_rule() {
    for (path, bytes, code) in [
        (
            "items/copy.json",
            include_str!("fixtures/artifacts/negative/duplicate.json"),
            "duplicate-id",
        ),
        (
            "items/a.json",
            include_str!("fixtures/artifacts/negative/dangling.json"),
            "dangling-reference",
        ),
        (
            "items/b.json",
            include_str!("fixtures/artifacts/negative/cycle.json"),
            "dependency-cycle",
        ),
        (
            "agov-config.yaml",
            include_str!("fixtures/artifacts/negative/malformed-config.yaml"),
            "invalid-config",
        ),
    ] {
        let d = fixture();
        fs::write(d.path().join(path), bytes).unwrap();
        let output = run(d.path(), &["artifact", "validate", "--format", "json"]);
        assert_eq!(output.status.code(), Some(1), "{code}");
        let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert!(
            json["problems"]
                .as_array()
                .unwrap()
                .iter()
                .any(|p| p["code"] == code),
            "{code}: {json}"
        );
    }
}
