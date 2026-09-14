// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jani Päijänen
//! Row 5 S2: self-announcing coordination-record supersession, command-level
//! coverage across all four surfaces `hqps`/S2-G1-F3 required a shared
//! resolution point for: `agov handoff next`, `agov coord status`, `agov
//! coord pending`, and `agov coord list`. Unit-level edge-validity coverage
//! (all six checks, plus the governor-authority chain-walk regressions from
//! S2-G1-R2-F2) lives in `agov-coord::supersession`'s own test module; this
//! file proves the wiring reaches the real CLI commands, not only the
//! shared helper functions.
use std::{fs, path::PathBuf, process::Command};

struct Repo(tempfile::TempDir);
impl Repo {
    fn new() -> Self {
        Self(tempfile::tempdir().unwrap())
    }
    fn write(&self, path: &str, content: &str) {
        let p = self.0.path().join(path);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, content).unwrap();
    }
    fn run(&self, args: &[&str]) -> (i32, String) {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(4)
            .unwrap()
            .to_path_buf();
        let out = Command::new(env!("CARGO_BIN_EXE_agov"))
            .args(args)
            .env("AGOV_SCHEMA_DIR", root.join("agf/schemas"))
            .env("AGOV_TOKEN_ROI", "0")
            .current_dir(self.0.path())
            .output()
            .unwrap();
        (
            out.status.code().unwrap(),
            format!(
                "{}{}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            ),
        )
    }
}

const HANDOFF: &str = "---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260915T100000Z-planner-architect-aaaa
timestamp: 2026-09-15T10:00:00Z
immutable: true
references-to: []
topic: test-handoff
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task: [do the thing]
  must_read: []
  do_not_touch: []
---
Body.
";

fn valid_superseder(target_id: &str) -> String {
    format!(
        "---
kind: note
from: planner-architect
to: coder-tester
id: 20260915T110000Z-planner-architect-bbbb
timestamp: 2026-09-15T11:00:00Z
immutable: true
references-to:
  - {target_id}
topic: test-supersede
requires-ack: false
artifacts_owned: []
supersedes:
  - {target_id}
---
Supersession body.
"
    )
}

/// Positive: a handoff with a valid superseder (matching `supersedes` and
/// `references-to`, same audience, same/authorized sender, later
/// timestamp) is excluded from `agov handoff next`.
#[test]
fn handoff_next_excludes_validly_superseded_handoff() {
    let repo = Repo::new();
    repo.write(
        "docs/coordination/20260915T100000Z-planner-architect-aaaa.md",
        HANDOFF,
    );
    repo.write(
        "docs/coordination/20260915T110000Z-planner-architect-bbbb.md",
        &valid_superseder("20260915T100000Z-planner-architect-aaaa"),
    );
    let (code, out) = repo.run(&["handoff", "next", "coder-tester"]);
    assert_eq!(code, 2, "{out}");
    assert!(out.contains("No handoff found"), "{out}");
}

/// Negative (fail-safe): `supersedes` without a matching `references-to`
/// citation (S2-G1-R2-F1) must NOT exclude the handoff.
#[test]
fn handoff_next_keeps_handoff_live_when_references_to_is_missing() {
    let repo = Repo::new();
    repo.write(
        "docs/coordination/20260915T100000Z-planner-architect-aaaa.md",
        HANDOFF,
    );
    repo.write(
        "docs/coordination/20260915T110000Z-planner-architect-bbbb.md",
        "---
kind: note
from: planner-architect
to: coder-tester
id: 20260915T110000Z-planner-architect-bbbb
timestamp: 2026-09-15T11:00:00Z
immutable: true
references-to: []
topic: test-supersede
requires-ack: false
artifacts_owned: []
supersedes:
  - 20260915T100000Z-planner-architect-aaaa
---
Missing the causal reference.
",
    );
    let (code, out) = repo.run(&["handoff", "next", "coder-tester"]);
    assert_eq!(code, 0, "{out}");
    assert!(out.contains("Handoff for: coder-tester"), "{out}");
}

/// Shared-function regression (S4-F1's own precedent, applied here
/// pre-emptively): `agov handoff next` and `agov coord status` must agree
/// on the same candidate's resolved state for the same fixture.
#[test]
fn handoff_next_and_coord_status_agree_on_superseded_handoff() {
    let repo = Repo::new();
    repo.write(
        "docs/coordination/20260915T100000Z-planner-architect-aaaa.md",
        HANDOFF,
    );
    repo.write(
        "docs/coordination/20260915T110000Z-planner-architect-bbbb.md",
        &valid_superseder("20260915T100000Z-planner-architect-aaaa"),
    );

    let (next_code, next_out) = repo.run(&["handoff", "next", "coder-tester"]);
    assert_eq!(next_code, 2, "{next_out}");

    let (status_code, status_out) = repo.run(&["coord", "status", "--dir", "docs/coordination"]);
    assert_eq!(status_code, 0, "{status_out}");
    assert!(
        status_out.contains("Pending handoffs: none"),
        "{status_out}"
    );
}

