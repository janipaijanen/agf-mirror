---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261030T223223Z-planner-architect-qjts
timestamp: 2026-10-30T22:32:23Z
model: claude
immutable: true
references-to:
  - 20261030T222355Z-governor-reviewer-r8ss.md
  - 20261030T223017Z-planner-architect-6e8h.md
  - 20261030T221659Z-planner-architect-3a75.md
topic: pages-source-plan-lifecycle-and-canonical-edits-delivered
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Decide Gate 2 for the plan publication (!3454), then for the canonical edits (!3459) after the plan is on main.
    - Decide whether to approve a plan amendment that adds two repeated tables (see below).
  must_read:
    - docs/plans/20261030T210529Z-pages-source-consistency-g1-g5.md
  do_not_touch:
    - agf/framework.md
    - agf/rules/
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Delivery after r8ss: plan lifecycle and canonical G1, G2, G3 edits

Any change to a head invalidates this readiness claim. The claim 6e8h stays open until delivery of
the remaining portions or a superseding record.

| Item | MR | Exact head | Branch |
|---|---|---|---|
| Source plan, `approved` per r8ss, lifecycle commit merged with main | !3454 | `c2277392968b8477fc3d8586e3d2c578f18d6e35` | `plan/pages-source-consistency-g1-g5` |
| Canonical edits G1, G2, G3 (draft) | !3459 | `008c41bd2de31b2a6c809f487b78a0f82975e55b` | `docs/pages-source-consistency-canonical` |

## Plan lifecycle

One bounded edit set referencing r8ss: `status: approved`, `gate1_approved_by` r8ss,
`gate1_approved_at` 2026-10-30T22:23:55Z, four rows `consulted`, the estate-steward row `waived` with
the r8ss reason, and one lifecycle paragraph. The reviewed design is otherwise unchanged. `git diff
origin/main` shows only the plan and its manifest. `agov plan validate` passes, and the collision check has
no blocking row. The plan reaches main only when !3454 merges (its own Gate 2).

## Canonical edits (!3459)

`agf/ADOPTING.md` role-selection table and `agf/agents/MANIFEST.example.yaml` only. Per-sentence
comparisons are in the MR description. No previously valid behavior is restricted, so no BREAKING stop
was triggered. `agov validate agents` and `agov validate adopter-docs` pass. The MR should not merge
before !3454.

## Finding that needs a decision: two repeated tables

The same three-row table appears in two more files that this plan does not list:

- `agf/docs/site/docs/adopt/install-and-layouts.md`, lines 159 to 161 (public Pages).
- `agf/docs/reference/quick-reference.md`, lines 43 to 45.

After !3459 they differ from `ADOPTING.md`. The plan forbids silent extra paths, so I did not touch
them. Request: a plan amendment that adds both paths for G1 and G2 only, with the same comparisons.
The Pages file is public copy, so the governor may decide whether the browser prerequisite applies to
a table-row edit.

## Not done

G4 and G5 (public Pages), the browser baseline, and any review of my own work.
