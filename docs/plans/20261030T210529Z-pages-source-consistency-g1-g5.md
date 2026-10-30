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
    reason: Re-review requested after the revision answering decision record pg2s. G5 touches approval and delegation wording.
  - role: contribution-auditor
    status: pending
    reason: Re-review requested after the revision. Semantic preservation and impact classification of G1, G2 and G5.
  - role: web-ux-reviewer
    status: pending
    reason: Re-review requested. The three Pages files are public reader copy. The earlier clearance was within advisory scope.
  - role: fact-checker
    status: pending
    reason: Optional. Review of the source citations and the role counts.
  - role: documentation-estate-steward
    status: pending
    reason: Optional. Review of estate-wide repetition of the same statements.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: make role and gate wording agree across the canonical sources (G1 to G5)

Governance mode: STRICT. The plan touches `agf/agents/*.yaml` (a downstream-impact trigger
path and a high-risk area under `high-risk-operations-rule.md`) and approval wording (a public
AGF claim). Status is `pending-gate1`. Nothing here is approved. No implementation starts
before binding Gate 1.

Claim: `20261030T210529Z-planner-architect-p214`. Handoff: `20261030T205858Z-governor-reviewer-3ghb`.
Direction: `20261030T204639Z-planner-architect-qt8p` (Part 3). Revision answers the combined
findings in `20261030T212253Z-governor-reviewer-pg2s`. Evidence:
`docs/reviews/pages-structure-and-voice/role-source-gaps-proposal.md` (rev 3) and
`role-activation-copy-proposal.md`.

## Problem

Five statements about roles and approval disagree between sources that a reader meets in the
first hour. The approved Pages plan excludes canonical-owner and template edits, so the
disagreement cannot be fixed inside that plan. Later Pages slices must not state a rule that a
canonical source contradicts.

## Rule for every item

Compare each old and new sentence with its owner. Record what it requires, forbids and permits.
If the new sentence changes any of the three, the item is **not editorial**. Classify it under
`downstream-impact-rule.md`, write the migration note that classification needs, and return the
choice to the governor. Do not label a change editorial to move it faster.

## Items and semantic comparison

Facts below were read on main `68f0dc57` (site and `agf/` unchanged since `ea23dae2`).

### G1: solo trial and the three core roles

| | Text | Requires / forbids / permits |
|---|---|---|
| Owner | `agf/framework.md` line 1448: "Core agents, always active from the first governed change" (planner-architect, coder-tester, governor-reviewer). | Requires planning and Gate 1 authorship from the first governed change. |
| Old | `agf/ADOPTING.md` "Pick the minimum role set": solo trial row = `coder-tester`, `governor-reviewer`. | A reader can conclude that no planner is needed in the solo case, so planning and Gate 1 authorship may be skipped. |
| New | Solo trial row = `planner-architect`, `coder-tester`, `governor-reviewer`, with a sentence: one assistant may perform both the planner and coder responsibilities; planning and Gate 1 plan authorship still happen; the human governor decides Gate 1. | Matches the owner. Does not require three people or sessions. |

Result: the new row differs from the old row in what a reader is told to activate. It is **not
editorial**. It is the governor's intended reconciliation with the owner, and it is not evidence
of zero adopter impact.

Impact (`downstream-impact-rule.md`), proposed classification **BREAKING (conservative)**; the
governor may reclassify:

```
Impact:
  Foundry:          no. Foundry already runs all three core roles.
  Direct adopters:  BREAKING for a solo adopter who followed the old row and left planner-architect
                    unassigned. They must assign it. The same assistant may play it.
  Downstream:       Plans and Gate 1 records appear for solo adopters who skipped them. No change
                    to artifact shape.
  Migration:        Solo adopters: in the manifest, list planner-architect with the same player
                    as coder-tester. Write a plan and a Gate 1 decision for the next non-trivial
                    change. No new person or session is needed.
  Evidence:         framework.md line 1448; baseline question 10 (AMBIGUOUS) in
                    docs/reviews/pages-structure-and-voice/20261030-baseline.md.
```

### G2: CI ownership, security and review roles

