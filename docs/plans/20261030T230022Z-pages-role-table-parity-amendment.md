---
id: 20261030T230022Z-pages-role-table-parity-amendment
status: pending-gate1
topic: pages-role-table-parity-amendment-g1-g2
affected_paths:
  - docs/plans/20261030T230022Z-pages-role-table-parity-amendment.md
  - agf/docs/site/docs/adopt/install-and-layouts.md
  - agf/docs/reference/quick-reference.md
  - docs/reviews/rule-projection/20261030T230022Z-pages-role-table-parity-amendment.yaml
depends_on:
  - 20261030T210529Z-pages-source-consistency-g1-g5
  - 20261030T163253Z-pages-structure-and-voice
  - 20261030T054200Z-pages-reader-journey-patch
  - 20261027T052118Z-gitlab-pages-homepage-modernization
  - 20261030T064744Z-v2131-release-publication
  - 20261030T072047Z-v2131-readme-journey-followup
  - 20260806T122121Z-get-started-commissioning-ux-v1
  - 20260808T181423Z-independent-implementation-reviewer
  - 20260823T203540Z-adoption-version-drift-check
  - 20260803T192952Z-pages-readme-visual-refresh-v251
  - 20260823T203534Z-upgrade-overlay-provenance-protection
  - 20260823T203546Z-dist-path-ignore-protection
  - adoption-surface-clarity
  - agents-govern-2.0-directory-structure
advisory_consultation:
  - role: contribution-auditor
    status: pending
    reason: Review requested for semantic preservation of the two mirrored tables.
  - role: infosec-sentinel
    status: pending
    reason: Review requested for the trigger-accurate wording of the learning-record and contribution rows.
  - role: web-ux-reviewer
    status: pending
    reason: Review requested for table readability and the narrow browser-baseline treatment.
  - role: fact-checker
    status: pending
    reason: Optional. Review of the quoted triggers against the role files.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Amendment: mirror the G1 and G2 role-table corrections in two more files (proposed)

Governance mode: STRICT, because it adds paths to an approved plan's scope. Status is
`pending-gate1`. Nothing here is approved.

This is a separate plan file on purpose. The approved source plan
`20261030T210529Z-pages-source-consistency-g1-g5` keeps its own approval, its own `affected_paths`
and its own metadata. Governor decision r8ss covers only that original scope. A machine that reads the
source plan therefore cannot read its approval as covering the two new paths. They become approved only
when a `governor-reviewer` record approves this file at Gate 1 and names it.

Origin: review `20261030T224343Z-governor-reviewer-63rh`. Delivery finding: `20261030T223223Z-planner-architect-qjts`.
Claim: this amendment is drafted under claim `20261030T210529Z-planner-architect-p214`.

## Problem

The same role-selection table as `agf/ADOPTING.md` appears in two more files. After the G1 and G2 edits to
`ADOPTING.md`, the two copies would contradict it. 63rh requires all three tables to reach main
consistently in one Gate 2 with no intermediate contradictory rows.

## Scope: exactly two paths, G1 and G2 only

- `agf/docs/site/docs/adopt/install-and-layouts.md`, the "Minimum commissioning matrix" (lines 157 to 162
  at `33ba5655`). Public Pages.
- `agf/docs/reference/quick-reference.md`, the "Minimum commissioning" table (lines 41 to 46). It ships in the
  AGF distribution and is an applicable projection although it is not a Pages navigation page.

Not in scope: G3, G4 and G5 content in these files; commands, layouts, navigation or styling.

## Candidate edits and semantic comparison

Both tables keep their third column. The comparison below uses the owners from the source plan:
`framework.md` lines 1448 and 1486, and the role files for `build-engineer`, `infosec-sentinel`,
`contribution-auditor` and `fact-checker`.

