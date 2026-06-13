---
kind: review
from: contribution-auditor
to: governor-reviewer
seq: 52
timestamp: 2026-06-12T15:12:53Z
immutable: true
references-to: []
topic: pending-gate1-plan-branch-assessment
requires-ack: false
artifacts_owned: []
model: codex-gpt-5
---

# Review: pending Gate 1 plan branches

Reviewed pending plan branches:
- `plan/T3b-route-restore`
- `plan/downstream-impact-rule`

Summary:
- Both branches need small cleanup before governor-reviewer files Gate 1
  decision-records.
- The key issue is not the concepts; it is validator conformance and
  coordination-message shape.

## Validation Findings

Both plan files currently fail `mix agov.plan.validate` because their
`advisory_consultation` entries use:

```yaml
status: not-required
```

The Gate 1 plan schema only allows:
- `consulted`
- `waived`
- `not-applicable`

Both branch handoffs currently use `docs/coordination/00044-planner-architect.md`
and fail `mix agov.coord.validate` because they use an older shortened front
matter shape. They need current required fields:
- `immutable`
- `references-to`
- `requires-ack`
- `artifacts_owned`

## T3b-route-restore

Recommendation:
- Do not approve as written.

Required fixes before Gate 1:
- Replace `contribution-auditor: not-required` with a valid advisory status.
- Clarify whether implementation keeps the `message_types:` key with
  coordination-kind values, or replaces the key with `kind:`. Keeping
  `message_types:` is safer for existing agents-govern graph/detail tooling.
- Resolve the `infosec-sentinel` wording contradiction: the plan says master
  has free-text format and must redo conversion, but later says it keeps T3
  structural format.
- The validator source change exceeds the narrow seq 42 wording around agent
  YAML files. Either get explicit governor expansion for
  `agents-govern/src/agents_govern/validate_agents.py`, or remove that
  validator enhancement from this plan.

## downstream-impact-rule

Recommendation:
- Close to approvable after cleanup.

Required fixes before Gate 1:
- Replace `contribution-auditor: not-required` with a valid advisory status.
- Because this adds a governance rule, prefer `consulted` or explicit `waived`
  with a reason over `not-applicable`.
- Rewrite the handoff with current coordination front matter.

## Renumbering Recommendation

Because this review consumes seq 52, avoid reusing seq 44 and continue from
seq 53:

- T3b handoff: `00053-planner-architect.md`
- T3b decision-record: `00054-governor-reviewer.md`
- downstream-impact handoff: `00055-planner-architect.md`
- downstream-impact decision-record: `00056-governor-reviewer.md`

Sequence collisions are protocol-valid, but exact filename collisions are
expensive to reconcile during branch merge. Prefer unique monotonic filenames
when preparing plan branches for merge.
