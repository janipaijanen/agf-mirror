---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260903T040339Z-coder-tester-tk2n
timestamp: 2026-09-03T04:03:39Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260902T063431Z-coder-tester-mjak.md
  - 20260902T064458Z-planner-architect-a7jr.md
  - 20260902T084517Z-governor-reviewer-jqdd.md
  - 20260902T182347Z-coder-tester-s5t7.md
  - 20260902T191658Z-coder-tester-mmms.md
  - 20260902T192529Z-coder-tester-b8ja.md
  - 20260902T193228Z-planner-architect-ersd.md
topic: governor-directed-release-and-queue-convergence-plan
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Formalize the approved direction as a bounded Gate 1 plan without competing with the active v2.9.0 shipment.
    - Revise, split, or supersede MR !2269 so the canonical-roadmap and queue-convergence work has one current authority and a candidate-release disposition.
    - Define evidence-based terminal dispositions for every open MR, issue, TODO section, and roadmap item, with a short active queue and explicit WIP limits.
  must_read:
    - docs/plans/20260831T050110Z-v290-release-publication.md
    - docs/TODO.md
    - agf/docs/site/roadmap.md
    - docs/reviews/backlog-prioritization/20260813-final-proposal.md
    - docs/reviews/todo-and-sol-high-synthesis-prioritized-actions.md
  do_not_touch:
    - .gitlab-ci.yml
    - tooling/rust/scripts/agov-oci-image.sh
    - refs/tags/v2.8.0
    - release-2.9.0
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Governor directs a release-centred queue-convergence plan

The human governor accepted the proposed operating plan in session with:
"This is a legit plan. File it. Continue as proposed."

This records binding direction to prepare and continue the plan. It is **not**
Gate 1 approval for a not-yet-reviewed plan head, does not authorize closing or
merging any MR or issue, and does not widen the approved v2.9.0 work order.

## Verified baseline

At `origin/main` `86e92ad90e14fdf323211531ad0e4c9914adde65`:

- 54 open MRs: 16 plan, 14 documentation, 10 learning, 7 coordination, and
  7 other; 20 conflict, 9 require CI, and 2 are drafts;
- six unresolved discussions remain across !341, !407, and !1451;
- 54 open issues are all unmilestoned and 52 are unassigned; 49 carry the
  `release::unmilestoned-backlog` label;
- 191 plan files have multiple lifecycle spellings and four have no `status`,
  so plan metadata alone is not yet a reliable execution queue;
- `docs/TODO.md` contains nine substantial work areas without one consistent
  release-work-order projection; and
- `agf/docs/site/roadmap.md` still calls v2.8.0 a candidate although v2.8.0 is
  already an immutable published release.

These are queue-governance findings, not authority to mutate the items.

## Approved direction to formalize

### 1. Complete v2.9.0 first

Keep A1/A2 as the complete committed v2.9.0 scope. First prepare the bounded
Gate 1 correction for the two mandatory-`needs` implication defects identified
in `s5t7`/`mmms`. After approval: correct them, use `b8ja`/`ersd` CI Lint as the
no-runner preflight, run one canonical exact-head acceptance path, obtain the
vulnerable-base negative transcript, complete Gate 2, and publish v2.9.0. Do
not admit unrelated backlog into the release.

### 2. Make open MRs active delivery vehicles, not backlog

Prepare evidence-based disposition batches in this order:

1. publish, consolidate, or explicitly supersede the seven coordination MRs;
2. resolve the live discussions on !341, !407, and !1451;
3. inspect the 20 conflicted MRs and close only those whose content is proven
   shipped, superseded, rejected, or deliberately deferred;
4. disposition the 16 plan MRs as approve/revise/supersede/close; and
5. disposition the documentation, learning, and other MRs by exact head.

Never close by age alone. Every terminal close names its evidence and any
successor. Target fewer than ten open MRs; parked work belongs in issues or the
roadmap, not in an indefinitely open MR.

### 3. Turn issues into a versioned release backlog

Every issue receives exactly one state: committed to a named release;
research/radar only; deferred with a concrete trigger; shipped-closeout;
rejected; or superseded. Each committed item needs an owner, acceptance
criteria, dependencies, and a terminal shipment disposition. Verify #32 and
#37 as shipped-closeout candidates and split #36's delivered incremental-list
surface from its still-absent watch surface before changing issue state.

Radar and source candidates remain non-committal until separately promoted.

### 4. Retire `docs/TODO.md` as a parallel backlog

Map each actionable TODO section to an issue and, only when committed, to a
release work order. Preserve the historical rationale, but replace actionable
duplicate prose with canonical links after governed migration. Do not allow an
issue, TODO entry, plan, MR, and roadmap row to compete as five owners of the
same commitment.

### 5. Establish one roadmap authority

Revise, split, or supersede MR !2269 rather than opening a competing roadmap
plan. `docs/roadmap.md` should become the internal commitment owner;
`agf/docs/site/roadmap.md` and GitLab milestones should be projections. Remove
the overtaken v2.9.0 authority from !2269, give the remaining work an explicit
candidate-release disposition, and route the proposed v2.10 independence and
enforcement, v2.11 adopter safety, and v2.12 commercial/compliance clusters
through their own evidence and gates.

The stale public v2.8.0 text remains in the already-approved v2.9.0 release
metadata correction scope; do not create a second correction path.

### 6. Keep a hard WIP limit

At most one shipment, one implementation approaching Gate 2, and one
planning/review batch may be active. A work item becomes active only when it
has a release/work-order relationship, owner, acceptance criteria, and terminal
condition. A merge is progress; the release is the shipment.

## Expected planner output

Return one exact-head Gate 1 proposal that:

- preserves v2.9.0 as the active shipment and sequences queue work after it;
- reuses or explicitly supersedes MR !2269;
- defines the disposition ledger fields and small batch boundaries;
- distinguishes decisions that assistants can prepare from closures and merges
  requiring governor authority;
- includes negative controls against closing by age, promoting radar implicitly,
  duplicating commitment authority, and exceeding WIP limits; and
- defines measurable terminal results for MR count, issue assignment/version
  coverage, TODO migration, and roadmap projection consistency.

Continue safe read-only classification and plan drafting while CI capacity is
unavailable. Do not retry quota-failed jobs merely to seek green status.
