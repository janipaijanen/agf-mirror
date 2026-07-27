use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn token_roi_report_prints_log_summary() {
    let log = temp_log_path();
    fs::write(
        &log,
        r#"{"event":"tool_output","command":"agov","workflow":"coord-validate","chars":40,"approx_tokens":10,"approx_saved_min_tokens":20,"approx_saved_max_tokens":30}
{"event":"measurement","command":"manual","workflow":"cold-start","chars":100,"manual_chars":500,"approx_tokens":25,"approx_saved_min_tokens":100,"approx_saved_max_tokens":100}
"#,
    )
    .unwrap();

    let output = agov(&["token", "roi", "report", "--log", log.to_str().unwrap()]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Token ROI telemetry"));
    assert!(stdout.contains(&format!("Log: {}", log.display())));
    assert!(stdout.contains("Records: 2"));
    assert!(stdout.contains("Measured tool output: 140 chars / 35 approx tokens"));
    assert!(stdout.contains("Commands:"));
    assert!(stdout.contains("agov"));
    assert!(stdout.contains("Workflows:"));
    assert!(stdout.contains("coord-validate"));
}

#[test]
fn dotted_token_roi_alias_is_supported() {
    let log = temp_log_path();
    fs::write(&log, "").unwrap();

    let output = agov(&["agov.token.roi", "report", "--log", log.to_str().unwrap()]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Token ROI telemetry"));
    assert!(stdout.contains("Commands: none"));
}

#[test]
fn project_defaults_are_isolated_and_nested_directories_share_the_root() {
    let first = temp_project("first");
    let second = temp_project("second");
    let nested = first.join("nested/deeper");
    fs::create_dir_all(&nested).unwrap();

    for cwd in [&nested, &second] {
        let output = Command::new(env!("CARGO_BIN_EXE_agov"))
            .env_remove("AGOV_TOKEN_ROI")
            .env_remove("AGOV_TOKEN_ROI_LOG")
            .arg("version")
            .current_dir(cwd)
            .output()
            .expect("run agov");
        assert!(output.status.success(), "{output:?}");
    }

    assert!(first.join("tmp/agov-token-roi.jsonl").is_file());
    assert!(second.join("tmp/agov-token-roi.jsonl").is_file());
    let _ = fs::remove_dir_all(first);
    let _ = fs::remove_dir_all(second);
}

#[test]
fn inherited_outside_log_is_honored_with_safe_boundary_warning() {
    let project = temp_project("outside-warning");
    let log = temp_log_path();
    let output = Command::new(env!("CARGO_BIN_EXE_agov"))
        .env_remove("AGOV_TOKEN_ROI")
        .env("AGOV_TOKEN_ROI_LOG", &log)
        .arg("version")
        .current_dir(&project)
        .output()
        .expect("run agov");

    assert!(output.status.success(), "{output:?}");
    assert!(log.is_file());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("outside project root"), "{stderr}");
    assert!(stderr.contains("suggested project-local path"), "{stderr}");
    assert!(
        !stderr.contains("\"event\""),
        "warning leaked JSONL content: {stderr}"
    );
    let _ = fs::remove_file(log);
    let _ = fs::remove_dir_all(project);
}

#[test]
fn explicit_report_log_overrides_environment_and_warns_at_the_boundary() {
    let project = temp_project("cli-precedence");
    let cli_log = temp_log_path();
    let env_log = temp_log_path();
    fs::write(&cli_log, "").unwrap();
    fs::write(&env_log, "").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_agov"))
        .env("AGOV_TOKEN_ROI", "0")
        .env("AGOV_TOKEN_ROI_LOG", &env_log)
        .args(["token", "roi", "report", "--log"])
        .arg(&cli_log)
        .current_dir(&project)
        .output()
        .expect("run agov");

    assert!(output.status.success(), "{output:?}");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stdout.contains(&format!("Log: {}", cli_log.display())),
        "{stdout}"
    );
    assert!(
        !stdout.contains(&format!("Log: {}", env_log.display())),
        "{stdout}"
    );
    assert!(stderr.contains("outside project root"), "{stderr}");
    let _ = fs::remove_file(cli_log);
    let _ = fs::remove_file(env_log);
    let _ = fs::remove_dir_all(project);
}

#[test]
fn report_without_project_or_explicit_path_fails_without_creating_a_log() {
    let cwd = std::env::temp_dir().join(format!("agov-no-project-{}", unique_suffix()));
    fs::create_dir_all(&cwd).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_agov"))
        .env("AGOV_TOKEN_ROI", "0")
        .env_remove("AGOV_TOKEN_ROI_LOG")
        .args(["token", "roi", "report"])
        .current_dir(&cwd)
        .output()
        .expect("run agov");

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("no AGF project root found"), "{stderr}");
    assert!(!cwd.join("tmp/agov-token-roi.jsonl").exists());
    let _ = fs::remove_dir_all(cwd);
}

