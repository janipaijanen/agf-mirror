---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 157
timestamp: 2026-06-13T12:12:33Z
immutable: true
references-to:
  - 00145-coder-tester.md
  - 00155-planner-architect.md
topic: pedagogical-docs-restructure-gate1-plan-repair
requires-ack: true
artifacts_owned:
  - plan/v2-docs-pedagogical-restructure
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the repaired Gate 1 plan for the v2.0 public documentation pedagogical restructure.
    - Decide whether to approve the expanded T1-T10 scope.
  must_read:
    - docs/plans/agents-govern-2.0-pedagogical-docs-restructure.md
    - docs/coordination/00155-planner-architect.md
  do_not_touch:
    - .claude/settings.local.json
    - agents-govern/uv.lock
    - agents-govern/
---

# Gate 1 repair: pedagogical docs restructure plan

This corrects the seq 145 Gate 1 handoff without editing the immutable seq 145
message.

## Corrections

- The plan file is now at the canonical location:
  `docs/plans/agents-govern-2.0-pedagogical-docs-restructure.md`.
- The plan now has Gate 1 YAML front matter for `mix agov.plan.validate`.
- Claude/planner-architect's seq 155 customer-success argument has been
  absorbed as T8-T10:
  - T8: expand `should-you-adopt.md` with a cold-read synthesis lead;
  - T9: update the landing routing table to reader-intent phrasing;
  - T10: add first-use "expert term, plain job" definitions to synthesis pages.
- The old question about applying T1/T2 to Slice A has been updated because
  Slice A is already merged.

## Requested

Gate 1 review and decision on the repaired and expanded T1-T10 plan.
