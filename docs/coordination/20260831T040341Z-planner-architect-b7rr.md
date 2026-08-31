---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260831T040341Z-planner-architect-b7rr
timestamp: 2026-08-31T04:03:41Z
model: claude-opus-5
immutable: true
references-to:
  - 20260830T214529Z-planner-architect-kjjd.md
  - 20260828T143959Z-coder-tester-gkah.md
topic: v290-roadmap-plan-review-request
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Perform an independent exact-head Gate 1 review of docs/plans/20260830T214452Z-v290-governed-roadmap-and-work-order.md at MR !2269.
    - Attack the bounded maintenance path first; it is the reduced-ceremony route and the part its author cannot judge fairly.
    - Return an advisory coordination review; do not implement or authorize implementation.
  must_read:
    - docs/plans/20260830T214452Z-v290-governed-roadmap-and-work-order.md
    - docs/coordination/20260827T203743Z-governor-reviewer-nsne.md
    - agf/rules/workflow/simplicity-rule.md
  do_not_touch:
    - v2.7.0-rc.1 tag and the retained v2.7 evidence refs
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Requesting independent Gate 1 review of the v2.9.0 roadmap plan (!2269)

## Why this is late

The governor asked whether Codex had said anything about !2269. Nothing — and
the cause is mine, not a prioritisation call by anyone.

Compare the two handoffs I filed within an hour of each other on 2026-08-30:

- `ahrt` (visibility-correlation plan) says *"Requesting adversarial Gate 1
  review from Codex before the governor's decision"* — and got seven rounds
  and sixteen findings.
- `kjjd` (this plan) is addressed to the governor, asks for a Gate 1 decision
  and a role-assumption confirmation, and **never requests a review.**

So the asymmetry I kept reporting to the governor — one plan absorbing seven
review rounds while the release work order had none — was produced by my own
omission, and I observed the gap repeatedly without diagnosing it. Correcting
that here.

## What I most want attacked

**The bounded maintenance path.** `nsne` asked for a route allowing routine
roadmap edits without a fresh full Gate 1 each time. That is a reduced-ceremony
route, and reduced-ceremony routes are where governance gets exploited — the
governor commissioned the whole gate-change discipline stream over exactly this
concern.

I modelled it on the existing sources/radar fast path rather than inventing
one, kept branch, MR, pipeline and Gate 2 fully intact, and wrote the
disqualifier list so ambiguity resolves toward full Gate 1. The plan carries a
negative test requiring an ineligible edit — moving an item between version
clusters — to be *shown* rejected rather than described as rejectable.

I authored that boundary and cannot judge whether it is tight enough. Assume it
is not.

## Second and third priorities

**Issue dispositions.** I verified #32 and #37 as shipped and #36 as partial
against the built CLI and crate tree, and recommend splitting #36 rather than
closing it whole. The verification commands are in the plan; please re-run them
rather than trusting the transcript.

**The projection screen.** The plan asserts that `docs/roadmap.md` becomes the
single governed owner with `agf/docs/site/roadmap.md` and GitLab milestones as
projections. If that ownership split is wrong, everything downstream inherits
the error.

## Known open question, not a finding

The plan cannot close the `release-v2.7.0-rc.2` disposition; `m2pr` abandoned
rc.1 and deliberately declined to infer rc.2's fate. That awaits the governor
and is flagged in the plan rather than resolved in it.

## Context worth having

This plan has had **no** review. The two plans that went through your review
this session each ended materially smaller and more honest than they started,
and in both cases the most valuable findings were things I had already looked
at and misjudged — a revert I recorded and then branched from anyway, a
reference-hygiene rule I wrote and broke eight minutes later. Please assume the
same is true here.
