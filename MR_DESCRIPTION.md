## Summary

- Replaces the static `coordination-lifecycle.svg` figure with an embedded
  Mermaid `sequenceDiagram` on the coordination protocol page.
- Enables the MkDocs Mermaid custom fence so Mermaid blocks render as diagrams
  rather than highlighted code.
- Removes the lifecycle DOT/SVG files from the Graphviz figure inventory.
- Renumbers the remaining reference figure entries after removing that static
  figure.

## Verification

- `rg -n 'coordination-lifecycle\\.(svg|dot)' agf/docs MR_DESCRIPTION.md docs/coordination/20260621T181158Z-coder-tester-hbgq.md`
- `rg -n 'sequenceDiagram|```mermaid' agf/docs/site/docs/concepts/coordination-protocol.md`
- `rg -n 'class="mermaid"|sequenceDiagram' agf/site-html/docs/concepts/coordination-protocol/index.html`
- `git diff --check`
- `/home/devi/.local/share/mise/installs/uv/0.11.20/uv-x86_64-unknown-linux-musl/uv run --project agents-govern --group docs mkdocs build -f agf/docs/build/mkdocs.yml --strict`
- `agov coord validate docs/coordination/20260621T181158Z-coder-tester-hbgq.md`

## Review Focus

- [ ] The coordination lifecycle appears as Mermaid, not a direct SVG image.
- [ ] Mermaid renders as a diagram in MkDocs, not as a highlighted code block.
- [ ] The obsolete `/reference/figures/coordination-lifecycle.svg` asset is no
      longer documented as a figure.
- [ ] The remaining figure inventory numbering is coherent.
