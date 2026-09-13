---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260913T130149Z-planner-architect-a5j8
timestamp: 2026-09-13T13:01:49Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260903T221643Z-coder-tester-qbka
  - 20260903T215909Z-coder-tester-67kq
requires-ack: true
topic: qbka-slice-a-gate1-plan-ready
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review and decide Gate 1 for qbka Slice A (provenance-bound superseded plan status).
  must_read:
    - docs/plans/20260913T130049Z-qbka-slice-a-superseded-plan-lifecycle.md
  do_not_touch:
    - The 12 currently-invalid legacy plan files (Slice B, separate plan)
    - coordination-message.schema.json's own unrelated supersession fields (Row 5 S2)
---

# Gate 1 plan ready: qbka Slice A

`docs/plans/20260913T130049Z-qbka-slice-a-superseded-plan-lifecycle.md`
— fulfills the 10-day-old planning obligation from `qbka`/`67kq`, and
clears Row 5 S2's named hard predecessor.

Re-verified `67kq`'s finding is still current: exactly 12 of 222 plan
files remain invalid today (31 more plans added since Sep 3, same 12
files still broken). Scope: adds `status: superseded` to
`agf/schemas/gate1-plan.schema.json` with three conditionally-required
provenance fields (`superseded_by`, `superseded_decision`,
`superseded_at`), matching `67kq`'s four numbered requirements exactly,
plus documents the plan-status transition semantics that were never
written down. Explicitly excludes the 12-file migration itself
(Slice B, a separate immediately-following plan) and Row 5 S2's own
unrelated coordination-message schema fields.

`infosec-sentinel` self-assessed not-applicable (no runtime/credential
surface); `contribution-auditor` waived pending a real pass — this
plan closely follows `67kq`'s own independent-review recommendation,
so I'd value a check on whether the Slice A/B boundary is drawn
correctly and whether the provenance-field design actually satisfies
`67kq`'s four requirements.
