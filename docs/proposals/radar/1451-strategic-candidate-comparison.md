<!-- Radar carrier. Retained verbatim from MR !1451 (source branch plan/strategic-comparison-queued, head 947be2ea), file docs/plans/20260811T194007Z-strategic-candidate-comparison-and-release-mapping.md. Front-matter status was pending-gate1 and is not an approval.
     Non-authoritative: parked on the radar, grants no Gate 1, rule, template, validator or implementation authority. -->
---
id: strategic-candidate-comparison-and-release-mapping
status: pending-gate1
topic: deferred-strategic-comparison-scoring-and-release-mapping
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Filed jointly for planner-architect and contribution-auditor
      (Codex) to work through together once the current backlog triage
      is done — not a request for review of this stub, but a queued
      joint task. Notified via coordination handoff alongside this file.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      No code, fixture, CI, or security-relevant content is proposed —
      this is a deferred planning/scoring exercise definition only.
  - role: fact-checker
    status: not-applicable
    reason: >-
      Nothing is asserted as fact yet; the scoring and candidate
      dispositions this document defines are future work, not claims
      made here.
  - role: web-ux-reviewer
    status: not-applicable
    reason: >-
      No UI, template, or Pages content is touched.
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Deferred: strategic candidate comparison, scoring, and release mapping

## Do not start yet

This is explicitly **not** next up. Per the human Governor's direct
instruction, it queues behind the backlog triage
(`docs/plans/20260811T191249Z-post-dd-roadmap-and-backlog-triage.md`,
task 1 — currently blocked pending revision per contribution-auditor
review `20260811T192217Z-contribution-auditor-hmgq.md`) and whatever
bounded near-term work comes out of that. This document exists so the
task isn't lost, not to start it now.

## Why this is a separate document from the backlog triage

The Governor drew an explicit line: the triage work is **housekeeping**
(rake dead leaves, classify what's live vs. dead vs. blocked) — it is
not a roadmap and should not be dressed up as one. This document is the
actual strategic exercise: comparing candidate directions and deciding
what AGF should become next, not just what's currently open. The two
should stay separate documents so triage doesn't quietly turn into
unexamined roadmap decisions, and vice versa.

## What this task is

Planner-architect and contribution-auditor jointly compare the
project's "draft-start"/synopsis-stage strategic candidates — not
maintenance backlog — from three angles the Governor specified:

1. **DX (downstream/adopter experience).** How does each candidate
   change what an adopter bringing AGF into their own project actually
   experiences — onboarding friction, clarity, day-to-day usefulness?
2. **AGF development itself.** Token/cost savings and quality
   improvement for the framework's *own* AI-assisted development
   process, not the shipped product.
3. **OpenClaw (or a similar coordinator agent) as a supervisory/
   monitoring layer over other agents** — up to and including
   simulating human Gate 1/Gate 2 decision-making. Directly relevant to
   issue #104 and to the delegated-approval model already specified in
   `agf/rules/workflow/deputy-governor-rule.md` (currently shipped with
   its delegable-categories list empty, i.e. delegation is off by
   default — this comparison is where a real proposal to fill that in,
   if warranted, would come from).

## Candidate inventory (starting point, not exhaustive)

The existing "Radar:" issues are the clearest examples of strategic
(not maintenance) candidates:

- #60 Reviewer-attention economics and execution-backed Gate 2
- #61 From audit trail to bounded assurance argument
- #62 Modality-changing context compression and evidence fidelity
- #77 Cross-repository scientific and promotional intake
- #98 Hermes issue-to-PR autonomous loop, verified worker/orchestrator
  separation
- #99 Optional live agent transport versus repository-native audit
  authority
- #100 Expectation signals for material caveats and blind spots
- #104 Isolated OpenClaw-governor spin-off and tiered runtime authority

Each agent should also surface any other draft-start or synopsis-stage
material it is independently aware of that isn't yet a formal issue —
the list above is a floor, not the full candidate set.

## What "done" looks like

1. **A proposed scoring rubric, stated explicitly before anything is
   scored against it.** The Governor's own framing flagged this as
   unresolved ("scored against what?") — planner-architect and
   contribution-auditor propose the actual criteria jointly; this
   document does not assume one already exists.
2. Each candidate scored against that rubric, across all three angles
   above (a candidate can score well on one angle and poorly on
   another — the rubric should surface that, not collapse it into one
   number prematurely).
3. A proposed shortlist of next items to implement, with reasoning tied
   back to the scores, not asserted independently of them.
4. An explicit release mapping — which candidate(s), if any, target
   which release, stated as a real proposal (e.g. "v2.5.1 carries X",
   "v2.6 carries Y", "Z stays parked") rather than left implicit.

## Process constraint (carried forward from the DD-failure lesson)

Per `docs/coordination/20260811T192217Z-contribution-auditor-hmgq.md`
(Question 2): this must stay bounded. It is a comparison-and-scoring
exercise, not a new multi-round Gate 1 saga in itself. Whatever rubric
gets proposed should be usable in one pass — if it needs its own
governance apparatus to apply, that is itself a sign the rubric is
wrong, not a reason to build the apparatus.

## Trigger to start

Planner-architect or contribution-auditor picks this up once the
backlog triage's task 1 is complete and its resulting near-term bounded
work (if any) is either done or clearly and stably in progress. No
further Governor sign-off is needed to *begin* this comparison work
itself — the outputs (rubric, scores, shortlist, release mapping) are
what require Governor review before anything is actually implemented or
released.
