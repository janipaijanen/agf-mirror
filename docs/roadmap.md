# Foundry roadmap

This file is the single governed internal owner for foundry roadmap state.
`agf/docs/site/roadmap.md` is its adopter-facing projection. GitLab milestones,
issue labels, planning proposals, and advisory release horizons are derived
views or inputs; they do not create roadmap commitments independently.

| Field | Value |
|---|---|
| Status | v2.12.0 is the latest published, independently verified stable release. The Improvement Propeller is Level 5 Deployable (`20260918T162825Z-governor-reviewer-8hs2`). |
| Observed at | 2026-10-28T16:13:58Z |
| Observation baseline | Annotated tag `v2.12.0` (`008ec0c3c633db62ff7e593aa9fb12c405cf9b27`) at `e045d20b073ce31341cd0d460469fac60cdbab5a` |
| Roadmap authority | This file owns roadmap status and sequencing. Binding plans and governor decisions authorize work; exact tags and release evidence establish what shipped. |

## Released baseline

**Latest stable release: v2.12.0.** Published 2026-10-28 from the verified
annotated tag. Its 23-job tag pipeline, GitLab Release, generic package, OCI
image, checksums, provenance, packaged executable, and anonymous downloads
were independently verified
(`20261028T161358Z-coder-tester-gm8j`). Earlier releases remain available
and are not removed from history.

The public artifacts are available from the
[v2.12.0 GitLab Release](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases/v2.12.0).

No later version number or release scope is binding merely because it appears
in an issue label, proposal, or advisory horizon. A later release requires its
own governed plan, Gate 1 approval, exact-head Gate 2 decision, and explicit
tag/publication authorization.

## Active post-v2.11 sequence

The governing inputs are the Gate-1-approved
`docs/plans/20260903T043752Z-queue-convergence-and-roadmap-authority.md`, its
approval `docs/coordination/20260903T045255Z-governor-reviewer-j4rg.md`, the
collision disposition
`docs/coordination/20260908T183242Z-planner-architect-ds4k.md`, and the
governor confirmation
`docs/coordination/20260908T185629Z-governor-reviewer-j2gs.md`.

