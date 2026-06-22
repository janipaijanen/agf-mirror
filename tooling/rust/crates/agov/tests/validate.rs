use std::path::PathBuf;
use std::process::Command;

#[test]
fn validate_prints_coverage_note() {
    let output = agov(&["validate"]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("OK agov.validate.coverage: framework checks complete"));
    assert!(stdout.contains("agov plan validate"));
    assert!(stdout.contains("agov coord validate"));
    assert!(stdout.contains("the umbrella command does not validate those records"));
}

fn agov(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_agov"))
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
