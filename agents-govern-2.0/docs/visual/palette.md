<!--
SPDX-License-Identifier: CC-BY-SA-4.0
-->
# Visual palette

agents-govern v2.0 uses a small, explicit colour set. Hex values for
palette tokens are defined in the table below and used in DOT source
files. A future palette-substitution build step will allow figures to
reference tokens by name rather than hex literal.

## Design constraints

- **Colorblind-safe.** Distinguishable for the most common forms of
  colour-vision deficiency (deuteranopia, protanopia, tritanopia).
  Checked against the Paul Tol palettes and Okabe–Ito recommendations.
- **Print-legible.** Sufficient contrast against white when printed
  in greyscale; shape and label carry meaning, colour reinforces it.
- **Light and dark backgrounds.** Fills work on white (docs site) and
  light-cream (PDF). Dark-mode variants are not yet defined.
- **Restrained.** The palette has six named roles. Diagrams use at most
  four in a single figure.

## Palette

| Token | Hex | Contrast on white | Role | Usage |
|---|---|---|---|---|
| `blue-mid` | `#0077BB` | 4.82:1 ✓ AA text | Primary structural | Boxes, arrows, role nodes |
| `blue-light` | `#99CCEE` | 1.72:1 — fill only | Secondary / background fill | Gate shading, lane fills |
| `green-mid` | `#009944` | 3.72:1 — large text / fill | Positive outcome / approval | Approved state, merge |
| `amber` | `#EE7700` | 2.89:1 — fill only | Caution / pending | Pending gate, draft state |
| `red-mid` | `#CC3311` | 5.19:1 ✓ AA text | Block / rejection | Blocked finding, reject |
| `grey-mid` | `#888888` | 3.54:1 — large text / fill | Neutral / deferred | Deferred scope, N/A |

`blue-mid` and `red-mid` meet the WCAG AA threshold (4.5:1) for normal
text on white. `green-mid` and `grey-mid` meet the large-text threshold
(3:1). `blue-light` and `amber` do not meet either threshold for text —
use them for fills and strokes only, always paired with a dark label.

## Usage rules

1. Use `blue-mid` for primary structural elements (agents, gates,
   channels). Do not use it for status indicators.
2. Use `green-mid` for approved / merged / passing states only.
3. Use `amber` for pending states, not for warnings or errors.
4. Use `red-mid` sparingly — only for blocked or rejected states.
5. Use `grey-mid` for deferred, out-of-scope, or not-applicable elements.
6. Use `blue-light` as a background fill when a lane or group needs a
   visual boundary without visual weight.

## Figures

Generated figures live in [`docs/figures/`](../figures/figures.md).
All figures use this palette. DOT source files reference tokens by hex
value; a future build step may introduce a palette variable substitution.

## Greyscale rendering

When printed in greyscale, the palette maps approximately as follows:

| Token | Greyscale value (approx.) |
|---|---|
| `blue-mid` | Medium grey (~40% black) |
| `blue-light` | Light grey (~15% black) |
| `green-mid` | Medium-dark grey (~55% black) |
| `amber` | Medium grey (~45% black) |
| `red-mid` | Dark grey (~65% black) |
| `grey-mid` | Medium grey (~47% black) |

`green-mid` and `amber` are close in greyscale. Diagrams relying on
both must use distinct shapes or labels to remain legible without colour.
