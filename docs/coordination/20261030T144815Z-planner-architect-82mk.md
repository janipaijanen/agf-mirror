---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261030T144815Z-planner-architect-82mk
timestamp: 2026-10-30T14:48:15Z
model: claude
immutable: true
references-to:
  - 20261030T141210Z-governor-reviewer-67p3.md
  - 20261030T144438Z-planner-architect-tgjg.md
topic: header-repair-plan-gate1-ready
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Dispose D1 (SPDX expression), D1b (copyright line) and D2 (learning-record template) in the pending-gate1 plan on MR !3411 at ef1f25caf8682824460fb5641de2322d52563338.
    - Name the independent contribution-auditor and infosec-sentinel reviewers, or waive the rows, before the Gate 1 decision.
  must_read:
    - docs/plans/20261030T144439Z-header-repair-plan.md
    - docs/coordination/20261030T000640Z-coder-tester-tgrk.md
  do_not_touch:
    - CHANGELOG.md
    - agf/CHANGELOG.md
    - agf/templates/learning-record.yaml
    - LICENCE
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Header-repair plan: ready for disposition and Gate 1

Planner-architect delivers the planning task assigned in
`20261030T141210Z-governor-reviewer-67p3`. This record also releases the claim
`20261030T144438Z-planner-architect-tgjg`.

## Delivered

- Plan `docs/plans/20261030T144439Z-header-repair-plan.md`, status
  `pending-gate1`, on MR !3411 at `ef1f25caf8682824460fb5641de2322d52563338`.
- It validates. The blocking collision check on the committed plan reports 0
  unacknowledged and 8 acknowledged overlaps with landed release plans.
- It covers the three files in `tgrk`, their exact current hashes, the
  path-based licensing text, header-only acceptance criteria, a projection
  assessment, risks and rollback, and an include or exclude recommendation.
  No product file is edited and no release version is chosen.

## Two findings that need the governor

- **D1, which expression.** Plain `CC-BY-SA-4.0` equals the files' effective
  terms today. The dual expression with `LicenseRef-AgentsGovern-Commercial`
  matches most sibling files (209 of 252 headered) but would also mark these
  files as available under the commercial alternative, which `licensing.md`
  says applies only where expressly granted. The plan recommends plain
  `CC-BY-SA-4.0`, identifier only, and does not decide it.
- **D2, the YAML template.** `agf/ADOPTING.md` tells adopters to copy
  `agf/templates/learning-record.yaml` to their own, adopter-owned learning
  file. A header there would be copied into every adopter record. The plan
  recommends adding it with a short note that adopters replace or remove it, or
  excluding the template as a documented residual. The governor chooses.

## Recommendation for the next maintenance batch

Include the two changelog headers once D1 is decided and the header-only
criteria pass. Include the template header only with a D2 decision; otherwise
exclude it. Exclude all other headerless files and any policy or enforcement
change. No version, milestone or date is chosen.

## Not requested

No Gate 1 approval is assumed. Both advisory rows are `pending`. This record
grants no implementation, license-change, Gate 2, merge or release authority.
