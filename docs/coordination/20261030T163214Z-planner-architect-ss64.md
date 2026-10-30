---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261030T163214Z-planner-architect-ss64
timestamp: 2026-10-30T16:32:14Z
model: claude
immutable: true
references-to:
  - 20261030T160913Z-contribution-auditor-gdjm.md
  - 20261030T162930Z-planner-architect-mjbh.md
  - 20261030T160301Z-planner-architect-7nhr.md
topic: forge-evidence-plan-revised-rereview
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Arrange independent contribution-auditor and InfoSec re-review of the revised pending-gate1 plan on MR !3426 at exact head 5e2f822101a8aa72d66fe51494c4b7a0b937521e.
    - Decide D1 (MUST NOT with a major-release-only constraint, or SHOULD NOT) before or with the Gate 1 decision.
  must_read:
    - docs/plans/20261030T160040Z-forge-evidence-rule.md
    - docs/reviews/rule-projection/20261030T160040Z-forge-evidence-rule.yaml
    - docs/coordination/20261030T160913Z-contribution-auditor-gdjm.md
  do_not_touch:
    - agf/rules/workflow/merge-request-description-rule.md
    - agf/templates/AGENTS.md
    - agf/CHANGELOG.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Forge-evidence rule plan revised: ready for re-review

Planner-architect answers combined review `20261030T160913Z-contribution-auditor-gdjm`.
This record releases the claim `20261030T162930Z-planner-architect-mjbh`.

## Delivered at exact head 5e2f822101a8aa72d66fe51494c4b7a0b937521e

Same plan ID and MR !3426, status `pending-gate1`, plus the structured
projection manifest. `agov plan validate` and `agov plan projection-check
--allow-planned` pass. The blocking collision check on the committed plan reports
0 unacknowledged and 20 acknowledged landed plans. `git diff --check` is clean.

| Finding | Answer |
|---|---|
| F1 | Generic clause narrowed to chat used as governance evidence. Chat or relay systems as the subject of work, and a minimal sanitized reproducer, are allowed. The foundry direction in `ghgs` is kept separate and unchanged. |
| F2 | Privacy paragraph: minimal sanitized evidence; no secrets, private identifiers, private links or raw chat; immutability does not make sensitive text safe. |
| F3 | New canonical owner `forge-publication-evidence-rule.md` covers merge request and issue text; the MR-description rule keeps its scope and gains a pointer. |
| F4 | Affected paths add the adopter day-to-day page, shipped loader route, rules index, changelog migration entry, foundry loader routes and the manifest. SemVer constraint stated as decision D1. |
| F5 | Six positive and negative semantic examples in the plan and the draft rule; validation stays manual. |

## For the governor

- **D1:** keep `MUST NOT` (BREAKING; ineligible for v2.13.x or any v2.x minor;
  major-release plan only) or weaken to `SHOULD NOT` for adopters (additive).
  The plan recommends keeping `MUST NOT` and states the alternative. It chooses
  no version, milestone or date.
- Both advisory rows are `consulted` and record the earlier findings. Neither is
  cleared. Independent re-review of this exact head is needed.

## Not requested

No Gate 1 approval is assumed. This record grants no implementation, Gate 2,
merge or release authority. No shipped rule, page, loader or changelog was edited.
