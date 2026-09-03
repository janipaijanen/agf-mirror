---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260902T063431Z-coder-tester-mjak
timestamp: 2026-09-02T06:34:31Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260831T050008Z-governor-reviewer-r2kb.md
  - 20260831T035836Z-governor-reviewer-jsjk.md
  - 20260831T083954Z-coder-tester-k6s2.md
  - 20260901T114736Z-planner-architect-rhqb.md
topic: release-and-queue-convergence-under-ci-capacity-gap
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Reconcile the current release, plan, issue, TODO, and merge-request queues into one short ordered work map without creating a competing release plan.
    - Propose terminal dispositions for stale, duplicate, superseded, unowned, or weakly specified items and identify the smallest separately gated tracker/roadmap correction.
    - Challenge or accept the proposed WIP limits and release sequencing below, then publish the result through validated visible coordination.
  must_read:
    - docs/plans/20260828T194721Z-v290-release-integrity-and-tracker-truth.md
    - docs/plans/20260831T050110Z-v290-release-publication.md
    - docs/plans/20260830T213258Z-coordination-visibility-correlation.md
    - agf/docs/site/roadmap.md
    - docs/TODO.md
    - docs/portfolio/post-v248-issue-disposition.md
    - docs/portfolio/post-v248-mr-disposition.md
  do_not_touch:
    - .claude/settings.local.json
    - release tags or published artifacts
    - release branches
    - claimed implementation paths
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Reconcile release and work queues while CI capacity is unavailable

The governor's 2026-09-02 session instruction is operationally clear: AI
assistant capacity remains available even when paid CI capacity does not.
Planning, review, implementation, local validation, issue decomposition, and
MR correction should therefore advance to the next real gate. Missing CI is a
named acceptance-evidence blocker, not a reason to leave authorized work
unclaimed. This handoff reports that instruction; it does not manufacture a
governor decision or weaken any required check.

## Current evidence

There is a detailed release path, but not a current unified work queue:

- The approved v2.9.0 work order is
  `20260828T194721Z-v290-release-integrity-and-tracker-truth.md`, limited to A1
  and A2.
- MR !2302 carries the reviewed v2.9.0 publication plan at exact head
  `3cb9f28a6ac0d9fcf7515d76c57a9ad67db44d9e`. Review `k6s2` recommends CLEAR,
  and `rhqb` records successful exact-head pipeline `2804468124`. Binding Gate
  1 is still absent.
- The coordination-visibility-correlation plan is approved and merged to
  `main`, but no current coder-tester claim exists. It is explicitly excluded
  from the v2.9.0 publication plan and therefore needs a release disposition
  before merge, even though implementation can proceed on an isolated branch.
- MR !2292 carries the calm explanatory-visual plan at exact head
  `c8fa182acd97f16e69dd634ff6cfb400476c10ba`; `rhqb` records successful
  pipeline `2804253638`. It remains pending Gate 1 and needs an explicit
  candidate-release placement at its next revision or decision.
- MR !2306 is a reversible AgentConnect source/radar intake at exact head
  `20a6f27e2ca324c007369710e5573ab958364fa5`. It may wait for CI and Gate 2; it
  must not silently enlarge v2.9.0 or block that shipment.

The tracker projections are not current enough to answer the broader queue
question:

- `agf/docs/site/roadmap.md` still calls v2.6.1 the latest fully published
  stable release and v2.8.0 a candidate in construction, although v2.8.0 is
  published. This is material documentation drift already enumerated in the
  v2.9.0 publication plan.
- The live issue API returned 54 open issues, all without a milestone. Most are
  labelled `release::unmilestoned-backlog`; that is a parking class, not an
  actionable order.
- `docs/portfolio/post-v248-issue-disposition.md` and
  `docs/portfolio/post-v248-mr-disposition.md` are observed-at 2026-07-31
  snapshots. They remain useful history but cannot be treated as the current
  complete queue.
- `docs/TODO.md` mixes multiple active investigations, candidate outputs, and
  historical migration notes rather than linking every actionable item to one
  current work order and release disposition.
- There are 187 plan files on `main`: 125 say `approved`, 41
  `pending-gate1`, and the remainder use several older lifecycle spellings or
  omit status. Those counts are repository metadata, not proof that 166 items
  are live work.
- `agov coord status` still reports numerous old pending handoffs whose later
  disposition must be reconstructed. Age alone does not close them, but they
  must not be presented as the active queue without correlation.
- The current project bot token can read issues but receives HTTP 403 for MR
  and pipeline inventory. Therefore no exact current open-MR count or pipeline
  state is claimed in this record. The governor's newer statement that CI
  capacity is unavailable supersedes `rhqb` operationally until the surface
  can be read again; the earlier successful exact-head evidence remains valid
  for the unchanged heads it names.

## Proposed short queue

1. **Shipment lane — v2.9.0 only.** Obtain the binding Gate 1 disposition for
   !2302. Until then, continue only read-only/local release evidence work that
   its existing authority permits. Once approved and CI capacity is confirmed,
   freeze the exact release base before any unrelated merge changes the
   inventory. Do not create a tag without the required successful pipeline and
   pre-tag proofs.
2. **One implementation lane.** Prepare the already-approved
   visibility-correlation implementation on an isolated branch, locally
   validated to a stable exact head, but do not merge it into the v2.9.0 base.
   Route it to Gate 2 with an explicit candidate-release disposition and the
   exact missing CI evidence.
3. **One planning/review lane.** Finish the calm-visual Gate 1 decision or
   disposition before opening another feature plan. A rejection, deferral, or
   later-release placement is a valid terminal result.
4. **Maintenance lane does not block shipment.** Hold !2306 and similar narrow
   source/radar work at their next gate when CI is unavailable. Do not spend
   assistant capacity repeatedly polling an unavailable runner.
5. **Backlog convergence, not mass implementation.** Classify every open issue,
   TODO item, and open MR into exactly one of: current release work order;
   next named candidate release; externally blocked; radar/research; duplicate
   or superseded; or close/reject candidate. Only the first two are actionable
   engineering queues.

Proposed WIP limit: one shipment, one implementation, and one planning/review
item project-wide. This is a starting constraint, not binding authority.
Please challenge it if it would strand an independent low-cost task, but do
not answer queue length by opening more plans.

## Requested output

Publish one compact coordination record, not a new competing roadmap, with a
table containing: item; authoritative artifact; candidate release or explicit
`none`; owner/role; present gate; assistant work allowed now; exact CI or human
decision blocker; and terminal close condition. Name any proposed tracker,
roadmap, TODO, or stale-MR cleanup as a separately gated smallest change.

The desired result is a short executable queue and an honest parked backlog,
not a promise that every historical plan or issue will be implemented.
