use std::path::PathBuf;
use std::process::Command;

#[test]
fn validate_prints_coverage_note() {
    let output = agov(&["validate"]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("agov.validate.adopter_docs"));
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

#[test]
fn validate_adopter_docs_blocks_missing_downstream_docs() {
    let repo = temp_agf_repo();
    write(&repo, "tooling/rust/src/main.rs", "fn main() {}\n");
    git(&repo, &["add", "."]);
    git(&repo, &["commit", "-m", "tooling without adopter docs"]);

    let output = agov_in(repo.path(), &["validate", "adopter-docs"]);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ERR agov.validate.adopter_docs"));
    assert!(stderr.contains("downstream-facing docs/templates"));
}

#[test]
fn validate_adopter_docs_accepts_downstream_docs() {
    let repo = temp_agf_repo();
    write(&repo, "tooling/rust/src/main.rs", "fn main() {}\n");
    write(&repo, "agf/ADOPTING.md", "Adopter docs parity\n");
    git(&repo, &["add", "."]);
    git(&repo, &["commit", "-m", "tooling with adopter docs"]);

    let output = agov_in(repo.path(), &["validate", "adopter-docs"]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("OK agov.validate.adopter_docs"));
}

fn agov(args: &[&str]) -> std::process::Output {
    agov_in(&repo_root(), args)
}

fn agov_in(cwd: &std::path::Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_agov"))
        .args(args)
        .current_dir(cwd)
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

fn temp_agf_repo() -> tempfile::TempDir {
    let repo = tempfile::tempdir().expect("temp repo");
    git(&repo, &["init"]);
    git(&repo, &["config", "user.email", "test@example.invalid"]);
    git(&repo, &["config", "user.name", "Test User"]);
    write(&repo, "agf/schemas/.keep", "");
    write(&repo, "tooling/.keep", "");
    write(&repo, "README.md", "baseline\n");
    git(&repo, &["add", "."]);
    git(&repo, &["commit", "-m", "baseline"]);
    git(&repo, &["tag", "v1.0.0"]);
    repo
}

fn write(repo: &tempfile::TempDir, rel: &str, content: &str) {
    let path = repo.path().join(rel);
    std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
    std::fs::write(path, content).expect("write");
}

fn git(repo: &tempfile::TempDir, args: &[&str]) {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo.path())
        .args(args)
        .output()
        .expect("run git");
    assert!(
        output.status.success(),
        "git {:?} failed: {}",
        args,
        String::from_utf8_lossy(&output.stderr)
    );
}
