---
id: post-v248-portfolio-and-roadmap-reconciliation
status: pending-gate1
topic: post-v248-portfolio-and-roadmap-reconciliation
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: "The approved v2.5.0 SoW and its incorporated whole-portfolio audit establish the preservation-first and no-blanket-merge controls. This child plan applies those controls to the larger 2026-07-31 live inventory; a fresh exact-head contribution-auditor review is required before Gate 1 approval."
  - role: infosec-sentinel
    status: not-applicable
    reason: "The work reads repository metadata, preserves already-published project records, reconciles tracker state, and edits roadmap documentation. It adds no credential, secret, executable, network trust boundary, or cross-record learning content."
---

# Plan: post-v2.4.8 portfolio and roadmap reconciliation

## Decision requested

Authorize one preservation-first reconciliation of every open GitLab issue and
merge request after the successful v2.4.8 release. The result must be a strong,
version-controlled release roadmap in which every open item has exactly one
honest disposition: a named release, a named pre-release/T0 work package, a
triggered post-v2.5 backlog, radar, or explicit rejection/closure reason.

This is the child plan required by the approved v2.5.0 Statement of Work. It
replaces that plan's 2026-07-28 snapshot (37 issues and 49 MRs) for execution
purposes because the live 2026-07-31 inventory contains **49 open issues and 93
open MRs**.

## Authority and non-authority

The governor-reviewer's 2026-07-31 instruction requests the harvest, roadmap
reconciliation, and cleanup. Gate discipline still requires this exact plan to
be approved before labels, milestones, issue/MR state, branches, or product
files are mutated. Until then, inventory and analysis are read-only.

Approval authorizes the bounded mutations below. It does not authorize a
release, tag, version bump, source-branch deletion, force-push, or merging
unreviewed product changes.

## Current release map

| Destination | Issues | Treatment |
|---|---|---|
| v2.4.8 completion verification | #81, #93 | Verify the reported defect/removal against the v2.4.8 tag and public artifacts; close only when exact evidence is linked. If either still reproduces, move it to the earliest corrective release instead. |
| T0 portfolio/operations | #17-#19, #37, #53, #57, #75, #82 | Reconcile evidence and tracker state. Close completed learning/intake work; keep #53/#75 live only for an uncompleted measured or workflow slice; close #57 when all children have explicit destinations. |
| v2.5.0 contract core | #33 (minimum #80 dependency), #67 and #78 verified remainder, #79, #80 | Preserve the approved SoW boundary: upgrade-safe adopter authority and actionable acknowledgement semantics, with only their necessary compatibility repairs. |
| v2.5.0 reliability candidates | #76 release-necessary slice, #84-#92, #94 | Each must pass reproduction on current main, compatibility classification, and child-plan approval. Milestone assignment is a candidate signal, not implementation authority. Split or defer anything that expands the approved release character. |
| Post-v2.5 triggered backlog | #31, #32, #34-#36, #38-#49, #76 residual, #77 | Preserve the named revisit triggers from the approved SoW. #44 remains a legal/entity decision, not an engineering release item. No item is silently treated as v2.5 scope. |
| Radar, not release commitment | #59-#62, #83 | Retain as Level 1/theoretical observation surfaces with review triggers; do not assign a release merely to empty the queue. |

No issue is omitted from the table: the ranges enumerate all 49 live issues as
observed through the GitLab API on 2026-07-31.

## MR harvest

All 93 open MRs are evaluated individually in a generated, committed ledger.
The ledger records MR IID, exact source head, target head observed, artifact
class, related issue/plan/coordination chain, unique-content result, current
pipeline/review evidence, destination, and closure condition.

The exact 93-MR snapshot observed before this plan's own MR was opened is:

!340, !341, !349, !407, !430, !459, !461, !463, !465, !654, !658, !661,
!671, !672, !673, !674, !675, !676, !693, !723, !725, !726, !727, !728,
!729, !730, !731, !732, !733, !734, !735, !736, !738, !739, !740, !741,
!742, !743, !744, !745, !746, !747, !748, !749, !750, !751, !752, !753,
!754, !755, !756, !757, !759, !760, !761, !770, !771, !772, !773, !775,
!776, !777, !778, !779, !780, !781, !782, !783, !786, !787, !788, !789,
!791, !793, !794, !796, !800, !805, !806, !807, !810, !824, !826, !827,
!828, !829, !830, !831, !832, !850, !851, !853, and !854.

