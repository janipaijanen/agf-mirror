<!--
SPDX-License-Identifier: CC-BY-SA-4.0
-->
# Figures

All figures are generated from DOT source files in
`agf/docs/figures/src/`. SVG outputs are committed to
`docs/site/reference/figures/` as source-of-truth artifacts.

**To regenerate all figures:**

```bash
cd agf/docs/figures
for f in src/*.dot; do
  name=$(basename "$f" .dot)
  dot -Tsvg "$f" -o "../site/reference/figures/${name}.svg"
done
```

Requires Graphviz (`dot` version 2.x or later).

## Governance loop

Figure 1 shows the framework's self-applied governance cycle.

![Governance loop](figures/governance-loop.svg)

The self-applied cycle: plan → implement → review → merge → learn →
plan. Incidents and observations feed the learning record; the learning
record feeds new rules, tools, and documentation updates.

**Source:** `docs/figures/src/governance-loop.dot`

---

## Gate 1 / Gate 2 pipeline

Figure 2 shows the two-gate quality pipeline from request to merge.

![Gate pipeline](figures/gate-pipeline.svg)

The two-gate quality pipeline. Gate 1 approves the plan before
implementation starts. Gate 2 approves the implementation before merge.
Both gates can return for changes or reject outright.

**Source:** `docs/figures/src/gate-pipeline.dot`

---

## Coordination channel

Figure 3 shows how append-only coordination entries route attention and
record outcomes between agents.

![Coordination channel](figures/coordination-channel.svg)

The append-only coordination channel. Handoffs direct attention to the
next agent; decision records gate or block; completions close tasks;
notes carry advisory or status information.

**Source:** `docs/figures/src/coordination-channel.dot`

---

## Operating responsibility map

Figure 4 separates human-only authority, AI-executed work, and mixed
work in the day-to-day audit trail.

![Day-to-day responsibility modes](figures/operating-responsibility-map.svg)

The day-to-day split between human-only authority, AI-executed work, and
mixed work in the audit trail. "AI-executed" means delegated execution,
not autonomous approval authority.

**Source:** `docs/figures/src/operating-responsibility-map.dot`

---

## Evidence lifecycle

Figure 5 shows how incidents and observations become learning records,
then rules, validators, or documentation updates.

![Evidence lifecycle](figures/evidence-lifecycle.svg)

From incident to improvement. An observation becomes a learning record;
learning records promote to rules, validators, or documentation updates;
sufficient corroboration (2+ adopters) promotes a rule from Provisional
to Recommended.

**Source:** `docs/figures/src/evidence-lifecycle.dot`

---

## Learning loop

Figure 6 shows the evidence-driven feedback loop as a recurring cycle.

![Learning loop](figures/learning-loop.svg)

The evidence-driven feedback loop. Governed work produces incidents,
friction, or validation results; learning records preserve the evidence;
system-level analysis asks which control should change; governed approval
precedes implementation; and observed project use confirms, challenges,
revises, or retires the change before feeding the next cycle.

**Source:** `docs/figures/src/learning-loop.dot`

---

## Adoption and exit flow

Figure 7 shows the adoption path and the documented exit paths.

![Adoption and exit flow](figures/adoption-exit-flow.svg)

From assessment to decommission. Step 0 assesses risk profile; install
and first governed change establish the governance habit. Exit paths
include freeze, replace-on-update, and full decommission — all preserve
the coordination record history.

**Source:** `docs/figures/src/adoption-exit-flow.dot`

---

## Coordination map example

Figure 8 shows a dated example communication map from the AGF foundry
coordination channel.

![Example AGF Foundry coordination map](figures/coordination-map-example.svg)

Parenthesized numbers on edges (e.g. `handoff (22)`, `decision-record (9)`)
are message counts in this captured coordination channel sample; they reflect
the state of the foundry channel at the time the figure was generated, not
universal ratios for all projects. The embedded legend reads
"Edge label = kind (message count)".

This is an example, not a required topology. Adopting projects may define
more specialized agent roles and may show different communication patterns
as their governance model matures.

**Source:** `docs/figures/src/coordination-map-example.dot`

---

## Usage rules

- Embed SVG in Markdown with `![Alt text](figures/name.svg)` from this page,
  or with the full relative path from other pages.
- Do not hand-edit generated SVG files — edit the DOT source and regenerate.
- Every figure must include an embedded legend in the generated image. Do not
  rely on page prose or captions to explain color, shape, or line semantics.
- Legend samples should remain visual samples: do not put literal shape names
  such as `oval`, `diamond`, or `box` inside the swatch. Put the meaning in the
  adjacent label text instead.
- **Numeric labels must explain their unit.** Any figure that displays numeric
  labels (counts, percentages, durations) must state the unit and sampling
  context inside the figure itself (in the legend or a label), and the page
  text immediately before or after the figure must repeat the meaning in plain
  language. "Edge label = kind (22)" is insufficient on its own; add
  "(message count)" or equivalent in the legend and explain what the sample
  covers in the surrounding text.
- Keep figure text readable at page scale: node labels should normally be
  `11pt`, edge labels `10pt`, and rendered SVGs should stay near or below
  `720pt` wide unless the page intentionally provides a scrollable view.
- Use white backgrounds for standalone SVGs so direct figure URLs remain
  readable outside the MkDocs page chrome.
- Use the [visual palette](palette.md) for all new figures.

## See also

- [Visual palette](palette.md) — colour tokens and usage rules
- Source workflow and authoring standards live in
  `agf/docs/figures/figures.md`.
