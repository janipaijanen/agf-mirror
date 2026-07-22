---
kind: handoff
from: governor-reviewer
to: coder-tester
id: 20260722T094600Z-governor-reviewer-rbqj
timestamp: 2026-07-22T09:46:00Z
immutable: true
references-to:
  - 20260722T093814Z-governor-reviewer-rkqp
  - 20260722T092237Z-governor-reviewer-jgrh
  - 20260722T092927Z-governor-reviewer-p22r
  - 20260722T092516Z-contribution-auditor-bjkm
topic: v2-4-0-workstreams-a-d-implementation-handover
requires-ack: false
artifacts_owned:
  - docs/plans/v2.4.0-workstream-a-release-asset-names.md
  - docs/plans/v2.4.0-workstream-d-figure-system.md
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Publish separate claims for approved Workstreams A and D before editing.
    - Implement and route Workstream A to Gate 2 first.
    - Then begin Workstream D with its Class 2 nutshell-flyer placement MR and preserve rendered evidence.
  must_read:
    - docs/plans/v2.4.0-workstream-a-release-asset-names.md
    - docs/plans/v2.4.0-workstream-d-figure-system.md
    - docs/coordination/20260722T093814Z-governor-reviewer-rkqp.md
    - docs/coordination/20260722T092237Z-governor-reviewer-jgrh.md
    - docs/coordination/20260722T092516Z-contribution-auditor-bjkm.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Execute approved Workstreams A and D

Coder-tester should take the already-approved implementation work rather than
leave the handoffs idle. Publish distinct claims and branches so ownership and
Gate 2 evidence remain separable.

Complete Workstream A first because it is the smaller release-path change:
rename the public archive surfaces and every enumerated consumer while
preserving the real Rust target triple in provenance and technical metadata.
Route its exact tested head to governor-reviewer for Gate 2.

After A is routed, start Workstream D's Class 2 flyer-placement slice. Keep the
post-release public-experience Slice 3 out of D's branch, retain the D-first
ownership boundary, and attach built/rendered evidence required by the plan.
Do not bump a version or create a tag on either implementation branch.
