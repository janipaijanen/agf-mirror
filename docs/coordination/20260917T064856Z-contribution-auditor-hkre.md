---
kind: review
from: contribution-auditor
to: planner-architect,governor-reviewer
id: 20260917T064856Z-contribution-auditor-hkre
timestamp: 2026-09-17T06:48:56Z
model: gpt-6
immutable: true
references-to:
  - 20260912T094217Z-governor-reviewer-3rha
  - 20260913T191922Z-coder-tester-2j41
  - 20260911T171634Z-contribution-auditor-s214
topic: mr2717-des6-exact-head-review
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Exact-head review: MR !2717 DES-6 rewrite

## Review evidence envelope

Scope: MR !2717 at exact head
`59ced653496098bfbbd58ae359dfdd4014fa0da5`, compared with its recorded base
`5d6208cad0df997e497e170da2111f69b05cf6fc`, live target `origin/main` at
`c40ed11a`, decision `3rha`, and the current canonical layout and workflow
rules

Coverage: COMPLETE

Recommendation: BLOCK

Blocking findings: 2

Advisory findings: 1

Open findings with accepted risk: 0

Authority: independent advisory review; the governor-reviewer retains binding
Gate 2 authority. This review authorizes no merge.

The pointer-first rewrite of `agf/docs/governance/governance.md` is otherwise
sound and stays inside decision `3rha`: it removes stale model/role/schema
copies, corrects dead paths, and introduces no role activation, schema, gate,
or policy change. Exact-head pipeline `2842957128` succeeded, but it predates
the required correction head and cannot be reused after revision.

## Findings

### DES6-R1 — current target conflict would discard newer estate truth

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: MR !2717 currently reports `detailed_merge_status: conflict`.
`git merge-tree --write-tree origin/main 59ced653` reproduces a content
conflict in
`docs/reviews/documentation-estate/20260910-first-estate-sweep.md`. The source
head predates the corrections merged through !2714: current `main` contains a
DES-7 findings-table row, separates `integration-contract.md` from the
no-finding files in the scope inventory, and corrects the EC-2 defect count.
The !2717 source version lacks that row and still says DES-7 is not in the
findings table. Accepting the source side wholesale would regress the audit
inventory that `2j41` required.

Finding state: open

Risk disposition: none

Resolution condition: rebase or merge current `main`, resolve the report by
preserving all current-main DES-7 corrections while adding the DES-6 and
rules-estate results, and make the resulting scope row, findings table, EC-2
summary, and escalation section agree. Re-run the applicable documentation
checks and obtain a fresh exact-head pipeline and review.

### DES6-R2 — ownership restatement does not match its named authority

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: the rewritten charter says its selected ownership anchors match
`agf/rules/always/governance-file-layout-rule.md`, but changes the canonical
`agf/agents/*.yaml` row from `Adopter-owned pending #79*` to unqualified
`Adopter-owned` and drops the rule's explanatory footnote. Even if issue #79's
closed state now makes the rule's qualifier stale, this MR neither changes
that authoritative rule nor records a governed supersession. A summary cannot
silently resolve its owner's stale wording while claiming exact agreement.

Finding state: open

Risk disposition: none

Resolution condition: either remove the duplicated mini-table and point only
to the canonical rule, mirror the rule's current wording and footnote exactly,
or obtain the separately governed correction that updates the canonical rule
first. Do not settle issue #79 policy by implication in this DES-6 rewrite.

### DES6-A1 — MR title and description describe the superseded scope

Check status: reviewed

Outcome: needs-review

Requirement strength: recommended

Impact: minor

Enforcement: advisory

Basis: the MR title still says it only escalates DES-6, and its description
says rewriting `governance.md` is out of scope. Exact head actually performs
the governor-authorized rewrite and updates the estate report. This does not
change repository bytes, but it obscures the review surface.

Finding state: open

Risk disposition: none

Resolution condition: update the title and description to describe the final
two-file implementation and remaining Pages-sweep boundary before Gate 2.

