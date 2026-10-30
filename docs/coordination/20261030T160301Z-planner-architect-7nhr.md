---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261030T160301Z-planner-architect-7nhr
timestamp: 2026-10-30T16:03:01Z
model: claude
immutable: true
references-to:
  - 20261030T155116Z-governor-reviewer-ghgs.md
  - 20261030T160040Z-planner-architect-gr51.md
topic: forge-evidence-plan-gate1-ready
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Arrange both advisory consultations on the pending-gate1 plan in MR !3426 at exact head 66e5ab2e5c80677eb1f57a279ecc651c8c401ad9.
    - Gate 1 decision only after both consultations answer that exact head.
  must_read:
    - docs/plans/20261030T160040Z-forge-evidence-rule.md
    - docs/coordination/20261030T155116Z-governor-reviewer-ghgs.md
  do_not_touch:
    - agf/rules/workflow/merge-request-description-rule.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Forge-evidence rule plan: ready for consultation and Gate 1

Planner-architect delivers the planning task in
`20261030T155116Z-governor-reviewer-ghgs`. This record releases the claim
`20261030T160040Z-planner-architect-gr51`.

## Delivered at exact head 66e5ab2e5c80677eb1f57a279ecc651c8c401ad9

Plan `docs/plans/20261030T160040Z-forge-evidence-rule.md`, status `pending-gate1`, MR !3426. It validates. The
blocking collision check on the committed plan reports 0 unacknowledged and 1
acknowledged overlap (the landed rule-drift plan, which declares `agf/rules/**`).

- **Owner located:** `agf/rules/workflow/merge-request-description-rule.md`. It
  already states what a description MUST name and MUST NOT contain. Other
  candidates and why they are not the owner are in the plan.
- **Boundary:** merge request titles, descriptions and notes, and issue text
  cite coordination records, plan files, commit SHAs, pipeline IDs and other
  Git or forge items, and do not refer to a chat surface. Draft clause text uses
  no product or tool name. It adds no check, no history rewrite and no deletion.
- **Projection:** one surface updated (the owner). No Pages page or runbook
  changes, so no manifest is proposed. The shipped loader already points to the
  rules directory, and the foundry loader already routes merge request
  descriptions to this rule.
- **Classification:** BREAKING by the Parnas test (a new MUST NOT), with a
  migration note and a changelog note when it ships. Evidence status:
  Theoretical.

## For the governor

Name the independent contribution-auditor and InfoSec reviewers, or waive the
rows by a governor record. Both rows are `pending`. Project-local merge request
and issue template reminders are outside this plan; say if you want them
planned separately.

## Not requested

No Gate 1 approval is assumed. This record grants no implementation, Gate 2,
merge or release authority. No shipped rule was edited.
