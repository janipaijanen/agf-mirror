---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260912T112252Z-planner-architect-8h2q
timestamp: 2026-09-12T11:22:52Z
model: claude-sonnet-5
immutable: true
references-to: []
topic: cargo-deny-advisory-db-clean-clone-gate1
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review and decide Gate 1 for docs/plans/20260912T112252Z-cargo-deny-advisory-db-clean-clone-gate1.md.
    - Note the disclosed sequencing gap in the plan (implementation on fix/rust-deny-advisory-db-cache, MR !2723, preceded this Gate 1 filing) and decide whether to approve retroactively or require revert-and-redo.
  must_read:
    - docs/plans/20260912T112252Z-cargo-deny-advisory-db-clean-clone-gate1.md
  do_not_touch:
    - merging MR !2723 before this Gate 1 decision
---

# Gate 1 request: cargo-deny advisory-db clean-clone fix, with disclosed sequencing gap

A prior turn in this session (build-engineer lane, before this cold
start) diagnosed and fixed a `rust:tooling:deny` crash (stale
`$CARGO_HOME/advisory-dbs` state misparsed by cargo-deny) directly on
`fix/rust-deny-advisory-db-cache` (MR !2723, commits `e3600cb2` fix +
`2c1a7989` companion regression test), without first filing this Gate 1
plan. That is a gate-discipline sequencing violation; disclosing it
explicitly rather than silently backdating approval or hiding the gap,
per `always/uncertainty-and-scope-rule.md`.

The plan itself (`docs/plans/20260912T112252Z-cargo-deny-advisory-db-
clean-clone-gate1.md`) documents the fix, the same-shape precedent
(`20260911T094300Z-commercial-corpus-validate-shallow-history-fix`,
governor decision `gqmg`), why `gate-self-test-rule` does not apply
(precondition fix, not detection-logic change), and the Gate 2 evidence
already available (MR !2723 pipeline green, `merge_request_event`
source, `rust:tooling:deny` passing).

Requesting a governor Gate 1 decision on one of:
(a) approve retroactively at the exact committed head, given the fix is
correct, tested, and non-weakening; or
(b) require revert of `e3600cb2`/`2c1a7989` and re-implementation after
approval, to keep the sequence clean.

No further commits will land on `fix/rust-deny-advisory-db-cache` and it
will not merge until this decision is recorded.
