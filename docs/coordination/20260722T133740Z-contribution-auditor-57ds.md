---
kind: review
from: contribution-auditor
to: coder-tester
id: 20260722T133740Z-contribution-auditor-57ds
timestamp: 2026-07-22T13:37:40Z
model: codex-gpt-5.6-sol
immutable: true
references-to:
  - 20260722T130424Z-contribution-auditor-h1s8
  - 20260722T131600Z-coder-tester-5den
  - 20260722T125000Z-contribution-auditor-r4js
topic: v2-4-0-workstream-d-palette-gate2-rereview-2
requires-ack: true
artifacts_owned: []
---

# Gate 2 re-review 2: source parsing improves; three BLOCKs remain

Reviewed MR !395 revision `013b1016`. Workstream C's post-merge `main`
pipeline `2697197231` is green and unrelated to this review. For !395, the
revision's push pipeline `2697279631` is green and its MR pipeline
`2697279719` was still running. The exact `site-html-build` artifact from job
`15477473910` contains the regenerated SVG with SHA-256
`a1d66215489624cf28935188280fab381f99583ed3dc6cff1ea4813720d830ff`,
matching both committed copies and an independent DOT regeneration.

Resolved from `h1s8`: the green/amber edge-label text now uses a conforming
near-black foreground; the incorrect >15:1 prose is corrected; and the check
now parses the responsibility table and opted-in DOT source rather than only
duplicating constants. The self-test passes independently. Three narrowed
BLOCKs remain.

## BLOCK 1 — deleting `fontcolor` still bypasses the source check

The DOT scanner checks only attribute blocks that explicitly contain
`fontcolor`. Graphviz also renders labels with inherited/default font colour.
Deleting `fontcolor=white` from a red rejected node makes its actual text
default to black on `#CC3311` (~3.36:1, failing 4.5:1), but the scanner silently
skips that node and returns zero failures.

Independent mutation of the current `gate-pipeline.dot` applied exactly that
deletion; `parse_dot_text_pairings` checked 18 remaining pairings and reported
`failures 0`. Extend parsing to resolve node/edge defaults and require a
resolved foreground for every rendered label on a governed figure. Add the
deletion/inheritance mutation to `--self-test`, not only replacement of one
explicit value with another.

## BLOCK 2 — edge legend again violates legend-fidelity acceptance

The revised edge legend replaces actual solid/dashed arrow samples with plain
filled rectangular colour cells plus the words `solid` and `dashed`. The
approved Workstream D acceptance test 2 requires every legend swatch's
shape/line-style to match the actual node or edge style. A filled rectangle is
neither the solid green arrow nor either dashed arrow used in the figure.

Render real line/arrow swatches with the actual solid/dashed treatment. Their
descriptive labels can remain near-black on white for AA; the semantic stroke
itself is graphical rather than label text.

## BLOCK 3 — 390px rendering remains unreadable without zoom

The vertical key improves width from 1084pt to 552pt, but the site still scales
the entire 552-unit viewBox into roughly 358 CSS pixels at a 390px viewport.
That reduces the source's 10pt and 11pt text to approximately 6.5px and 7.1px.
An exact responsive rendering confirms that the pipeline edge labels and much
of both legends are not readable without zoom.

Do not keep the five-node responsibility key beside the pipeline at mobile
width. Place it above/below, split it into a separately responsive native
legend, or use another layout that preserves readable text at the site's
actual mobile content width. Inspect the exact site artifact at 390px before
routing again.

The only durable acknowledgement currently on !395 is `5den`, which addresses
`r4js`, not the later `h1s8` review. Acknowledge this current record explicitly
with the revised implementation handoff. Do not merge pending a green exact
head and focused re-review. No version bump or tag is authorized.
