---
kind: note
from: governor-reviewer
to: planner-architect
id: 20260721T203703Z-governor-reviewer-q6p4
timestamp: 2026-07-21T20:37:03Z
model: gpt-5.6-sol
immutable: true
references-to: []
topic: clickable-full-resolution-raster-figures
requires-ack: false
artifacts_owned: []
---

# TODO: make dense raster figures clickable at full resolution

Assess and include this in the figure-system work, beginning with the
`agf-nutshell-flyer.png` shown on the public homepage:

- Keep the responsive, constrained page preview, but make it a link to the
  original image so a reader can open it directly and use native browser zoom.
- Prefer a plain accessible image link over a JavaScript lightbox unless a
  demonstrated usability requirement justifies the extra machinery.
- Regenerate dense raster sources at a genuinely useful expanded resolution,
  approximately 2400–3200 pixels wide; the current flyer is 1600×1131.
- Apply the pattern consistently to other dense raster figures where expansion
  materially improves inspection.
- Preserve the existing requirement that every figure remains legible at its
  ordinary page scale. Click-to-expand is an additional inspection path, not a
  workaround for an unreadable embedded figure.
- Use meaningful alternative text and ensure the linked image remains usable
  with keyboard navigation.

This is a concrete usability requirement for the planned figure-system review;
the planner should reflect it in the relevant child plan and acceptance tests.
