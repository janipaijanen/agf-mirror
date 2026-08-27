---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260824T204903Z-contribution-auditor-8prd
timestamp: 2026-08-24T20:49:03Z
model: gpt-5.6-sol
immutable: true
references-to:
  - 20260824T203312Z-contribution-auditor-61gp.md
  - 20260824T204345Z-contribution-auditor-h4q7.md
topic: minimum-sufficient-value-rule
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - "Create one minimal Gate 1 plan to consolidate the governor's wording, implementation, and delivery directives into the existing simplicity rule."
    - "Use one rule edit, one acceptance matrix, and one consultation round; do not create separate rules, schemas, commands, or plans for the three clauses."
    - "Return either CLEAR on the wording below or one consolidated revision with a material reason; do not start phrase-by-phrase review rounds."
  must_read:
    - agf/rules/workflow/simplicity-rule.md
    - docs/coordination/20260824T203312Z-contribution-auditor-61gp.md
    - docs/coordination/20260824T204345Z-contribution-auditor-h4q7.md
  do_not_touch:
    - docs/plans/20260824T195950Z-review-convergence-abstraction-reset.md
    - agf/learnings/foundry.yaml
    - .claude/settings.local.json
---

# Consolidate three simplicity directives into one minimum-sufficient-value rule

The governor supplied three aligned directives: say things simply, implement
simply, and deliver quality simply. Treat them as one rule, not three new
governance surfaces.

## Proposed consolidated rule

> Say, build, and deliver the smallest complete solution that meets the
> approved requirements and quality standard. Remove unnecessary wording,
> scope, states, abstractions, options, process, and delay. Do not remove
> meaning, correctness, safety, evidence, maintainability, uncertainty, or
> required user value.

Operational clauses:

1. **Communication:** Use the shortest clear wording that preserves necessary
   meaning. Remove jargon, repetition, and decorative detail. Do not omit
   material facts, constraints, uncertainty, or required actions.
2. **Implementation:** Implement the simplest solution that fully meets the
   approved requirements. Remove unnecessary states, abstractions, options,
   and special cases. Do not simplify away correctness, safety, evidence, or
   required behavior.
3. **Delivery:** Deliver the smallest complete solution that meets the approved
   quality requirements. Remove unnecessary scope, process, abstraction, and
   delay. Do not remove correctness, safety, evidence, maintainability, or
   required user value.

## Smallest governed delivery path

Extend `agf/rules/workflow/simplicity-rule.md`; do not add another rule file.
The plan should specify observable checks for the three clauses and classify
the downstream impact honestly, because converting current preferences into
MUST-level behavior restricts previously permitted work. Keep enforcement in
human/agent review unless an existing validator already proves a criterion;
do not invent a prose-quality linter.

MR !2003 is already green and awaiting its own Gate 1 decision. Do not mutate
that exact head or attach this broader rule change to it. MR !2008 is the
learning evidence and likewise needs no wording mutation. This is one narrow
follow-on plan whose purpose is to turn the lesson into downstream value
without reopening either completed review surface.
