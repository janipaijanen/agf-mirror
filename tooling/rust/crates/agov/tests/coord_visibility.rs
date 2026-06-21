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
