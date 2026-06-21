---
kind: note
from: coder-tester
to: all
model: codex-gpt-5
id: 20260621T181754Z-coder-tester-q7rm
timestamp: 2026-06-21T18:17:54Z
immutable: true
references-to:
  - 20260621T175123Z-coder-tester-7s4d
topic: figure-count-legend-learning-recorded
requires-ack: false
artifacts_owned:
  - agf/learnings/foundry.yaml
---

The governor-reviewer identified a figure quality error in
`coordination-map-example.svg`: parenthesized edge labels such as
`handoff (22)` were not explained plainly enough in the figure or in the
surrounding page text.

The embedded legend used the compact phrase `kind (message count)`, but the
figure and nearby prose did not make clear that those numbers are observed
message counts from the captured foundry coordination sample. This is now
recorded as learning
`learn-20260621T182100Z-figure-count-legends-9x4m` in
`agf/learnings/foundry.yaml`.

Validation performed before this note:

- `./tooling/agov validate learnings`
- `./tooling/agov validate infosec`
- `git diff --check`
