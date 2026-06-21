use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn cold_start_lists_active_roles_and_pending_handoffs() {
    let fixture = Fixture::new();
    fixture.write_coord("agov-config.yaml", "from_aliases: {}\n");
    fixture.write_coord(
        "20260621T000000Z-planner-architect-a001.md",
        &handoff(
            "20260621T000000Z-planner-architect-a001",
            "coder-tester",
            "implement session resume",
            &[],
        ),
    );
    fixture.write_coord(
        "20260621T000100Z-coder-tester-a002.md",
        &handoff(
            "20260621T000100Z-coder-tester-a002",
            "governor-reviewer",
            "review session resume",
            &[],
        ),
    );
    let manifest = fixture.write_file(
        "MANIFEST.yaml",
        r#"
agents:
  coder-tester:
    status: active
  governor-reviewer:
    status: active
  build-engineer:
    status: suspended
"#,
    );

    let output = agov(&[
        "session",
        "resume",
        "--dir",
        fixture.coord_dir().to_str().unwrap(),
        "--manifest",
        manifest.to_str().unwrap(),
        "--base-branch",
        "main",
    ]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Session bootstrap"));
    assert!(stdout.contains("Role: not selected"));
    assert!(stdout.contains("Active roles:"));
    assert!(stdout.contains("coder-tester  -> agov session resume --role coder-tester"));
    assert!(stdout.contains("governor-reviewer  -> agov session resume --role governor-reviewer"));
    assert!(!stdout.contains("build-engineer  ->"));
    assert!(stdout.contains("Pending handoffs by role:"));
    assert!(stdout.contains("coder-tester:"));
    assert!(stdout.contains("governor-reviewer:"));
}

#[test]
fn role_resume_shows_pending_handoffs_claims_decisions_and_must_read() {
    let fixture = Fixture::new();
    fixture.write_coord("agov-config.yaml", "current_role: planner-architect\n");
    fixture.write_coord(
        "20260621T000000Z-planner-architect-b001.md",
        &handoff(
            "20260621T000000Z-planner-architect-b001",
            "coder-tester",
            "old resolved handoff",
            &[],
        ),
    );
    fixture.write_coord(
        "20260621T000100Z-governor-reviewer-b002.md",
        &decision(
            "20260621T000100Z-governor-reviewer-b002",
            "approve old handoff",
            &["20260621T000000Z-planner-architect-b001.md"],
        ),
    );
    fixture.write_coord(
        "20260621T000200Z-planner-architect-b003.md",
        &handoff(
            "20260621T000200Z-planner-architect-b003",
            "coder-tester",
            "new rust session resume handoff",
            &["docs/plans/rust-agov-complete.md"],
        ),
    );
    fixture.write_coord(
        "20260621T000300Z-coder-tester-b004.md",
        &claim(
            "20260621T000300Z-coder-tester-b004",
            "coder-tester",
            "tooling/rust/crates/agov/src/main.rs",
            None,
        ),
    );
    fixture.write_coord(
        "20260621T000400Z-coder-tester-b005.md",
        &claim(
            "20260621T000400Z-coder-tester-b005",
            "coder-tester",
            "docs/coordination/session-resume.md",
            Some("2025-01-01T00:00:00Z"),
        ),
    );
    fixture.write_coord(
        "20260621T000500Z-planner-architect-b006.md",
        &claim(
            "20260621T000500Z-planner-architect-b006",
            "planner-architect",
            "docs/plans/other.md",
            None,
        ),
    );

    let output = agov(&[
        "session",
        "resume",
        "--role",
        "coder-tester",
        "--dir",
        fixture.coord_dir().to_str().unwrap(),
    ]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Session resume"));
    assert!(stdout.contains("Role: coder-tester"));
    assert!(stdout.contains("new rust session resume handoff"));
    assert!(!stdout.contains("old resolved handoff  [ready]"));
    assert!(stdout.contains("Open claims for coder-tester:"));
    assert!(stdout.contains("tooling/rust/crates/agov/src/main.rs  id "));
    assert!(stdout.contains("Stale claims for coder-tester:"));
    assert!(stdout.contains("docs/coordination/session-resume.md  id "));
    assert!(!stdout.contains("docs/plans/other.md"));
    assert!(stdout.contains("Recent decisions:"));
    assert!(stdout.contains("approve old handoff"));
    assert!(stdout.contains("Must read:"));
    assert!(stdout.contains("docs/plans/rust-agov-complete.md"));
}

#[test]
fn dotted_session_resume_alias_is_supported() {
    let fixture = Fixture::new();
    fixture.write_coord("agov-config.yaml", "current_role: coder-tester\n");

    let output = agov(&[
        "agov.session.resume",
        "--dir",
        fixture.coord_dir().to_str().unwrap(),
    ]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Session resume"));
    assert!(stdout.contains("Role: coder-tester"));
}

fn agov(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_agov"))
        .args(args)
        .current_dir(repo_root())
        .output()
        .expect("run agov")
}

struct Fixture {
    root: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "agov_session_resume_{}_{}",
            std::process::id(),
            unique_suffix()
        ));
        fs::create_dir_all(root.join("coord")).unwrap();
        Self { root }
    }

    fn coord_dir(&self) -> PathBuf {
        self.root.join("coord")
    }

    fn write_coord(&self, name: &str, content: &str) -> PathBuf {
        self.write_at(&self.coord_dir().join(name), content)
    }

    fn write_file(&self, name: &str, content: &str) -> PathBuf {
        self.write_at(&self.root.join(name), content)
    }

    fn write_at(&self, path: &Path, content: &str) -> PathBuf {
        fs::write(path, content).unwrap();
        path.to_path_buf()
    }
}

impl Drop for Fixture {
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

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("repo root")
        .to_path_buf()
}

fn handoff(id: &str, to: &str, topic: &str, must_read: &[&str]) -> String {
    let must_read = yaml_list(must_read);
    format!(
        r#"---
kind: handoff
from: planner-architect
to: {to}
id: {id}
timestamp: 2026-06-21T00:00:00Z
immutable: true
references-to: []
topic: {topic}
requires-ack: false
artifacts_owned: []
model: test
handoff:
  next_agent: {to}
  status: ready
  task:
    - {topic}
  must_read:
{must_read}
  do_not_touch: []
---

Handoff fixture.
"#
    )
}

fn claim(id: &str, from: &str, path: &str, expires_on: Option<&str>) -> String {
    let expires_on = expires_on
        .map(|value| format!("expires_on: \"{value}\"\n"))
        .unwrap_or_default();
    format!(
        r#"---
kind: claim
from: {from}
to: all
id: {id}
timestamp: 2026-06-21T00:00:00Z
immutable: true
references-to: []
topic: claim {path}
requires-ack: false
artifacts_owned:
  - {path}
model: test
{expires_on}---

Claim fixture.
"#
    )
}

fn decision(id: &str, topic: &str, refs: &[&str]) -> String {
    let refs = yaml_list(refs);
    format!(
        r#"---
kind: decision-record
from: governor-reviewer
to: all
id: {id}
timestamp: 2026-06-21T00:01:00Z
immutable: true
references-to:
{refs}
topic: {topic}
requires-ack: false
artifacts_owned: []
model: test
---

Decision fixture.
"#
    )
}

fn yaml_list(items: &[&str]) -> String {
    if items.is_empty() {
        "    []".to_string()
    } else {
        items
            .iter()
            .map(|item| format!("    - {item}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
