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

#[test]
fn validate_learnings_accepts_explicit_file_path() {
    let temp = tempfile::tempdir().expect("tempdir");
    let learning = temp.path().join("upstream-learning.yaml");
    std::fs::write(
        &learning,
        r#"project:
  codename: amber-register
  description: "Existing adopter repository using AGF."
  started_adopting: "2026-06-25"

learnings:
  - id: "20260625T072500Z-svch"
    date: "2026-06-25"
    category: gap
    scope: framework
    title: schema-validation-prerequisites-unclear
    description: "Validates an upstream-submittable learning outside the default directory."
    agents_involved:
      - coder-tester
      - governor-reviewer
    framework_section: "learning schema validation"
    business_impact:
      type: cost_overhead
      description: "Manual validation prerequisites create friction."
    recommendation: "Allow agov to validate named learning files."
    severity: significant
    confidence:
      level: single-adopter
      evidence_count: 1
      evidence_refs:
        - "test fixture"

sentinel_ack: []
"#,
    )
    .expect("write learning");

    let output = agov(&[
        "validate",
        "learnings",
        learning.to_str().expect("utf-8 path"),
    ]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("OK agov.validate.learnings"),
        "expected learning validation OK, got: {stdout}"
    );
    assert!(
        stdout.contains("upstream-learning.yaml"),
        "expected explicit file in output, got: {stdout}"
    );
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
