---
artifacts_owned: []
from: governor-reviewer
id: 20260826T045810Z-governor-reviewer-q1am
immutable: true
kind: decision-record
model: human
references-to:
- 20260825T204314Z-coder-tester-mjhk.md
- 20260826T044815Z-planner-architect-h6hg.md
requires-ack: false
timestamp: 2026-08-26T04:58:10Z
to: all
topic: work-order-rule-no-new-ceremony-direction-approved
---

# Work-order rule direction: extend existing machinery, no new ceremony

This record transcribes Jani's explicit in-session response to
planner-architect's `h6hg` critical response (PARTIAL) to `mjhk`:

> your point on the release shipment is solid and valid. no new
> ceremonies, but more improved usage of what we already have, e.g.,
> agov coord pending.

## Decision

The governor approves `h6hg`'s core architectural recommendation as the
direction for any eventual work-order rule: do not introduce a new object
type, lifecycle, or tracking surface parallel to what this project already
has. Extend the existing machinery instead — Gate 1 plans, coordination
records, and `agov coord pending` (or a periodic sweep of it) as the
staleness/traceability backbone.

This confirms the shape of the solution. It does not itself:

- finalize the work-order rule's text or resolve the three adversarial
  examples `h6hg` raised in detail (fictional-release placeholders,
  collision with `simplicity-rule.md`'s fast-track calibration, and the
  emergency-work-order reconciliation gap);
- approve the two specific plan-frontmatter fields or the
  completion-citation validator check `h6hg` proposed as the concrete
  mechanism — those still need their own Gate 1 plan once the rule text
  is settled;
- create or activate any new schema, tooling, or role authority.

## Boundaries

Not a Gate 1 or Gate 2 approval of any specific implementation. The
`release-oriented-work-order-rule-challenge` thread (`mjhk`) remains open
for coder-tester's own response to `h6hg`'s adversarial examples before
a final rule shape is proposed for Gate 1.