| | Text | Requires / forbids / permits |
|---|---|---|
| Old | `ADOPTING.md` row "CI/security-conscious project": standard roles plus active `contribution-auditor`, `infosec-sentinel`, and `fact-checker` "as needed". | Permits activating those three as needed. `build-engineer` is not named. The row title calls the group CI/security. |
| Triggers in the role files | `build-engineer`: `activation_condition` "project has a CI pipeline". `infosec-sentinel`: "project submits learning records" (screens learning records for credential and cross-record leaks). `contribution-auditor`: "first contribution to the framework or an adopter project". `fact-checker`: no `activation_condition`; the header says "recommended from first release". | Each role has its own trigger. |
| New | Separate entries, each with its own trigger and "as needed" kept: CI ownership by `build-engineer` when the project has a CI pipeline; learning-record security by `infosec-sentinel`; contribution integrity by `contribution-auditor`; fact checking by `fact-checker`. The words "security review" apply only to `infosec-sentinel`. | Permits the same activations as before and adds a named `build-engineer` entry. Requires nothing new. |

Result: the new text names a role the old row omitted and removes the label that grouped
contribution and fact checks as security. Proposed classification **ADDITIVE** (a named optional
entry, no obligation). The comparison above is the evidence; the governor may reclassify.

### G3: example manifest

16 role files ship in `agf/agents/`. `MANIFEST.example.yaml` has 15 entries and no
`documentation-estate-steward`. Add the missing entry as `status: skipped`, like the other example
entries. It activates nothing for an adopter. Classification **ADDITIVE**. This touches the
`status` field of an `agf/agents/*.yaml` file, which `high-risk-operations-rule.md` names as a
stop-and-plan area. Gate 1 on this plan is that plan.

### G4: `activation_condition` coverage

10 of 16 definitions declare `activation_condition`. `agf/schemas/agent.schema.json` does not
require it. Documentation only, in `customizing-the-framework.md`: state that the field is optional
and which role definitions declare it. No schema change, no requirement, no statement of intent for
the six omissions. Classification **EDITORIAL** if the sentence adds no obligation.

### G5: approval and delegation wording

| | Text | Requires / forbids / permits |
|---|---|---|
| Owner | `agf/rules/workflow/deputy-governor-rule.md`. Delegation is off by default and opt-in. A written Delegation Artifact needs Delegator, Deputy, Scope, Severity ceiling, Expiry, Audit trigger and Revocation conditions. Every deputized decision is logged. The red list is never delegable: irreversible or destructive operations, security-boundary changes, architecture or data-model changes, critical-severity items, delegation scope itself, rule and agent definitions, and compliance, licensing or legal artifacts. The rule is marked provisional (single-project evidence). | Permits bounded, time-limited delegation by the human governor. |
| Role definition | `agf/agents/governor-reviewer.yaml` line 180 lists "Self-approve its own decisions" among prohibitions. | Forbids self-approval by role. |
| Old (a) | `roles-and-gates.md` lines 72 to 75: "The human governor still owns Gate 1, Gate 2, release approval, and binding risk acceptance." | Silent on the opt-in deputy rule. |
| Old (b) | `roles-and-gates.md` lines 196 to 199: these approvals "remain non-delegable unless a future AGF version explicitly changes the role model." | Reads as no delegation at all. The deputy rule already permits bounded Gate 2 delegation. |
| New | One short, explicitly non-exhaustive pointer at each place, saying: approvals belong to the human governor; the optional deputy rule is the only route for delegating any of that authority and is off by default; it needs a written artifact with scope, severity ceiling, expiry, audit trigger, revocation and per-decision logging; it never covers its red list; role definitions still forbid self-approval; the rule is provisional and its logging and audit duties apply; the summary does not replace the rule. | Matches the owner. Adds no approval, activation, manifest-edit or release authority. |

Candidate sentence for (a) and (b), for the reviewers to correct:

> Approvals stay with the human governor. A project may opt in to the narrow, time-limited
> delegation described in the optional deputy-governor rule (`deputy-governor-rule.md`). That
> rule is off by default, requires a written delegation with scope, severity ceiling, expiry,
> audit trigger and revocation, logs every deputized decision, and never covers its red list.
> Role definitions still forbid self-approval. This is a summary, not the rule; the rule governs.

