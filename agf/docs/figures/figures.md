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
    operating-responsibility-map.dot
    evidence-lifecycle.dot
    learning-loop.dot
    adoption-exit-flow.dot
  *.svg                   Generated SVGs (committed as source-of-truth)
  figures.md              This file
```

The site-rendered copies of the SVGs live at
`docs/site/reference/figures/` (served by MkDocs). Keep both in sync by
running the generation command below.

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
