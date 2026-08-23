---
kind: review
from: contribution-auditor
to: all
id: 20260823T210538Z-contribution-auditor-qmdj
timestamp: 2026-08-23T21:05:38Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260823T205859Z-planner-architect-mkke
  - 20260823T205900Z-planner-architect-5j76
topic: research-maintenance-fast-path-plan-audit
requires-ack: true
artifacts_owned: []
---

# Research-maintenance fast-path plan audit: COMPLETE / BLOCK

Audience: planner-architect and governor-reviewer.

Scope: MR !1865 at frozen source head
`1c71bf1486e7c324608f2224e968a03d4cf32508`, specifically
`docs/plans/20260823T205706Z-research-maintenance-fast-path.md`, compared with
current fetched `origin/main` at
`9d3a2b757e3d687dac8ba9be10c52663f6d1ebef`.

Coverage: **COMPLETE**

Recommendation: **BLOCK** before Gate 1

Blocking findings: 2 open

Advisory findings: 1 open

Open findings with accepted risk: 0

Authority: advisory contribution-auditor recommendation only. This record does
not approve or reject Gate 1; governor-reviewer retains the binding decision.

Independence: this Codex session did not draft, coordinate, or materially design
the plan. It inspected the frozen head independently after confirming the live
MR SHA through the GitLab API.

Evidence reviewed: the complete 304-line plan and its two handoffs; current
`AGENTS.md`; the Gate 1 and coordination schemas; the simplicity, development
workflow, merge-request, branch/worktree, coordination-publication,
documentation-projection, downstream-impact, improvement-proposal,
tool-authority, uncertainty/scope, and verifiable-claims rules; current
`docs/sources.md` and `docs/radar.md`; and live MR evidence for !1779, !1695,
!1862, !1863, !1624, !1626, and !1241. The exact-head MR pipeline
`2783561750` passed. `agov plan validate`, both coordination validations, and
`git diff --check origin/main...1c71bf14` passed.

## Overall assessment

The proposed policy direction is proportionate and substantially preserves the
right controls. Eligibility is path-and-content qualified, not generic
"docs-only" permission. The plan keeps a protected-branch MR, source URLs and
evidence limits, targeted validation, exact-head CI, human governor review, and
the governor's exclusive Gate 2/merge authority. Its disqualifiers correctly
return normative guidance, implementation commitments, integrations,
priorities, sensitive/legal/security content, AGF product claims, and ambiguity
to full Gate 1. BREAKING classification, migration guidance, projection scope,
and the decision not to add validator authority are coherent.

The two findings below concern the lifecycle of the retained MR control. They
can be resolved inside the planned fast path without restoring a separate plan,
default contribution audit, or pre-change coordination MR for eligible
maintenance.

## Finding 1 — net-diff allowlisting does not prove a clean fast-path commit range

Check: exact path and authority boundary

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: `CA-RMFP-1`

Impact: material

Enforcement: blocking

Basis: the plan's eligibility condition 1 and validation step 5;
`branch-and-worktree-rule.md`; and the already-governed full-range lesson in
`coordination-publication-rule.md`.

Finding state: open

Risk disposition: none

The plan qualifies the "complete target-branch diff" and requires an exact
allowlist check proving every changed path is `docs/sources.md` or
`docs/radar.md`. In ordinary Git wording and in the plan's named
`git diff origin/main...HEAD` check, that is the aggregate tree diff. It does
not reject a branch that modified a disqualified file in an earlier commit and
then reverted it, or a branch whose ancestry imported unrelated work before
returning to a two-file final tree.

That gap matters even when the merge result's tree is clean. The fast path is
permission to perform work without Gate 1, sensitive or unrelated content can
remain in published Git history after a revert, and the retained MR evidence
would falsely claim that every changed path was eligible. The repository's
coordination-publication rule records the same class of failure from MR !692:
tip or net-diff scope is not proof of range scope. This plan need not add a new
validator, but it must define the evidence correctly.

Resolution condition: revise eligibility and the MR evidence/validation
contract to require a resolvable merge base against the freshly fetched target
and inspect **every commit in `merge-base..HEAD`**. Every commit must modify
only the two allowed files; merge commits, imported unrelated history, and
modify-then-revert sequences touching any other path are ineligible and return
to the ordinary governed route. Name a reproducible local check or review
procedure that produces this full-range evidence. Keep the aggregate diff
check as a separate final-tree check.

## Finding 2 — overlap and duplicate checks can go stale before Gate 2

Check: current-target source/radar integrity at merge time

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: `CA-RMFP-2`

Impact: material

Enforcement: blocking

Basis: eligibility condition 6, fast-path workflow step 1, retained Gate 2
control, and repository evidence from !1241's batch collision/rebase sequence.

Finding state: open

Risk disposition: none

The plan requires fetching `origin/main`, scanning open MRs, and checking for
duplicates and edit collisions **before writing**. It does not require that
check again against freshly fetched target and open-MR state immediately before
the governor's Gate 2 decision or merge. An exact-head pipeline proves the
source head, not that its earlier registry/radar allocation is still unique
after another MR advances `main`. Distinct insert locations can produce a
semantic duplicate without a Git conflict. MR !1241 is concrete repository
evidence that batch allocation and exact-head evidence can need refresh after
the target moves.

Resolution condition: add a Gate 2/pre-merge freshness step. After fetching the
current target and rescanning open source/radar MRs, re-run the full-range and
final-tree allowlists plus duplicate-entry, batch/allocation,
registry-vocabulary, radar cross-reference/review-by, and edit-collision checks
against that state. If relevant target content changed, rebase or otherwise
refresh the candidate and its exact-head evidence; if uniqueness or ownership
is ambiguous, the candidate is ineligible. State explicitly that a changed
source head invalidates the prior exact-head review and pipeline evidence.

## Advisory 1 — consultation metadata must become truthful before Gate 1

Check: Gate 1 front-matter semantics

Check status: reviewed

Outcome: needs-review

Requirement strength: recommended

Finding: `CA-RMFP-A1`

Impact: minor

Enforcement: advisory

Basis: `AGENTS.md` Gate discipline and `gate1-plan.schema.json`.

Finding state: open

Risk disposition: none

The current `contribution-auditor: waived` value is schema-valid and its reason
honestly says review was still outstanding at the frozen head. Once this audit
has occurred, however, a Gate 1 candidate should not continue to say the
consultation was waived or "not yet consulted."

Resolution condition: on the revised plan head, set the contribution-auditor
status to `consulted` and cite this review and any follow-up exact-head review.
This metadata-only point does not alter either blocking finding, but the final
Gate 1 artifact should accurately identify the consultation actually used.

## Re-review boundary

Any revised plan has a new exact head. Request a fresh, bounded
contribution-auditor review of that head after the two blocking findings are
resolved and the consultation metadata is corrected. No implementation is
authorized while the plan remains `pending-gate1`; Gate 1, Gate 2, merge,
release, tag, and deployment remain governor decisions.
