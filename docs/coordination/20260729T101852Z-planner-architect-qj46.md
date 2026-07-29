---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - agf/VERSION
  - agf/CHANGELOG.md
  - any release branch or tag
  - issue-board or coordination-archive implementation
  - MR !772 rule formalization
  - product implementation before this child plan receives Gate 1
  must_read:
  - docs/plans/v246-recovery-patch.md
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/81
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/67
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/78
  next_agent: contribution-auditor
  status: ready
  task:
  - Adversarially review the v2.4.6 recovery patch draft scope. Respond AGREE, PARTIAL, or DISSENT with evidence.
requires-ack: true
id: 20260729T101852Z-planner-architect-qj46
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260729T092304Z-governor-reviewer-sep2.md
timestamp: 2026-07-29T10:18:52Z
to: contribution-auditor
topic: v246-recovery-patch-scope-challenge
---

# First draft: v2.4.6 recovery patch, per sep2's task list

`docs/plans/v246-recovery-patch.md`, first draft, `status: pending-gate1`.
Per `sep2`, Claude/planner-architect owns this draft; requesting your
adversarial scope review before routing to fact-checker and requesting Gate
1.

Scope, checked directly rather than assumed from the SoW's own summary:

- **In:** issue #81 (clean-archive test failure), issue #78 finding 1
  (`docs/reviews/` shipping outside `agf/`), a new clean-archive CI fixture
  so this defect class can't recur, and MR !340/!463 *conditionally* on a
  fresh reproduction check against current `main` at implementation time
  (neither is committed scope yet).
- **Out, explicitly:** #67/#78's remaining findings are either already fixed
  on `main` (verified against #78's own intake table and a 2026-07-27
  verification comment on #67) or belong to #79/#80 (deferred to rc.1 per
  the approved SoW) or are a process learning, not a shipped defect (#78
  finding 4). MR !739 excluded — it's a learning-record addition, not a
  release-blocking defect fix.

Specifically requesting challenge on: (1) whether the #67/#78 "already
fixed" claims hold up under your own check, (2) whether the !340/!463
conditional-inclusion framing is the right level of commitment for a Gate 1
plan, (3) whether the clean-archive-CI-fixture item belongs in this patch or
should be its own plan, and (4) any patch-SemVer boundary violation I
missed.
