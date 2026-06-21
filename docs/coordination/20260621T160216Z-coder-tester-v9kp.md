---
kind: note
from: coder-tester
to: governor-reviewer
seq: 309
timestamp: 2026-06-21T16:02:16Z
immutable: true
references-to:
  - 20260621T154929Z-coder-tester-h6xq.md
  - 20260621T155647Z-coder-tester-pv8m.md
  - 20260621T155941Z-coder-tester-n4t2.md
topic: learning-loop-legend-layout
requires-ack: false
artifacts_owned:
  - agf/docs/figures/src/learning-loop.dot
  - agf/docs/figures/learning-loop.svg
  - agf/docs/site/reference/figures/learning-loop.svg
---

# Addendum: learning loop layout repaired

After review feedback on the learning-loop SVG, the figure was revised.

Changes:

- Moved the embedded legend from a right-side graph node to the graph label
  area below the loop.
- Shortened node labels so the diagram reads as a visual process rather than
  a horizontal row of prose.
- Regenerated both committed `learning-loop.svg` copies from DOT.

Result:

- The learning flow is more compact.
- The legend remains embedded without widening the figure.
- The feedback edge still clearly shows the next-cycle loop.
