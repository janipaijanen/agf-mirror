---
id: framework-map-full-size-figure
status: pending-gate1
topic: framework-map-full-size-figure
advisory_consultation:
  - role: contribution-auditor
    status: not-applicable
    reason: "the governor reported the defect and fixed the outcome boundary directly: make the existing map a zoomable/openable figure and record the learning; the plan freezes that two-part scope and excludes adjacent site or figure-system redesign"
  - role: infosec-sentinel
    status: not-applicable
    reason: "scope is a public documentation page, one DOT/SVG figure pair, the public figure gallery, and one learning record; no secrets, personal data, authentication boundary, executable integration, schema, or external data flow is introduced"
---

# Plan: readable framework map with a full-size figure path

## Trigger and problem

Issue #64 records a direct user report on 2026-07-25: the published
`docs/start/framework-map/` diagram is unreadable at page scale and gives the
reader no clear way to open or zoom it.

The live source is one wide Mermaid `flowchart LR` with seven first-level
branches and 31 child nodes. The browser renders it as an inline SVG inside the
article width. Responsive shrinking makes the labels too small; the page does
not link to a standalone asset. Existing `.figure-scroll` CSS is not used on
this page, and horizontal scrolling alone would not supply a clean full-size
inspection path.

This is a public-documentation usability defect, not a change to AGF authority,
behavior, or adopter contracts.

## User outcome

A reader can understand the map at ordinary page scale, activate either the
figure or an explicit text link to open the standalone SVG, and use the
browser's normal zoom controls without losing vector clarity. Readers who do
not use the image retain the existing node guide, edge guide, and explanatory
prose.

## Frozen implementation scope

1. Add `agf/docs/figures/src/framework-map.dot` as the editable figure source.
   Recompose the existing concepts into a page-readable Graphviz layout; do not
   add, remove, or strengthen framework claims. Use the governed AGF palette,
   readable label sizes, an embedded legend, and a white standalone background.
2. Generate and commit
   `agf/docs/site/reference/figures/framework-map.svg` using the documented
   Graphviz workflow. The SVG must carry a useful title/description and remain
   legible when opened directly.
3. Replace the Mermaid fence in
   `agf/docs/site/docs/start/framework-map.md` with a linked Markdown image.
   Both the image and an adjacent, plainly labelled “Open the full-size
   framework map (SVG)” link point to the standalone SVG. Do not force a new
   browser tab; the user controls navigation and browser zoom.
4. Add the framework map to `agf/docs/site/reference/figures.md`, including its
   source path and a concise description. Update the gallery numbering if
   required, without rewriting unrelated figure prose.
5. Append one learning to `agf/learnings/foundry.yaml` using a timestamp ID.
   Record the observed failure and the general lesson: a dense orientation
   diagram must have an explicit full-size inspection path; responsive
   shrink-to-fit or scrolling alone is insufficient evidence of readability.
   Keep evidence status single-adopter/theoretical as appropriate; do not turn
   one report into a universal rule.

## Presentation and accessibility requirements

- The page-scale composition prioritizes the seven load-bearing groups over
  decorative edges or exhaustive detail.
- Figure text follows the repository guidance: normally 11pt node labels and
  10pt edge labels. Any deviation must be justified by rendered evidence.
- The linked image has meaningful alternative text. The adjacent text link has
  a visible, self-explanatory accessible name and is keyboard reachable.
- Essential relationships remain stated in the existing prose, node guide, and
  edge guide; the SVG is not the sole carrier of meaning.
- The direct SVG has a white background and does not depend on site CSS,
  JavaScript, Mermaid initialization, hover, color alone, or pointer-only
  interaction.
- At 390px width, the page must not reduce the only available labels below
  practical readability: the explicit full-size link remains visible before
  the detailed guides, and the page must not introduce horizontal document
  overflow.
- At 200% browser zoom, the text link and surrounding prose remain operable.
  The standalone SVG remains sharp under browser zoom.

## Acceptance evidence

- `dot -Tsvg` regenerates the committed SVG from the DOT source; a second
  generation is byte-stable or any generator metadata variation is explained.
- Strict MkDocs build passes.
- Built HTML inspection confirms the figure image is inside a link to the SVG
  and the explicit full-size text link resolves to the same asset.
- The direct SVG URL returns successfully and renders independently with its
  title/description, white background, and complete labels.
- Rendered desktop and 390px checks record the page-scale figure, discoverable
  full-size affordance, absence of document overflow, and readable guides.
- Keyboard inspection confirms the full-size link receives visible focus and
  activation opens the SVG.
- A 200% browser-zoom check confirms surrounding page content remains usable.
- `agov validate adopter-docs`, umbrella `agov validate`, learning validation,
  `git diff --check`, and the repository's applicable CI jobs pass.
- Review compares the new DOT labels, page node guide, and edge guide against
  the removed Mermaid source and records that no concept or claim boundary was
  lost or added.

## Commit and review structure

One implementation MR, with reviewable commits:

1. `docs: make framework map an openable SVG figure` — DOT source, generated
   SVG, page embed/link, and figure-gallery entry.
2. `learning: record full-size path for dense figures` — append-only learning
   record with the concrete issue #64 evidence boundary.

Gate 2 reviews the exact final SHA and the rendered evidence together. Any
change to diagram concepts, site-wide figure behavior, JavaScript, MkDocs
plugins, or accessibility claims requires a plan amendment and new Gate 1.

## Explicit exclusions

- No change to the current v2.4.2 release MR, SHA, tag, or publication scope.
  This fix follows in a later patch/release decision.
- No site-wide lightbox, zoom library, JavaScript dependency, or new MkDocs
  plugin.
- No retrofit of every existing figure with the same link pattern in this MR.
  The learning may inform a separately scoped inventory if later evidence
  shows the pattern is broader.
- No change to roles, gates, rules, schemas, commands, maturity labels, or
  evidence claims.
- No removal of the node guide or edge guide merely because the standalone SVG
  becomes available.
- No formal WCAG-conformance or universal assistive-technology claim.

## Stop conditions

Stop and return to planning if the figure cannot preserve the existing concept
set at useful page scale, if opening/zooming requires new runtime JavaScript or
a site-wide dependency, or if review discovers that the page's current map is
factually inconsistent with shipped AGF behavior rather than merely unreadable.

## Gate requests

- **Gate 1:** governor-reviewer approves or blocks this frozen plan before any
  DOT, SVG, page, gallery, or learning implementation begins.
- **Gate 2:** governor-reviewer reviews the exact implementation SHA, rendered
  evidence, validation results, and issue #64 acceptance checks before merge.
