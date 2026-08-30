---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260830T214529Z-planner-architect-kjjd
timestamp: 2026-08-30T21:45:29Z
model: claude-opus-5
immutable: true
references-to:
  - 20260828T143959Z-coder-tester-gkah.md
  - 20260827T203743Z-governor-reviewer-nsne.md
  - 20260830T214141Z-governor-reviewer-m2pr.md
topic: v290-roadmap-plan-gate1-ready
requires-ack: true
artifacts_owned:
  - docs/plans/20260830T214452Z-v290-governed-roadmap-and-work-order.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Gate 1 decision on docs/plans/20260830T214452Z-v290-governed-roadmap-and-work-order.md (MR !2269).
    - "Answer the open question the plan cannot close: the disposition of release-v2.7.0-rc.2."
    - Confirm or correct the planner-architect role assumption recorded below.
  must_read:
    - docs/plans/20260830T214452Z-v290-governed-roadmap-and-work-order.md
    - docs/coordination/20260827T203743Z-governor-reviewer-nsne.md
    - docs/coordination/20260830T214141Z-governor-reviewer-m2pr.md
  do_not_touch:
    - v2.7.0-rc.1 tag and the retained v2.7 evidence refs
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.9.0 roadmap and work order ready for Gate 1

Answers the `gkah` work order under the binding direction in `nsne`.

## Role assumption, stated rather than assumed

This session has been operating as `coder-tester`. `gkah` addresses the work
order to `planner-architect`, and the governor authorized advancing the release
roadmap work in session. `planner-architect` is a Claude-played role in
`agf/agents/MANIFEST.yaml`, so the assumption is available — but role authority
is a governor matter, so it is recorded here explicitly for confirmation or
correction rather than taken silently.

## Sequencing condition is satisfied

`nsne` conditioned this work on the v2.7.0-rc.1 obligation being stable.
`m2pr` abandons rc.1 permanently. Abandonment is terminal, so the condition is
met — this is not a case of proceeding while a release obligation is still
open.

## Issue claims verified, not repeated

`nsne` directed verification before closure. Done mechanically against the
built CLI and crate tree:

- **#32** — `agov coord post`, `agov handoff take`, `agov handoff complete`
  all resolve. Shipped.
- **#37** — `tooling/rust/crates/agov-mcp` exists. Shipped.
- **#36** — `agov coord list --since` exists; `agov coord watch` does not.
  Partial, exactly as `nsne` anticipated.

#36 is therefore recommended for a **split**, not a whole closure: closing it
entire would overclaim delivered capability, and leaving it entire would
understate it. Closure itself is not authorized by the plan; the dispositions
are recommendations pending Gate 1.

## What I most want reviewed

**The bounded maintenance path.** `nsne` asked for a route for routine roadmap
edits without a fresh full Gate 1 each time. That is a reduced-ceremony route,
and reduced-ceremony routes are where governance gets exploited — the governor
raised exactly this concern when commissioning the gate-change discipline work.

I modelled it on the existing, proven sources/radar fast path rather than
inventing one, kept the branch/MR/pipeline/Gate 2 requirements fully intact,
and wrote the disqualifier list to resolve ambiguity toward full Gate 1. The
plan also carries a negative test requiring that an ineligible edit — moving an
item between version clusters — be *shown* rejected rather than merely
described as rejectable.

I am the author of that boundary and cannot fairly judge whether it is tight
enough. It is the part I would attack first.

## Open question I deliberately did not answer

`release-v2.7.0-rc.2` exists and `g586` acknowledged its construction. `m2pr`
abandons rc.1 and declines to infer rc.2's fate. v2.8.0 is already published
and accepted, which argues the whole v2.7.0 line is dead — but that is a
release decision, not a planner inference, and the two readings lead to
materially different work.

The plan's scope cannot be called complete until this is answered. No rc.2 work
proceeds meanwhile, because starting would presume the answer.

## Not done, deliberately

No version bumped, no milestone moved, no issue state changed, no release
branch or tag touched. `nsne` authorizes planning, not mutation, and this
handoff inherits that boundary unchanged.
