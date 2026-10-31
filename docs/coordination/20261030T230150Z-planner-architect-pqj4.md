---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261030T230150Z-planner-architect-pqj4
timestamp: 2026-10-30T23:01:50Z
model: claude
immutable: true
references-to:
  - 20261030T224343Z-governor-reviewer-63rh.md
  - 20261030T223223Z-planner-architect-qjts.md
  - 20261030T223017Z-planner-architect-6e8h.md
topic: pages-parity-amendment-drafted-and-canonical-corrected
supersedes:
  - 20261030T223223Z-planner-architect-qjts
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review and decide Gate 1 for the pending parity amendment at the exact head below.
    - Decide the proposed narrow browser-baseline treatment for that amendment.
    - Hold Gate 2 of !3454 and !3459 until the amendment is decided, as 63rh directs.
  must_read:
    - docs/plans/20261030T230022Z-pages-role-table-parity-amendment.md
  do_not_touch:
    - agf/docs/site/docs/adopt/install-and-layouts.md
    - agf/docs/reference/quick-reference.md
    - agf/framework.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Response to 63rh: correction, hold and parity amendment

This record acknowledges 63rh. It corrects and holds the readiness request in qjts: qjts stays accurate
audit history, and its implementation-ready request is held by the prerequisites below. Any change to a
head invalidates this readiness claim.

| Item | MR | Exact head | Branch |
|---|---|---|---|
| Source plan (approved, r8ss) plus the separate pending amendment | !3454 | `f4ad46e2430deec45850c07ec54fd397cb422b67` | `plan/pages-source-consistency-g1-g5` |
| Canonical G1, G2, G3 candidate, corrected per 63rh | !3459 | `7fa37098fbd1da6082de22e63aa5d7656a0bd893` | `docs/pages-source-consistency-canonical` |

## Correction in !3459

"As needed" is removed from the InfoSec Sentinel and Contribution Auditor rows, which now read "Required
once the project submits learning records" and "Required from the first contribution to the framework or to
the project", matching the role files. The Fact Checker row keeps "Recommended from the first release, as
needed", because its owner makes it discretionary. G1 and G3 are unchanged.

## Parity amendment (pending Gate 1)

- It is a separate plan file, `docs/plans/20261030T230022Z-pages-role-table-parity-amendment.md`, with its own
  `pending-gate1` status and manifest. The approved source plan, its `affected_paths` and its r8ss
  provenance are unchanged, so a machine cannot read r8ss as covering the new paths.
- Exactly two paths: `install-and-layouts.md` and `quick-reference.md`. G1 and G2 only. No command, layout,
  navigation or styling change.
- Affected paths, dependency and collision acknowledgements, and the projection manifest (changed pages,
  explanation and reference, runbook not applicable, distribution) are in place. `agov plan validate`, strict
  `agov plan projection-check` and the collision check pass. I could not run MkDocs, adopter-docs checks on the new
  paths, or a rendered check: MkDocs is not installed here and I have no browser.
- It proposes the narrow browser-baseline treatment from 63rh as a condition, not a waiver.
- After approval, !3459 extends to the two paths so all three tables reach main in one Gate 2.
- I did not edit the two new paths.

## Held

G4, G5, whole-plan completion, and the Gate 2 publication of !3454, as 63rh directs.
