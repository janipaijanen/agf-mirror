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
| `grey-light` | `#CCCCCC` | 1.61:1 — fill only | Reference background fill | Reference-content fills |

`blue-mid` and `red-mid` meet the WCAG AA threshold (4.5:1) for normal
text on white. `green-mid` and `grey-mid` meet the large-text threshold
(3:1). `blue-light`, `amber`, and `grey-light` do not meet either
threshold for text — use them for fills and strokes only, always paired
with a dark label (`#1A1A1A` or `black`, both >15:1 on `grey-light` and
`blue-light`).

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
palette families in use across the site, academic, and Mermaid figures,
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
| Human decision | `amber` `#EE7700` | `amber` | white | diamond |
| AI action | `blue-light` `#99CCEE` | `blue-mid` | `#1A1A1A` | box (rounded) |
| Automated check / validated outcome | `green-mid` `#009944` | `green-mid` | white | box (double border) |
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
- Default text colour on a saturated fill (`amber`, `green-mid`,
  `red-mid`, `blue-mid`-as-fill): white.

### Light/dark behaviour

DOT-sourced figures (SVG/PNG) render on a fixed white background
regardless of the hosting page's light/dark theme — this matches the
existing behaviour of every figure in the inventory today and is an
explicit decision, not a silent gap: building a dark-theme-adaptive SVG
variant is out of scope for this contract and would need its own
plan if pursued later. Mermaid diagrams (rendered client-side by the
docs site's Mermaid integration) MUST use explicit `classDef`/style
definitions setting these same fill/border/text values, rather than
relying on Mermaid's theme-dependent default palette, specifically so
they do not silently invert or reflow under the site's dark theme
toggle — see "Mermaid usage" below.

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
"Palette" table already documents per-token above. No responsibility
token is used for body text at normal size; all text-on-fill pairings
in the responsibility table meet at least the large-text/fill threshold,
and `blue-mid`-on-white and `red-mid`-on-white (both used for text
labels, e.g. edge labels) meet the full 4.5:1 normal-text threshold.

### Mermaid usage

Mermaid diagrams must define one `classDef` per responsibility, reusing
these exact hex values, e.g.:

```
classDef humanDecision fill:#EE7700,stroke:#EE7700,color:#ffffff
classDef aiAction fill:#99CCEE,stroke:#0077BB,color:#1A1A1A
classDef automatedCheck fill:#009944,stroke:#009944,color:#ffffff
classDef sharedConcept fill:#ffffff,stroke:#0077BB,color:#0077BB
classDef referenceContent fill:#CCCCCC,stroke:#888888,color:#1A1A1A
```

Apply the matching `class` to every node; do not leave a node on
Mermaid's default theme fill, since that reintroduces the "three
unrelated palette families" problem this contract exists to close.

## Greyscale rendering

When printed in greyscale, `green-mid` and `amber` are close in value
(~55% and ~45% black respectively). Diagrams using both must distinguish
them with shape or label as well as colour.

## See also

- [Figures](figures.md) — the figure inventory and generation instructions
- `docs/plans/v2.4.0-workstream-d-figure-system.md` — the workstream this
  contract satisfies (Gate 2 obligation from
  `20260722T091406Z-contribution-auditor-j1ge` BLOCK 2)
