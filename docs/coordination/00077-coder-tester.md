---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 77
timestamp: 2026-06-12T17:01:00Z
immutable: true
references-to: []
topic: T2-8-small-task-fast-path-gate1
requires-ack: true
artifacts_owned: []
model: claude-sonnet-4-6
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 1 review of T2-8 plan — small-task fast-path formalization"
    - "Approve or reject docs/plans/T2-8-small-task-fast-path.md"
  must_read:
    - docs/plans/T2-8-small-task-fast-path.md
    - docs/proposals/synthesis.md (§T2-8)
  do_not_touch:
    - agents-govern/framework.md
    - .claude/settings.local.json
    - agents-govern/uv.lock
---

# Gate 1 Handoff: T2-8 — Small-task fast-path formalization

Plan submitted for Gate 1 review on branch `plan/T2-8-small-task-fast-path`.

## Summary

Single-file change to `agents-govern/framework.md` Pattern 6: replace
the existing prose "When to use / When NOT to use" bullet lists with a
formal criteria table. No schema changes, no new coordination kinds, no
tooling changes.

`kind: fast-track-waiver` already exists in the coordination channel.
T2-8 adds the machine-readable allowability criteria that give it meaning.

## Downstream impact

ADDITIVE — adds formal criteria to existing Pattern 6 prose. No existing
pipeline step or contract changed.

## Key design decisions requiring Gate 1 input

1. **Threshold: ≤ 3 files.** Framework currently says "1-3 files" in prose.
   Plan proposes to formalize this as `≤ 3`. Confirm or adjust.
2. **Completion note required (not full Gate 2).** Fast-tracked tasks still
   close the audit trail with a `kind: completion`, but no Gate 2 ceremony.
   Confirm this is the intended Gate 2 waiver shape.
3. **Scope-increase escalation trigger.** Plan says Coder-Tester must stop
   and escalate if the task grows beyond criteria. Confirm the escalation
   path is a `kind: note` to governor-reviewer (not a new handoff kind).
