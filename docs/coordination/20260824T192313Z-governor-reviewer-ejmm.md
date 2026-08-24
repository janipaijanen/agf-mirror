---
kind: handoff
from: governor-reviewer
to: planner-architect
id: 20260824T192313Z-governor-reviewer-ejmm
timestamp: 2026-08-24T19:23:13Z
model: human-via-codex
immutable: true
references-to:
  - 20260824T163113Z-governor-reviewer-36rr.md
  - 20260824T185045Z-governor-reviewer-a86h.md
topic: convergence-abstraction-reset-rule-highest-priority
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - "Draft a Gate 1 plan for an AGF review-convergence and abstraction-reset rule, as the highest-priority planning item before new feature or backlog work."
    - "Define a mechanical correction-loop trigger, a mandatory abstraction-reset decision, a materiality boundary for blockers, and a bounded final-verification sequence."
  must_read:
    - 20260824T163113Z-governor-reviewer-36rr.md
    - 20260824T185045Z-governor-reviewer-a86h.md
  do_not_touch:
    - .claude/settings.local.json
---

# Highest priority: propose a convergence and abstraction-reset rule

The governor makes this the highest-priority AGF planning item before new
feature, TODO, or backlog work. Finish any currently executing atomic
publication or exact-head verification safely, then plan this rule before
claiming another ordinary workstream. Active security incidents and actions
already stopped at an irreversible-operation boundary remain governed by
their existing emergency and safety rules.

This handoff authorizes planning only. It does not approve Gate 1, begin rule
implementation, or modify current role authority.

## Problem demonstrated

MR !1847 required repeated correction attempts because each patch refined a
permissive, bespoke symlink-resolution algorithm while another shape in the
same bug class remained. Review was correctly finding material containment
bypasses, but the workflow lacked a point at which agents had to stop patching
examples and replace the underlying abstraction. The later governor decision
to classify every uncanonicalizable symlink component as `Invalid` reduced the
state space and eliminated the dangling-target bug class with an explicit,
conservative tradeoff.

The opposite failure is also possible: a targeted reviewer can keep adding
theoretical completeness conditions that do not materially affect an approved
acceptance criterion. Both failure modes consume review sessions, paid CI,
coordination records, and human attention while delaying delivery.

## Rule outcome to plan

The plan should propose one AGF rule with two coupled controls.

### 1. Mandatory abstraction reset

When targeted verification rejects two consecutive correction candidates for
the same named invariant or bug class, agents must stop case-by-case patching.
Before another correction, the owner must present an abstraction-reset
disposition that:

- identifies the shared cause and the state space creating the repeated
  failures;
- compares simplification, fail-closed restriction, use of a proven primitive,
  explicit risk acceptance, and deferral;
- prefers the smallest conservative invariant that satisfies the approved
  requirement;
- documents intentional false negatives, compatibility effects, and operator
  impact; and
- obtains governor disposition when the choice changes accepted behavior or
  risk.

Fail-closed behavior is a preferred option where it is safe and usable, not a
universal answer. A simpler rule must not silently discard a required use case.

### 2. Materiality boundary for targeted review

A targeted verification may block only on concrete evidence that the exact
candidate still violates:

- the named mandatory acceptance criterion;
- a security, containment, privacy, or data-integrity boundary;
- an explicit compatibility or operator-safety promise; or
- a mechanically required governed artifact.

Review observations outside that boundary, speculative completeness concerns,
style refinements, and edge cases without a plausible material consequence
must be recorded as advisory evidence or backlog candidates. They do not
expand the correction scope or prevent the already-approved delivery.

## Bounded convergence sequence

The plan should make the resulting sequence explicit and auditable:

1. one consolidated correction for all findings in the approved review scope;
2. one targeted verification of those findings;
3. after two same-invariant correction failures, mandatory abstraction reset;
4. one redesigned correction and one targeted verification; and
5. if a material failure remains, return once to the governor for risk
   acceptance, re-scope, or stop—not another automatic review loop.

The plan must define how tools and coordination records count correction
attempts without treating rebases, publication-only commits, CI retries, or
duplicate reviews as new rounds.

## Rationale

This rule should improve all primary perspectives:

- users receive working behavior sooner and clearer descriptions of rejected
  inputs;
- implementers solve the invariant instead of accumulating special cases;
- reviewers retain authority to stop material defects without becoming an
  unbounded source of new scope;
- sysadmins gain predictable fail-closed behavior and fewer ambiguous states;
- project management gets a measurable convergence budget and escalation
  point;
- the governor receives a small, explicit risk decision instead of a long
  diary of near-identical rounds; and
- CI resources are spent on acceptance evidence rather than repeated nuance.

The proposed rule should preserve the full immutable audit trail, but the
audit diary must not itself become a reason to repeat work.

## Required planning consultations

The Gate 1 plan must consult `contribution-auditor` on blocker materiality and
scope containment, and `infosec-sentinel` on whether fail-closed guidance or
risk acceptance could weaken an existing security boundary. Consult
`build-engineer` on deterministic round-count evidence and duplicate-CI cost.