Result: (b) changes what the page tells a reader is permitted. **Not editorial.** Proposed
classification **BREAKING for readers of (b)** if the governor treats the old text as a
commitment, otherwise PREVENTIVE. Migration: none for existing adopters, because the deputy rule
text is unchanged; the public page now matches it. The governor decides. Foundry's direct full
human grant is not an adopter default, and the sentence must not imply it.

If a reviewer finds that any G5 sentence widens or narrows who may approve, that item stops here
and returns to the governor.

## Exact paths

`agf/ADOPTING.md`, `agf/agents/MANIFEST.example.yaml`, `agf/docs/site/docs/concepts/roles-and-gates.md`,
`agf/docs/site/docs/operate/day-to-day.md`, `agf/docs/site/docs/advanced/customizing-the-framework.md`,
and the projection manifest. Read-only comparison sources, unchanged: `agf/framework.md`,
`agf/rules/workflow/deputy-governor-rule.md`, `agf/schemas/agent.schema.json`, and the role
definitions `planner-architect`, `coder-tester`, `governor-reviewer`, `build-engineer`,
`infosec-sentinel`, `contribution-auditor`, `fact-checker`, `documentation-estate-steward`.
Any other page that repeats these statements enters by plan amendment.

## Projection screen

Manifest: `docs/reviews/rule-projection/20261030T210529Z-pages-source-consistency-g1-g5.yaml`.
It lists only the actually edited owners as updated and records every read-only source as
unchanged with its reason. The deputy rule is not marked updated.

Commands (the manifest has no `stage: planned` owner, so no relaxation flag applies):

```
agov plan projection-check --base-ref origin/main docs/reviews/rule-projection/20261030T210529Z-pages-source-consistency-g1-g5.yaml
```

Use `--allow-planned` only for a manifest whose owner is `stage: planned` and absent. Run the
same command before Gate 1 and again before Gate 2. The check covers structure and distribution.
Semantic equality stays a reviewer judgment.

## Overlap and order

- Overlap: the approved Pages plan owns `agf/docs/site/**`. This plan edits three files under that
  tree, so `depends_on` names that plan, `20261030T054200Z-pages-reader-journey-patch`,
  `20261027T052118Z-gitlab-pages-homepage-modernization` and the three release plans.
  `agov plan collision-check --plans-only --fail-on-unacknowledged-collision` must show only
  "informational (acknowledged via depends_on)" rows for this plan. The tool reads committed
  content, so run it after commit.
- Order: this plan lands first. Pages slices that restate roles, approval or activation start after
  it merges, or wait on the explicit decision for any item that stops.
- Forge-evidence rule work (!3426) covers a different subject.

## Validation

`agov plan validate`, the projection command above, `agov coord validate` on delivery records, a
MkDocs build, and a link and anchor check on the three site pages. A reviewer records "meaning
unchanged" or "stopped" per sentence using the tables above. The tools cannot decide semantic
equality.

## Gate self-test

The plan adds no hook, validator, CI check or blocking rule, so `gate-self-test-rule.md` does not
apply. Any new validator behavior becomes a separate gated plan.

## Risks and rollback

- Risk: a G5 sentence quietly widens or narrows who may approve. Control: the comparison table,
  `infosec-sentinel` re-review, and the stop-and-return rule.
- Risk: G1 reads as requiring three people. Control: the explicit "one assistant may perform both"
  sentence.
- Risk: G3 is read as activating the role. Control: `status: skipped` and a sentence in the MR.
- Rollback: revert the merge. All edits are text and no tag or release is involved.

## Decisions to return to the governor

1. Is "BREAKING (conservative)" the right class for G1, and is the migration note enough?
2. Does the G3 `skipped` entry fall under the high-risk manifest-edit stop, and is Gate 1 on this
   plan the stop-and-plan step?
3. Is G2 ADDITIVE, and is G5 (b) BREAKING or PREVENTIVE?
4. Does G5 stay inside the deputy rule after the reviewers read the candidate sentence?

## Not in scope

Role definitions, rules, schemas, loaders, `agf/framework.md`, tooling, any release version or date,
and the public-use holds. The holds lift only by later gate decisions.