| Row | Old (install-and-layouts) | New | Requires / forbids / permits |
|---|---|---|---|
| Solo | `coder-tester`, `governor-reviewer` / "Human governor checks plans and merges; use fast track only when criteria fit" | `planner-architect`, `coder-tester`, `governor-reviewer` / same third column, plus: "One assistant may play both `planner-architect` and `coder-tester`; planning and Gate 1 plan authorship still happen" | Matches the core-role owner. No new person or session. |
| Standard | unchanged | unchanged | none |
| CI | part of "CI/security-conscious project" | "Project with a CI pipeline" / Standard roles plus active `build-engineer` / "CI runs validators; security-sensitive changes require explicit evidence" | Names the conditional CI owner. The original third-column sentence stays with this row. |
| Learning records | part of the same row, "as needed" | "Project that submits learning records" / Standard roles plus active `infosec-sentinel`, which screens learning records for credential and cross-record leaks. Required once the project submits learning records / "Security-sensitive changes require explicit evidence" | Trigger-accurate, no "as needed", as 63rh requires. |
| Contributions | part of the same row, "as needed" | "Project that takes contributions" / Standard roles plus active `contribution-auditor`, which reviews contribution integrity. Required from the first contribution to the framework or to the project / (reviewers to propose the third column) | Trigger-accurate, no "as needed". |
| Facts | part of the same row, "as needed" | "Project that publishes factual claims" / Standard roles plus active `fact-checker`, which checks factual claims. Recommended from the first release, as needed / (reviewers to propose the third column) | Discretionary only where the owner makes it discretionary. |
| Independent review | "Standard or CI-conscious roles plus ..." | "Standard roles, plus any rows above, plus ..." | Permission to combine is unchanged. |

`quick-reference.md` gets the same role cells, with its "Use when" column mapped to the triggers: CI
pipeline; submits learning records; takes contributions; publishes factual claims. The original phrase
"Security, compliance, or external evidence matters" stays attached to the learning-records and
factual-claims rows so that no previously covered situation loses its pointer.

Open point for the reviewers: the old third-column sentence "CI runs validators; security-sensitive changes
require explicit evidence" applied to the whole group. Keeping it on the CI and learning-records rows avoids
narrowing it. If a reviewer finds the split changes its meaning, the item stops and returns to the governor.

Classification follows the source plan: **PREVENTIVE** on the unchanged-owner basis. No previously valid
behavior is restricted. If the sentence-level comparison shows a real contract restriction, stop and return the
item as BREAKING with ADR, migration and major-release sequencing.

```
Impact:
  Foundry:          no.
  Direct adopters:  PREVENTIVE. Mirrors the corrections already approved for ADOPTING.md.
  Downstream:       none.
  Migration:        not required. Same remediation sentence as the source plan.
  Evidence:         framework.md lines 1448 and 1486; the four role files; the source plan comparisons.
```

## Projection and distribution

Manifest: `docs/reviews/rule-projection/20261030T230022Z-pages-role-table-parity-amendment.yaml`. Owners stay
unchanged (`ADOPTING.md` is edited under the source plan). Changed pages: both files. Adopter explanation: both.
Runbook: not applicable, no procedure changes. Loaders: existing pointer sufficient. Distribution: both files ship,
and `quick-reference.md` ships as a reference file.

## One implementation candidate

After approval, the existing candidate (!3459) extends to the two paths, so that all three tables reach main
consistently in one Gate 2. The plan publication (!3454) and this file go to main first. There is no intermediate state
with contradictory rows.

## Browser treatment (proposed condition, not a waiver)

63rh supports a narrow governor-recorded exception to the pre-rewrite viewport baseline only for these two mirrored
table corrections. The conditions that stay: independent sentence, HTML and anchor review; a strict MkDocs build;
bounded rendered readability and horizontal-overflow checks of the two tables before Gate 2; the parent redesign's
pre-rewrite viewport baseline; and all per-slice and final rendered acceptance. No visual pass is claimed without
observation. This section proposes the condition. It does not implement a waiver.

## Validation

`agov plan validate`; strict `agov plan projection-check --base-ref origin/main <manifest>`;
`agov plan collision-check --plans-only --fail-on-unacknowledged-collision` (read after commit);
`agov validate adopter-docs`; a MkDocs strict build and an anchor and link check on the changed Pages file, which this
authoring session cannot run because MkDocs is not installed here.

## Risks and rollback

- Risk: splitting the shared third column narrows a sentence. Control: keep it on two rows and the stop clause.
- Risk: the three tables drift again. Control: one Gate 2 for all three.
- Rollback: do not approve, or revert the merge of this plan. The source plan is unchanged.

## Decisions for the governor

1. Approve the two paths for G1 and G2 only.
2. Record or reject the narrow browser-baseline treatment above.
3. Choose the third-column text for the contributions and facts rows after the reviewers answer.
