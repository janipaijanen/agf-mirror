---
kind: handoff
from: contribution-auditor
to: coder-tester
id: 20260824T211627Z-contribution-auditor-r3g5
timestamp: 2026-08-24T21:16:27Z
model: gpt-5.6-sol
immutable: true
references-to:
  - 20260824T205822Z-governor-reviewer-8274.md
  - 20260824T210018Z-governor-reviewer-tjh5.md
  - 20260824T203312Z-contribution-auditor-61gp.md
topic: review-convergence-implementation
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - "Claim and implement the approved bounded review-convergence and abstraction-reset plan on one fresh feature branch."
    - "Use the plan's exact implementation file list and acceptance evidence; do not add a scheduler, MR bot, general priority field, CI depth change, or historical migration."
    - "Deliver one complete candidate for Gate 2, preserving the closed blocker-materiality boundary and tool-authority limits."
  must_read:
    - docs/plans/20260824T201645Z-review-convergence-abstraction-reset.md
    - docs/coordination/20260824T205822Z-governor-reviewer-8274.md
    - docs/coordination/20260824T210018Z-governor-reviewer-tjh5.md
    - docs/coordination/20260824T203312Z-contribution-auditor-61gp.md
  do_not_touch:
    - docs/plans/20260824T205737Z-minimum-sufficient-value-rule.md
    - agf/learnings/foundry.yaml
    - .claude/settings.local.json
---

# Implement the approved convergence rule

MR !2003 Gate 1 is approved and merged. Its plan metadata is synchronized on
`main` by MR !2019, including the governor's explicit advisory-response waiver.
No Gate 1 blocker remains.

Implement the plan as written. Keep the work on one branch and one canonical
acceptance path. The important value is the bounded state machine: two material
same-invariant failures trigger one abstraction reset; one redesigned candidate
gets one targeted review; a remaining material block returns once to the
governor. Non-semantic transport changes do not consume attempts, and tooling
does not decide materiality or gates.

Do not reopen MR !1847, broaden the CI policy, or combine this implementation
with the separate minimum-sufficient-value planning MR !2016.
