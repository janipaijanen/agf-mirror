---
kind: note
from: coder-tester
to: all
id: 20260830T100622Z-coder-tester-tnsh
timestamp: 2026-08-30T10:06:22Z
model: claude-opus-5
immutable: true
references-to:
  - 20260811T194155Z-planner-architect-jgbq.md
topic: stranded-coordination-record-published-after-19-days
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# `jgbq` published 19 days late; the plan it names stays at Gate 1

`20260811T194155Z-planner-architect-jgbq.md` is published to `main` alongside
this note. It was authored 2026-08-11 and has been invisible on `main` ever
since.

## What it was, and the irony

`jgbq` queues a joint strategic-comparison task filed at the human governor's
direct instruction: build a scoring rubric, compare the eight `Radar:` issues
(#60, #61, #62, #77, #98, #99, #100, #104) plus anything not yet a formal
issue, across three specified angles — adopter experience, AGF's own
development efficiency, and a coordinator/monitor agent over other agents —
then produce a shortlist and an explicit release mapping. The governor flagged
the scoring question as genuinely open.

Its own closing line is:

> this is a record so the task isn't lost

It was then lost for nineteen days.

## Why it was invisible, which is not what the scan first suggested

`agov coord visibility --base-branch main` reported `bridge: missing`, which
reads like a record pushed with no merge request. That is not what happened.
MR **!1451** was opened 2026-08-11, its pipeline passed, and it has zero
notes and no activity since. The record was correctly authored, correctly
pushed, and correctly proposed for merge. It simply never merged, and nothing
surfaced that among 59 open merge requests.

So the failure is not authoring discipline. It is that a green, uncontroversial
merge request can sit indefinitely with no mechanism noticing, and coordination
records inherit that latency even though
`coordination-publication-rule.md` exists specifically to let them publish
ahead of the work they describe.

## What is published, and what is not

Published: `jgbq` only, as a pure addition on its own branch.

**Not** published: `docs/plans/20260811T194007Z-strategic-candidate-comparison-and-release-mapping.md`,
which is `status: pending-gate1` and remains in MR !1451 awaiting Gate 1.
Merging !1451 wholesale would have placed an unapproved plan on `main` — the
exact defect of MR !2202, which the governor ordered reverted and which MR
!2206 is still open to undo. Splitting the record from the plan is what lets
the visibility failure be fixed without repeating that one.

## Scope this does not change

Publishing does not start the queued task. `jgbq` says plainly it is "not to
be started now" and queues behind other work; nineteen days of governor
priorities have passed since, so whether it is still wanted is a governor
question, not an inference from this note. Nothing here reprioritises it.

## Root cause is broader than this branch

One stranded record is a symptom of 59 open merge requests with no sweep.
Issue #110 and its plan MR !1617 (open-MR hygiene disposition) already own
that problem; this note is filed as concrete evidence for it rather than as a
competing proposal. A single visibility scan found this one — a full sweep
would likely find more.
