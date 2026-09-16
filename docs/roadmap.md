# Foundry roadmap

This file is the single governed internal owner for foundry roadmap state.
`agf/docs/site/roadmap.md` is its adopter-facing projection. GitLab milestones,
issue labels, planning proposals, and advisory release horizons are derived
views or inputs; they do not create roadmap commitments independently.

| Field | Value |
|---|---|
| Status | v2.10.1 is the latest published stable release (tag pipeline, GitLab Release, and anonymous artifact verification complete -- `20260912T210138Z-planner-architect-asdh`, `20260912T215750Z-coder-tester-4jss`, `20260912T220356Z-planner-architect-grqd`). Post-v2.10 work also includes the active, Level 4 Validated Improvement Propeller; Level 5 awaits a genuine unattended scheduled wake. |
| Observed at | 2026-09-16T04:24:48Z |
| Observation baseline | `main` at `f8a8025e984f8d59a3adb4d1d672812e790379d5` |
| Roadmap authority | This file owns roadmap status and sequencing. Binding plans and governor decisions authorize work; exact tags and release evidence establish what shipped. |

## Released baseline

**Latest stable release: v2.10.1.** Published 2026-09-12, superseding v2.10.0
as latest (v2.10.0 remains available, not removed from history). Tag
pipeline, GitLab Release, generic package, OCI image, checksums, and
anonymous downloads independently verified
(`20260912T215750Z-coder-tester-4jss`, `20260912T220356Z-planner-architect-grqd`).
v2.10.0's own Level 5 Deployable verification remains on record in
`docs/coordination/20260908T152605Z-coder-tester-re48.md`.

The public artifacts are available from the
[v2.10.1 GitLab Release](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases/v2.10.1).

No later version number or release scope is binding merely because it appears
in an issue label, proposal, or advisory horizon. A later release requires its
own governed plan, Gate 1 approval, exact-head Gate 2 decision, and explicit
tag/publication authorization.

## Active post-v2.10 sequence

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
| Issue #129 | Assessment required | Complete and review the linked-data/evidence-model assessment before `qbka` Slice A freezes provenance semantics. This is assessment-first, not linked-data implementation. |
| `qbka` plan lifecycle | Done | Slice A (superseded plan-lifecycle schema/validator/provenance) merged and Gate 2 approved (`20260913T160721Z-governor-reviewer-5jjg`). Slice B (legacy-plan migration) merged after an F1 evidence correction (see `git log --grep=qbka`: `41d27775`, `c246a322`). |
| Issue #121 | Row 5 S1 shipped session-level addressing (`for_session`), merged `20260911T035333Z-planner-architect-nh1j` | Unresolved whether this fully satisfies #121's scope, or leaves the execution-lane/claim-and-reassignment work its own comments describe (`hqps`, 2026-09-05) as a separate slice. Needs verification before closing the issue. |
| Issue #122 | Implementation done; issue closure pending | Resolved by Row 5 S2 self-announcing supersession, Gate 2 approved `20260914T115129Z-governor-reviewer-4bm1`. GitLab issue #122 itself remains open -- this row records shipped implementation, not tracker closure. |
| MR !2027 | Closed | Closed on GitLab 2026-09-13T10:04:36Z, after the prior roadmap baseline. |
| Issue #123 | Partially done | Plan-vs-plan `affected_paths` collision detection shipped and MR-blocking (Row 6 S3 advisory + Row 6 S4 blocking promotion, Gate 2 approved `20260915T050307Z-governor-reviewer-g7j1`). The plan's original third bullet -- a post-implementation `affected_paths`-vs-actual-diff drift check -- remains open and unclaimed; needs its own Gate 1 plan if pursued. |
| CPB2 (publication-hold urgency) | Done | Merged and closed (`20260913T085749Z-coder-tester-b1nb`). |
| Row 5 S3 (priority ordering and challenge response) | Done | Merged (`20260913T121333Z-planner-architect-gkqh`). |
| Issue #131 | Activation merged; genuine unattended wake still outstanding | Foundry activation is Level 4 Validated under plan `20260909T015237Z-propeller-stage4-activation` and decision `djh1`; the tracked activation commit (`96cad939`) is on `main`. The scheduler's own live wake state was last confirmed paused, `last_weekly_completed_at: null` (`20260911T135557Z-contribution-auditor-sqj4`). Only a first genuine unattended conforming wake, recorded separately, can supply Level 5 evidence -- it has not yet occurred as of this observation. |
| Issue #132 | Blocked | Do not export the Propeller method as a skill until #131 has genuine Level 5 wake evidence. |
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
