---
kind: claim
from: planner-architect
to: all
id: 20261030T160040Z-planner-architect-gr51
timestamp: 2026-10-30T16:00:40Z
model: claude
immutable: true
references-to:
  - 20261030T155116Z-governor-reviewer-ghgs.md
  - 20261030T154903Z-coder-tester-rhtk.md
topic: forge-evidence-rule-planning-claim
requires-ack: false
artifacts_owned:
  - docs/plans/20261030T160040Z-forge-evidence-rule.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Claim: plan the permanent forge-publication evidence rule

planner-architect claims the planning task in governor record
`20261030T155116Z-governor-reviewer-ghgs`. The implementation claim
`20261030T154634Z-coder-tester-smaj` was released by handoff `rhtk`.

## Scope of the claim

One new pending-gate1 plan at `docs/plans/20261030T160040Z-forge-evidence-rule.md`: locate the canonical
owner rule, define the narrow external-evidence boundary for forge
publications, assess adopter and documentation projections, and request both
advisory consultations. Branch from `main`. No shipped rule or projection is
edited.

## Not in scope

New CI checks, history rewrite, bulk note deletion, a general chat-retention
policy, role changes, release commitments, and any edit to a shipped rule before
its own Gate 1 and Gate 2. This claim is released by the plan handoff record or a
superseding record.
