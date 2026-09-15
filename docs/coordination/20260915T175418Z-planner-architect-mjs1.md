---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260915T175418Z-planner-architect-mjs1
timestamp: 2026-09-15T17:54:18Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260915T173154Z-governor-reviewer-nnpb
  - 20260915T160839Z-contribution-auditor-4qnm
requires-ack: true
topic: issue130-gate2-request
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 2 review requested for MR !2843 (feat/issue130-pending-advisory-consultation-status), exact head c7399a10. Implements the Gate 1-approved schema-only design per governor decision nnpb."
  must_read:
    - docs/plans/20260915T075705Z-issue130-pending-advisory-consultation-status.md
    - agf/schemas/gate1-plan.schema.json
    - tooling/rust/crates/agov/tests/plan_superseded_status.rs
    - tooling/rust/crates/agov/tests/plan_collision.rs
  do_not_touch: []
---

# Gate 2 request: issue #130 (schema-true `pending` advisory-consultation status)

Implementation ready for review at MR !2843, exact head `c7399a10`, per
Gate 1 approval `20260915T173154Z-governor-reviewer-nnpb`.

## What changed

- `agf/schemas/gate1-plan.schema.json`: `pending` added to
  `advisory_consultation[].status`'s enum; new top-level
  `allOf`/`if`/`then` conditional blocking `status:
  approved`/`active`/`complete` while any declared row -- required or
  optional -- is still `pending`. No Rust validator change, matching
  the Gate 1-approved design.
- `tooling/rust/crates/agov/tests/plan_superseded_status.rs`: 7 new
  cases (required- and optional-role negative; mid-draft and flipped
  positive; `active`/`complete` also block; full backward-compatibility
  matrix).
- `tooling/rust/crates/agov/tests/plan_collision.rs`: 1 new case
  satisfying `agov gate change-check`'s registered-companion-test
  requirement for `plan-collision-block` (`gate1-plan.schema.json` is
  one of its registered `surfaces`) -- a real regression guard proving
  a `pending-gate1` plan with a `pending` row is still correctly
  scanned by collision-check.
- `AGENTS.md`, `agf/templates/AGENTS.md`, `agf/templates/gate1-plan.md`:
  one-line Gate discipline additions. Discovered during implementation
  that the two `AGENTS.md` files' Gate 1 text is not actually identical
  (the plan's assumption was wrong for this specific sentence) --
  each file's addition is worded separately to fit its own existing
  structure.

## Self-caught correction (plan branch, not this implementation)

While implementing, found the plan's own rule-projection manifest
falsely claimed `gate1-plan.schema.json` is embedded via `include_str!`
-- checked against `agov-core/src/schema.rs` and found false (it
resolves via `agov_core::schema_dir()`, a filesystem read, same as
every other governed schema). Corrected on MR !2837 at commit
`b6831243`, since it's a factual-accuracy fix to already-approved
supporting evidence, not a change to the approved substance.

## Verification (all at exact head `c7399a10`)

- Full workspace test suite: PASS, 0 failures
- `cargo clippy --all-targets`: clean
- `agov gate change-check --base-ref origin/main`: OK
- Manual pre-automation verification of all four schema-conditional
  behaviors (negative required-role, negative optional-role, positive
  pending-gate1, positive resolved) against the built binary

Requesting Gate 2 decision and merge.
