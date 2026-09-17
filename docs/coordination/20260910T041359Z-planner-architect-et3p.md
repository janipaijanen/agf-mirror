---
kind: handoff
from: planner-architect
to: capability-developer
id: 20260910T041359Z-planner-architect-et3p
timestamp: 2026-09-10T04:13:59Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260909T124606Z-governor-reviewer-sh3m.md
  - 20260909T194219Z-governor-reviewer-j8bq.md
topic: documentation-estate-steward-capability-and-v211-inclusion
requires-ack: true
artifacts_owned:
  - agf/learnings/foundry.yaml
handoff:
  next_agent: capability-developer
  status: ready
  task:
    - "Assess the documentation-estate consistency/currency ownership gap (learning 20260910T041223Z-qvzj) and decide the artifact shape: a rule / Gate-1-and-Gate-2 checklist (single-adopter default) or a standing MANIFEST agent (needs 2+ adopter corroboration per the framework calibration)."
    - "Return a bounded proposal: the capability's mandate, its touchpoint manifest reconciled with docs/reviews/rule-projection/, its event-driven cadence classes, its boundaries against web-ux-reviewer / marketing-tone-rule / fact-checker, and its routing. Do not create a MANIFEST entry or grant authority; that is a governor decision."
  must_read:
    - agf/learnings/foundry.yaml
    - agf/rules/workflow/documentation-rule-projection-rule.md
    - agf/rules/docs/marketing-tone-rule.md
    - agf/rules/docs/documentation-audience-rule.md
    - docs/coordination/20260909T041405Z-planner-architect-asjp.md
  do_not_touch:
    - agf/agents/MANIFEST.yaml (no role activation before a governor decision)
    - v2.11 scope plan (the planner amendment is separate)
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# New capability: documentation-estate steward — assess shape, then v2.11 inclusion

## Governor directive (2026-09-10, restated / improved as invited)

> 1. **Draft the new governed capability** that owns adopter-facing
>    **documentation-estate consistency and currency** (working title:
>    *documentation-estate steward*; the assistant proposal called it
>    "Product Marketing Touchpoint Keeper"). Per learning
>    `20260910T041223Z-qvzj` and the framework's rule-vs-agent calibration,
>    **capability-developer first decides** whether it ships as a **rule /
>    Gate-1-and-Gate-2 checklist** (single-adopter evidence — the default)
>    or a **standing MANIFEST agent** (needs 2+ adopter corroboration).
>    Claude drafts; Codex and other roles review independently; the
>    governor decides.
> 2. **Once that capability is Gate 1 approved and merged**, amend the
>    v2.11 (RUW) scope plan to add it as an **IN workstream** — a hard
>    release-cut precondition, on equal footing with the other IN rows.
> 3. Make an **estate-wide review and assessment of the existing
>    documentation, performed by the new capability, a PRECONDITION of
>    finalizing v2.11 release content**. Before the release-publication
>    plan's metadata allowlist and CHANGELOG scope are frozen, the steward
>    produces a documentation-estate assessment (drift, stale claims,
>    feature-status inconsistency, cross-touchpoint contradiction, missing
>    currency) that the v2.11 release content must resolve or explicitly
>    defer with a recorded disposition.
> 4. **Sequencing:** the capability definition is done collaboratively now;
>    the v2.11 scope amendment and the release-content work then proceed
>    under the existing Codex/Claude until-release collaboration mandate
>    (`33dn` / `j8bq`), each role working independently within its lane.

## This handoff

Routes step 1 to capability-developer. Learning `20260910T041223Z-qvzj`
(filed in `agf/learnings/foundry.yaml`) is the evidence base. My planner
recommendation, non-binding: **rule / checklist first** — foundry is
pre-customer and most of the machinery already exists across
`documentation-rule-projection-rule`, `marketing-tone-rule`
(message-integrity + canonical descriptors), the release plan's derived
allowlist, and `improvement-propeller-rule`'s cadence/dedup lanes; the
genuinely new pieces are a **single owner**, one **unified touchpoint
manifest**, and an **estate sweep** as a companion to the per-change
projection screen.

Planner follow-ups after capability-developer returns and the governor
decides:
- A Gate 1 plan for the capability (rule text or agent definition per the
  decision), with its own advisory consultation.
- A v2.11 parent-scope amendment adding the IN row (steps 2–3), the same
  shape as the `#129` withdrawal and `j8bq` amendments — that amendment is
  a separate governor Gate 1 decision.
- Folding the "steward estate assessment before release content is frozen"
  precondition into the v2.11 release-publication child plan (MR !2646).
