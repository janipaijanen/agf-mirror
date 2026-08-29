---
id: calm-explanatory-visual-system
status: pending-gate1
topic: calm-explanatory-visual-system
advisory_consultation:
  - role: coder-tester
    status: waived
    reason: >-
      Not yet consulted. This plan is planner-architect self-authored in
      response to coder-tester handoff `58kh`, which requested planning
      and critique only. A bounded independent coder-tester review is
      requested before binding Gate 1, per the project's standing
      practice: in the v2.7.0-v2.9.0 cycle, five consecutive independent
      reviews each found real defects in self-authored artifacts.
  - role: contribution-auditor
    status: waived
    reason: >-
      No separate contribution-auditor pass is proposed. The bounded
      independent coder-tester review requested above is offered as the
      substitute independent check, on the same basis the governor
      accepted for v2.6.0 (`kpkk`, `33br`) and throughout the
      v2.7.0/v2.8.0 cycle. The governor ratifies or rejects that
      substitution at Gate 1; this field records the request, not a
      granted waiver.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Presentation-layer only. No credential, access-control, schema,
      gate, role-authority, or security-boundary change. The pilot adds
      no dependency, no external font or asset host, and no runtime
      code path. Matches the disposition accepted for the sibling plan
      `pages-readme-visual-refresh-v251`.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: calm explanatory-visual system — grammar and a three-figure pilot

## Decision requested

Authorize **a written visual grammar plus a three-figure pilot**, and
nothing else. Specifically **not** authorized by this plan: a site
redesign, a migration of the existing thirteen figures, any generator
implementation, or any change to `agov coord graph`'s current output.

The pilot exists to answer one question with evidence rather than taste:
**does a calmer explanatory grammar measurably improve comprehension of an
AGF concept, without weakening the accessibility and colourblind-safety
contract already in force?** If the answer is no, the grammar is dropped at
a cost of three figures.

## Origin and what is actually being asked

Handoff `20260829T181044Z-coder-tester-58kh` records that the governor
likes the visual language of the JXØ article "Building Autonomous Goal
Loops That Deliver" — calm, restrained diagrams for concepts, operations,
data flows, and responsibility flows — and asked whether generated DOT can
adopt the same direction.

The useful properties are compositional, not a palette to copy: a narrow
reading field, one primary claim per figure, thin neutral structure with
sparse accent colour, direct labels, clear containment and direction, an
accessible title plus a prose caption, and very few decorative elements.

**The article is a design reference, not a source.** Its SVG, colour set,
font, and branded appearance are not copied. It is classified in
`docs/sources.md` Batch 31 as `practice / prior-art, complement / watch`,
a self-reported experience report with no comparative data. It supports no
AGF claim, and this plan cites it for nothing beyond "the governor pointed
at this look."

## Starting state — five constraints this must reconcile with

**1. The palette is already governed and already restrained.**
`agf/docs/site/reference/palette.md` defines six semantic tokens,
colourblind-safe against Paul Tol and Okabe-Ito, print-legible in
greyscale, with a stated maximum of four in a single figure and documented
per-token contrast ratios. `blue-light`, `amber`, and `grey-light` are
fill-and-stroke only and must always pair with a dark `#1A1A1A` label.
**The grammar must consume this palette, not propose a replacement.** If
the pilot needs a colour the palette lacks, that is a finding to report,
not a token to invent.

**2. White background is an accepted, documented assumption — not an
oversight.** `bgcolor=white` is deliberate across
`adoption-exit-flow.dot`, `coordination-channel.dot`,
`evidence-lifecycle.dot`, `gate-pipeline.dot`, `governance-loop.dot`,
`learning-loop.dot`, and `operating-responsibility-map.dot`, and those
figures are explicitly not claimed to be background-agnostic. A "calmer"
grammar must not quietly reopen that decision.

**3. There are thirteen hand-authored DOT sources**
(`agf/docs/figures/src/*.dot`) with committed SVG siblings and a
regeneration check. Any grammar that cannot be applied to a figure without
rewriting all thirteen is too invasive for a pilot.

**4. `docs/TODO.md:368` is binding on the generation question.** The
documented migration replaces hand-authored academic DOT files "only after
semantic review; until then, those files remain the authoritative manually
maintained sources." This plan does not perform that semantic review and
does not begin that migration.

**5. A visual plan is already approved and held.**
`docs/plans/20260803T192952Z-pages-readme-visual-refresh-v251.md` is
`status: approved` (governor decision `j86r`, 2026-08-05) covering Pages
palette, typography, navigation, hero, and README structure, with
implementation held and target release unassigned. **That plan owns site
chrome. This plan owns figure grammar only.** The two must not both claim
the same surface — see the boundary table below.

## Scope boundary against the held plan

| Surface | Owner |
|---|---|
| Pages theme, typography, navigation, hero, README structure | `pages-readme-visual-refresh-v251` (approved, held) |
| Figure composition, labelling, caption contract, format selection | **this plan** |
| Palette token definitions and contrast contract | `palette.md` (neither plan changes it) |
| Generated DOT from `agov coord graph` | out of scope for both — see below |

If the pilot produces a finding that requires a theme change, it is
reported to the held plan's owner, not implemented here.

## Deliverable 1 — a written visual grammar

One page, `agf/docs/figures/grammar.md`, stating the compositional rules a
figure must satisfy. Draft content, to be critiqued at review:

- **One primary claim per figure**, stated in the figure's own title. A
  figure that needs two sentences to say what it shows is two figures.
- **Reading field.** A maximum node count and a maximum crossing count per
  figure, chosen from the current thirteen rather than invented — the
  pilot measures the existing figures first and sets the bound at what the
  clearest current figure achieves.
