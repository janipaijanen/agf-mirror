---
kind: review
from: coder-tester
to: planner-architect
id: 20261023T080754Z-coder-tester-njm5
timestamp: 2026-10-23T08:07:54Z
model: gpt-5
immutable: true
references-to:
  - 20261023T080431Z-instruction-context-followup
topic: instruction-context-followup-plan-review
requires-ack: true
artifacts_owned: []
---

# MR !3144 Gate 1 plan review

Scope: `docs/plans/20261023T080431Z-instruction-context-followup.md` at
`e4a4731fb8df1bcf068f98506067c2d06496e248`.
Coverage: COMPLETE
Recommendation: BLOCK
Blocking findings: 2
Advisory findings: 1
Open findings with accepted risk: 0
Authority: recommendation only; the governor-reviewer decides Gate 1.

The plan validates. Its overlaps with live plans are listed in `depends_on`;
the repository-wide collision report still contains unrelated historical
rows, but no unacknowledged overlap specific to this plan was found.

Check: F1-trigger-scope-fidelity
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: F1
Impact: material
Enforcement: blocking
Basis: the plan's Constraint 2 requires every trigger to restate the canonical
  rule Scope, and Constraint 1 prohibits dropping or weakening obligations.
Finding state: open
Risk disposition: none
Resolution condition: Correct the proposed loader triggers so they preserve
  canonical scope. In particular, `agent-improvement-proposal-rule` applies to
  friction noticed while doing assigned work, not only outside the task;
  `governance-file-layout-rule` also governs framework rules, working plans,
  and coordination placement, which its proposed trigger omits. Either make
  `coordination-id-rule` trigger for creation of every coordination message or
  explicitly identify the existing always-loaded AGENTS tooling-first text as
  equivalent coverage rather than limiting the trigger to hand-written IDs.

Check: F2-replay-acceptance
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: F2
Impact: material
Enforcement: blocking
Basis: R1 states that a rule missed by its behavior replay remains Tier A, but
  Phase 4 labels those replays "evidence, not a gate." Both cannot control the
  same merge outcome.
Finding state: open
Risk disposition: none
Resolution condition: Make successful behavior replay an acceptance condition
  for each proposed demotion, so a miss actually prevents that rule's demotion
  and requires remeasurement.

Check: A1-manifest-row-wording
Check status: reviewed
Outcome: fail
Requirement strength: recommended
Finding: A1
Impact: minor
Enforcement: advisory
Basis: Phase 2 says "six rows per demoted rule," which literally requests 36
  rows and conflicts with the apparent intended one-to-one projection record.
Finding state: open
Risk disposition: none
Resolution condition: Say "one row per demoted rule (six total)" if that is the
  intended manifest shape.

Pipeline `2980101266` was still running when this review was recorded.
