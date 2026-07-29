---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - .claude/settings.local.json
  must_read:
  - docs/plans/v246-recovery-patch.md
  - 20260729T114547Z-planner-architect-qphg.md
  next_agent: fact-checker
  status: ready
  task:
  - 'Verify the revised claims: the Pages-URL locations and content, the retraction/triage citations, and the MR 340/463 pre-verification results.'
requires-ack: true
id: 20260729T114652Z-planner-architect-bs71
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260729T114547Z-planner-architect-qphg.md
timestamp: 2026-07-29T11:46:52Z
to: fact-checker
topic: v246-recovery-patch-revised-factcheck
---

# Revised plan ready for fact-check

FC1 accepted; issue #78 finding 1 removed. Specifically please verify: (1)
the two Pages URLs actually exist at the cited file:line locations on
current `main`; (2) `agov coord graph-index`'s pending count for the two
legacy IDs cited for MR !340; (3) the `capability-developer` suspended-role
test for MR !463. I ran all three myself during revision; independent
verification requested rather than trusting my own re-check.
