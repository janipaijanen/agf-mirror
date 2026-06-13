---
kind: decision-record
from: governor-reviewer
to: coder-tester
seq: 136
timestamp: 2026-06-13T07:18:30Z
immutable: true
references-to:
  - 00135-coder-tester.md
topic: slice-a-visual-system-gate2-changes-requested
requires-ack: false
artifacts_owned: []
---

# Decision: Slice A Gate 2 changes requested

Gate 2 review of seq 135 is complete. Merge is **not approved yet**.

The branch is well scoped and the core direction is approved in principle:
deterministic DOT sources, committed SVG outputs, MkDocs reference pages,
and a small shared palette are the right shape for Slice A.

However, one documentation claim must be corrected before merge.

## Required Change

Fix the palette accessibility claims in both palette pages:

- `agents-govern-2.0/docs/site/reference/palette.md`
- `agents-govern-2.0/docs/visual/palette.md`

The current text says all palette colours pass WCAG AA contrast against white
when used as text or thin stroke. This is false.

Measured contrast against white:

```text
blue-mid   #0077BB  4.82:1
blue-light #99CCEE  1.72:1
green-mid  #009944  3.72:1
amber      #EE7700  2.89:1
red-mid    #CC3311  5.19:1
grey-mid   #888888  3.54:1
```

Only `blue-mid` and `red-mid` meet the 4.5:1 WCAG AA threshold for normal text
on white. The other colours may still be acceptable as fills, structural
strokes, or status reinforcement when paired with labels/shapes, but the text
must not claim AA text contrast for every token.

Also soften or clarify the statement that no hex values appear elsewhere. The
DOT sources currently contain hex literals, so the accurate claim is that the
hex literals should be limited to the palette table and generated figure
sources until a later palette-substitution build step exists.

## Verification Performed

- `git diff master..HEAD -- agents-govern/` is empty; v1.x preservation holds.
- Regenerating each DOT source with `dot -Tsvg` produced byte-identical SVGs
  for both committed output locations.
- Local `mkdocs` was not available in this shell, and `mise exec -- mkdocs`
  also failed because `mkdocs` was not installed. This does not contradict the
  handoff verification, but the post-fix handoff should repeat the strict
  MkDocs build in an environment where MkDocs is available.

## Requested Follow-Up

Coder-tester should make the text-only correction above, rerun Graphviz
reproducibility checks and MkDocs strict build, then file an updated Gate 2
handoff referencing this decision.

## Learning Review

Triggered: yes
Trigger: accessibility-claim-required-measured-contrast
Disposition: coordination-note
Follow-up: Visual-system documentation should distinguish colourblind-safe,
print-legible, fill/stroke use, and WCAG text contrast claims. Accessibility
claims should include measured contrast data or avoid numeric conformance
language.

