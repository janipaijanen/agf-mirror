---
kind: handoff
from: governor-reviewer
to: coder-tester
id: 20260721T204707Z-governor-reviewer-r4hg
timestamp: 2026-07-21T20:47:07Z
model: gpt-5.6-sol
immutable: true
references-to:
- 20260721T203446Z-governor-reviewer-5dbm.md
topic: v2-4-0-workstream-b-implementation-start
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
  - Implement Workstream B within the approved unified release-provenance contract plan
  - Validate the implementation and publish a separate Gate 2 handoff
  must_read:
  - docs/plans/v2.4.0-workstream-b-provenance-contract.md
  - docs/coordination/20260721T203446Z-governor-reviewer-5dbm.md
  do_not_touch:
  - .claude/settings.local.json
  - tooling/elixir/mix.lock
---

# Handoff: begin approved Workstream B implementation

The Workstream B child plan and its durable Gate 1 decision record are visible
on `main` through merged MR !345 (`6cea382`). The previous hold condition is
therefore cleared.

The coder-tester may begin implementation of the unified release-provenance
contract, strictly within the approved child-plan scope, on a feature branch.
Run the planned validation and publish a separate Gate 2 handoff when the
implementation is ready for governor review.

This handoff authorizes implementation only. It is not Gate 2 approval and does
not authorize merging the implementation.

An MR note on !345 previously communicated the same instruction, but it did
not satisfy the project's durable coordination requirement. This record is the
canonical authorization and repairs that audit-trail gap.
