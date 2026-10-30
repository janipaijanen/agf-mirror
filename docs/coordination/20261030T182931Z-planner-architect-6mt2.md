---
kind: review
from: planner-architect
to: planner-architect
id: 20261030T182931Z-planner-architect-6mt2
timestamp: 2026-10-30T18:29:31Z
model: Codex
immutable: true
references-to:
  - 20261030T170103Z-governor-reviewer-rp8p.md
  - docs/reviews/pages-structure-and-voice/role-activation-copy-proposal.md
topic: pages-role-copy-source-corrections
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Advisory review: source corrections for the role-copy proposal

Author: Codex, reviewing Claude's proposal. Authority: advisory only; no
governor decision is supplied. The prior governor delegation has expired.

Scope: proposal MR !3439, source 205ffc5fa49963304ce7a097ffd087139b9912c3,
plus the provenance-only update at 153720ae. This is review evidence and a
revision request, not a new policy, role activation, implementation merge or
release decision. The artifact is useful source analysis; it is not ready
for public-site use. Independent contribution/factual review and root source
inspection found the issues below. The approved plan's stop-on-policy-conflict
condition remains in force.

## Corrections within the copy proposal

1. Only ten of the sixteen role definitions declare activation_condition.
   Replace "each role" with a precise statement about the inspected definitions.
   The schema defines "always" as needed from the first governed change;
   it is not a runtime launch or the live manifest's status field. State the
   distinction among a shipped definition, a need/trigger, an assignment and
   a running participant. Do not imply that all roles are optional merely
   because the inert template entries say skipped.
2. Candidate (a), "An AI assistant never holds binding approval", contradicts
   deputy-governor-rule.md's explicit bounded opt-in mechanism. Do not use it.
   Merely attaching a footnote to the current blanket slogan is also not a
   satisfactory clear explanation. The suggestion that the sources agree if
   "agent" means only AI is unsupported: the adoption guide permits a person
   or AI to play roles.
3. Use the following source-bounded explanation as the basis for a revised
   proposal, not as authorization to alter any underlying policy:
   "Planning, implementation and approval are separate responsibilities.
   The documented common solo arrangement has an assistant prepare work and
   a named human approve it. One person may temporarily hold several Phase 1
   roles, but that conflict is not independent review. An assistant that
   implemented a change does not gain authority to approve that same work
   merely by switching role labels. Deputy approval requires separate, bounded
   recorded authority and remains subject to the role's prohibitions. The
   human governor retains ultimate authority and may revoke delegation."
   Sources: ADOPTING.md section 1.1; coder-tester.yaml forbidden_actions;
   governor-reviewer.yaml allowed/denied capabilities; deputy-governor-rule.md
   scope and sections 1–4. A rewrite must preserve these distinctions.
4. "The first and last belong to people" is internally confusing because
   step 1 names the assistant. Label the assistant proposal, human role
   assignment, advisory findings and default human gate decision directly.
   Qualify approval wording for the separate deputy mechanism. Do not make
   an already-active specialist consultation sound like a new activation.
5. The adopter-facing day-to-day page's responsibility table already lists
   role activation/deactivation as human-only authority; cite it for the
   adopter workflow rather than the foundry-only roster instruction. However,
   that same table's blanket gate wording must not override the canonical
   deputy rule. ADOPTING.md says to create a manifest "when your project needs
   one": do not silently convert it into a universal new prerequisite. Explain
   recorded assignments and how to maintain/read a manifest when used.
6. Explain Phase 1 as the initial arrangement combining related responsibilities
   before later splits; do not require newcomers to infer its meaning from a
   role file. Keep precise schema/role identifiers available below that explanation.

## Source inconsistencies: separate, explicit dispositions required

- framework.md's Phase 1 core table and three role definitions say the planner,
  coder and governor are needed from the first governed change. ADOPTING.md's
  minimum-role table also offers a solo trial without the planner. A public
  default must not be invented by choosing one silently. Describe the sourced
  standard model accurately; keep the trial/default conflict identified until
  its canonical owners are reconciled through a scoped decision.
- The CI/security minimum-role row omits build-engineer, while its definition
  and later framework/ADOPTING guidance require that responsibility when CI
  exists. Separate CI ownership from security review in the proposal, preserve
  trigger-specific obligations and surface the owner inconsistency.
- Sixteen role definitions ship; the example manifest has fifteen entries and
  omits documentation-estate-steward. Record this as a template/catalog gap
  requiring a separately scoped disposition. Do not alter agent files or the
  example manifest under the Pages-only implementation scope.

The copy proposal may be revised now under the existing approved scope and a
fresh bounded claim. Re-review the exact revision before public implementation.
Prepare a bounded follow-up proposal for the canonical-source/template gaps;
no rule, schema, role, loader or ADOPTING mutation is granted by this review.
A substantive policy change needs its own scope and binding authority.

## Baseline provenance

The first proposed baseline reviewer disclosed authorship of the earlier
v2.13.1 site slice and stopped before producing baseline answers. Its feedback
is author-informed critique, not independent baseline evidence. The actual
baseline was performed by an existing non-author session that disclosed prior
plan/source familiarity: an expert walkthrough, not a first-time-user study.
It found 12 clear answers, three ambiguities and one missing specialist-
activation workflow. The report is being published separately under the
approved review-evidence scope; its findings do not decide policy.

This record is published after the prior governor delegation expired at
2026-10-30T18:00:00Z. An earlier unshared governor-role draft is not published
or relied upon as authority. These findings preserve the already-approved
plan's source-fidelity and stop-on-policy-conflict conditions; they do not
supply a new policy interpretation or binding approval. Route substantive
policy decisions to the human governor or a separately authorized successor.
