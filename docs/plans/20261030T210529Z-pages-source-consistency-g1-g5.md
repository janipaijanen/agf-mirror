---
id: 20261030T210529Z-pages-source-consistency-g1-g5
status: pending-gate1
topic: pages-source-consistency-role-and-gate-wording-g1-g5
affected_paths:
  - agf/ADOPTING.md
  - agf/agents/MANIFEST.example.yaml
  - agf/docs/site/docs/concepts/roles-and-gates.md
  - agf/docs/site/docs/operate/day-to-day.md
  - agf/docs/site/docs/advanced/customizing-the-framework.md
  - docs/reviews/rule-projection/20261030T210529Z-pages-source-consistency-g1-g5.yaml
depends_on:
  - 20261030T163253Z-pages-structure-and-voice
  - 20261030T054200Z-pages-reader-journey-patch
  - 20261027T052118Z-gitlab-pages-homepage-modernization
  - 20261027T235807Z-v212-release-publication
  - 20261029T220500Z-v213-release-publication
  - 20261030T064744Z-v2131-release-publication
advisory_consultation:
  - role: infosec-sentinel
    status: pending
    reason: Review requested for G5, because it touches approval and delegation wording.
  - role: contribution-auditor
    status: pending
    reason: Review requested for semantic preservation of all five items.
  - role: fact-checker
    status: pending
    reason: Optional. Review of the source citations and the 16/15/10 role counts.
  - role: documentation-estate-steward
    status: pending
    reason: Optional. Review of estate-wide repetition of the same statements.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: make role and gate wording agree across the canonical sources (G1 to G5)

Governance mode: STRICT. The plan touches `agf/agents/*.yaml` (a downstream-impact
trigger path and a high-risk area under `high-risk-operations-rule.md`) and
approval wording (a public AGF claim). Status is `pending-gate1`. Nothing here is
approved, and no implementation starts before binding Gate 1.

Claim: `20261030T210529Z-planner-architect-p214`. Handoff: `20261030T205858Z-governor-reviewer-3ghb`.
Direction agreed in `20261030T204639Z-planner-architect-qt8p` (Part 3) and recorded in 3ghb.
Evidence: `docs/reviews/pages-structure-and-voice/role-source-gaps-proposal.md` (rev 3)
and `role-activation-copy-proposal.md`, both at the head merged under qbh7.

## Problem

Five statements about roles and approval disagree between sources that a reader
meets in the first hour. The approved Pages plan excludes canonical-owner and
template edits, so the disagreement cannot be fixed inside that plan. Later Pages
slices must not state a rule that a canonical source contradicts.

## Intent and rule for every item

Make the sources agree **without changing what any rule requires, forbids or
permits** (the Parnas test in `downstream-impact-rule.md`). Where a comparison
cannot keep that meaning, stop for that item. Isolate the policy choice and
return it to the governor as an explicit decision. Do not label it editorial.

## Items

| Item | Verified fact (read on `ea23dae2`) | Change in this plan | Not changed |
|---|---|---|---|
| G1 | `agf/ADOPTING.md` lines 113 to 118: the solo trial row lists `coder-tester` and `governor-reviewer`; the standard row adds `planner-architect`. `framework.md` makes planner, coder and governor the three core responsibilities. | Add one sentence to the solo row: who owns planning and the Gate 1 plan in the solo case. Candidate wording: the same assistant may play both, and the human governor still records Gate 1. | The core set. No claim that three people or sessions are needed. |
| G2 | `ADOPTING.md` line 117: the CI/security row lists `contribution-auditor`, `infosec-sentinel`, `fact-checker`. It omits `build-engineer`. | Split the row: CI ownership by `build-engineer` when the project has a CI pipeline; security review by the other roles. | Which roles are active by default. |
| G3 | 16 role files ship in `agf/agents/`. `MANIFEST.example.yaml` has 15 entries and no `documentation-estate-steward`. | Add the missing entry as `status: skipped`, matching the other example entries. | Any role's activation. The entry activates nothing for an adopter. |
| G4 | 10 of 16 definitions declare `activation_condition`. The schema does not require it. | Documentation only, in `customizing-the-framework.md`: state that the field is optional and which roles declare it. | The schema. No requirement. No statement of intent for the six omissions. |
| G5 | `roles-and-gates.md` lines 72 to 75 and 196 to 199 and `day-to-day.md` call approvals human-only. `deputy-governor-rule.md` allows bounded, opt-in delegation with a red list. | Reword the public passages so they match the deputy rule: human ultimate authority, explicit written delegation, expiry, and the whole red list. | The deputy rule. No new delegation, activation, manifest-edit or release authority. Foundry's direct human grant is not an adopter default. |

