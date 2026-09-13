// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jani Päijänen
//! Row 5 S4 (CP-A), fixed for finding S4-F1
//! (`20260913T172241Z-coder-tester-bsbr`): the named `agov handoff next
//! <role>` command must itself show the derived `answered — ack pending`
//! state, not only the list-view renderers exercised by main.rs's own unit
//! tests. This is the command-level positive/negative test the review
//! required.
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
    fn run(&self, role: &str) -> (i32, String) {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(4)
            .unwrap()
            .to_path_buf();
        let out = Command::new(env!("CARGO_BIN_EXE_agov"))
            .args(["handoff", "next", role])
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
id: 20260913T173000Z-planner-architect-aaaa
timestamp: 2026-09-13T17:30:00Z
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

/// Negative: an untouched handoff still shows the bare declared status via
/// the named `agov handoff next <role>` command, not `answered`.
#[test]
fn handoff_next_shows_ready_with_no_reply() {
    let repo = Repo::new();
    repo.write(
        "docs/coordination/20260913T173000Z-planner-architect-aaaa.md",
        HANDOFF,
    );
    let (code, out) = repo.run("coder-tester");
    assert_eq!(code, 0, "{out}");
    assert!(out.contains("Status:      ready"), "{out}");
    assert!(!out.contains("answered"), "{out}");
}

/// Positive (S4-F1 fix): a qualifying later `review` from the handoff's own
/// `next_agent`, referencing it, makes the named `agov handoff next <role>`
/// command itself show `answered — ack pending` -- not only the list-view
/// renderers.
#[test]
fn handoff_next_shows_answered_ack_pending_with_qualifying_reply() {
    let repo = Repo::new();
    repo.write(
        "docs/coordination/20260913T173000Z-planner-architect-aaaa.md",
        HANDOFF,
    );
    repo.write(
        "docs/coordination/20260913T173100Z-coder-tester-bbbb.md",
        "---
kind: review
from: coder-tester
to: planner-architect
id: 20260913T173100Z-coder-tester-bbbb
timestamp: 2026-09-13T17:31:00Z
immutable: true
references-to:
  - 20260913T173000Z-planner-architect-aaaa
topic: test-reply
requires-ack: false
artifacts_owned: []
---
Reply body.
",
    );
    let (code, out) = repo.run("coder-tester");
    assert_eq!(code, 0, "{out}");
    assert!(out.contains("Status:      answered — ack pending"), "{out}");
}
