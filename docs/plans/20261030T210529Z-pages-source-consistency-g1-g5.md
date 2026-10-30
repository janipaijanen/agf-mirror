---
id: 20261030T210529Z-pages-source-consistency-g1-g5
status: approved
gate1_approved_by: 20261030T222355Z-governor-reviewer-r8ss
gate1_approved_at: "2026-10-30T22:23:55Z"
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
    status: consulted
    reason: Independent InfoSec review cleared exact head e6362888, including the G5 authority wording. Recorded in governor decision r8ss.
  - role: contribution-auditor
    status: consulted
    reason: Independent contribution review cleared exact head e6362888 and the unchanged-owner interpretations. Recorded in r8ss.
  - role: web-ux-reviewer
    status: consulted
    reason: Web UX review cleared exact head e6362888 within advisory scope. The reviewer helped author the current site but not this draft; no viewport acceptance was supplied. Recorded in r8ss.
  - role: fact-checker
    status: consulted
    reason: Separately identified factual review cleared exact head e6362888. Recorded in r8ss.
  - role: documentation-estate-steward
    status: waived
    reason: Optional consultation waived by r8ss for this bounded source plan. Exact owner inventories, three independent reviews and the extra-path stop cover its Gate 1 scope. The parent plan's final estate sweep and broader consistency review remain mandatory. No estate sweep has occurred.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: make role and gate wording agree across the canonical sources (G1 to G5)

Governance mode: STRICT. The plan touches `agf/agents/*.yaml` (a downstream-impact trigger
path and a high-risk area under `high-risk-operations-rule.md`) and approval wording (a public
AGF claim).

Lifecycle: Gate 1 approved by governor decision `20261030T222355Z-governor-reviewer-r8ss` at
reviewed exact head `e636288806e8b536f77aa5123c9668505ed451f5` (MR !3454). The only edits after that
head are this paragraph, the approval metadata and the consultation rows, as r8ss authorizes. The
reviewed design is otherwise unchanged. This is Gate 1 only. It is not Gate 2, a version choice, a
release, a tag or a manual deployment approval. The original Pages implementation gates and the
browser prerequisites remain in force. r8ss approves the one missing example-roster entry as
`skipped` only, which satisfies the high-risk stop-and-plan requirement for that one edit. It changes no
live manifest and activates no role.

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
If the new sentence changes any of the three, the item is **not editorial**. G1, G2 and G5 are
classified PREVENTIVE under governor decision `20261030T215549Z-governor-reviewer-gpqq`, only
because the existing canonical owners already hold the constraint and the comparison below keeps
previously valid behavior. If a sentence-level comparison shows a real contract restriction, stop
that item and return it as BREAKING with ADR, migration note and major-release sequencing. The
classification cannot be used to label an actual breaking change non-breaking.

## Items and semantic comparison

Facts below were read on main `68f0dc57` (site and `agf/` unchanged since `ea23dae2`).

### G1: solo trial and the three core roles

| | Text | Requires / forbids / permits |
|---|---|---|
| Owner | `agf/framework.md` line 1448: "Core agents, always active from the first governed change": planner-architect, coder-tester, governor-reviewer. `agf/agents/planner-architect.yaml` `local_deactivation`: not meaningfully deactivatable while using the gate model, because Gate 1 authorship has no other owner in the minimum system. | Requires planning and Gate 1 authorship from the first governed change. No exception for a solo trial. |
| Old | `agf/ADOPTING.md` "Pick the minimum role set": solo trial row = `coder-tester`, `governor-reviewer`. | A reader can conclude that governed work may omit planning. The row is not an explicit exception, so it contradicts its owner. |
| New | Solo trial row = `planner-architect`, `coder-tester`, `governor-reviewer`, plus a sentence: one assistant may perform both the planner and coder responsibilities; planning and Gate 1 plan authorship still happen; the human governor decides Gate 1. | Matches the owner. Requires no new person or session. |

Classification: **PREVENTIVE**, on this unchanged-owner basis only: the owner already required the
three core roles, so no previously compliant adopter loses permitted behavior. The old row was the
contradictory projection. Per `downstream-impact-rule.md`, PREVENTIVE is treated as ADDITIVE unless
it restricts previously permitted behavior.

Stop condition: if the sentence-level comparison shows a real contract restriction (for example an
existing adopter practice the owner allowed and the new row forbids), stop this item and return it as
BREAKING, with ADR, migration note and major-release sequencing. This plan's classification cannot be
used to label an actual breaking change non-breaking.

```
Impact:
  Foundry:          no. Foundry already runs all three core roles.
  Direct adopters:  PREVENTIVE. The old row misled some solo readers. No compliant adopter loses
                    permitted behavior. No completed adopter work is invalidated retroactively.
  Downstream:       none. No change to artifact shape.
  Migration:        not required. Remediation for a reader misled by the old row: when the project
                    uses a manifest, name planner-architect there, with the same player as
                    coder-tester if one assistant does both. A manifest is not required universally.
                    Write the plan and Gate 1 record for the next non-trivial change. No new person
                    or session is needed.
  Evidence:         framework.md line 1448; planner-architect.yaml local_deactivation; baseline
                    question 10 (AMBIGUOUS) in docs/reviews/pages-structure-and-voice/20261030-baseline.md.
```

### G2: CI ownership, security and review roles

