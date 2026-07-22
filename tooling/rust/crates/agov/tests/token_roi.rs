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
