use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn usage_report_counts_kinds_senders_inactive_and_unknowns() {
    let fixture = Fixture::new();
    fixture.write_coord(
        "agov-config.yaml",
        "from_aliases:\n  old-name: coder-tester\n",
    );
    fixture.write_coord(
        "20260621T000000Z-old-name-u001.md",
        &message("20260621T000000Z-old-name-u001", "old-name", "note"),
    );
    fixture.write_coord(
        "20260621T000100Z-mystery-u002.md",
        &message("20260621T000100Z-mystery-u002", "mystery", "review"),
    );
    fixture.write_coord("20260621T000200Z-bad-u003.md", "not frontmatter\n");
    let manifest = fixture.write_file(
        "MANIFEST.yaml",
        r#"
agents:
  coder-tester:
    status: active
  planner-architect:
    status: active
  build-engineer:
    status: suspended
"#,
    );

    let output = agov(&[
        "usage",
        "report",
        "--dir",
        fixture.coord_dir().to_str().unwrap(),
        "--manifest",
        manifest.to_str().unwrap(),
    ]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stdout.contains("Channel:"));
    assert!(stdout.contains("(2 messages, 2026-06-21T00:00:00Z to 2026-06-21T00:01:00Z)"));
    assert!(stdout.contains("Window:  all messages"));
    assert!(stdout.contains("Kind distribution:"));
    assert!(stdout.contains("review"));
    assert!(stdout.contains("note"));
    assert!(stdout.contains("Sender activity (aliases applied):"));
    assert!(stdout.contains("coder-tester"));
    assert!(!stdout.contains("old-name"));
    assert!(stdout.contains("Inactive active-status agents:"));
    assert!(stdout.contains("planner-architect"));
    assert!(!stdout.contains("build-engineer"));
    assert!(stdout.contains("Unknown senders (after alias resolution):"));
    assert!(stdout.contains("mystery"));
    assert!(stderr.contains("WARN  1 invalid coordination file(s) skipped:"));
    assert!(stderr.contains("20260621T000200Z-bad-u003.md"));
}

#[test]
fn usage_report_last_limits_window_after_timestamp_sorting() {
    let fixture = Fixture::new();
    fixture.write_coord("agov-config.yaml", "from_aliases: {}\n");
    fixture.write_coord(
        "20260621T000200Z-agent-a-u003.md",
        &message("20260621T000200Z-agent-a-u003", "agent-a", "note"),
    );
    fixture.write_coord(
        "20260621T000000Z-agent-b-u001.md",
        &message("20260621T000000Z-agent-b-u001", "agent-b", "review"),
    );
    fixture.write_coord(
        "20260621T000100Z-agent-c-u002.md",
        &message("20260621T000100Z-agent-c-u002", "agent-c", "review"),
    );

    let output = agov(&[
        "usage",
        "report",
        "--dir",
        fixture.coord_dir().to_str().unwrap(),
        "--last",
        "1",
    ]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Window:  last 1 messages"));
    assert!(stdout.contains("agent-a"));
    assert!(!stdout.contains("agent-b"));
    assert!(!stdout.contains("agent-c"));
}

#[test]
fn dotted_usage_report_alias_is_supported() {
    let fixture = Fixture::new();
    fixture.write_coord("agov-config.yaml", "from_aliases: {}\n");

    let output = agov(&[
        "agov.usage.report",
        "--dir",
        fixture.coord_dir().to_str().unwrap(),
    ]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Window:  all messages"));
    assert!(stdout.contains("Kind distribution: (none)"));
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
            "agov_usage_report_{}_{}",
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

fn message(id: &str, from: &str, kind: &str) -> String {
    let timestamp = readable_timestamp(id);
    format!(
        r#"---
id: {id}
timestamp: {timestamp}
from: {from}
to: governor-reviewer
kind: {kind}
topic: usage report test
immutable: true
references-to: []
requires-ack: false
---

# Usage report test
"#
    )
}

fn readable_timestamp(id: &str) -> String {
    let raw = id.split('-').next().expect("readable id timestamp");
    format!(
        "{}-{}-{}T{}:{}:{}Z",
        &raw[0..4],
        &raw[4..6],
        &raw[6..8],
        &raw[9..11],
        &raw[11..13],
        &raw[13..15]
    )
}
