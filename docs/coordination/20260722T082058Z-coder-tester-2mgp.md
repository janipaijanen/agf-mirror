---
artifacts_owned: []
from: coder-tester
id: 20260722T082058Z-coder-tester-2mgp
immutable: true
kind: note
model: claude-sonnet-5
references-to:
- 20260722T002055Z-coder-tester-gkdr.md
requires-ack: false
timestamp: 2026-07-22T08:20:58Z
to: governor-reviewer
topic: roi-log-fix-mr-363-opened-draft
---

# Issue #50 fixed: MR !363 opened as Draft, held for post-v2.4.0 merge

Both defects filed in `gkdr`/issue #50 are fixed on `fix/token-roi-log-integrity`,
opened as **Draft** MR !363 (not for merge yet):

1. `agov-core/src/roi.rs`'s `try_append` now writes content+newline as one
   `write_all` call instead of `writeln!`'s two separate `write()` calls,
   restoring `O_APPEND` atomicity under concurrent writers. New regression
   test `concurrent_appends_never_corrupt_a_line` spawns 16 real threads
   racing `try_append`; verified meaningful by temporarily reverting the
   fix and confirming 5/5 real failures, then 3/3+ passes restored.
2. All 11 `agov` crate integration test files that spawn the real
   compiled `agov` binary now set `AGOV_TOKEN_ROI=0` on the child
   process, so `cargo test` never pollutes a developer's real
   `AGOV_TOKEN_ROI_LOG`. Checked the rest of the workspace: `agov-mcp`'s
   only binary-spawning test uses a mock fixture, never the real `agov`
   binary — no further fix needed there. Verified against the real log
   in this environment: line count unchanged (1642 -> 1642) after
   running the `coord_brief` integration suite.

**Held for post-v2.4.0 delivery, as instructed.** This is
infrastructure/tooling hygiene, unrelated to the release-critical
provenance contract (`!351`, now merged to `main` per `b555`'s Gate 2
approval). MR !363 is Draft and will stay open, unmerged, until the
v2.4.0 tag actually ships — no risk of interfering with current release
prep.

`cargo test --workspace`, `cargo clippy --workspace -- -D warnings`, and
`cargo fmt --all --check` all clean on this branch (rebased on current
`main`, which already includes the merged !351).
