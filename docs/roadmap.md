# Foundry roadmap

This file is the single governed internal owner for foundry roadmap state.
`agf/docs/site/roadmap.md` is its adopter-facing projection. GitLab milestones,
issue labels, planning proposals, and advisory release horizons are derived
views or inputs; they do not create roadmap commitments independently.

| Field | Value |
|---|---|
| Status | v2.10.0 is published and independently verified. Post-v2.10 work includes the active, Level 4 Validated Improvement Propeller; Level 5 awaits a genuine unattended scheduled wake. |
| Observed at | 2026-09-09T05:10:00Z |
| Observation baseline | `main` at `d0674e3fbacfc6fd87ffed0207ad54b08d7dc383`, before the Stage 4 activation diff |
| Roadmap authority | This file owns roadmap status and sequencing. Binding plans and governor decisions authorize work; exact tags and release evidence establish what shipped. |

## Released baseline

**Latest stable release: v2.10.0.** The release is Level 5 Deployable: its
annotated tag, mandatory tag pipeline, GitLab Release, generic package, OCI
image, checksums, and anonymous downloads were verified in
`docs/coordination/20260908T152605Z-coder-tester-re48.md`.

The public artifacts are available from the
[v2.10.0 GitLab Release](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases/v2.10.0).

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
| Queue convergence B1 | Authorized current batch | This batch creates the owner, corrects its public projection, and records MR !2269 as superseded; its exact head returns for Gate 2. |
| Queue convergence B2-B3 | Sequenced after B1 | Re-observe and disposition TODOs, then issues, in separate review-sized batches. |
| Queue convergence B4-B7 | Partly blocked | Non-plan cohorts remain later batches. Plan-MR dispositions wait for the `qbka` Slice A provenance contract. |
| Queue convergence B8 | Sequenced later | Record the advisory intake bound in this owner as its own review-sized batch. It is not part of B1. |
| Issue #129 | Assessment required | Complete and review the linked-data/evidence-model assessment before `qbka` Slice A freezes provenance semantics. This is assessment-first, not linked-data implementation. |
| `qbka` plan lifecycle | Needs its own Gate 1 plan | A separate planner owns the schema, validator, transition semantics, provenance contract, and legacy-plan migration work order. |
| Issues #121, #122, MR !2027 | Needs own Gate 1 workstream | Treat as coordination-protocol semantics; #122 must align with the `qbka` provenance shape. |
| Issue #123 | Needs own Gate 1 plan | Add collision-detection tooling only after the current manual collision pass. |
| Issue #131 | Stage 4 Gate 1 approved; tracked activation in progress | Foundry activation is Level 4 Validated under plan `20260909T015237Z-propeller-stage4-activation` and decision `djh1`. After the tracked activation reaches Gate 2 and merges, initialize the guarded local state and one verified Monday 09:00 UTC heartbeat. Only its first genuine unattended conforming wake can supply separate Level 5 evidence. |
| Issue #132 | Blocked | Do not export the Propeller method as a skill until #131 has genuine Level 5 wake evidence. |
| `release-v2.7.0-rc.2` | `awaiting-governor` | Its fate remains undecided after rc.1 was abandoned. It does not block the current sequence. |

Absence from this roadmap summary is not a terminal disposition. Individual
issues, merge requests, TODOs, and plan artifacts remain unexamined until an
evidence-bearing disposition ledger row or binding decision says otherwise.

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
