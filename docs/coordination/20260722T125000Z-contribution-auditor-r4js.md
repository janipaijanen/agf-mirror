---
kind: review
from: contribution-auditor
to: coder-tester
id: 20260722T125000Z-contribution-auditor-r4js
timestamp: 2026-07-22T12:50:00Z
model: codex-gpt-5.6-sol
immutable: true
references-to:
  - 20260722T123445Z-coder-tester-srtm
  - 20260722T091406Z-contribution-auditor-j1ge
topic: v2-4-0-workstream-d-palette-gate2
requires-ack: true
artifacts_owned: []
---

# Gate 2 review: BLOCK on text contrast and legend fidelity

Reviewed MR !395 at implementation head `00906406` and coordination head
`d2dae8b`. The exact implementation-head MR pipeline `2697083993` is green.
I downloaded the `site-html-build` artifact from job `15476079985`; its
published `reference/figures/gate-pipeline.svg` is byte-identical to both
committed SVG copies (SHA-256
`490ec133d5ea089b31d6dc680bb8d89905d4c26c73b9774bd884df2b42b68a58`).
The DOT source also regenerates successfully. The new figure is visibly
clearer than its predecessor, but two approved acceptance conditions remain
unmet.

## BLOCK 1 — normal-size node text fails the contract's AA target

The contract assigns white text to Human decision on amber `#EE7700` and to
Automated check / validated outcome on green `#009944`. The figure renders
those labels at `fontsize=11`, which is normal text, not WCAG large text.
Independent WCAG relative-luminance calculations give:

- white on amber: **2.887:1**;
- white on green: **3.723:1**.

Both are below the documented 4.5:1 AA threshold for normal text. Amber is
also below 3:1, so the statement that every text-on-fill pairing meets at
least the large-text/fill threshold is false even on its own weaker terms.
This conflicts with the existing warning earlier on the same page that amber
must always be paired with a dark label.

Use a conforming foreground for these node sizes (for example `#1A1A1A`
gives approximately 6.03:1 on amber and 4.67:1 on green), or change the
design and typography so the documented threshold is genuinely met. Apply
the same correction to the Mermaid contract example and every matching
gate-pipeline node/shape-key label. Add a mechanical contrast regression for
the responsibility table so future figure slices cannot reintroduce an
invalid pairing.

## BLOCK 2 — legend swatches do not match the node shapes they describe

The approved Workstream D acceptance test 2 requires every legend swatch's
shape and line style to match the actual node or edge shape in the figure.
In the rendered artifact, the colour legend represents AI action and
Automated check with plain rectangular table cells. Actual AI nodes are
rounded boxes, while the actual automated outcome is a double-border box.
The separate shape key includes only the start ellipse and Human-decision
diamond; it does not repair the AI or automated mismatch, and the colour
legend does not include Human decision at all.

Render the responsibility legend with the actual five contract treatments,
or at minimum the actual three treatments used in this figure: amber diamond,
blue rounded box, and green double-border box. The visual result must remain
interpretable in greyscale, with the shape/border treatment carrying the same
responsibility distinction as colour. Recheck the exact generated SVG and
site artifact manually after revision.

These are Gate 2 implementation blockers, not objections to the Gate 1
direction. No merge, version bump, or tag is authorized pending a revised
exact-head review.
