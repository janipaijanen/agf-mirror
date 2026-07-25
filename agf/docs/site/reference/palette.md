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
  light-cream (PDF). The white diagram canvas (`bgcolor=white`) is the same
  accepted light-background assumption in `adoption-exit-flow.dot`,
  `coordination-channel.dot`, `evidence-lifecycle.dot`, `gate-pipeline.dot`,
  `governance-loop.dot`, `learning-loop.dot`, and
  `operating-responsibility-map.dot`; these figures are not claimed to be
  background-agnostic.
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
| `grey-light` | `#CCCCCC` | 1.61:1 — fill only | Reference background fill | Reference-content fills |

`blue-mid` and `red-mid` meet the WCAG AA threshold (4.5:1) for normal
text on white. `green-mid` and `grey-mid` meet the large-text threshold
(3:1). `blue-light`, `amber`, and `grey-light` do not meet either
threshold for text — use them for fills and strokes only, always paired
with a dark label (`#1A1A1A`, ~10.1:1 on `blue-light` and ~10.8:1 on
`grey-light` — both well clear of the 4.5:1 AA normal-text threshold).

## Usage rules

1. Use `blue-mid` for primary structural elements (agents, gates,
   channels). Do not use it for status indicators.
2. Use `green-mid` for approved / merged / passing states only.
3. Use `amber` for pending states, not for warnings or errors.
4. Use `red-mid` sparingly — only for blocked or rejected states.
5. Use `grey-mid` for deferred, out-of-scope, or not-applicable elements.
6. Use `blue-light` as a background fill when a lane or group needs a
   visual boundary without visual weight.
7. Use `grey-light` as a background fill for reference/deferred content
   blocks (see the semantic responsibility palette below).

## Semantic responsibility palette (who acts, not what happened)

v2.4.0 Workstream D's closed figure inventory found three unrelated
palette families in use across the site and academic figures,
plus figures that used colour as the *only* signal for who is
responsible for a node (violating colour-independence). This section is
the single agreed contract required before any figure conversion: it
covers the axis of *"which kind of actor is responsible for this
node"* — Human decision, AI action, Automated check/validated outcome,
Shared concept, and Reference/deferred content — reusing this page's
existing token set rather than introducing a second one. It is
deliberately the same five-way vocabulary already established in
`docs/index.md`'s "How to read this site" section (which uses four of
the five; "Automated check/validated outcome" is the one addition new
figures need that page-kind prose didn't) and matches the colour scheme
the (now-removed) `docs-audience-map` figure used for the same four
categories — this contract makes that existing precedent official and
extends it by one category, rather than inventing an unrelated fifth
system.

| Responsibility | Token (fill) | Border/line | Text colour | Shape (colour-independence) |
|---|---|---|---|---|
| Human decision | `amber` `#EE7700` | `amber` | `#1A1A1A` | diamond |
| AI action | `blue-light` `#99CCEE` | `blue-mid` | `#1A1A1A` | box (rounded) |
| Automated check / validated outcome | `green-mid` `#009944` | `green-mid` | `#1A1A1A` | box (double border) |
| Shared concept | white (no fill) | `blue-mid` | `blue-mid` | box (plain border) |
| Reference / deferred content | `grey-light` `#CCCCCC` | `grey-mid` | `#1A1A1A` | box (dashed border) |

**Colour is never the only signal.** Each responsibility also has a
distinct shape/border treatment (final column) so the category survives
greyscale, colourblind rendering, and colour-independent contexts
unchanged — this is what
`docs/plans/v2.4.0-workstream-d-figure-system.md` acceptance test 3
requires. `red-mid` remains a separate, orthogonal accent for a
*rejected/blocked outcome* (an edge or border colour applied on top of
any responsibility fill, per the existing per-edge convention already
used in `gate-pipeline`) — it is not a sixth responsibility category.

### Neutral foreground/background tokens

- Page/figure background: white (`#FFFFFF`) only. DOT-sourced figures do
  not currently ship a dark-theme variant — see "Light/dark behaviour"
  below.
- Default text colour on an unfilled or light-fill node: `#1A1A1A`
  (near-black; higher contrast than pure grey text at every fill in this
  table).
- Default text colour on a saturated fill: `#1A1A1A` for `amber`
  (~6.03:1) and `green-mid` (~4.67:1) — white on either fails the
  4.5:1 normal-text AA threshold (2.89:1 and 3.72:1 respectively) at
  the node label sizes these figures actually render. White remains
  correct for `red-mid` (~5.19:1, meets AA normal text) and for
  `blue-mid`-as-fill.

### Light/dark behaviour

DOT-sourced figures (SVG/PNG) render on a fixed white background
regardless of the hosting page's light/dark theme — this matches the
existing behaviour of every figure in the inventory today and is an
explicit decision, not a silent gap: building a dark-theme-adaptive SVG
variant is out of scope for this contract and would need its own
plan if pursued later. Public documentation figures use the governed
DOT-to-SVG workflow, with explicit fill, border, text, and shape values in
their committed DOT sources rather than a theme-dependent runtime palette.

### Print / greyscale behaviour

When printed or rendered in greyscale, `green-mid` (Automated check) and
`amber` (Human decision) remain close in relative value (~55% and ~45%
black respectively) — the existing "Greyscale rendering" note below
still applies. This is exactly why the responsibility palette pairs each
token with a distinct shape/border in the table above: a greyscale or
colourblind reader distinguishes Human decision (diamond) from Automated
check (double-border box) by shape even where the fill values are close.

### Minimum contrast target

WCAG AA: 4.5:1 for normal text, 3:1 for large text (≥18px or ≥14px
bold) or fill-only usage — the same target this page's existing
"Palette" table already documents per-token above. Figure node labels
(e.g. `gate-pipeline`'s 11pt node text) are normal text, not large
text, so every text-on-fill pairing in the responsibility table must
clear the full 4.5:1 threshold — not just the weaker 3:1 fill/large-text
one. All five do: `#1A1A1A`-on-`amber` (~6.03:1), `#1A1A1A`-on-`green-mid`
(~4.67:1), `#1A1A1A`-on-`blue-light` (~10.1:1) and `#1A1A1A`-on-`grey-light`
(~10.8:1), and `blue-mid`-on-white (the Shared concept text colour,
~4.82:1). White text is reserved for `red-mid` fills (~5.19:1), which
also clears 4.5:1.

### DOT-to-SVG usage

DOT figures must set the exact palette hex values in source, pair responsibility
colours with the matching shape or border treatment, and render with the
governed figure workflow. Do not rely on renderer defaults: a direct SVG must
retain the same meaning without site CSS, JavaScript, or a theme runtime.

## Greyscale rendering

When printed in greyscale, `green-mid` and `amber` are close in value
(~55% and ~45% black respectively). Diagrams using both must distinguish
them with shape or label as well as colour.

## See also

- [Figures](figures.md) — the figure inventory and generation instructions
- `docs/plans/v2.4.0-workstream-d-figure-system.md` — the workstream this
  contract satisfies (Gate 2 obligation from
  `20260722T091406Z-contribution-auditor-j1ge` BLOCK 2)
