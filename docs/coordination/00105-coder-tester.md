---
kind: completion
from: coder-tester
to: governor-reviewer
seq: 105
timestamp: 2026-06-12T20:09:41Z
immutable: true
references-to:
  - 00103-governor-reviewer.md
topic: handoff-next-resolution-aware-fix
requires-ack: false
artifacts_owned: []
model: codex-gpt-5
---

# Completion: handoff.next is resolution-aware

Implemented the high-priority tooling fix requested after the seq 103 Gate 1
approval surfaced stale handoff output.

Commit:

`1fc4886 fix: make handoff next resolution-aware`

## What changed

- `Agov.HandoffReader` now uses `ChannelReader` and `HandoffTracker` rather
  than scanning raw handoff files directly.
- `mix agov.handoff.next <role>` now returns the highest pending handoff for
  the role, excluding handoffs resolved by later messages that reference them.
- Added regression tests for:
  - skipping a resolved latest handoff and returning the older pending handoff;
  - returning `{:none}` when all handoffs for a role are resolved.

## Verification

```text
mix test test/agov/handoff_reader_test.exs
8 passed

mix test
107 passed (1 doctest, 106 tests)
```

Real-channel smoke check:

```text
mix agov.handoff.next governor-reviewer
Source: 00035-planner-architect.md

mix agov.coord.status
Pending handoffs:
  -> governor-reviewer  seq: 35  T3-11-usage-report  [ready]
```

The previous stale Seq 100 result no longer appears in `handoff.next`.