/// `agov coord pending` (the non-handoff, `find_pending_all_kinds`
/// surface): a validly superseded `requires-ack: true` record is excluded.
#[test]
fn coord_pending_excludes_validly_superseded_record() {
    let repo = Repo::new();
    repo.write(
        "docs/coordination/20260915T100000Z-contribution-auditor-aaaa.md",
        "---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260915T100000Z-contribution-auditor-aaaa
timestamp: 2026-09-15T10:00:00Z
immutable: true
references-to: []
topic: test-review
requires-ack: true
artifacts_owned: []
---
Body.
",
    );
    repo.write(
        "docs/coordination/20260915T110000Z-contribution-auditor-bbbb.md",
        &valid_superseder("20260915T100000Z-contribution-auditor-aaaa")
            .replace("from: planner-architect", "from: contribution-auditor")
            .replace("to: coder-tester", "to: planner-architect"),
    );
    let (code, out) = repo.run(&["coord", "pending", "--dir", "docs/coordination"]);
    assert_eq!(code, 2, "{out}");
    assert!(out.contains("No pending requires-ack records"), "{out}");
}

/// `agov coord list`: a validly superseded record is annotated, not
/// dropped -- `coord list` is a chronological audit log, not a
/// live-obligation queue.
#[test]
fn coord_list_annotates_but_does_not_drop_superseded_record() {
    let repo = Repo::new();
    repo.write(
        "docs/coordination/20260915T100000Z-planner-architect-aaaa.md",
        HANDOFF,
    );
    repo.write(
        "docs/coordination/20260915T110000Z-planner-architect-bbbb.md",
        &valid_superseder("20260915T100000Z-planner-architect-aaaa"),
    );
    let (code, out) = repo.run(&[
        "coord",
        "list",
        "--dir",
        "docs/coordination",
        "--since",
        "20260915T095959Z",
    ]);
    assert_eq!(code, 0, "{out}");
    assert!(
        out.contains("superseded — see 20260915T110000Z-planner-architect-bbbb"),
        "{out}"
    );
    // Not dropped: both records still appear as rows.
    assert!(
        out.contains("20260915T100000Z-planner-architect-aaaa.md"),
        "{out}"
    );
    assert!(
        out.contains("20260915T110000Z-planner-architect-bbbb.md"),
        "{out}"
    );
}

/// `agov coord list --format json`: the same annotation appears as a
/// `superseded` field, and is absent (not merely null) for an unaffected row.
#[test]
fn coord_list_json_includes_superseded_field() {
    let repo = Repo::new();
    repo.write(
        "docs/coordination/20260915T100000Z-planner-architect-aaaa.md",
        HANDOFF,
    );
    repo.write(
        "docs/coordination/20260915T110000Z-planner-architect-bbbb.md",
        &valid_superseder("20260915T100000Z-planner-architect-aaaa"),
    );
    let (code, out) = repo.run(&[
        "coord",
        "list",
        "--dir",
        "docs/coordination",
        "--format",
        "json",
        "--since",
        "20260915T095959Z",
    ]);
    assert_eq!(code, 0, "{out}");
    let rows: serde_json::Value = serde_json::from_str(&out).expect("valid JSON");
    let rows = rows.as_array().expect("array of rows");
    let superseded_row = rows
        .iter()
        .find(|r| r["file"] == "20260915T100000Z-planner-architect-aaaa.md")
        .expect("candidate row present");
    assert_eq!(
        superseded_row["superseded"],
        serde_json::json!("20260915T110000Z-planner-architect-bbbb")
    );
    let superseder_row = rows
        .iter()
        .find(|r| r["file"] == "20260915T110000Z-planner-architect-bbbb.md")
        .expect("superseder row present");
    assert!(superseder_row.get("superseded").is_none());
}

// --- S2-I3 (round 4 recheck `dbaq`): real-CLI regression fixtures for the
// two round-4 semantic fixes (S2-I1 mixed-offset chronological ordering,
// S2-I2 governor-priority equal-instant tie-break), reproducing `hgrg`'s
// own controlled fixtures at the command level, not only in
// `agov-coord::supersession`'s unit tests. ---

