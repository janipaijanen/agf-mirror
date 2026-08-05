use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn reports_unbridged_obligations_and_sequence_collisions() {
    let repo = GitFixture::new();
    repo.write(
        "docs/coordination/00100-governor-reviewer.md",
        &decision(100, "base decision", &[]),
    );
    repo.write(
        "docs/coordination/00188-planner-architect.md",
        &decision(188, "base collision", &[]),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "base"]);
    repo.git(&["branch", "-M", "main"]);
    repo.git(&["checkout", "-b", "feat/hidden"]);
    repo.write(
        "docs/coordination/00111-coder-tester.md",
        &handoff(111, "governor-reviewer", "unbridged gate2"),
    );
    repo.write(
        "docs/coordination/00188-coder-tester.md",
        &handoff(188, "governor-reviewer", "collision"),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "hidden coordination"]);

    let output = repo.agov(&[
        "coord",
        "visibility",
        "--base-branch",
        "main",
        "--branch",
        "feat/hidden",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ERR sequence collision"));
    assert!(stdout.contains("seq: 188"));
    assert!(stdout.contains("WARN branch-local coordination obligations not visible"));
    assert!(stdout.contains("branch: feat/hidden"));
    assert!(stdout.contains("topic: unbridged gate2"));
}

#[test]
fn ignores_handoffs_bridged_by_reference_or_body() {
    let repo = GitFixture::new();
    repo.write(
        "docs/coordination/00100-governor-reviewer.md",
        &decision(100, "base decision", &[]),
    );
    repo.write(
        "docs/coordination/00120-coder-tester.md",
        &decision(
            120,
            "reference bridge",
            &["docs/coordination/00110-coder-tester.md"],
        ),
    );
    repo.write(
        "docs/coordination/00130-coder-tester.md",
        &(decision(130, "body bridge", &[])
            + "\nfeat/bridged docs/coordination/00121-coder-tester.md\n"),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "base"]);
    repo.git(&["branch", "-M", "main"]);
    repo.git(&["checkout", "-b", "feat/bridged"]);
    repo.write(
        "docs/coordination/00110-coder-tester.md",
        &handoff(110, "governor-reviewer", "referenced bridge"),
    );
    repo.write(
        "docs/coordination/00121-coder-tester.md",
        &handoff(121, "planner-architect", "body bridge"),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "bridged coordination"]);

    let output = repo.agov(&[
        "coord",
        "visibility",
        "--base-branch",
        "main",
        "--branch",
        "feat/bridged",
    ]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("OK coordination visibility clean relative to main"));
}

#[test]
fn emits_stable_json_fields() {
    let repo = GitFixture::new();
    repo.write(
        "docs/coordination/00100-governor-reviewer.md",
        &decision(100, "base decision", &[]),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "base"]);
    repo.git(&["branch", "-M", "main"]);
    repo.git(&["checkout", "-b", "feat/json"]);
    repo.write(
        "docs/coordination/00111-coder-tester.md",
        &handoff(111, "governor-reviewer", "json handoff"),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "json coordination"]);

    let output = repo.agov(&[
        "coord",
        "visibility",
        "--base-branch",
        "main",
        "--branch",
        "feat/json",
        "--format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    assert_eq!(json["base_branch"], "main");
    assert_eq!(json["exit_code"], 1);
    assert_eq!(json["unbridged_obligations"][0]["branch"], "feat/json");
    assert_eq!(json["unbridged_obligations"][0]["seq"], 111);
}

/// Issue #85, outcome (a) of the required exit-code matrix: sequence
/// collisions confined purely to base-branch history (both colliding
/// files exist only on the base, unrelated to the branch under
/// inspection) must not block -- exit `0`, reported as a single summary
/// line, not a per-collision `ERR` block.
#[test]
fn legacy_base_only_collision_does_not_block_and_reports_summary() {
    let repo = GitFixture::new();
    repo.write(
        "docs/coordination/00050-governor-reviewer.md",
        &decision(50, "legacy base collision one", &[]),
    );
    repo.write(
        "docs/coordination/00050-planner-architect.md",
        &decision(50, "legacy base collision two", &[]),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "base with legacy collision"]);
    repo.git(&["branch", "-M", "main"]);
    repo.git(&["checkout", "-b", "feat/clean"]);
    repo.write("docs/coordination/.gitkeep", "");
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "unrelated branch commit, no coordination messages"]);

    let output = repo.agov(&[
        "coord",
        "visibility",
        "--base-branch",
        "main",
        "--branch",
        "feat/clean",
    ]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("ERR sequence collision"));
    assert!(stdout.contains(
        "WARN 1 legacy sequence collision group(s) confined to base branch history; run agov coord validate for detail"
    ));
}

