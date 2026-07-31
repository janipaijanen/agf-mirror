---
kind: decision-record
from: governor-reviewer
to: all
id: 20260729T134714Z-governor-reviewer-8jr7
timestamp: 2026-07-29T13:47:14Z
model: human
immutable: true
references-to: []
topic: v246-recovery-patch-gate1-approved
requires-ack: false
artifacts_owned:
  - docs/plans/v246-recovery-patch.md
---

# Gate 1 approved: v2.4.6 recovery patch

The governor-reviewer approves Gate 1 for MR !774 at exact source head
`395069e22669bfd8418fa47c2753d977977cfc1f`.

The binding instruction was given directly in the current human/AI session:

> Approved. Proceed.

This record transcribes that decision; it does not infer or expand it. The
approval is grounded in the final exact-head contribution-auditor CLEAR
`20260729T131821Z-contribution-auditor-5k85` and fact-checker CLEAR
`20260729T131822Z-fact-checker-1kjk`, published in MR !781, and exact-head
pipeline `2715375639`, which passed.

Gate 1 authorizes only the four implementation items enumerated in the plan:

1. issue #81's clean-archive fixture/test correction;
2. issue #67 Slice B's two residual Pages-URL corrections and broadened
   negative gate;
3. the clean-archive CI fixture; and
4. MR !340 and/or !463 only if each defect still reproduces at implementation
   time.

The pending plan status may be mechanically changed to `approved` without
further scope review. This decision does not authorize a version bump, release
branch, tag, GitLab Release, issue-board/archive work, MR !772 formalization,
or v2.5.0-rc.1 scope. Implementation requires Gate 2 before merge.
