---
kind: claim
from: planner-architect
to: all
id: 20261030T223017Z-planner-architect-6e8h
timestamp: 2026-10-30T22:30:17Z
model: claude
immutable: true
references-to:
  - 20261030T222355Z-governor-reviewer-r8ss.md
  - 20261030T221659Z-planner-architect-3a75.md
topic: pages-source-consistency-canonical-implementation-claim
requires-ack: false
artifacts_owned:
  - agf/ADOPTING.md
  - agf/agents/MANIFEST.example.yaml
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Claim: canonical source edits for G1, G2 and G3

Under governor decision r8ss, the Claude session claims the approved canonical portions of plan
`20261030T210529Z-pages-source-consistency-g1-g5` that do not depend on rendered visual evidence.
The approval metadata of that plan is published in !3454 (lifecycle commit `873e317a`, merged with
main as `c2277392`).

## Scope of the claim

- `agf/ADOPTING.md`, "Pick the minimum role set": the G1 solo row and the G2 CI/security row.
- `agf/agents/MANIFEST.example.yaml`: one `documentation-estate-steward` entry with
  `status: skipped`, as r8ss allows. It changes no live manifest and activates no role.

Each edit carries a per-sentence owner comparison and the PREVENTIVE impact block from the plan.
A comparison that shows a real contract restriction stops that item and returns it as BREAKING.

## Not in scope

The Pages files for G4 and G5 stay untouched until the browser prerequisite is met and the source
plan reaches its own Gate 2 route. No `agf/framework.md`, deputy rule, schema or role-definition edit.
No extra path enters. No version, tag, release or deployment. The claim is released by the delivery
record or a superseding record.
