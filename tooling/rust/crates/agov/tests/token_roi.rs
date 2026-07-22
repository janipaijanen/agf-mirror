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