## Exact paths

`agf/ADOPTING.md`, `agf/agents/MANIFEST.example.yaml`,
`agf/docs/site/docs/concepts/roles-and-gates.md`, `agf/docs/site/docs/operate/day-to-day.md`,
`agf/docs/site/docs/advanced/customizing-the-framework.md`, plus the projection manifest.
Implementation must record any extra page that repeats these statements and bring it in
by plan amendment. It must not widen the scope silently.

## Impact classification (`downstream-impact-rule.md`)

- G1, G2, G4, G5: expected EDITORIAL if the Parnas test holds. G5 is the item most likely to fail it.
- G3: ADDITIVE. A new `skipped` template entry. Adopters who copy the template get one more skipped line.
- `Adopter docs parity: updated` is expected, because the work is adopter-facing docs.
- Impact block: Foundry: no break. Direct adopters: ADDITIVE for G3, otherwise none. Downstream: none. Migration: not required unless a G5 comparison is reclassified BREAKING, in which case the item stops.

## Projection screen

Six-row manifest at `docs/reviews/rule-projection/20261030T210529Z-pages-source-consistency-g1-g5.yaml`.
Owners: `deputy-governor-rule.md` (G5), `agf/agents/MANIFEST.example.yaml` (G3), `agf/ADOPTING.md` (G1, G2).
Gate 1 check: `agov plan projection-check --allow-planned`. Gate 2 check: the same command without the flag.

## Overlap and order

- Overlap: the approved Pages plan owns `agf/docs/site/**`. This plan edits three files under that tree, so it lists that plan, `20261030T054200Z-pages-reader-journey-patch` and `20261027T052118Z-gitlab-pages-homepage-modernization` in `depends_on`. `agov plan collision-check` result is attached before Gate 1.
- Order: this plan lands first. The Pages slices that restate roles, approval or activation start after it merges, or wait on the explicit decision for any item that stops.
- The forge-evidence rule work (!3425 to !3436) touches a different subject. The collision check records the result.

## Validation

`agov plan validate`, strict `agov plan projection-check`, `agov coord validate` on the delivery records, a MkDocs build, and a link and anchor check on the three site pages. A reviewer compares each changed sentence with its owner and records "meaning unchanged" or "stopped". The tools cannot decide semantic equality.

## Gate self-test

The plan adds no hook, validator, CI check or blocking rule, so `gate-self-test-rule.md` does not apply. If the projection manifest needs a new validator behavior, that becomes a separate gated plan.

## Risks and rollback

- Risk: a G5 rewording quietly widens or narrows who may approve. Control: line-by-line comparison with the deputy rule, `infosec-sentinel` consultation, and a stop-and-return rule.
- Risk: G3 is read as activating the role. Control: `status: skipped` plus one sentence in the PR description.
- Rollback: revert the merge. All edits are text, and no tag or release is involved.

## Decisions to return to the governor

1. Is the G1 wording (assistant plays planner, human records Gate 1) acceptable as editorial?
2. Do the G3 edit to `MANIFEST.example.yaml` and the `status` field fall under the high-risk "manifest edits" stop? This plan treats them as covered by Gate 1 for this one added `skipped` line.
3. Does G5 stay editorial after the line-by-line comparison? If not, which reading does the governor choose?

## Not in scope

Role definitions, rules, schemas, loaders, `agf/framework.md`, tooling, any release version or date, and the public-use holds. The holds lift only by later gate decisions.
