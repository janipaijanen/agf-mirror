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

/// Issue #80 (a): an `ack` record requiring its own acknowledgement is not a
/// meaningful obligation, so the schema forbids the combination outright.
#[test]
fn ack_kind_with_requires_ack_true_fails_schema_validation() {
    let fixture = tempfile::tempdir().unwrap();
    write_message(
        &fixture,
        "20260720T100000Z-coder-tester-abcd.md",
        r#"---
kind: ack
from: coder-tester
to: governor-reviewer
id: 20260720T100000Z-coder-tester-abcd
timestamp: 2026-07-20T10:00:00Z
immutable: true
references-to:
  - 20260719T100000Z-governor-reviewer-wxyz.md
topic: ack-requires-ack-negative-fixture
requires-ack: true
artifacts_owned: []
---

# Ack
"#,
    );

    let validate = agov(&[
        "coord",
        "validate",
        fixture
            .path()
            .join("20260720T100000Z-coder-tester-abcd.md")
            .to_str()
            .unwrap(),
    ]);

    assert_ne!(
        validate.status.code(),
        Some(0),
        "an ack record with requires-ack: true must fail schema validation"
    );
}

/// Positive control: `ack` with `requires-ack: false` (the normal case) is
/// unaffected by the new constraint.
#[test]
fn ack_kind_with_requires_ack_false_passes_schema_validation() {
    let fixture = tempfile::tempdir().unwrap();
    write_message(
        &fixture,
        "20260720T100000Z-coder-tester-abcd.md",
        r#"---
kind: ack
from: coder-tester
to: governor-reviewer
id: 20260720T100000Z-coder-tester-abcd
timestamp: 2026-07-20T10:00:00Z
immutable: true
references-to:
  - 20260719T100000Z-governor-reviewer-wxyz.md
topic: ack-requires-ack-positive-fixture
requires-ack: false
artifacts_owned: []
---

# Ack
"#,
    );

    let validate = agov(&[
        "coord",
        "validate",
        fixture
            .path()
            .join("20260720T100000Z-coder-tester-abcd.md")
            .to_str()
            .unwrap(),
    ]);

    assert_eq!(
        validate.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&validate.stderr)
    );
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

/// Issue #80: `coord pending` is a distinct surface from `handoff next` --
/// a non-handoff, non-ack `requires-ack: true` record shows as pending
/// until a later record's `references-to` names it directly, and a
/// pre-cutoff record never appears regardless of resolution state.
#[test]
fn coord_pending_tracks_non_handoff_requires_ack_and_respects_cutoff() {
    let fixture = tempfile::tempdir().unwrap();
    write_message(
        &fixture,
        "20260720T090000Z-contribution-auditor-0000.md",
        r#"---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260720T090000Z-contribution-auditor-0000
timestamp: 2026-07-20T09:00:00Z
immutable: true
references-to: []
topic: pre-cutoff-review
requires-ack: true
artifacts_owned: []
---

# Pre-cutoff review (must never appear, regardless of cutoff)
"#,
    );
    write_message(
        &fixture,
        "20260720T100000Z-contribution-auditor-aaaa.md",
        r#"---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260720T100000Z-contribution-auditor-aaaa
timestamp: 2026-07-20T10:00:00Z
immutable: true
references-to: []
topic: post-cutoff-review
requires-ack: true
artifacts_owned: []
---

# Post-cutoff review
"#,
    );

    let before_resolution = agov(&[
        "coord",
        "pending",
        "--dir",
        fixture.path().to_str().unwrap(),
        "--cutoff",
        "2026-07-20T09:30:00Z",
    ]);
    assert_eq!(before_resolution.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&before_resolution.stdout);
    assert!(stdout.contains("20260720T100000Z-contribution-auditor-aaaa.md"));
    assert!(
        !stdout.contains("20260720T090000Z-contribution-auditor-0000.md"),
        "pre-cutoff record must never appear:\n{stdout}"
    );

    write_message(
        &fixture,
        "20260720T110000Z-governor-reviewer-bbbb.md",
        r#"---
kind: decision-record
from: governor-reviewer
to: contribution-auditor
id: 20260720T110000Z-governor-reviewer-bbbb
timestamp: 2026-07-20T11:00:00Z
immutable: true
references-to:
  - 20260720T100000Z-contribution-auditor-aaaa.md
topic: post-cutoff-review-resolution
requires-ack: false
artifacts_owned: []
---

# Resolves the post-cutoff review
"#,
    );

    let after_resolution = agov(&[
        "coord",
        "pending",
        "--dir",
        fixture.path().to_str().unwrap(),
        "--cutoff",
        "2026-07-20T09:30:00Z",
    ]);
    assert_eq!(after_resolution.status.code(), Some(2));
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
        .env("AGOV_TOKEN_ROI", "0")
        .args(args)
        .output()
        .unwrap()
}
