---
kind: decision-record
from: planner-architect
to: all
id: 20260912T151723Z-planner-architect-31pr
timestamp: 2026-09-12T15:17:23Z
model: claude-sonnet-5
immutable: true
references-to: []
requires-ack: true
topic: pause-active-work-for-customer-plan
artifacts_owned: []
---

# Governor instruction: pause active work for a potential customer

Jani instructed directly (2026-09-12, not yet in coordination until this
record): "we have a potential customer around. I want you and codex
pause all current work and take proper handling of customer plan and
proposed AGF enhancements. We may need to cut 2.10.1 for the customer,
but not yet though." Claude transcribes the human decision; authority
is the human's.

**Effective now:**

- Pause the estate/release-planning poll cycle and any other active
  implementation, review, or gate-progression work not already merged.
  This session's build-engineer/planner-architect poll loop is stopped.
- CP-B-2 implementation (`codex/cpb2-publication-hold-implementation`,
  branch not yet pushed as of this record) should pause where it stands
  — do not push, open a new MR, or continue implementation until this
  pause lifts. If work in progress is mid-commit, finish to a clean,
  reviewable state and stop there; do not leave the worktree dirty.
- No new Gate 1/Gate 2 submissions expected from either agent until the
  customer plan work is scoped.
- **v2.10.1 is explicitly not authorized yet** — "not yet though" is
  load-bearing. No version bump, tag, or release-branch work.

**What's still open:** the customer's identity, needs, and the specific
proposed AGF enhancements are not yet known to this session — Claude is
requesting those details directly from Jani before drafting anything.
Codex: if you have separate context on this customer or the proposed
enhancements from elsewhere, please share it in coordination rather than
assuming Claude already has it.

This record itself is the pause notice; a follow-up plan or scope
record will come once details are available. No implementation, gate,
or release authority is exercised by this record beyond the pause
itself.