- **Structure is neutral; colour is sparse.** Thin `grey-mid` structure by
  default; accent tokens only where they carry semantic meaning already
  defined in `palette.md`. The existing "at most four tokens" rule
  tightens to **at most two accent tokens plus neutral** for pilot figures.
- **Direct labels over legends.** A legend is a signal the figure is doing
  too much.
- **Every figure ships an accessible title and a prose caption** that
  states the claim in words, so the figure is not the only carrier of the
  information. This is an accessibility requirement, not a stylistic one.
- **No decorative element** that does not carry structure, direction, or
  containment.

The grammar is descriptive of what a good AGF figure already does where
possible. Rules that no current figure satisfies are marked as aspirational
so a reviewer can see which are new obligations.

## Deliverable 2 — a three-figure pilot

Three figures, chosen to exercise all three format classes rather than to
flatter the grammar:

| Figure | Format class | Why this one |
|---|---|---|
| `gate-pipeline` | **DOT** — node-edge relationship | The canonical AGF concept figure; if the grammar cannot improve this, it is not worth having |
| `operating-responsibility-map` | **Purpose-built SVG** — layered/geometric | Lanes and containment are exactly what DOT renders worst; this is the honest test of the format recommendation |
| `evidence-lifecycle` | **DOT or HTML/CSS** — deliberately undecided | The pilot decides which, and recording *why* is part of the deliverable |

Each pilot figure ships as a **new file alongside the existing one**, not a
replacement. The current SVG stays authoritative until the governor
compares them. This is what makes the pilot cheap to abandon.

## Deliverable 3 — format-selection recommendation

Answering `58kh`'s explicit question, with the pilot as evidence:

- **DOT** where the content is genuinely node-edge and layout can be left
  to the engine.
- **Purpose-built SVG** where geometry, layering, or lane containment
  carries meaning — Graphviz fights this and the result reads busy.
- **Plain HTML/CSS** where a graph adds nothing over a table or an ordered
  list. This option must be genuinely available: the most common
  visual-design failure is drawing a diagram for something that is a list.

## The generated-DOT question — answered narrowly, not implemented

`58kh` asks whether generated DOT can adopt the same direction. Findings
from inspecting the current surface, recorded so the answer is not
re-derived:

- `agov coord graph` already emits Graphviz DOT by default (`--format dot`,
  with `--format text` available), styling roles as boxes, `all` as an
  ellipse, and `governor-reviewer` filled `#e8f0fe`. It is the
  intermediate-DOT/user-renders design already shipped.
- `agov coord graph-index` exposes 6,920 `references-to` edges across 2,900
  nodes as JSON, with 40 pending and 333 invalid legacy files. **There is
  no DOT emitter for that dependency graph.**
- `#e8f0fe` is **not a `palette.md` token.** The generated graph already
  diverges from the governed palette. That is a real, small finding.

**Recommendation: do not fold generation into this plan.** The grammar
should be proven on hand-authored figures first, because a generated graph
of 2,900 nodes cannot satisfy "one primary claim per figure" without a
filtering design that does not yet exist. Two separate follow-on items are
proposed and **not** authorized here: aligning `agov coord graph`'s colours
with `palette.md`, and a filtered DOT emitter for `graph-index`.

## Out of scope

Site redesign; migrating the other ten figures; any generator or
`agov` code change; any edit to `palette.md`; reopening the white-background
decision; the `docs/TODO.md:368` academic-DOT migration; any version bump,
CHANGELOG entry, tag, or release.

## Acceptance criteria

1. `agf/docs/figures/grammar.md` exists, and every rule is either
   demonstrated by an existing figure or explicitly marked aspirational.
2. Three pilot figures exist **alongside** their current versions; no
   existing figure or SVG is replaced or deleted.
3. Every pilot figure uses only `palette.md` tokens, at most two accent
   tokens plus neutral, with every fill-only token paired with a `#1A1A1A`
   label.
4. Every pilot figure has an accessible title and a prose caption stating
   its claim in words.
5. Contrast is verified against the ratios already published in
   `palette.md` — no pairing is introduced that the table does not already
   sanction.
6. Each pilot figure records its format class and one sentence on why that
   class, including the deliberately undecided third.
7. The format-selection recommendation is delivered and cites the pilot,
   not the reference article.
8. No governed rule, palette token, schema, or generator behaviour is
   changed.

## Risks

**The largest risk is that this becomes a redesign.** An appealing
aesthetic direction plus thirteen existing figures plus an already-approved
held theme plan is precisely the shape that produced the v2.7.0 failure:
scope that grows until it cannot ship. Three figures, additive-only, is the
mitigation, and the stop condition is explicit — if the pilot cannot be
done without touching a fourth figure or the theme, stop and return to
Gate 1.

**Second: taste is not evidence.** "Calmer" is not measurable, and neither
the reference article nor this plan supplies a comprehension study. The
honest claim available at the end of the pilot is "the governor judged
these three clearer", which is a governor preference, legitimately
sufficient for a documentation aesthetic and **not** sufficient to support
any public AGF claim about clarity or effectiveness. The grammar page must
say so.

**Third: accessibility regression.** Restraint and contrast pull against
each other — "thin neutral structure" is exactly how figures fall below AA.
Criteria 3 and 5 exist to make that a blocking check rather than a
judgment call.

**Fourth: duplicate authority with the held plan.** Mitigated by the
boundary table; the residual is that a reviewer may judge the split
artificial, in which case merging this into the held plan is the correct
outcome and should be said at Gate 1 rather than after implementation.
