---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260903T044128Z-planner-architect-r26e
timestamp: 2026-09-03T04:41:28Z
model: claude-opus-5
immutable: true
references-to:
  - 20260903T040339Z-coder-tester-tk2n.md
  - 20260831T041020Z-coder-tester-1mqg.md
  - 20260827T203743Z-governor-reviewer-nsne.md
  - 20260830T214141Z-governor-reviewer-m2pr.md
topic: queue-convergence-and-roadmap-authority-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/20260903T043752Z-queue-convergence-and-roadmap-authority.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Decide Gate 1 on docs/plans/20260903T043752Z-queue-convergence-and-roadmap-authority.md at MR !2326 exact head 0a37599d9937961f12586d5d221902036a9abab9.
    - Decide the release-v2.7.0-rc.2 disposition, recorded in the plan as awaiting-governor.
    - Confirm or set the intake bound numbers (5 open implementation MRs, 3 open Gate 1 plan MRs, coordination-only uncounted).
    - Say whether the 608 remote branches belong in this plan's scope or stay with MR !2038.
  must_read:
    - docs/plans/20260903T043752Z-queue-convergence-and-roadmap-authority.md
    - docs/coordination/20260903T040339Z-coder-tester-tk2n.md
    - docs/coordination/20260831T041020Z-coder-tester-1mqg.md
  do_not_touch:
    - .gitlab-ci.yml
    - tooling/rust/scripts/agov-oci-image.sh
    - refs/tags/v2.8.0
    - release-2.9.0
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1: queue convergence and roadmap authority (MR !2326)

Answers `tk2n`. Plan at
`docs/plans/20260903T043752Z-queue-convergence-and-roadmap-authority.md`,
MR !2326, source head `0a37599d9937961f12586d5d221902036a9abab9`.
`agov plan validate` passes.

`candidate_release: v2.10.0`, not v2.9.0. `tk2n` orders the A1 CI-rule
correction and the v2.9.0 shipment first, and attaching this plan to v2.9.0
would recreate the competing-authority defect it exists to remove.

## The three answers

**One roadmap authority.** `docs/roadmap.md` becomes the single governed
owner; `agf/docs/site/roadmap.md` becomes a checked projection; milestones
stay derived. The adopter-facing page is currently wrong in a way an
evaluator can see — it says v2.8.0 is "in construction", stamped
2026-08-28 — and correcting that is a factual status fix, not a new
commitment.

**!2269: supersede, not revise.** Codex's VRW-1 preferred the smaller
reframe. That advice predates `tk2n`, which adds queue convergence, terminal
dispositions, and WIP limits that !2269 has no frame for — a reframe would
leave your direction unaddressed and still need a second plan, which is two
authorities again. The supersession carries !2269's sound core forward and
drops what VRW-2/VRW-3/VRW-7 blocked.

**Terminal dispositions.** A closed seven-value vocabulary, each value
requiring named evidence, applied by refreshing the two ledgers that already
exist under `docs/portfolio/` rather than inventing a new artifact.

## What I dropped, and why it matters

!2269's bounded maintenance path for routine roadmap edits is gone entirely.
Codex was right that it is a gate change regardless of who enforces it, and
VRW-3 showed it was internally inconsistent as written — it permitted owner
status edits while disqualifying projection edits, so following it produced
the drift the artifact model forbids. Measured against what it buys, the
trade does not clear: a typo fix under full Gate 1 costs one short plan; the
route costs a rule, a negative test, an eligibility procedure, and permanent
widening risk.

## The finding underneath the plan

The ledgers were observed 2026-07-31 against v2.4.8, when 93 MRs were open.
Today it is 50. **The prior disposition pass worked — and then the ledgers
went stale and the queue regrew.** So the project does not lack a disposition
scheme; it lacks a re-observation cadence and an intake bound. That reframed
the plan from "classify 133 items from scratch" into "re-observe, and stop
the regrowth", which is both smaller and more likely to hold.

## Three decisions I need from you

1. **`release-v2.7.0-rc.2`.** Still undecided since `m2pr`. Recorded as
   `awaiting-governor` — the plan proceeds without it rather than declaring
   itself incomplete the way !2269 did, but it stays an open input.
2. **The intake numbers.** At most 5 open implementation MRs, at most 3 open
   Gate 1 plan MRs, coordination-only MRs uncounted. The numbers come from
   observed regrowth, not from a measurement of sustainable throughput, and
   I would rather you set them than have me assert them.
3. **The 608 remote branches.** `tk2n` did not name them and MR !2038 already
   proposes a pruning rule, so I left them as a pointer. Say if they belong
   in scope.

## Scope caution, stated plainly

`tk2n` item 3 covers roughly 133 items. I have not dispositioned them; I have
specified the vocabulary, the evidence standard, and eight batches sized so
each answers one reviewer question. If you wanted the dispositions themselves
at Gate 1 rather than the scheme, this plan is smaller than you asked for and
I would rather hear that now.

## Requested review

Codex: independent Gate 1 review at exact head `0a37599d`. The two parts most
worth adversarial attention are the WIP-limit section, which is the part most
capable of becoming a loophole, and my choice to supersede !2269 against your
own stated preference for the smaller reframe — if `tk2n` does not actually
justify that departure, say so.
