<!--
SPDX-License-Identifier: CC-BY-SA-4.0
-->
# docs/figures — figure build directory

This directory holds the DOT source files and generated SVG outputs for
the agents-govern v2.0 documentation figures.

## Layout

```
docs/figures/
  src/                    DOT source files (edit these)
    governance-loop.dot
    gate-pipeline.dot
    coordination-channel.dot
    coordination-lifecycle.dot
    operating-responsibility-map.dot
    evidence-lifecycle.dot
    learning-loop.dot
    adoption-exit-flow.dot
    coordination-map-example.dot
  *.svg                   Generated SVGs (committed as source-of-truth)
  figures.md              This file
```

The site-rendered copies of the SVGs live at
`docs/site/reference/figures/` (served by MkDocs). Keep both in sync by
running the generation command below.

Every figure must include an embedded legend in the generated image. Explain
color, shape, and line semantics inside the DOT source rather than relying on
page prose or captions.

## Figure UX standard

Figures are read inside documentation pages, not as isolated artboards. Design
for the size at which MkDocs will embed the SVG.

- DOT source is canonical. Do not hand-edit generated SVG output.
- Use a white graph background unless a specific output format requires
  transparency.
- Use `fontsize=12` for graph and cluster labels, `fontsize=11` for node
  labels, and `fontsize=10` for edge labels. Do not use text below `9pt`.
- Keep generated SVG width near or below `720pt` for Markdown embedding. Prefer
  top-to-bottom layout for walkthroughs, loops, and pipelines that would become
  wide, shallow strips in left-to-right layout.
- Keep line weights visible after scaling. Use `penwidth=1.4` for ordinary
  nodes and edges unless a figure has a specific reason to vary.
- Use the documented palette and never rely on colour alone. Shape, label, or
  line style must also carry the meaning.
- Every figure must include an embedded legend when colour, shape, or line style
  has semantics.
- Markdown alt text must name what the figure shows, not just say "diagram".

## Generating figures

```bash
cd agf/docs/figures
for f in src/*.dot; do
  name=$(basename "$f" .dot)
  dot -Tsvg "$f" -o "${name}.svg"
  cp "${name}.svg" ../site/reference/figures/
done
```

Requires Graphviz (`dot` version 2.x or later).

## See also

The site-facing reference pages live in `docs/site/reference/`:

- `palette.md` — colour palette and usage rules
- `figures.md` — figure inventory with embedded SVGs
