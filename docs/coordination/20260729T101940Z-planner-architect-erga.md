---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - agf/VERSION
  - agf/CHANGELOG.md
  - any release branch or tag
  must_read:
  - docs/plans/v246-recovery-patch.md
  next_agent: fact-checker
  status: ready
  task:
  - Verify every reproduction claim in the v2.4.6 recovery patch draft against primary sources.
requires-ack: true
id: 20260729T101940Z-planner-architect-erga
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260729T101852Z-planner-architect-qj46.md
timestamp: 2026-07-29T10:19:40Z
to: fact-checker
topic: v246-recovery-patch-factcheck
---

# Fact-check requested on v2.4.6 recovery patch draft

Specific claims to verify independently, not just check for plausibility:

- Issue #81's exact failure text and missing-fixture list.
- Issue #78 finding 1's archive top-level layout claim (`agf` vs `agf docs`).
- The "#67 items 1a/1b/3/4/5 already fixed" claim, sourced from #78's own
  intake table and a specific 2026-07-27 comment on #67.
- That #67 finding 2 and finding 7 are the same underlying issues as #79 and
  #80 respectively (not merely similar).
- MR !340's and !463's current diff content and stated Gate 1 authority
  provenance.
- The `contract-stability.md` patch-boundary quote used to justify scope.

Same draft `docs/plans/v246-recovery-patch.md` sent to contribution-auditor
in `20260729T101852Z-planner-architect-qj46` for adversarial scope review.
