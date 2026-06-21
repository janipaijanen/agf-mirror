---
kind: handoff
from: coder-tester
to: governor-reviewer
model: codex-gpt-5
id: 20260621T173327Z-coder-tester-ssb2
timestamp: 2026-06-21T17:33:27Z
immutable: true
references-to: []
topic: coordination-map-example-gate2
requires-ack: false
artifacts_owned:
  - MR_DESCRIPTION.md
  - docs/coordination/20260621T173327Z-coder-tester-ssb2.md
  - agf/docs/figures/src/coordination-map-example.dot
  - agf/docs/figures/coordination-map-example.svg
  - agf/docs/site/reference/figures/coordination-map-example.svg
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review docs/coord-map-example branch for Gate 2 approval.
    - Merge into main if approved so Pages publishes the example communication map.
  must_read:
    - MR_DESCRIPTION.md
    - agf/docs/site/docs/concepts/coordination-protocol.md
    - agf/docs/site/reference/figures/coordination-map-example.svg
  do_not_touch:
    - docs/coordination/
---

# Gate 2 handoff: coordination map example

Branch: `docs/coord-map-example`
MR: pending creation

## What changed

- Added a rendered SVG example map to the Coordination protocol page.
- Added nearby caveat text: the map is an AGF Foundry example, not a required
  topology for adopters.
- Added DOT source and generated SVG copies for the figure workflow.
- Added the map to the reference figure inventory.
- Embedded a legend inside the generated SVG.

## Verification

- `dot -Tsvg agf/docs/figures/src/coordination-map-example.dot`: clean.
- `mkdocs build --strict`: clean.
- `git diff --check`: clean.
- Generated SVG includes `Legend` and `Edge label = kind (message count)`.

## Review focus

- Confirm the map is visible on the Coordination protocol page.
- Confirm the adopter caveat is clear and close to the figure.
- Confirm the embedded legend explains nodes and edge labels.
