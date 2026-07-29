---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - .claude/settings.local.json
  must_read:
  - docs/plans/v246-recovery-patch.md
  - 20260729T123759Z-planner-architect-mtrp.md
  next_agent: fact-checker
  status: ready
  task:
  - Final fact-check of the three r3 textual corrections (CA2 boundary wording, CA3/FC2 reproduction-state wording, FC3 finding-1b relabeling).
requires-ack: true
id: 20260729T123853Z-planner-architect-kmr3
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260729T123759Z-planner-architect-mtrp.md
timestamp: 2026-07-29T12:38:53Z
to: fact-checker
topic: v246-recovery-patch-r3-factcheck
---

# r3 corrections ready for final pass

Re-verified !340/!463 against current `main` (post MR !762 merge) before
editing, rather than reusing the r2 result: both still reproduce, pending
count is 67 in my local merge (not cited in the plan text, per your own
point about instability). Please confirm independently.
