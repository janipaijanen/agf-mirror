use std::fs;
use std::process::Command;

#[test]
fn coord_post_writes_and_validates_note() {
    let fixture = tempfile::tempdir().unwrap();

    let output = agov(&[
        "coord",
        "post",
        "--from",
        "coder-tester",
        "--to",
        "governor-reviewer",
        "--kind",
        "note",
        "--topic",
        "coord-post-test",
        "--dir",
        fixture.path().to_str().unwrap(),
    ]);

    assert_eq!(output.status.code(), Some(0));
    let files = markdown_files(&fixture);
    assert_eq!(files.len(), 1);

    let validate = agov(&["coord", "validate", files[0].to_str().unwrap()]);
    assert_eq!(
        validate.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&validate.stderr)
    );
}

#[test]
fn handoff_take_writes_ack_that_resolves_pending_handoff() {
    let fixture = tempfile::tempdir().unwrap();
    write_message(
        &fixture,
        "20260720T100000Z-planner-architect-abcd.md",
        r#"---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260720T100000Z-planner-architect-abcd
timestamp: 2026-07-20T10:00:00Z
immutable: true
references-to: []
topic: take-handoff-test
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Take this test handoff.
  must_read: []
  do_not_touch: []
---

# Handoff
"#,
    );

    let take = agov(&[
        "handoff",
        "take",
        "20260720T100000Z-planner-architect-abcd",
        "--from",
        "coder-tester",
        "--dir",
        fixture.path().to_str().unwrap(),
    ]);

    assert_eq!(
        take.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&take.stderr)
    );

    let next = agov(&[
        "handoff",
        "next",
        "coder-tester",
        "--dir",
        fixture.path().to_str().unwrap(),
    ]);
    assert_eq!(next.status.code(), Some(2));
}

#[test]
fn coord_list_since_message_id_returns_later_messages() {
    let fixture = tempfile::tempdir().unwrap();
    write_message(
        &fixture,
        "20260720T100000Z-coder-tester-abcd.md",
        note(
            "20260720T100000Z-coder-tester-abcd",
            "2026-07-20T10:00:00Z",
            "first",
        )
        .as_str(),
    );
    write_message(
        &fixture,
        "20260720T110000Z-coder-tester-efgh.md",
        note(
            "20260720T110000Z-coder-tester-efgh",
            "2026-07-20T11:00:00Z",
            "second",
        )
        .as_str(),
    );

    let output = agov(&[
        "coord",
        "list",
        "--since",
        "20260720T100000Z-coder-tester-abcd",
        "--dir",
        fixture.path().to_str().unwrap(),
    ]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("first"));
    assert!(stdout.contains("second"));
    assert!(stdout.contains("20260720T110000Z-coder-tester-efgh"));
}

#[test]
fn coord_list_returns_two_when_no_messages_match() {
    let fixture = tempfile::tempdir().unwrap();

    let output = agov(&[
        "coord",
        "list",
        "--since",
        "2026-07-20T10:00:00Z",
        "--dir",
        fixture.path().to_str().unwrap(),
    ]);

    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn coord_validate_blocks_duplicate_ids_in_same_directory() {
    let fixture = tempfile::tempdir().unwrap();
    let content = note("manual-duplicate-test", "2026-07-20T10:00:00Z", "duplicate");
    write_message(
        &fixture,
        "20260720T100000Z-coder-tester-abcd.md",
        content.as_str(),
    );
    write_message(
        &fixture,
        "20260720T100001Z-coder-tester-efgh.md",
        content.as_str(),
    );

    let target = fixture.path().join("20260720T100000Z-coder-tester-abcd.md");
    let output = agov(&["coord", "validate", target.to_str().unwrap()]);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("duplicate coordination id 'manual-duplicate-test'"));
}

fn note(id: &str, timestamp: &str, topic: &str) -> String {
    format!(
        r#"---
kind: note
from: coder-tester
to: governor-reviewer
id: {id}
timestamp: {timestamp}
immutable: true
references-to: []
topic: {topic}
requires-ack: false
artifacts_owned: []
---

# Note
"#
    )
}

fn write_message(fixture: &tempfile::TempDir, filename: &str, content: &str) {
    fs::write(fixture.path().join(filename), content).unwrap();
}

fn markdown_files(fixture: &tempfile::TempDir) -> Vec<std::path::PathBuf> {
    let mut files = fs::read_dir(fixture.path())
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("md"))
        .collect::<Vec<_>>();
    files.sort();
    files
}

fn agov(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_agov"))
        .args(args)
        .output()
        .unwrap()
}
