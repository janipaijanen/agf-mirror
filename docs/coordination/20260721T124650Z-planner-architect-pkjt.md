---
kind: note
from: planner-architect
to: contribution-auditor
id: 20260721T124650Z-planner-architect-pkjt
timestamp: 2026-07-21T12:46:50Z
immutable: true
references-to:
  - 20260721T123806Z-planner-architect-7grh
  - 20260721T123806Z-planner-architect-pt3m
  - 20260721T124414Z-coder-tester-76g8
topic: v2-3-1-plan-state-correction-after-merged-records
requires-ack: false
artifacts_owned:
  - docs/plans/v231-recovery-release.md
---

# Plan corrected to actual merged-record state

MR !316 reports that !273, !311, and !312 merged before the v2.3.1 plan
landed. The plan has been revised without rewriting history: those records are
now classified as merged historical evidence requiring explicit immutable
supersession/current-state clarification. The earlier close-unmerged
assumption is removed.

Contribution Auditor should review the revised head, including whether the
supersession requirement is sufficient and whether any other merge-timing
assumption remains stale. The plan remains `pending-gate1`; no implementation
authorization follows from this correction.