| Workstream | State | Next authorized boundary |
|---|---|---|
| Queue convergence B1 | Done | Roadmap owner (`5314240b`), B1 dispositions (`e3d2a214`), and review advisories closure (`161c9281`) all merged to `main`. |
| Queue convergence B2-B3 | Sequenced after B1 | Re-observe and disposition TODOs, then issues, in separate review-sized batches. |
| Queue convergence B4-B7 | Partly blocked | Non-plan cohorts remain later batches. Plan-MR dispositions wait for the `qbka` Slice A provenance contract (now satisfied -- see `qbka` row below; this row's own re-observation has not yet been separately confirmed). |
| Queue convergence B8 | Advisory bound defined below | Apply the bound during discovery and selection; promotion to a blocking gate requires separate evidence, planning, and approval. |
| Issue #129 | Withdrawn | The linked-data / semantic-data assessment was withdrawn by governor decision `20260909T135843Z-governor-reviewer-6sj1`. It is historical backlog only, is not actionable, and did not block `qbka` or v2.11. |
| `qbka` plan lifecycle | Done | Slice A (superseded plan-lifecycle schema/validator/provenance) merged and Gate 2 approved (`20260913T160721Z-governor-reviewer-5jjg`). Slice B (legacy-plan migration) merged after an F1 evidence correction (see `git log --grep=qbka`: `41d27775`, `c246a322`). |
| Issue #121 | Done; issue closed after verified v2.11.0 publication | Row 5 S1 shipped session-level addressing (`for_session`), merged `20260911T035333Z-planner-architect-nh1j`. Any future execution-lane or claim-and-reassignment work requires its own scope and authority. |
| Issue #122 | Done; issue closed after verified v2.11.0 publication | Resolved by Row 5 S2 self-announcing supersession, Gate 2 approved `20260914T115129Z-governor-reviewer-4bm1`. |
| MR !2027 | Closed | Closed on GitLab 2026-09-13T10:04:36Z, after the prior roadmap baseline. |
| Issue #123 | Done; issue closed after verified v2.11.0 publication | Plan-vs-plan `affected_paths` collision detection shipped and is MR-blocking (Row 6 S3 advisory + Row 6 S4 blocking promotion, Gate 2 approved `20260915T050307Z-governor-reviewer-g7j1`). A post-implementation `affected_paths`-vs-actual-diff drift check remains a separate, unclaimed follow-up and needs its own Gate 1 plan if pursued. |
| CPB2 (publication-hold urgency) | Done | Merged and closed (`20260913T085749Z-coder-tester-b1nb`). |
| Row 5 S3 (priority ordering and challenge response) | Done | Merged (`20260913T121333Z-planner-architect-gkqh`). |
| Issue #131 | Done | Foundry activation is Level 5 Deployable after the genuine unattended wake `sg5e`, independently reviewed and confirmed by governor decision `20260918T162825Z-governor-reviewer-8hs2`. |
| Issue #132 | Done | The optional Improvement Propeller skill export is merged (`32fd0134`). Adoption remains inactive until an adopter's own governor supplies the required activation decision and wake configuration. |
| `release-v2.7.0-rc.2` | `awaiting-governor` | Its fate remains undecided after rc.1 was abandoned. It does not block the current sequence. |

Absence from this roadmap summary is not a terminal disposition. Individual
issues, merge requests, TODOs, and plan artifacts remain unexamined until an
evidence-bearing disposition ledger row or binding decision says otherwise.

## Advisory intake bound

The queue-convergence plan observed that the merge-request queue regrew after
an earlier disposition pass. Foundry therefore adopts this project-local
working agreement:

- at most **five open non-coordination implementation merge requests**;
- at most **three open Gate 1 plan merge requests awaiting a decision**; and
- coordination-only merge requests are not counted, because their audit-trail
  publication is expected to remain prompt and independent.

While a class is above its bound, do not open new work in that class. The only
exceptions are work needed for the active release, a correction to an already
open item, or an explicit governor override recorded in coordination. The
bound does not authorize any of those actions; their existing scope, role,
gate, and approval requirements still apply.

This is an advisory working agreement, not a validator, CI gate, merge-policy
change, or adopter-facing AGF rule. Each poll observes the queue before
selecting work. Evidence that the advisory bound is insufficient must return
through a separate Gate 1 plan before any mechanical enforcement is added.

## Collision and ownership boundaries

- `qbka` owns plan-file lifecycle states, transition semantics, supersession
  provenance, validators, and the legacy-plan migration.
- Queue convergence owns this roadmap, its public projection, and the
  issue/MR/TODO disposition ledgers.
- When a later queue-convergence ledger row marks a plan artifact
  `close-superseded`, it must cite the same replacement identifier and governor
  decision required by the eventual `qbka` provenance contract. The ledger
  records that lifecycle disposition; it does not create a competing one.
- Issue #75/MR !745 split at that boundary: plan-lifecycle mechanics route to
  `qbka`; board projection routes to queue convergence. MR !1617 is superseded
  by queue convergence. MR !1817 remains a separate Gate 1 item sequenced after
  B8.

## Superseded roadmap plan

MR !2269 merged a prior roadmap work-order plan. It is superseded by the
current queue-convergence plan because the latter retains its sound
owner/projection model while adding the governor-requested queue dispositions
and removing the unjustified reduced-ceremony maintenance path.

MR !2269 is already merged, so there is no open MR to close. Its GitLab record
must instead receive a note pointing to the current plan, this roadmap owner,
and the governor decisions. The plan schema currently has no `superseded`
status; the future `qbka` work owns that lifecycle correction.

## Update discipline

Every roadmap change must:

1. name the plan or governor decision that authorizes the changed commitment;
2. refresh the observation timestamp and source revision;
3. update `agf/docs/site/roadmap.md` in the same batch when the public
   projection is affected; and
4. preserve the distinction between planned, validated, deployable, and
   empirically supported claims.

Routine maintenance currently follows the normal Gate 1/Gate 2 workflow. No
special roadmap fast path exists. If this owner and its projection disagree,
the mismatch is a blocking documentation finding and this owner prevails until
the projection is corrected.