/// S2-I1 regression, `coord list`: a schema-valid successor at a non-UTC
/// offset that is chronologically EARLIER than its target (despite its raw
/// RFC3339 string sorting later) must not annotate the target as
/// superseded.
#[test]
fn coord_list_does_not_annotate_when_offset_successor_is_chronologically_earlier() {
    let repo = Repo::new();
    repo.write(
        "docs/coordination/20260915T100000Z-planner-architect-aaaa.md",
        HANDOFF,
    );
    repo.write(
        "docs/coordination/20260915T110000Z-planner-architect-bbbb.md",
        "---
kind: note
from: planner-architect
to: coder-tester
id: 20260915T110000Z-planner-architect-bbbb
timestamp: 2026-09-15T11:00:00+02:00
immutable: true
references-to:
  - 20260915T100000Z-planner-architect-aaaa
topic: test-supersede
requires-ack: false
artifacts_owned: []
supersedes:
  - 20260915T100000Z-planner-architect-aaaa
---
Chronologically 09:00Z -- earlier than the 10:00Z target despite the
later-looking raw offset string.
",
    );
    let (code, out) = repo.run(&[
        "coord",
        "list",
        "--dir",
        "docs/coordination",
        "--since",
        "20260915T095959Z",
    ]);
    assert_eq!(code, 0, "{out}");
    assert!(
        !out.contains("superseded — see"),
        "a chronologically-earlier offset successor must not annotate the target: {out}"
    );
}

/// S2-I1 regression, `coord list`, positive counterpart: a schema-valid
/// successor at a non-UTC offset that IS chronologically later (despite a
/// raw string that sorts earlier) must annotate the target as superseded.
#[test]
fn coord_list_annotates_when_offset_successor_is_chronologically_later() {
    let repo = Repo::new();
    repo.write(
        "docs/coordination/20260915T100000Z-planner-architect-aaaa.md",
        HANDOFF,
    );
    repo.write(
        "docs/coordination/20260915T090000Z-planner-architect-bbbb.md",
        "---
kind: note
from: planner-architect
to: coder-tester
id: 20260915T090000Z-planner-architect-bbbb
timestamp: 2026-09-15T09:00:00-02:00
immutable: true
references-to:
  - 20260915T100000Z-planner-architect-aaaa
topic: test-supersede
requires-ack: false
artifacts_owned: []
supersedes:
  - 20260915T100000Z-planner-architect-aaaa
---
Chronologically 11:00Z -- later than the 10:00Z target despite the
earlier-looking raw offset string.
",
    );
    let (code, out) = repo.run(&[
        "coord",
        "list",
        "--dir",
        "docs/coordination",
        "--since",
        "20260915T085959Z",
    ]);
    assert_eq!(code, 0, "{out}");
    assert!(
        out.contains("superseded — see 20260915T090000Z-planner-architect-bbbb"),
        "a chronologically-later offset successor must annotate the target: {out}"
    );
}

const EQUAL_INSTANT_AUTHOR: &str = "---
kind: note
from: planner-architect
to: coder-tester
id: 20260915T110000Z-planner-architect-aaaa
timestamp: 2026-09-15T11:00:00Z
immutable: true
references-to:
  - 20260915T100000Z-planner-architect-target
topic: test-supersede
requires-ack: false
artifacts_owned: []
supersedes:
  - 20260915T100000Z-planner-architect-target
---
Ordinary author, sorts before the governor id alphabetically.
";

const EQUAL_INSTANT_GOVERNOR: &str = "---
kind: note
from: governor-reviewer
to: coder-tester
id: 20260915T110000Z-governor-reviewer-zzzz
timestamp: 2026-09-15T11:00:00Z
immutable: true
references-to:
  - 20260915T100000Z-planner-architect-target
topic: test-supersede
requires-ack: false
artifacts_owned: []
supersedes:
  - 20260915T100000Z-planner-architect-target
---
Governor-reviewer, sorts after the author id alphabetically but must win.
";

const EQUAL_INSTANT_TARGET_HANDOFF: &str = "---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260915T100000Z-planner-architect-target
timestamp: 2026-09-15T10:00:00Z
immutable: true
references-to: []
topic: test-handoff
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task: [do the thing]
  must_read: []
  do_not_touch: []
---
Body.
";

