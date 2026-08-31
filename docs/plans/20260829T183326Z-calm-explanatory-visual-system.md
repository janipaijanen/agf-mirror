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

The pilot exists to answer one question honestly:

**Does the governor prefer figures drawn under a calmer explanatory grammar
to the current authoritative versions, without weakening the accessibility
and colourblind-safety contract already in force?**

This is an **aesthetic-preference pilot** (review `s8ks` V1). The first
revision asked whether the grammar "measurably improve[s] comprehension",
which the plan's own risk section then contradicted by admitting calmness is
not measurable here. No baseline, comparison task, target-reader sample, or
comprehension measure was ever defined, so the plan promised evidence it had
no mechanism to produce.

**Evidence status: Theoretical**, per `agf/docs/site/reference/
maturity-vocabulary.md`. The outcome is a governor preference between drawn
alternatives, plus a mechanical accessibility check. It is **not** evidence of
clarity, comprehension, or effectiveness, and no adopter-facing or public
claim may describe it as such.

A comprehension pilot remains a legitimate future option. It would need an
old/new comparison, target-reader tasks, a stated success measure, narrow and
wide viewport checks, and independent `web-ux-reviewer` evidence — all
specified before implementation, none of which this pilot carries. That is a
separate work order.

If the governor prefers the current figures, the grammar is dropped at a cost
of three files.

## Origin and what is actually being asked

Handoff `20260829T181044Z-coder-tester-58kh` records that the governor
likes the visual language of the JXØ article "Building Autonomous Goal
Loops That Deliver" — calm, restrained diagrams for concepts, operations,
data flows, and responsibility flows — and asked whether generated DOT can
adopt the same direction.

The useful properties are compositional, not a palette to copy: a narrow
reading field, one primary claim per figure, restrained use of colour beyond
what semantics require, clear containment and direction, an accessible title
plus a prose caption, and very few decorative elements. Note that the
reference article's neutral-structure treatment is **not** adopted: AGF's
palette assigns `blue-mid` to primary structure and reserves `grey-mid` for
deferred content, and that assignment governs.

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

One page, `agf/docs/figures/grammar.md`, stating a **provisional pilot
profile** that governs **only the three new pilot variants named in
Deliverable 2**.

Review `s8ks` V3 found that the first revision described these as rules "a
figure must satisfy", which would make the other thirteen authoritative
figures immediately nonconforming — precisely the thirteen-figure migration
this pilot exists to avoid. The page must therefore state, in its own opening
lines, that it binds no existing figure, and that applying it to any
authoritative figure requires a later work order and its own Gate 1 decision.

Draft content, to be critiqued at review:

- **One primary claim per figure**, stated in the figure's own title. A
  figure that needs two sentences to say what it shows is two figures.
- **Reading field.** A maximum node count and a maximum crossing count per
  pilot figure, derived mechanically from the current thirteen rather than
  invented. Review `barm` CVR-V6 found the first revision set the bound at
  "what the clearest current figure achieves", which smuggles back the clarity
  ranking V1 removed — this pilot has no comprehension measurement and cannot
  identify a clearest figure as evidence.

  The bound is therefore the **median node count and median crossing count
  across all thirteen current figures**, a stated statistic requiring no
  ranking. The counting method is specified with the bound (what counts as a
  node, what counts as a crossing) and its output recorded, so a reviewer can
  reproduce the number. The bound is labelled **pilot preference**, not
  evidence of clarity.
- **Existing semantic assignments override aesthetic restraint.** This is
  stated first because review `s8ks` V2 found the first revision inverted it.
  `agf/docs/visual/palette.md` assigns `blue-mid` to **primary structural
  elements** (line 27, rule 41) and restricts `grey-mid` to **deferred,
  out-of-scope, or not-applicable** elements (line 32, rule 46). The first
  revision proposed "thin `grey-mid` structure by default", which would have
  marked every structural element as deferred — a semantic error, not a
  stylistic preference. **Withdrawn.** Primary structure stays `blue-mid`.
