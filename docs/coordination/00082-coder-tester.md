---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 82
timestamp: 2026-06-12T20:23:00Z
immutable: true
references-to:
  - 00080-governor-reviewer.md
  - 00081-coder-tester.md
topic: T2-8-small-task-fast-path-gate2
requires-ack: true
artifacts_owned:
  - agents-govern/framework.md
model: claude-sonnet-4-6
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 2 review of T2-8 — Pattern 6 fast-track criteria formalization"
    - "Approve or reject feat/T2-8-small-task-fast-path for merge to master"
  must_read:
    - docs/plans/T2-8-small-task-fast-path.md
  do_not_touch:
    - agents-govern/framework.md
    - .claude/settings.local.json
    - agents-govern/uv.lock
---

# Gate 2 Handoff: T2-8 — Pattern 6 fast-track criteria

Branch: `feat/T2-8-small-task-fast-path`
Implementation commit: `912e83f`

## What was changed

`agents-govern/framework.md` Pattern 6 only. The two prose bullet lists
("When to use fast track" / "When NOT to use fast track") are replaced with:

1. **Allowability criteria table** (6 rows, ALL must hold): files touched ≤ 3,
   no shared interface change, no data model/schema change, no security/privacy/
   auth boundary, no active claim conflict, no scope uncertainty.

2. **Automatic disqualifiers list** (ANY triggers return to full pipeline):
   PII/tokens/credentials/permissions, architecture change, multi-jurisdiction,
   Coder-Tester uncertainty.

3. **Required actions when fast-track is granted** (4 items): Governor-Reviewer
   files `kind: fast-track-waiver`; Coder-Tester checks ownership; Gate 2 still
   applies in full; scope escalation via `kind: note` + stop-and-wait.

The anti-pattern warning line is preserved unchanged.

## Gate 2 verification checklist (per seq 80 required checks)

- [x] Diff is confined to Pattern 6 in `agents-govern/framework.md` — no other
      sections modified
- [x] Criteria table, automatic disqualifiers, and required actions match the
      revised plan (`docs/plans/T2-8-small-task-fast-path.md`)
- [x] Old prose bullet lists are replaced, not duplicated
- [x] No unrelated files modified

## Downstream impact

Classification: ADDITIVE — prose replacement making existing Pattern 6 criteria
explicit. No pipeline step removed. No existing contract changed.
