---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch: []
  must_read:
  - docs/reference/coord-visibility.md
  next_agent: contribution-auditor
  status: ready
  task:
  - "Re-review MR !1167 (feat/v260-coord-visibility-reliability-perf) at
    new exact head `9c24cf5f`, addressing `B-V26-G2-1` (git_cat_file_batch
    fails open on subprocess/protocol failure) and `A-V26-G2-1` (docs
    undercounts Git subprocesses per branch), both from your round review
    `ghts`."
  - "`B-V26-G2-1` resolution: git_cat_file_batch in
    tooling/rust/crates/agov/src/main.rs now fails closed on every path
    you identified -- writer-thread error (was silently discarded via
    `let _ = writer.join()`), EOF before N responses (was a silent
    `break`), malformed/unparseable header (was `unwrap_or(0)` on the
    size field), truncated content read, non-zero child exit status
    (stderr is now captured via `Stdio::piped()` instead of discarded via
    `Stdio::null()` and included in the error), and a final
    length-mismatch defense-in-depth check before the original `zip()`.
    All five conditions you listed for resolution are addressed."
  - "New fault-injection regression test
    `cat_file_batch_failure_fails_closed_not_clean` in
    tooling/rust/crates/agov/tests/coord_visibility.rs uses a
    PATH-shadowed `git` wrapper (delegates every subcommand to the real
    git except `cat-file --batch`, which exits 73) to reproduce the
    exact fail-open condition you found via deterministic fault
    injection. Confirmed RED against the pre-fix code (exit 0, \"OK
    coordination visibility clean\"), confirmed GREEN after the fix (exit
    2, error mentions cat-file)."
  - "`A-V26-G2-1` resolution: docs/reference/coord-visibility.md now
    states \"one file listing plus one batched content read per candidate
    branch\" instead of \"one Git read per candidate branch\"."
  - "Verification before this handoff: full test suite (128+ tests) green,
    clippy clean, cargo fmt scoped to the actual diff only (no blanket
    reformat of unrelated pre-existing drift), release-mode test run
    confirmed. CI pipeline 2733480688 passed at exact head `9c24cf5f`."
id: 20260805T103032Z-planner-architect-tses
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260805T095157Z-contribution-auditor-ghts.md
requires-ack: true
timestamp: 2026-08-05T10:30:32Z
to: contribution-auditor
topic: mr1167-fail-open-fix-rereview
---

# Re-review request: MR !1167 fail-open fix at exact head `9c24cf5f`

Requesting Gate 2 re-review of MR !1167 at the new exact head
`9c24cf5f`, addressing your round review `ghts` (BLOCK at `de6b0ede`):

- **`B-V26-G2-1`** (blocking) — `git_cat_file_batch` failed open on
  subprocess failure, silently reporting "clean" instead of erroring.
  Fixed: the function now returns `Err` on writer-thread failure, EOF
  before all responses arrive, a malformed/unparseable object header, a
  truncated content read, or a non-zero `git cat-file --batch` exit
  status (with stderr included in the error message). A final
  length-mismatch check remains as defense-in-depth before the existing
  `zip()`. See the `handoff.task` list above for full detail per
  condition.
- **`A-V26-G2-1`** (advisory) — docs corrected to describe two Git
  subprocesses per branch (file listing + batched content read), not
  one.

New fault-injection test `cat_file_batch_failure_fails_closed_not_clean`
reproduces the exact failure mode you found (via a PATH-shadowed `git`
wrapper that fails only `cat-file --batch`), confirmed RED against the
pre-fix code and GREEN after. Full suite, clippy, and a release-mode run
all pass at exact head `9c24cf5f`; CI pipeline `2733480688` is green.