/// Issue #85, outcome (b) -- the mixed case `p438`/`61gt` required:
/// base-history-only legacy noise plus a genuine unbridged obligation on
/// the branch under test must still exit `1` (the legacy noise never
/// escalates to `2`, and never suppresses the `1`), with the obligation
/// block rendered before the legacy-collision summary line.
#[test]
fn legacy_base_only_collision_plus_unbridged_obligation_exits_one_with_obligation_first() {
    let repo = GitFixture::new();
    repo.write(
        "docs/coordination/00050-governor-reviewer.md",
        &decision(50, "legacy base collision one", &[]),
    );
    repo.write(
        "docs/coordination/00050-planner-architect.md",
        &decision(50, "legacy base collision two", &[]),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "base with legacy collision"]);
    repo.git(&["branch", "-M", "main"]);
    repo.git(&["checkout", "-b", "feat/obligation"]);
    repo.write(
        "docs/coordination/00300-coder-tester.md",
        &handoff(300, "governor-reviewer", "unbridged gate2"),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "unbridged obligation"]);

    let output = repo.agov(&[
        "coord",
        "visibility",
        "--base-branch",
        "main",
        "--branch",
        "feat/obligation",
    ]);

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("ERR sequence collision"));
    let obligation_pos = stdout
        .find("WARN branch-local coordination obligations not visible")
        .expect("obligation block must be present");
    let summary_pos = stdout
        .find("legacy sequence collision group(s) confined to base branch history")
        .expect("legacy-collision summary line must be present");
    assert!(
        obligation_pos < summary_pos,
        "obligation block must render before the legacy-collision summary:\n{stdout}"
    );
}