/// S2-I2 regression, `coord list --format json`: at an exactly equal
/// instant, a governor-reviewer-authored successor must be selected over a
/// non-governor one, even when the non-governor id would otherwise sort
/// first alphabetically -- reproducing `hgrg`'s exact mixed-authority tie.
#[test]
fn coord_list_json_selects_governor_at_equal_instant_tie() {
    let repo = Repo::new();
    repo.write(
        "docs/coordination/20260915T100000Z-planner-architect-target.md",
        EQUAL_INSTANT_TARGET_HANDOFF,
    );
    repo.write(
        "docs/coordination/20260915T110000Z-planner-architect-aaaa.md",
        EQUAL_INSTANT_AUTHOR,
    );
    repo.write(
        "docs/coordination/20260915T110000Z-governor-reviewer-zzzz.md",
        EQUAL_INSTANT_GOVERNOR,
    );
    let (code, out) = repo.run(&[
        "coord",
        "list",
        "--dir",
        "docs/coordination",
        "--format",
        "json",
        "--since",
        "20260915T095959Z",
    ]);
    assert_eq!(code, 0, "{out}");
    let rows: serde_json::Value = serde_json::from_str(&out).expect("valid JSON");
    let rows = rows.as_array().expect("array of rows");
    let target_row = rows
        .iter()
        .find(|r| r["file"] == "20260915T100000Z-planner-architect-target.md")
        .expect("target row present");
    assert_eq!(
        target_row["superseded"],
        serde_json::json!("20260915T110000Z-governor-reviewer-zzzz"),
        "the governor-reviewer candidate must win the equal-instant tie, not the \
         alphabetically-earlier author id: {out}"
    );
}

/// S2-I2 regression, queue surfaces: the same equal-instant mixed-authority
/// fixture, reused to prove `handoff next` (handoff-kind) and
/// `coord pending` (non-handoff kind) both correctly resolve the target as
/// superseded by the governor-reviewer record, not the author's -- since
/// either candidate alone (without the governor-priority fix) would also
/// exclude the target, only the JSON annotation test above can distinguish
/// "resolved correctly" from "resolved by the wrong record"; this test
/// proves the exclusion itself reaches both queue surfaces for the same
/// fixture that already proves *which* record won.
#[test]
fn queue_surfaces_agree_on_equal_instant_mixed_authority_resolution() {
    let repo = Repo::new();
    repo.write(
        "docs/coordination/20260915T100000Z-planner-architect-target.md",
        EQUAL_INSTANT_TARGET_HANDOFF,
    );
    repo.write(
        "docs/coordination/20260915T110000Z-planner-architect-aaaa.md",
        EQUAL_INSTANT_AUTHOR,
    );
    repo.write(
        "docs/coordination/20260915T110000Z-governor-reviewer-zzzz.md",
        EQUAL_INSTANT_GOVERNOR,
    );

    let (next_code, next_out) = repo.run(&["handoff", "next", "coder-tester"]);
    assert_eq!(next_code, 2, "{next_out}");
    assert!(next_out.contains("No handoff found"), "{next_out}");

    let (status_code, status_out) = repo.run(&["coord", "status", "--dir", "docs/coordination"]);
    assert_eq!(status_code, 0, "{status_out}");
    assert!(
        status_out.contains("Pending handoffs: none"),
        "{status_out}"
    );
}

/// S2-I2 regression, `coord pending` (non-handoff kind surface): the same
/// equal-instant mixed-authority pattern, applied to a `requires-ack: true`
/// non-handoff record, must also exclude the target once the
/// governor-reviewer candidate is the resolved superseder.
#[test]
fn coord_pending_resolves_equal_instant_mixed_authority_for_non_handoff_record() {
    let repo = Repo::new();
    repo.write(
        "docs/coordination/20260915T100000Z-contribution-auditor-target.md",
        "---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260915T100000Z-contribution-auditor-target
timestamp: 2026-09-15T10:00:00Z
immutable: true
references-to: []
topic: test-review
requires-ack: true
artifacts_owned: []
---
Body.
",
    );
    repo.write(
        "docs/coordination/20260915T110000Z-contribution-auditor-aaaa.md",
        "---
kind: note
from: contribution-auditor
to: planner-architect
id: 20260915T110000Z-contribution-auditor-aaaa
timestamp: 2026-09-15T11:00:00Z
immutable: true
references-to:
  - 20260915T100000Z-contribution-auditor-target
topic: test-supersede
requires-ack: false
artifacts_owned: []
supersedes:
  - 20260915T100000Z-contribution-auditor-target
---
Ordinary author, sorts before the governor id alphabetically.
",
    );
    repo.write(
        "docs/coordination/20260915T110000Z-governor-reviewer-zzzz.md",
        "---
kind: note
from: governor-reviewer
to: planner-architect
id: 20260915T110000Z-governor-reviewer-zzzz
timestamp: 2026-09-15T11:00:00Z
immutable: true
references-to:
  - 20260915T100000Z-contribution-auditor-target
topic: test-supersede
requires-ack: false
artifacts_owned: []
supersedes:
  - 20260915T100000Z-contribution-auditor-target
---
Governor-reviewer, sorts after the author id alphabetically but must win.
",
    );
    let (code, out) = repo.run(&["coord", "pending", "--dir", "docs/coordination"]);
    assert_eq!(code, 2, "{out}");
    assert!(out.contains("No pending requires-ack records"), "{out}");
}