- **No accent cap that erases required semantics.** The proposed "at most two
  accent tokens plus neutral" is **withdrawn as a cap**. The semantic
  responsibility contract requires distinct token/shape pairings for Human
  decision, AI action, Automated check, Shared concept, and
  Reference/deferred content; `gate-pipeline` and
  `operating-responsibility-map` intentionally exercise more than two. The
  existing "at most four tokens" rule stands unchanged. Restraint applies
  only to tokens carrying **no** required semantic — a figure may not add
  decorative colour, but it may not drop meaning to hit a count.
- **Embedded legends stay mandatory.** The first revision's "direct labels
  over legends" is **withdrawn**: `agf/docs/figures/figures.md` requires an
  embedded legend whenever colour, shape, or line style has semantics. Direct
  labelling is a preference to apply *in addition to* a legend where it
  reduces lookups, never instead of one.
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

Each pilot figure ships as **new files alongside the existing one**, never a
replacement. The current SVG stays authoritative until the governor compares
them.

**Exact allowed artifact paths (review `s8ks` V3).** The first revision said
"one new file", which is not achievable: the current workflow keeps DOT as
canonical source and writes identical committed SVGs to two directories. The
enumerated boundary is:

Review `barm` CVR-V5 found the first table still ambiguous — SVG and PNG
cannot share a filename, and "the SVG itself is canonical" did not say which
committed copy is the source. Filenames are therefore enumerated exactly, per
figure:

| Pilot | Canonical source (the one editable artifact) | Committed outputs |
|---|---|---|
| `gate-pipeline-pilot` (DOT) | `agf/docs/figures/src/gate-pipeline-pilot.dot` | `agf/docs/figures/gate-pipeline-pilot.svg` **and** `agf/docs/site/reference/figures/gate-pipeline-pilot.svg`, byte-identical |
| `operating-responsibility-map-pilot` (purpose-built SVG) | **`agf/docs/figures/operating-responsibility-map-pilot.svg` is canonical** and hand-maintained | `agf/docs/site/reference/figures/operating-responsibility-map-pilot.svg`, a byte-identical copy of the canonical file |
| `evidence-lifecycle-pilot` — **DOT option** | `agf/docs/figures/src/evidence-lifecycle-pilot.dot` | `agf/docs/figures/evidence-lifecycle-pilot.svg` **and** `agf/docs/site/reference/figures/evidence-lifecycle-pilot.svg` |
| `evidence-lifecycle-pilot` — **HTML/CSS option** | `agf/docs/figures/src/evidence-lifecycle-pilot.html` | `agf/docs/figures/evidence-lifecycle-pilot.svg` **and** `agf/docs/site/reference/figures/evidence-lifecycle-pilot.svg` — **SVG only; PNG is not authorized** |

The two `evidence-lifecycle-pilot` rows are **mutually exclusive**. The
format-selection decision record names which one was taken; the other's paths
are never created. Rendered output is SVG in both options, so the committed
output paths are identical either way and the choice changes only the source.

**Source-of-truth and drift contract.** Every alternative declares one
canonical source. Where an output is generated, regeneration must be
deterministic and the two committed copies must be byte-identical; where the
artifact is hand-maintained, the plan says so explicitly and no build step is
claimed. A drift check compares the two committed copies.

**Comparison surface.** The old/new comparison and prose captions are page
edits, not figure files. Exactly one page is authorized:
`agf/docs/site/reference/figures/pilot-comparison.md`. No other published page
is edited by this plan.

**Existing AI-disclosure contract applies unchanged (CVR-V7).** These figures
are produced with AI assistance, and `agf/docs/figures/figures.md` already
requires every such figure to carry descriptive Markdown alt text and the
visible disclosure *"AI-generated figure. Publication remains the
responsibility of the human maintainer."* at each published embed. The first
revision required captions, titles and legends but omitted this, and the plan
promises to change no existing figure contract — so omitting it would have
silently waived a governed requirement. Every pilot embed on the comparison
page carries both, with the disclosure reproduced verbatim and neither weakened
nor reworded here.

**Stop conditions.** Implementation stops and returns to Gate 1 if it needs a
fourth figure, any theme/CSS/navigation/README change, any edit to an existing
authoritative figure, or any path not listed above.

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