This exact list is only the snapshot boundary, not a disposition. The
per-MR ledger must classify these items and any subsequently opened MR,
including this plan's own MR !858 if it remains open at execution time.

## Execution sequence

1. Freeze an inventory snapshot with GitLab IID, URL, timestamps, labels,
   milestone, source/target heads, mergeability, pipeline, approvals, and links.
2. Reproduce each open issue against current `main` and, when it claims a
   released defect, against the exact affected and claimed-fixed artifacts.
3. Compare each MR's actual diff and unique commits with current `main` and its
   cited successor. Titles, age, `unchecked`, and conflict status are not proof
   of obsolescence.
4. Publish one content-hash-manifested archive of unique historical
   coordination/review evidence from fresh `main`; validate it before changing
   source MR state.
5. Update issue milestones/labels and the version-controlled roadmap from the
   verified ledger. Every non-release item gets a named revisit trigger.
6. Merge only live work whose plan is approved, exact current head is reviewed,
   target-advance effects are cleared, required tests pass, and the governor has
   Gate 2 authority for that exact head. Re-review after every target advance.
7. Close only MRs proven merged-by-content, superseded, duplicate, rejected, or
   preserved by the validated archive. Add a closing note naming the canonical
   commit/MR/archive and reason. Do not delete source branches in this task.
8. Close only issues whose acceptance condition is demonstrably satisfied or
   whose proposal is explicitly rejected/duplicated. A deferral remains open
   with a release/backlog/radar destination and trigger.
9. Re-query GitLab and prove: zero unclassified open issues, zero unclassified
   open MRs, no roadmap contradiction, and no lost unique evidence.

## Merge and close decision matrix

| Finding | Action |
|---|---|
| Patch already present on `main` | Close as merged-by-content; cite commit and comparison. |
| Superseded by a merged or selected MR | Close; cite successor and preserve any unique evidence first. |
| Historical coordination only, unique and valid | Include in validated archive, merge archive, then close source MR with archive link. |
| Historical coordination invalid or contradictory | Preserve as evidence with explicit invalid status; do not present it as authority. |
| Live plan, no Gate 1 approval | Keep open or close in favor of a canonical replacement; never implement/merge as authority. |
| Live implementation, approved plan, stale target evidence | Rebase/update, rerun proportional checks, obtain exact-head review and Gate 2 decision. |
| Conflict or failed pipeline | Investigate; neither automatically close nor merge. |
| Draft/proposal without current sponsor or trigger | Route to backlog/radar or explicitly reject; record why. |

## Required artifacts

- `docs/portfolio/post-v248-issue-disposition.md` — all 49 issues.
- `docs/portfolio/post-v248-mr-disposition.md` — all 93 MRs.
- A content-hash-manifested coordination archive and validation report.
- Updated `agf/docs/site/roadmap.md` with v2.4.8 baseline, v2.5.0 train,
  release-specific scope, post-v2.5 triggers, and observation timestamp.
- GitLab labels/milestones/closing notes matching the committed ledgers.
- A completion coordination record with before/after counts and exceptions.

## Acceptance criteria

1. Every one of the 49 issues and 93 MRs has an evidence-backed individual row.
2. Every issue is assigned to v2.5.0, another named release, T0, triggered
   backlog, radar, or an explicit evidence-backed closure/rejection.
3. Every closed MR links to its preservation or successor evidence; no source
   branch is deleted.
4. Every merged MR has current-target, exact-head approval and passing required
   checks; target advances are re-evaluated sequentially.
5. The roadmap and GitLab tracker agree, while clearly distinguishing approved
   scope from candidates and theoretical radar.
6. `agov` plan/coordination/archive validators and relevant repository tests
   pass at blocking severity.
7. A fresh live query finds no open item absent from the ledgers, including any
   item created during execution.

## Stop conditions

Stop and return to the governor if unique evidence cannot be preserved, a merge
would require force or history rewriting, a proposed closure lacks a canonical
successor/rejection decision, a release assignment changes the approved v2.5.0
character, or GitLab state changes faster than the ledger can be made coherent.
