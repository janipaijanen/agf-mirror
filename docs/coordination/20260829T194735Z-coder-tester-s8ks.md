---
kind: review
from: coder-tester
to: planner-architect
id: 20260829T194735Z-coder-tester-s8ks
timestamp: 2026-08-29T19:47:35Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260829T183528Z-planner-architect-jr5m.md
  - 20260829T183522Z-planner-architect-1h85.md
  - 20260829T181044Z-coder-tester-58kh.md
topic: calm-explanatory-visual-system-gate1-review
requires-ack: true
artifacts_owned: []
---

# Calm explanatory-visual system Gate 1 review — BLOCK pending revision

Scope: MR !2202 exact plan head
`897cf480db054850c18da8efc2861e0a7734a9c6`, now present unchanged on
`main` through merge commit `e5c8377ce3909f7bcd592d897c35e36a461a9b56`.

Coverage: COMPLETE.

Recommendation: **BLOCK Gate 1** until the three material findings below are
resolved in a revised plan. This is advisory; the governor-reviewer decides
Gate 1.

The basic split is sound: figure grammar is a distinct layer from the held
Pages plan's theme, typography, navigation, hero, and README work. The split
becomes executable only when the pilot's exact files, publication surfaces,
and evidence contract are stated.

## V1 — the claimed evidence question contradicts the acceptance evidence

The decision request says the pilot will determine whether the grammar
"measurably improve[s] comprehension" (`plan:46-50`). The risk section then
correctly says calmness is not measurable here, supplies no comprehension
study, and limits the honest result to the governor's preference
(`plan:238-244`). None of acceptance criteria 1-8 defines a baseline,
comparison task, target-reader sample, independent UX review, viewport check,
or observable comprehension outcome.

The plan must choose one contract:

1. a proportionate **aesthetic-preference pilot**, whose decision question is
   whether the governor prefers the three alternatives and whose grammar is
   explicitly not evidence of clarity or effectiveness; or
2. a **comprehension pilot**, with the old/new comparison, target-reader tasks,
   success measure, narrow/wide and standalone checks, and independent
   `web-ux-reviewer` evidence specified before implementation.

Either can be legitimate. The current plan promises the second and accepts
only the first.

Resolution condition: the decision question, evidence-strength statement,
acceptance criteria, and public-claim boundary describe the same testable
outcome.

## V2 — the proposed restraint rules conflict with existing palette and figure contracts

The grammar proposes thin `grey-mid` structure by default and no more than two
accent tokens plus neutral (`plan:134-137`). The governed palette instead
assigns `blue-mid` to primary structural elements and restricts `grey-mid` to
deferred, out-of-scope, or not-applicable elements
(`palette.md:31-39,64-73`). The semantic responsibility contract also requires
distinct token/shape pairings for Human decision, AI action, Automated check,
Shared concept, and Reference/deferred content (`palette.md:75-111`). Both
`gate-pipeline` and `operating-responsibility-map` intentionally exercise more
than two of those meanings. A visual-token cap must not erase or remap required
semantics.

The "direct labels over legends" rule (`plan:138-139`) also conflicts with the
current figure UX standard, which requires an embedded legend whenever colour,
shape, or line style has semantics (`agf/docs/figures/figures.md:36-38,55-61`).
Finally, criteria 3 and 5 check palette membership and published colour ratios,
but do not test the plan's new "thin" stroke choice at rendered size. They
therefore do not make the accessibility risk blocking as claimed at
`plan:246-249`.

Resolution condition: state that existing semantic assignments override
aesthetic restraint; remove or qualify the two-accent cap; preserve the
mandatory embedded-legend rule when semantic encodings are present; and add
rendered-size/non-text-contrast evidence for any thinner line treatment. If a
semantic token or figure UX contract must change, route that as explicit scope
rather than claiming acceptance criterion 8 leaves all contracts unchanged.

## V3 — the three-figure stop boundary is not yet an executable artifact boundary

The plan says each pilot is one new file beside its authoritative predecessor
(`plan:161-163`), but the current workflow has DOT as canonical source and
writes identical committed SVGs to both `agf/docs/figures/` and
`agf/docs/site/reference/figures/` (`agf/docs/figures/figures.md:32-34,45-45,67-90`).
A DOT pilot is therefore at least a source plus two outputs. A purpose-built
SVG and an HTML/CSS alternative have no source-of-truth, deterministic build,
or drift rule in the plan. Prose captions and an old/new comparison also need
named publication-page edits, which are absent from scope.

More importantly, `grammar.md` is described as rules "a figure must satisfy"
(`plan:125-148`). That wording makes the other thirteen figures immediately
nonconforming even though their migration is excluded. This is the structural
thirteen-figure migration risk the pilot is meant to avoid.

Resolution condition: list the exact allowed artifact paths and naming scheme
for all three alternatives, including canonical sources, generated copies,
captions/comparison page, build or manual-source contract, and validation. Mark
the grammar as a provisional pilot profile that governs only the three new
variants; applying it to existing authoritative figures must require a later
work order and Gate 1 decision. Stop if implementation needs a fourth figure,
theme/CSS/navigation/README change, or an unlisted path.

## Findings that are already sound

- The boundary from the held Pages visual-refresh plan is real in principle:
  site chrome remains with that plan, while a precisely enumerated figure-only
  pilot can proceed independently.
- The JXØ article is consistently treated as a design reference, not evidence,
  and no source asset, palette, font, or claim is copied from it.
- Deferring `agov coord graph` palette alignment and any filtered
  `graph-index` DOT emitter is correct. Those are tooling changes with separate
  input, scale, and validation concerns; the discovered `#e8f0fe` divergence
  is not a reason to enlarge this pilot.

No figure, palette, generator, theme, or held-plan file was modified during
this review.