#[test]
fn explicit_report_outside_a_project_works_with_unverifiable_warning() {
    let cwd = std::env::temp_dir().join(format!("agov-no-project-explicit-{}", unique_suffix()));
    let log = temp_log_path();
    fs::create_dir_all(&cwd).unwrap();
    fs::write(&log, "").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_agov"))
        .env("AGOV_TOKEN_ROI", "0")
        .env_remove("AGOV_TOKEN_ROI_LOG")
        .args(["token", "roi", "report", "--log"])
        .arg(&log)
        .current_dir(&cwd)
        .output()
        .expect("run agov");

    assert!(output.status.success(), "{output:?}");
    assert!(String::from_utf8_lossy(&output.stderr).contains("isolation cannot be verified"));
    let _ = fs::remove_file(log);
    let _ = fs::remove_dir_all(cwd);
}

#[test]
fn committed_codex_snapshot_reports_realistic_roi() {
    let log = repo_root().join(
        "tooling/rust/crates/agov/tests/fixtures/token_roi/codex-agov-token-roi-20260623.jsonl",
    );

    let output = agov(&["token", "roi", "report", "--log", log.to_str().unwrap()]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Token ROI telemetry"));
    assert!(stdout.contains("Records: 224"));
    assert!(stdout.contains("Measured tool output: 1013665 chars / 253417 approx tokens"));
    assert!(stdout.contains("potential saved: 276840-572694"));
    assert!(stdout.contains("agov.session.resume"));
    assert!(stdout.contains("coord-validate"));
}

/// Gate 2 review `d8h5` BLOCK 1 end-to-end regression: `dist
/// provenance-record file` succeeds with an arbitrary `--implementation`
/// value (no clap `value_parser` constrains it) and must not persist that
/// value verbatim in the ROI log. `d8h5` reproduced the leak live against
/// the pre-fix binary using this exact marker string.
#[test]
fn hostile_implementation_flag_value_is_redacted_after_successful_command() {
    let log = temp_log_path();
    let artifact = std::env::temp_dir().join(format!(
        "agov-roi-redaction-fixture-{}-{}.bin",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::write(&artifact, b"fixture artifact bytes").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_agov"))
        .env("AGOV_TOKEN_ROI_LOG", &log)
        .env_remove("AGOV_TOKEN_ROI")
        .args([
            "dist",
            "provenance-record",
            "file",
            "--product",
            "agov",
            "--version",
            "9.9.9",
            "--source-commit",
            "abc123",
            "--tag",
            "untagged",
            "--build-timestamp",
            "2026-07-22T00:00:00Z",
            "--implementation",
            "sensitive-marker-should-redact",
            "--packaging",
            "tar.gz",
            "--target-platform",
            "x86_64-unknown-linux-musl",
            "--artifact",
        ])
        .arg(&artifact)
        .current_dir(repo_root())
        .output()
        .expect("run agov");

    assert!(
        output.status.success(),
        "dist provenance-record file should succeed: {output:?}"
    );

    let content = fs::read_to_string(&log).unwrap_or_default();
    assert!(
        !content.contains("sensitive-marker-should-redact"),
        "hostile --implementation value leaked into the ROI log: {content}"
    );
    assert!(
        content.contains("\"--implementation\""),
        "expected the flag name to survive redaction: {content}"
    );

    let _ = fs::remove_file(&artifact);
    let _ = fs::remove_file(&log);
}

/// Gate 2 review `32qd` (narrowed BLOCK 4) end-to-end regression: the
/// real compiled binary accepts the clap alias `--base_branch` (for
/// `--base-branch`) on `session resume`. `32qd` reproduced this over-
/// redacting the flag *name* itself in the persisted ROI record because
/// `KNOWN_FLAG_NAMES` only listed the primary spelling. The flag name
/// must survive; its value still redacts (free-form branch name, not
/// closed vocabulary).
#[test]
fn session_resume_base_branch_alias_name_survives_redaction() {
    let log = temp_log_path();

    let output = Command::new(env!("CARGO_BIN_EXE_agov"))
        .env("AGOV_TOKEN_ROI_LOG", &log)
        .env_remove("AGOV_TOKEN_ROI")
        .args([
            "session",
            "resume",
            "--base_branch",
            "main",
            "--dir",
            "docs/coordination",
        ])
        .current_dir(repo_root())
        .output()
        .expect("run agov");

    assert!(
        output.status.success(),
        "session resume should succeed: {output:?}"
    );

    let content = std::fs::read_to_string(&log).unwrap_or_default();
    let record: Value = serde_json::from_str(content.lines().next().unwrap()).unwrap();
    let args = record["args"].as_array().unwrap();
    let args_str: Vec<&str> = args.iter().map(|v| v.as_str().unwrap()).collect();
    assert!(
        args_str.contains(&"--base_branch"),
        "the --base_branch flag NAME must survive redaction: {args_str:?}"
    );
    assert!(
        !args_str.contains(&"main"),
        "the --base_branch flag VALUE must still redact: {args_str:?}"
    );

    let _ = std::fs::remove_file(&log);
}

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

fn temp_log_path() -> PathBuf {
    std::env::temp_dir().join(format!(
        "agov_token_roi_report_{}_{}.jsonl",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}

fn unique_suffix() -> String {
    format!(
        "{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    )
}

fn temp_project(label: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("agov-roi-{label}-{}", unique_suffix()));
    fs::create_dir_all(root.join("agf/schemas")).unwrap();
    fs::create_dir_all(root.join("tooling")).unwrap();
    root
}
