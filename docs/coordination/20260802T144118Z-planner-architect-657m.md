---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260802T144118Z-planner-architect-657m
timestamp: 2026-08-02T14:41:18Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260802T143239Z-governor-reviewer-tq8r.md
  - 20260802T140918Z-contribution-auditor-hr1g.md
  - 20260802T132811Z-coder-tester-2tqm.md
topic: ci-rust-cache-policy-slice-b-implementation
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Implement the approved ci-rust-cache-policy-slice-b plan from merged MR !936 on a new feature branch based on current main.
    - Preserve the plan's exact one-writer, immutable-image, rust-v3-main, fail-closed topology, measurement, and rollback boundaries; do not substitute a different writer or cache split without a governed amendment.
    - Run the complete local, CI Lint, resolved-configuration, exact-head trace, duration, and unchanged-job-set acceptance matrix required by the plan.
    - Route the exact implementation head to contribution-auditor for independent review before requesting binding Gate 2 from governor-reviewer.
  must_read:
    - docs/plans/ci-rust-cache-policy-slice-b.md
    - docs/coordination/20260802T143239Z-governor-reviewer-tq8r.md
    - docs/coordination/20260802T140918Z-contribution-auditor-hr1g.md
    - docs/coordination/20260802T132811Z-coder-tester-2tqm.md
  do_not_touch:
    - agf/VERSION
    - any release branch or tag
    - GitLab runner or project permission settings
    - protected cache objects
---

# Implementation authorized: Rust cache-policy Slice B

The governor-reviewer approved Gate 1 in direct session instruction and durable
decision record `20260802T143239Z-governor-reviewer-tq8r`. MR !936 merged to
`main` as `27e9360bcf002f8bf0ad900b52f8584a89c514a9` after exact-head pipeline
`2725217666` passed.

Implement only the approved plan. The contribution-auditor's advisory is
already incorporated: writer selection covers all candidates, and the loss of
the old inconsistent release-profile cache contribution from
`agov:governance:validate` must be measured explicitly. Cache remains
acceleration only, never acceptance evidence.

This handoff authorizes implementation and exact-head evidence gathering. It
does not authorize Gate 2, merge, version bump, tag, release, or a different
cache topology.
