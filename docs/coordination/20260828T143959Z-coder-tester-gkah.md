---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260828T143959Z-coder-tester-gkah
timestamp: 2026-08-28T14:39:59Z
model: gpt-5-codex
immutable: true
references-to:
  - 20260827T203743Z-governor-reviewer-nsne.md
  - 20260827T190635Z-planner-architect-qsp1.md
  - 20260828T131042Z-governor-reviewer-4rh7.md
  - 20260828T132040Z-coder-tester-h1gp.md
topic: v290-release-work-order-preparation
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Prepare the proportionate Gate 1 plan authorized by nsne for the canonical internal roadmap, its projections, and the v2.9.0 release work order.
    - Re-read every proposed v2.9.0 issue and implementation state; distinguish shipped-closeout, required implementation, separately gated correction, and defer/exclude dispositions.
    - Propose the smallest coherent v2.9.0 committed scope, with explicit dependencies, owners, child Gate 1 needs, acceptance evidence, and terminal shipment disposition for every item.
    - Include the post-v2.8.0 release-control learnings as explicit candidate work carriers without treating their implementation as already approved.
    - Publish the plan and contribution-auditor consultation through normal Gate 1; do not mutate roadmap, milestones, issues, versions, release branches, tags, or artifacts from this handoff.
  must_read:
    - docs/coordination/20260827T203743Z-governor-reviewer-nsne.md
    - docs/coordination/20260827T190635Z-planner-architect-qsp1.md
    - docs/coordination/20260828T085514Z-planner-architect-rmna.md
    - docs/coordination/20260828T131042Z-governor-reviewer-4rh7.md on coordination MR !2185
    - docs/coordination/20260828T132040Z-coder-tester-h1gp.md on coordination MR !2185
    - agf/docs/site/roadmap.md
    - docs/TODO.md
    - GitLab issues #31, #32, #36, #37, #43, #53, #75, #101, #102, and #106
  do_not_touch:
    - immutable v2.8.0 tag or published Release, package, and OCI identities
    - retained v2.7 evidence refs
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Prepare the v2.9.0 release work order

The governor asks to prepare the next release. The default version is v2.9.0:
v2.8.0 is already public and accepted, while binding decision `nsne` approves
the v2.9-v2.12 clustering as the planning baseline and directs the planner to
prepare the governed roadmap plan once the prior release obligation is stable.

Preparation means making the shipment governable; it does not mean bumping a
version or opening a release branch now. Produce one release work order that
turns the approved v2.9.0 theme into actionable, reviewable work and prevents a
later claim that development is idle while committed shipment contents remain
unstable.

## Independent scope assessment

The full `qsp1` v2.9.0 cluster is directionally coherent but too broad to treat
as automatically committed scope. It mixes four different completion shapes:

1. **Tracker truth:** verify and close #32 and #37 as already shipped; retain
   #36 as partial unless `coord watch` is actually implemented.
2. **Workflow/product work:** #31, #36, #43, and #75 need their own exact
   contracts and normal gates before implementation.
3. **Release-cost engineering:** #53, #101, and #106 overlap and should be
   dependency-ordered or consolidated only where one implementation and one
   acceptance matrix genuinely cover them.
4. **Retirement cleanup:** #102 should enter the committed release only if its
   removal trigger is already satisfied and archive/package checks prove the
   exclusion machinery can safely be removed.

Treat those as a candidate inventory, not a promise to ship all nine. The plan
should recommend a must-ship core and explicitly deferred or conditional
items. A plausible smallest core is tracker truth plus the release-cost control
chain; #31/#36/#43/#75 can be included only where their dependencies and review
cost still produce one coherent shipment.

## New corrective candidates from the v2.8.0 cycle

Account for, but do not silently authorize, three release-control corrections:

- derive preflight Git, Release, package, and OCI namespace checks from the
  authoritative publisher/provenance mapping (`h1gp` on MR !2185), including
  a contract test for Git tag `vX.Y.Z` versus OCI tag `X.Y.Z`;
- add the MR-stage OCI build-and-pinned-scan control already routed to the
  v2.9.0 CI cluster by `rmna`/`j37k`; and
- create a governed carrier for the combined-release-MR projection mismatch
  identified in `k65q`, so plan-scoped manifests and MR-diff-scoped validation
  no longer diverge silently.

The first two plausibly belong in the release-cost core. The projection change
has a different normative owner and should remain a separate child plan even
if it ships in v2.9.0.

## Required work-order shape

For every proposed item, record:

- issue or backlog carrier and normative owner;
- committed, conditional, deferred, already-shipped-closeout, or excluded
  status;
- accountable implementing role and independent consultation;
- dependencies and the exact condition that makes the item actionable;
- child Gate 1 artifact, when implementation is non-trivial;
- acceptance evidence and exact-head CI expectations; and
- terminal shipment disposition: accepted into v2.9.0, explicitly excluded,
  deferred to a named later shipment, or canceled by binding decision.

Freeze release construction until this work order is stable and every included
item is either delivered or has a binding terminal disposition. Market
landscape, source, and radar maintenance remain outside the release work order
unless a separately governed decision promotes a concrete issue into it.

The canonical `docs/roadmap.md`, adopter-facing projection, and GitLab milestone
must agree after implementation. They must not be changed before Gate 1, and
routine roadmap maintenance needs the bounded authority boundary required by
`nsne`; it does not inherit the research-maintenance fast path by implication.

## Sequencing boundary

Planning may start now because v2.8.0 is public and the governor accepted its
disclosed process-evidence gap. Release construction, tagging, and publication
must not start until the independent fresh-Claude `/context` closure is
recorded, the v2.9.0 work order is approved and stable, and every prerequisite
named by the eventual release plan is satisfied.
