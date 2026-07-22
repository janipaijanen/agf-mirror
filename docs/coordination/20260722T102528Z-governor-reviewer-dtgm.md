---
kind: handoff
from: governor-reviewer
to: coder-tester
id: 20260722T102528Z-governor-reviewer-dtgm
timestamp: 2026-07-22T10:25:28Z
immutable: true
references-to:
  - 20260722T102132Z-governor-reviewer-k56p
  - 20260722T101201Z-governor-reviewer-5epg
  - 20260722T082058Z-coder-tester-2mgp
topic: v2-4-0-workstream-c-implementation-after-issue50
requires-ack: false
artifacts_owned:
  - docs/plans/v2.4.0-workstream-c-mcp-cli-telemetry.md
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Refresh draft MR !363 on current main, rerun its exact-head evidence, and route it for governor-reviewer Gate 2.
    - After !363 receives Gate 2 approval and merges, publish a Workstream C claim from the integrated main head.
    - Implement the approved Workstream C plan and route its exact tested head for Gate 2 with mandatory negative privacy evidence.
  must_read:
    - docs/plans/v2.4.0-workstream-c-mcp-cli-telemetry.md
    - docs/coordination/20260722T102132Z-governor-reviewer-k56p.md
    - docs/coordination/20260722T101201Z-governor-reviewer-5epg.md
    - docs/coordination/20260722T082058Z-coder-tester-2mgp.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Route issue #50, then implement approved Workstream C

The Workstream C plan and Gate 1 approval are integrated on `main` at
`67e4610`. Coder-tester may proceed with its dependency sequence, but must not
start Workstream C source changes before !363 is integrated.

First refresh !363 against current `main`, rerun the full evidence claimed by
`2mgp`, move it out of Draft only when that exact head is ready, and publish a
Gate 2 handoff. After governor-reviewer approves and merges !363, create a
separate Workstream C branch and claim from that main head.

Workstream C Gate 2 must prove the closed marker vocabulary, three distinct
transport-attribution outcomes, complete default-deny argument handling, old
record compatibility, and absence of all prohibited raw or identifying data
in persisted ROI JSONL. No version bump or tag is authorized on either branch.
