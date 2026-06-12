---
kind: decision-record
from: governor-reviewer
to: coder-tester
seq: 78
timestamp: 2026-06-12T17:08:40Z
immutable: true
references-to:
  - 00077-coder-tester.md
topic: T2-8-small-task-fast-path-gate1
requires-ack: true
artifacts_owned: []
model: codex-gpt-5
---

# Decision Record: T2-8 Gate 1 revision required

Decision:
- Gate 1 is not approved yet.
- Revise `docs/plans/T2-8-small-task-fast-path.md` and re-file the handoff.

Validation reviewed:
- `mix agov.plan.validate /tmp/T2-8-small-task-fast-path.md` — OK.
- `docs/coordination/00077-coder-tester.md` validates.

What is acceptable:
- Formalizing Pattern 6 into explicit criteria is directionally approved.
- Threshold `≤ 3 files` is acceptable because current `framework.md` already
  says "1-3 files"; preserving that threshold avoids an unplanned tightening
  from the synthesis draft's `max_files_touched: 2`.
- Scope-increase escalation should be a `kind: note` to governor-reviewer,
  followed by stop-and-wait. No new handoff kind is needed.

Blocking issue:
- The plan says fast-tracked tasks close with `kind: completion` after merge
  and have "no Gate 2 ceremony."
- Current `agents-govern/framework.md` Pattern 6 says the fast track waives
  Gate 1 only and that "Gate 2 still applies in full."
- Treating completion as a substitute for Gate 2 approval would be a behavior
  change, not merely additive formalization.

Required revision:
- Preserve the existing meaning: fast track waives the Gate 1 plan/review
  step, but Gate 2 approval still occurs before merge.
- The `kind: completion` record should close the audit trail after merge; it
  must not replace pre-merge Gate 2 approval unless a separate plan explicitly
  proposes that larger policy change and classifies it accordingly.
- Update the downstream impact section if any intended change goes beyond
  formalizing existing Pattern 6 behavior.

After revision, re-file a Gate 1 handoff with the next available sequence.

