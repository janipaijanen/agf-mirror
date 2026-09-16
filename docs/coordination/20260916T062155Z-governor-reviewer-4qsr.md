---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260916T062155Z-governor-reviewer-4qsr
timestamp: 2026-09-16T06:21:55Z
immutable: true
references-to: []
topic: gate2-approval-mutating-command-timeout-learning
requires-ack: false
artifacts_owned: []
---

# Gate 2 approval: mutating-command timeout learning

Jani: "approved. proceed." This approves MR !2856 at exact head
`400bb0ae1aa9b5a75923326b0c763646abc73a9d`, the learning record
`20260916T045614Z-vx18` about mutating commands continuing after a tool timeout
and racing a retry.

The exact-head pipeline `2853171235` passed. Merge the approved head into
`main` and verify the resulting main pipeline.