1. `agf/docs/figures/grammar.md` exists; every rule is either demonstrated by
   an existing figure or explicitly marked aspirational; and its opening lines
   state that it is a **provisional pilot profile binding only the three named
   pilot variants**, that it makes no existing figure nonconforming, and that
   applying it to any authoritative figure requires a later work order and
   Gate 1 decision.
2. Three pilot figures exist **alongside** their current versions at exactly
   the paths enumerated in Deliverable 2, including canonical sources and both
   committed output copies; the two committed copies of each generated figure
   are byte-identical; no existing figure or SVG is replaced or deleted; and
   no path outside that table plus
   `agf/docs/site/reference/figures/pilot-comparison.md` is touched.
3. Every pilot figure uses only `palette.md` tokens under their **governed
   semantic assignments** — `blue-mid` for primary structure, `grey-mid` only
   for deferred/out-of-scope/not-applicable — within the existing "at most
   four tokens" rule, with every fill-only token paired with a `#1A1A1A`
   label. No required semantic distinction is dropped to reduce token count.
4. Every pilot figure has an accessible title and a prose caption stating
   its claim in words.
5. Contrast is verified against the ratios already published in
   `palette.md` — no pairing is introduced that the table does not already
   sanction. **Two separate measurements, not one (per `barm`'s advisory):**
   (a) the colour-pair contrast ratio, which is a property of the two colours
   and is **unchanged by stroke width**; and (b) a rendered-size inspection at
   narrow and wide viewports for thin-line loss. Any stroke weight below that
   used by the current authoritative figures requires (b) recorded explicitly.
   Neither substitutes for the other, and the evidence must not imply that
   thinning a stroke changes its contrast ratio. `grey-mid` is 3.54:1 and is
   large-text/fill only; a thinner stroke in it is not assumed safe because
   the token appears in the table.
6. Each pilot figure records its format class and one sentence on why that
   class, including the deliberately undecided third.
7. The format-selection recommendation is delivered and cites the pilot,
   not the reference article.
8. No governed rule, palette token, schema, or generator behaviour is
   changed. Every pilot figure carrying semantic colour, shape, or line style
   ships an embedded legend, per `agf/docs/figures/figures.md`.
9. The comparison page states the evidence status explicitly: the pilot
   result is a **governor preference**, Theoretical evidence, and is not a
   claim about comprehension, clarity, or effectiveness. No adopter-facing or
   public text produced by this plan says otherwise.
9a. **Every pilot embed on the comparison page carries descriptive Markdown
   alt text and the verbatim visible disclosure "AI-generated figure.
   Publication remains the responsibility of the human maintainer.",** per
   `agf/docs/figures/figures.md`. All three variants are checked.
9b. The reading-field bound is recorded with its counting method and the
   computed median node and crossing counts over all thirteen current figures,
   reproducible by a reviewer, and labelled pilot preference rather than
   evidence of clarity.
9c. Every created file matches one of the exact paths enumerated in
   Deliverable 2; the mutually exclusive `evidence-lifecycle-pilot` option not
   taken has no files; and the total does not exceed the eleven authorized
   artifacts.
10. A drift check confirms the two committed copies of each generated figure
   are byte-identical, and each alternative's declared source-of-truth and
   regeneration (or explicit hand-maintained) status is recorded.

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

**Third: accessibility regression.** Restraint and contrast pull against each
other — thin neutral structure is exactly how figures fall below AA, which is
why the first revision's "thin `grey-mid` structure by default" rule is
withdrawn rather than merely qualified. Review `s8ks` V2 also found that
criteria 3 and 5 as first written did **not** make this blocking: they checked
palette membership and published ratios, neither of which tests a stroke
weight at rendered size. Criterion 5 now requires separate non-text-contrast
evidence at rendered size and at both viewports for any stroke thinner than
the current figures use. That is what makes this a blocking check rather than
a judgment call; the first revision claimed the property without having it.

**Fourth: duplicate authority with the held plan.** Mitigated by the
boundary table; the residual is that a reviewer may judge the split
artificial, in which case merging this into the held plan is the correct
outcome and should be said at Gate 1 rather than after implementation.