/// Issue #85 (B-V26-1 frozen contract), positive and negative JSON shape
/// assertions per `boundary-shape-testing-rule.md`: `legacy_base_collisions`
/// is always an object with integer `count` and array `seqs`, and
/// `sequence_collisions` no longer contains `"type": "base_duplicate_seq"`
/// entries -- the negative case that would silently pass if an
/// implementation kept the old field contents and only changed exit-code
/// logic around them.
#[test]
fn legacy_base_collisions_json_shape_and_narrowed_sequence_collisions() {
    let repo = GitFixture::new();
    repo.write(
        "docs/coordination/00050-governor-reviewer.md",
        &decision(50, "legacy base collision one", &[]),
    );
    repo.write(
        "docs/coordination/00050-planner-architect.md",
        &decision(50, "legacy base collision two", &[]),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "base with legacy collision"]);
    repo.git(&["branch", "-M", "main"]);
    repo.git(&["checkout", "-b", "feat/json-legacy"]);
    repo.write("docs/coordination/.gitkeep", "");
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "unrelated branch commit, no coordination messages"]);

    let output = repo.agov(&[
        "coord",
        "visibility",
        "--base-branch",
        "main",
        "--branch",
        "feat/json-legacy",
        "--format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    assert_eq!(json["legacy_base_collisions"]["count"], 1);
    assert_eq!(json["legacy_base_collisions"]["seqs"], serde_json::json!([50]));
    assert_eq!(json["sequence_collisions"], serde_json::json!([]));
    for entry in json["sequence_collisions"].as_array().unwrap() {
        assert_ne!(
            entry["type"], "base_duplicate_seq",
            "sequence_collisions must never contain base_duplicate_seq entries after this change"
        );
    }
}

/// Companion negative-shape check using the clean-repository case: even
/// with zero legacy collisions, `legacy_base_collisions` must be present
/// as `{"count": 0, "seqs": []}`, never `null` or omitted.
#[test]
fn legacy_base_collisions_present_with_empty_values_when_clean() {
    let repo = GitFixture::new();
    repo.write(
        "docs/coordination/00100-governor-reviewer.md",
        &decision(100, "base decision", &[]),
    );
    repo.write(
        "docs/coordination/00120-coder-tester.md",
        &decision(
            120,
            "reference bridge",
            &["docs/coordination/00110-coder-tester.md"],
        ),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "base"]);
    repo.git(&["branch", "-M", "main"]);
    repo.git(&["checkout", "-b", "feat/clean-json"]);
    repo.write(
        "docs/coordination/00110-coder-tester.md",
        &handoff(110, "governor-reviewer", "referenced bridge"),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "bridged coordination"]);

    let output = repo.agov(&[
        "coord",
        "visibility",
        "--base-branch",
        "main",
        "--branch",
        "feat/clean-json",
        "--format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    assert_eq!(
        json["legacy_base_collisions"],
        serde_json::json!({"count": 0, "seqs": []})
    );
}

struct GitFixture {
    root: PathBuf,
}

impl GitFixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "agov_coord_visibility_{}_{}",
            std::process::id(),
            unique_suffix()
        ));
        fs::create_dir_all(root.join("docs/coordination")).unwrap();
        let fixture = Self { root };
        fixture.git(&["init", "-q"]);
        fixture.git(&["config", "user.email", "test@example.invalid"]);
        fixture.git(&["config", "user.name", "AGOV Test"]);
        fixture
    }

    fn write(&self, path: &str, content: &str) {
        let path = self.root.join(path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, content).unwrap();
    }

    fn git(&self, args: &[&str]) {
        let output = Command::new("git")
            .args(args)
            .current_dir(&self.root)
            .output()
            .expect("run git");
        assert!(
            output.status.success(),
            "git {:?} failed\nstdout:\n{}\nstderr:\n{}",
            args,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    fn agov(&self, args: &[&str]) -> std::process::Output {
        Command::new(env!("CARGO_BIN_EXE_agov"))
            .env("AGOV_TOKEN_ROI", "0")
            .args(args)
            .current_dir(&self.root)
            .output()
            .expect("run agov")
    }
}

impl Drop for GitFixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn unique_suffix() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}

fn handoff(seq: i64, to: &str, topic: &str) -> String {
    frontmatter(&format!(
        r#"kind: handoff
from: coder-tester
to: {to}
seq: {seq}
timestamp: 2026-06-13T00:00:00Z
immutable: true
references-to: []
topic: {topic}
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: {to}
  status: ready
  task:
    - {topic}
  must_read: []
  do_not_touch: []
"#
    ))
}

fn decision(seq: i64, topic: &str, refs: &[&str]) -> String {
    let refs = if refs.is_empty() {
        "[]".to_string()
    } else {
        format!(
            "\n{}",
            refs.iter()
                .map(|item| format!("  - {item}"))
                .collect::<Vec<_>>()
                .join("\n")
        )
    };
    frontmatter(&format!(
        r#"kind: decision-record
from: governor-reviewer
to: all
seq: {seq}
timestamp: 2026-06-13T00:01:00Z
immutable: true
references-to: {refs}
topic: {topic}
requires-ack: false
artifacts_owned: []
"#
    ))
}

fn frontmatter(yaml: &str) -> String {
    format!("---\n{yaml}---\n\nFixture body.\n")
}
