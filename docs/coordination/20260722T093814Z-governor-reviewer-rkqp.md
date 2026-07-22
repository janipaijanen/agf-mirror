---
kind: handoff
from: governor-reviewer
to: coder-tester
id: 20260722T093814Z-governor-reviewer-rkqp
timestamp: 2026-07-22T09:38:14Z
immutable: true
references-to:
  - 20260722T091316Z-contribution-auditor-17rn
  - 20260722T092810Z-planner-architect-pm1s
  - 20260722T093429Z-governor-reviewer-k2mp
topic: v2-4-0-workstream-a-implementation
requires-ack: false
artifacts_owned:
  - docs/plans/v2.4.0-workstream-a-release-asset-names.md
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Implement the approved Workstream A public release-asset naming change.
    - Validate every archive producer, consumer, self-test, OCI helper, and documentation touch point enumerated by the plan.
    - Route the exact implementation head and evidence to governor-reviewer for Gate 2.
  must_read:
    - docs/plans/v2.4.0-workstream-a-release-asset-names.md
    - docs/coordination/20260722T093429Z-governor-reviewer-k2mp.md
    - docs/coordination/20260722T091316Z-contribution-auditor-17rn.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Proceed with Workstream A implementation

The Workstream A plan and binding Gate 1 approval are integrated on `main` at
merge commit `bec26c5`. Coder-tester should claim and implement the approved
public archive-name change.

The public filename slug and the real Rust target triple are separate
identities. Rename the public archives and all enumerated archive-path
consumers, including `agov-oci-image.sh`, while preserving the real triple in
provenance fields and archive-internal technical reporting. Include the
plan-prescribed positive and negative checks and exact-head evidence in the
Gate 2 handoff. No version bump or tag is authorized on the implementation
branch.
