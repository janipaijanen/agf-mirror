use std::path::PathBuf;
use std::process::Command;

fn agov(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_agov"))
        .env("AGOV_TOKEN_ROI", "0")
        .args(args)
        .current_dir(repo_root())
        .output()
        .expect("run agov")
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("repo root")
        .to_path_buf()
}

#[test]
fn top_level_version_includes_commit_and_dirty_state() {
    let output = agov(&["--version"]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.starts_with("agov "),
        "version output should start with command name, got: {stdout}"
    );
    assert!(
        stdout.contains('(') && stdout.contains(')'),
        "version output should include provenance tuple, got: {stdout}"
    );
    assert!(
        stdout.contains(" dirty)") || stdout.contains(" clean)"),
        "version output should include dirty state, got: {stdout}"
    );
}

#[test]
fn version_json_reports_structured_provenance() {
    let output = agov(&["version", "--format", "json"]);

    assert_eq!(output.status.code(), Some(0));
    let value: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("version json is valid JSON");
    assert_eq!(value["version"].as_str(), Some(env!("CARGO_PKG_VERSION")));
    assert!(
        value["commit"]
            .as_str()
            .is_some_and(|commit| !commit.is_empty()),
        "commit should be a non-empty string, got: {value}"
    );
    assert!(
        value["dirty"].is_boolean(),
        "dirty should be boolean, got: {value}"
    );
    assert!(
        value["source_dirty"].is_boolean(),
        "source_dirty should be boolean, got: {value}"
    );
    let commit = value["commit"].as_str().unwrap();
    assert!(
        commit == "unknown"
            || (commit.len() == 40 && commit.chars().all(|c| c.is_ascii_hexdigit())),
        "commit should be a full git object id or unknown, got: {value}"
    );
}
