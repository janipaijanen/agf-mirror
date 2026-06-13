<!--
SPDX-License-Identifier: CC-BY-SA-4.0
-->
# Visual palette

agents-govern v2.0 uses a small, explicit colour set. All colours are
drawn from this palette — no hex values appear elsewhere in templates,
figures, or diagrams.

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

| Token | Hex | Role | Usage |
|---|---|---|---|
| `blue-mid` | `#0077BB` | Primary structural | Boxes, arrows, role nodes |
| `blue-light` | `#99CCEE` | Secondary / background fill | Gate shading, lane fills |
| `green-mid` | `#009944` | Positive outcome / approval | Approved state, merge |
| `amber` | `#EE7700` | Caution / pending | Pending gate, draft state |
| `red-mid` | `#CC3311` | Block / rejection | Blocked finding, reject |
| `grey-mid` | `#888888` | Neutral / deferred | Deferred scope, N/A |

All colours pass WCAG AA contrast (4.5:1) against white when used as
text or thin stroke. For large filled areas, pair with a dark label.

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
