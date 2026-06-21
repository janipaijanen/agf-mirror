## Summary

- Normalizes DOT figure readability defaults across the AGF docs figure set.
- Replaces the ASCII coordination lifecycle sketch with a generated DOT/SVG
  figure.
- Reflows the widest lifecycle/pipeline figures from left-to-right strips into
  top-to-bottom layouts that stay readable in MkDocs pages.
- Regenerates canonical and site SVG copies from DOT source.
- Documents the figure UX standard for future figure work.

## Visual UX Changes

- Node labels now use `11pt`; edge labels use `10pt`; graph/cluster labels use
  `12pt`.
- Ordinary lines and borders use `penwidth=1.4`.
- Standalone SVGs render on white backgrounds.
- `coordination-lifecycle.svg` replaces the monospace sequence sketch on the
  coordination protocol page.
- Wide figures were reduced:
  - `coordination-map-example`: `926pt` -> `680pt`
  - `evidence-lifecycle`: `843pt` -> `426pt`
  - `gate-pipeline`: `984pt` -> `648pt`
  - `governance-loop`: `732pt` -> `570pt`
  - `learning-loop`: `886pt` -> `605pt`

## Advisory Consultation

- Figure audit agent found the main UX failure was oversized wide canvases that
  made page-embedded text shrink, not random SVG font drift.
- Figure workflow agent recommended placing the authoring standard in
  `agf/docs/figures/figures.md`, summarizing usage rules in the public figure
  reference, and keeping future figure validation separate from MkDocs strict.

## Verification

- Regenerated all SVGs from `agf/docs/figures/src/*.dot`.
- Browser-rendered spot checks with Chromium screenshots for:
  - `gate-pipeline.svg`
  - `learning-loop.svg`
  - `coordination-map-example.svg`
  - `evidence-lifecycle.svg`
  - `coordination-lifecycle.svg`
- `rg -n 'font-size="([0-8]\\.00|9\\.00)"' agf/docs/site/reference/figures/*.svg`
- `git diff --check`
- `/home/devi/.local/share/mise/installs/uv/0.11.20/uv-x86_64-unknown-linux-musl/uv run --project agents-govern --group docs mkdocs build -f agf/docs/build/mkdocs.yml --strict`

## Review Focus

- [ ] Figures remain readable at MkDocs page scale without horizontal strip
      shrinkage.
- [ ] Top-to-bottom reflows are acceptable for lifecycle/pipeline figures.
- [ ] Embedded legends remain present and readable.
- [ ] The coordination lifecycle is acceptable as DOT/SVG instead of ASCII.
- [ ] The documented standard is clear enough to prevent future drift.
