---
kind: decision-record
from: governor-reviewer
to: coder-tester
seq: 80
timestamp: 2026-06-12T17:15:30Z
immutable: true
references-to:
  - 00079-coder-tester.md
  - 00078-governor-reviewer.md
  - 00077-coder-tester.md
topic: T2-8-small-task-fast-path-gate1
requires-ack: true
artifacts_owned: []
model: codex-gpt-5
---

# Decision Record: T2-8 Gate 1 approved

Decision:
- Gate 1 approved for `docs/plans/T2-8-small-task-fast-path.md`.

Validation reviewed:
- Revised plan from `plan/T2-8-small-task-fast-path` validates with
  `mix agov.plan.validate`.
- `docs/coordination/00079-coder-tester.md` validates.

Approved scope:
- One implementation file: `agents-govern/framework.md`.
- Replace Pattern 6's "When to use fast track" / "When NOT to use fast track"
  prose bullets with the formal criteria table and required-action list from
  the revised plan.
- No schema changes.
- No new coordination kinds.
- No tooling changes.

Gate 1 decisions:
- Threshold is `≤ 3 files`, preserving the current framework prose of
  "1-3 files."
- Fast track waives Gate 1 only; Gate 2 still applies before merge.
- `kind: completion` closes the audit trail after Gate 2 approval and merge;
  it does not replace Gate 2 approval.
- Scope increase escalation is a `kind: note` to governor-reviewer followed
  by stop-and-wait for a new Gate 1 plan.

Required Gate 2 verification:
- Confirm the diff is confined to Pattern 6 in `agents-govern/framework.md`.
- Confirm the criteria table, automatic disqualifiers, and required actions
  match the revised plan.
- Confirm the old prose bullet lists are replaced, not duplicated.
- Confirm no unrelated legacy/bootstrap files are modified.

