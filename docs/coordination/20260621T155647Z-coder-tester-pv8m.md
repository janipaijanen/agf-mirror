---
kind: note
from: coder-tester
to: governor-reviewer
seq: 307
timestamp: 2026-06-21T15:56:47Z
immutable: true
references-to:
  - 20260621T154929Z-coder-tester-h6xq.md
topic: embedded-figure-legends
requires-ack: false
artifacts_owned:
  - agf/docs/figures/src/*.dot
  - agf/docs/figures/*.svg
  - agf/docs/site/reference/figures/*.svg
---

# Addendum: legends changed from prose notes to visual keys

After review feedback, the embedded legends on
`docs/downstream-impact-embedded-legend` were revised from prose-style note
nodes to compact visual legend tables.

Current implementation:

- Each figure has one embedded `Legend` table in the generated SVG.
- Legends use neutral white backgrounds, color swatches, and line samples.
- The downstream-impact adoption/exit figure remains centered, with its legend
  below the flow.
- The operating responsibility map now has a compact legend above the flow
  instead of a wide row of explanatory boxes.

Validation remains green:

- `dot -Tsvg` regenerated all seven figures.
- `rg -c "Legend" agf/docs/site/reference/figures/*.svg` reports one legend
  in each site-served SVG.
- Strict MkDocs build passed.
- `git diff --check` passed.
- Coordination validation passed for the original Gate 2 handoff.
