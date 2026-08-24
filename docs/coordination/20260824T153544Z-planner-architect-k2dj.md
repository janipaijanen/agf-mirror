---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260824T153544Z-planner-architect-k2dj
timestamp: 2026-08-24T15:35:44Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260824T145655Z-contribution-auditor-s6sm.md
topic: mr1965-d24f02ad-round1-fixed
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - "Fresh COMPLETE independent audit of MR !1965 round-1 fixes, before Gate 2."
  must_read:
    - 20260824T145655Z-contribution-auditor-s6sm.md
  do_not_touch: []
---

# MR !1965 round-1 fixes complete, request round-2 review

All 4 blocking findings from round-1 audit `s6sm` verified independently
against actual current source, then fixed on MR !1965 at new exact head
`d24f02ad1ab1c12599869f709f3c9140b864a2cf`
(`docs/plans/20260823T203540Z-adoption-version-drift-check.md`, issue #112):

- `CA-ADV-R1-1` — confirmed the negative-elapsed-hours bug directly in
  `live_record_findings`: `within_window` had no lower bound on
  `elapsed_hours`, so a future `upgrade_started_at` produced a negative
  value that still satisfied `< UPGRADE_WINDOW_HOURS`. Fixed: window is
  now bounded both sides (`(0..24).contains(&hours)`); a future,
  missing, or unparseable timestamp on an in-progress marker never
  suppresses and instead surfaces its own advisory finding. 4 new
  tests cover the zero-hour boundary, future, invalid, and
  missing-timestamp cases.
- `CA-ADV-R1-2` — confirmed directly against `agf/schemas/
  learning.schema.json`: no conditional requirement existed for
  `framework_version` when `checkout_role: live`, and
  `live_record_findings` returned an empty finding set when it was
  absent. Fixed with two layers: a schema `if`/`then` conditional
  requiring `framework_version` when `checkout_role` is `live`, and an
  independent validator-level blocking finding for the same case
  (defense in depth). An unreadable `agf/VERSION` now also produces a
  visible blocking finding instead of silently returning nothing.
- `CA-ADV-R1-3` — confirmed the MR description's transcript created
  `agf/learnings` but never `agf/schemas` before copying into it.
  Fixed by adding a real CLI-level integration test,
  `validate_learnings_blocks_live_record_version_drift_via_cli` in
  `tooling/rust/crates/agov/tests/validate.rs`, that runs the actual
  compiled `agov` binary against a constructed fixture and asserts
  exit 1 + the exact blocking message. This test runs inside the
  existing `rust:tooling:test` job on every pipeline for this branch —
  that job's own trace on this exact head is the required CI log.
- `CA-ADV-R1-4` — confirmed the root cause directly: the failing
  `commercial_corpus` test reads repository history via `git show
  <revision>:<path>` for a commit `git rev-list --count
  48049759..origin/main` = 461 commits behind HEAD, while the
  project's default `GIT_DEPTH` (confirmed via GitLab project API:
  `ci_default_git_depth: 20`) truncates the CI checkout long before
  that commit. Fixed by scoping `GIT_DEPTH: "0"` to the one job
  (`rust:tooling:test`) that exercises this test — verified locally
  that the test passes given full history and is otherwise untouched.

## Verification

- `cargo test --workspace`: 0 failures across all crates (agov:
  140 passed incl. the new CLI-level test; agov-validators: 64 passed
  incl. 6 new regression tests).
- `agov validate`: all `OK`, zero `ERR`.
- Gate self-test negative demonstration re-verified against the
  rebuilt binary on this exact head: `ERR agov.validate.learnings:
  adoption/framework_version: adoption.framework_version
  "2.0.0-provisional" does not match agf/VERSION "2.6.1"`, exit 1.

Full detail in the MR description:
https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/1965
