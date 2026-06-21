## Summary

- Changes the coordination map example DOT source to render on a white canvas.
- Regenerates both SVG copies used by the docs and reference figure pages.
- Keeps the embedded legend and existing coordination-map content unchanged.

## Verification

- `dot -Tsvg agf/docs/figures/src/coordination-map-example.dot -o agf/docs/figures/coordination-map-example.svg`
- `cp agf/docs/figures/coordination-map-example.svg agf/docs/site/reference/figures/coordination-map-example.svg`
- `rg -n 'fill="#ffffff"|fill="transparent"|Legend' agf/docs/figures/coordination-map-example.svg agf/docs/site/reference/figures/coordination-map-example.svg`
- `git diff --check`
- `/home/devi/.local/share/mise/installs/uv/0.11.20/uv-x86_64-unknown-linux-musl/uv run --project agents-govern --group docs mkdocs build -f agf/docs/build/mkdocs.yml --strict`

## Review Focus

- [ ] The public `coordination-map-example.svg` has an explicit white background.
- [ ] The DOT source, canonical SVG, and site SVG copy stay in sync.
- [ ] The embedded legend remains present and readable.
