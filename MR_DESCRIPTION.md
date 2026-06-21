# docs: publish coordination map example

## What changed

- Added a rendered example communication map to the Coordination protocol page.
- Added nearby wording that the map is an example from AGF Foundry, not a
  required adopter topology.
- Added a DOT source file for the example map and committed generated SVGs.
- Added the example to the reference figure inventory.
- Embedded a legend inside the SVG so the figure explains node and edge
  semantics without relying on page prose.

## Verification

- `dot -Tsvg agf/docs/figures/src/coordination-map-example.dot`: clean.
- `mkdocs build --strict`: clean.
- `git diff --check`: clean.
- Generated SVG contains embedded `Legend` and `Edge label = kind (message count)` text.

## Review Focus

Please verify:

- [ ] The Coordination protocol page shows the rendered communication map near
      the `agov coord graph` instructions.
- [ ] The text clearly says this is an example and adopters may have richer
      roles and different communication patterns.
- [ ] The SVG legend is embedded in the figure.
- [ ] The figure inventory and DOT source path are consistent with the existing
      docs figure workflow.