| | Text | Requires / forbids / permits |
|---|---|---|
| Owner | `agf/framework.md` line 1486: "Activate Build Engineer when the project has a CI pipeline." `agf/agents/build-engineer.yaml` `activation_condition`: "project has a CI pipeline". | Requires CI ownership by `build-engineer` when the project has CI. Not optional in that case. |
| Other triggers in the role files | `infosec-sentinel`: "project submits learning records" (screens learning records for credential and cross-record leaks). `contribution-auditor`: "first contribution to the framework or an adopter project". `fact-checker`: no `activation_condition`; the header says "recommended from first release". | Each role has its own trigger. |
| Old | `ADOPTING.md` row "CI/security-conscious project": standard roles plus active `contribution-auditor`, `infosec-sentinel`, and `fact-checker` "as needed". `build-engineer` is not named. The row groups the three as CI/security. | A reader can conclude that CI ownership is optional or covered by a security role. |
| New | Separate entries, each with its own trigger and the existing "as needed" kept where the owner says so: CI ownership by `build-engineer` when the project has a CI pipeline; learning-record security by `infosec-sentinel`; contribution integrity by `contribution-auditor`; fact checking by `fact-checker`. The words "security review" apply only to `infosec-sentinel`. | Makes the existing conditional obligation for CI explicit. Adds no new obligation. |

Classification: **PREVENTIVE**, a clarification of the existing conditional CI obligation. The earlier
"optional, requires nothing" wording is withdrawn. The same stop condition as G1 applies.

```
Impact:
  Foundry:          no. Foundry has CI and an active build-engineer.
  Direct adopters:  PREVENTIVE. An adopter with a CI pipeline and no assigned CI owner was already out
                    of line with the owner. No compliant adopter loses permitted behavior.
  Downstream:       none.
  Migration:        not required. Assignment check for CI adopters: if the project has a CI pipeline,
                    confirm a named player for build-engineer (a manifest entry when a manifest is
                    used). Learning security, contribution integrity and fact checking keep their
                    own triggers.
  Evidence:         framework.md lines 1486; build-engineer.yaml activation_condition; the three
                    other role files cited above.
```

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
| Owner | `agf/rules/workflow/deputy-governor-rule.md`. Scope: approval authority at Gate 2, and elsewhere where the Governor-Reviewer signs off, may be delegated to a Deputy Governor. Delegation is off by default and opt-in. A written Delegation Artifact needs Delegator, Deputy, Scope, Severity ceiling, Expiry, Audit trigger and Revocation conditions. Every deputized decision is logged. The red list is never delegable: irreversible or destructive operations, security-boundary changes, architecture or data-model changes, critical-severity items, delegation scope itself, rule and agent definitions, and compliance, licensing or legal artifacts. The rule is marked provisional (single-project evidence). | Permits a bounded, time-limited Gate 2 route. Other categories need express canonical coverage and a valid matching artifact. |
| Role definition | `agf/agents/governor-reviewer.yaml` line 180 lists "Self-approve its own decisions" among prohibitions. | Forbids self-approval by role. |
| Old (a) | `roles-and-gates.md` lines 72 to 75: "The human governor still owns Gate 1, Gate 2, release approval, and binding risk acceptance." | Silent on the opt-in deputy route. |
| Old (b) | `roles-and-gates.md` lines 196 to 199: these approvals "remain non-delegable unless a future AGF version explicitly changes the role model." | Reads as no delegation at all, while the owner permits the narrow Gate 2 route. |
| New | One short, explicitly non-exhaustive pointer at each place. It names the narrow Gate 2 route of the deputy rule. It does not say the rule delegates "any" Gate 1, release or binding-risk authority. Anything the canonical rule does not expressly cover, with a valid matching artifact, stays with the human governor. | Matches the owner. Adds no new delegable category, approval, activation, manifest-edit or release authority. |

Candidate sentence for (a) and (b), for the reviewers to correct:

> Approvals stay with the human governor. The optional deputy-governor rule
> (`deputy-governor-rule.md`) describes one narrow route: a named deputy may approve certain Gate 2
> items for a limited time under a written delegation. The rule is off by default, is provisional,
> requires a scope, severity ceiling, expiry, audit trigger and revocation, logs every deputized
> decision, and never covers its red list. Anything it does not expressly cover, including Gate 1,
> release approval and binding risk acceptance, stays with the human governor unless the rule and a
> valid matching delegation say otherwise. Role definitions still forbid self-approval. This is a
> summary, not the rule; the rule governs.

Classification: **PREVENTIVE**, only while the sentence preserves the current owner's full limits,
provisional status and role prohibitions. No new delegable category is approved. The same stop
condition applies: if any sentence widens or narrows who may approve, stop that item and return it
as BREAKING.

```
Impact:
  Foundry:          no. Foundry's direct full human grant is not an adopter default and the sentence
                    does not imply it.
  Direct adopters:  PREVENTIVE. The public pages now match the canonical rule. No adopter loses a
                    route the owner already permitted, and none gains one.
  Downstream:       none.
  Migration:        not required. The deputy rule text is unchanged.
  Evidence:         deputy-governor-rule.md; governor-reviewer.yaml line 180; roles-and-gates.md
                    lines 72 to 75 and 196 to 199.
```

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

1. Does the G3 `skipped` entry fall under the high-risk manifest-edit stop, and is Gate 1 on this
   plan the stop-and-plan step?
2. After the reviewers read the candidate sentence, does G5 stay PREVENTIVE? If any sentence widens or
   narrows who may approve, which reading does the governor choose?
3. Are the remediation notes for G1 and G2 enough, or does the governor want them in the adopter
   documents as well?

Decision gpqq already fixed the classification basis for G1, G2 and G5 (PREVENTIVE with a stop clause).

## Not in scope

Role definitions, rules, schemas, loaders, `agf/framework.md`, tooling, any release version or date,
and the public-use holds. The holds lift only by later gate decisions.
