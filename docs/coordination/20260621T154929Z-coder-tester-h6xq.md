---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 306
timestamp: 2026-06-21T15:49:29Z
immutable: true
references-to:
  - 00305-planner-architect.md
topic: embedded-figure-legends
requires-ack: true
artifacts_owned:
  - agf/docs/figures/src/adoption-exit-flow.dot
  - agf/docs/figures/src/coordination-channel.dot
  - agf/docs/figures/src/evidence-lifecycle.dot
  - agf/docs/figures/src/gate-pipeline.dot
  - agf/docs/figures/src/governance-loop.dot
  - agf/docs/figures/src/learning-loop.dot
  - agf/docs/figures/src/operating-responsibility-map.dot
  - agf/docs/figures/*.svg
  - agf/docs/site/reference/figures.md
  - agf/docs/site/reference/figures/*.svg
  - agf/docs/figures/figures.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Gate 2 review of docs/downstream-impact-embedded-legend.
  must_read:
    - agf/docs/site/docs/adopt/downstream-impact.md
    - agf/docs/figures/src/adoption-exit-flow.dot
    - agf/docs/site/reference/figures/adoption-exit-flow.svg
    - agf/docs/site/reference/figures.md
  do_not_touch:
    - agents-govern/
---

# Gate 2 handoff: embedded figure legends

Branch: `docs/downstream-impact-embedded-legend`

This branch addresses the downstream-impact page figure lacking a legend and
applies the same embedded-legend requirement to every committed AGF site
figure.

Changes:

- Added embedded legend notes to all seven DOT figure sources.
- Regenerated both committed SVG copies for every figure:
  `agf/docs/figures/*.svg` and `agf/docs/site/reference/figures/*.svg`.
- Updated the figure reference and figure build README to state that every
  figure must carry an embedded legend for color, shape, and line semantics.

Validation:

- `dot -Tsvg` regenerated all seven figures without errors.
- `rg -c "Legend" agf/docs/site/reference/figures/*.svg` reports one legend
  in each site-served SVG.
- `uv run --project agents-govern --group docs mkdocs build -f agf/docs/build/mkdocs.yml --strict` passed.
- `git diff --check` passed.
- Rendered `agf/docs/figures/src/adoption-exit-flow.dot` to PNG for visual
  inspection; the legend is readable and embedded inside the figure.

Review focus:

- [ ] The downstream-impact page's adoption/exit figure now has a readable
  embedded legend.
- [ ] Other AGF site figures also include embedded legends and remain legible.
- [ ] The new rule text is clear enough to prevent future caption-only legend
  regressions.
