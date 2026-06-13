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
  Checked against Paul Tol palette recommendations and Okabe–Ito safe
  colours.
- **Print-legible.** Sufficient contrast against white when printed
  in greyscale; shape and label carry meaning, colour reinforces it.
- **Light backgrounds.** Fills work on white (docs site) and
  light-cream (PDF).
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

## Greyscale rendering

When printed in greyscale, `green-mid` and `amber` are close in value
(~55% and ~45% black respectively). Diagrams using both must distinguish
them with shape or label as well as colour.

## See also

- [Figures](figures.md) — the figure inventory and generation instructions
