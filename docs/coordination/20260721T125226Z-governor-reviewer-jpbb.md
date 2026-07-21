---
kind: decision-record
from: governor-reviewer
to: all
id: 20260721T125226Z-governor-reviewer-jpbb
timestamp: 2026-07-21T12:52:26Z
immutable: true
references-to:
  - 20260721T123806Z-planner-architect-pt3m
  - 20260721T123806Z-planner-architect-7grh
  - 20260721T124650Z-planner-architect-pkjt
  - 20260721T124414Z-coder-tester-76g8
topic: v2-3-1-recovery-release-gate1-approved
requires-ack: false
artifacts_owned:
  - docs/plans/v231-recovery-release.md
---

# Gate 1 approved: v2.3.1 recovery release

The human governor approved MR !315 and the revised v2.3.1 recovery-release
plan at head `00cff1a004687abf19be228208416bfdf2d80428`.

The approved scope preserves immutable v2.3.0 history, requires supersession
of the already-merged premature completion records, includes the selected
release learnings and audit trail, corrects both changelog surfaces through
v2.3.1, and makes checksum/provenance sidecars discoverable. Deferred and
unrelated open MRs remain outside product scope exactly as listed in the plan.

Contribution Auditor review was requested with `requires-ack: true` in `7grh`
but no response had arrived at decision time. The governor exercised binding
Gate 1 authority after reviewing the revised scope. A later advisory response
may identify follow-up concerns, but it cannot silently expand or rewrite the
approved plan; material changes require a new governor disposition.

Implementation may begin only after this approved plan and decision record are
merged to `main`. Gate 2, merge, tag, and publication authority remain separate.
