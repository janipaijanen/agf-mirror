---
kind: note
from: planner-architect
to: governor-reviewer
id: 20260726T070541Z-planner-architect-mgsm
timestamp: 2026-07-26T07:05:41Z
immutable: true
references-to:
  - 20260726T070448Z-governor-reviewer-trra
  - 20260726T065401Z-fact-checker-aqjp
  - 20260726T064928Z-planner-architect-r5mm
  - 20260725T215458Z-fact-checker-hqq2
  - 20260725T214402Z-planner-architect-3krp
topic: public-feature-summary-synthesis
requires-ack: true
artifacts_owned: []
---

# Synthesis: public feature-summary content and placement (Gate 1 draft)

### Claude · planner-architect

Per `20260726T070448Z-governor-reviewer-trra`'s instruction, this synthesizes
the Pedagogist pass (`r5mm`, **non-blind advisory** — independence condition
waived, not a blind-independent pass) and the Marketing pass (`hqq2`, first
blind pass), folding in `aqjp`'s advisory findings P2-P5. This is a **draft
for Gate 1 review**, not an implementation authorization; `README.md` and
`agf/docs/site/` remain untouched.

## Content: six bullets, outcome before mechanism

Adopting `aqjp`'s content-synthesis guidance (both passes converge here):

1. **Human plan and merge decisions** — planning and merge decisions stay
   with a named human governor (Gate 1, Gate 2 — parenthetical, not
   leading).
2. **Named roles, explicit authority** — proposing, implementing, reviewing,
   and approving are separate, named responsibilities.
3. **Durable, reconstructible records** — plans, handoffs, reviews, and
   decisions form a repository-native trail tied to exact revisions.
4. **Evidence and maturity labels** — capability claims are labeled by
   maturity level to limit overclaiming.
5. **Reviewed learning, separately governed improvement** — operational
   evidence can become a reviewed learning record; a learning may motivate
   an improvement proposal, but neither step changes rules, roles, or scope
   automatically. (Resolves P3: kept as one bullet, but the two stages and
   the fresh-gate boundary stay explicit within the sentence, per `aqjp`'s
   own guidance — not silently compressed.)
6. **Repository-native, optional tooling** — validation tooling supports
   consistency; it is optional and does not replace human authority or
   named-role decisions. (Resolves P4: reinstated per `aqjp` and `hqq2`
   as a trust/lock-in differentiator, not merely implementation detail.)

## P2 — working-memory claim removed

`r5mm`'s uncited "4-6 item working-memory ceiling" claim is dropped per
`verifiable-claims-rule`. The six-item cap stands as pedagogical judgment
only, consistent with `aqjp`: "the five/six-item recommendation can stand as
pedagogical judgment" without the unsourced empirical framing.

## Placement: two canonical surfaces, links elsewhere

Both passes agree; adopted as-is:

- **Root README** (developer/evaluator tone): after the opening problem
  statement, before cost/adoption detail.
- **Docs landing page** (`agf/docs/site/docs/index.md`, newcomer tone):
  same canonical block, before "How to read this site."
- **All other public pages**: one-line pointer/link only, no repetition.
  Release and reference pages excluded by default (per `hqq2`).
- **Single canonical source**: if the build supports safe single-source
  inclusion, both placements render from one source. If not, the docs
  landing page is canonical and the README carries a shorter link-forward
  summary rather than a second full paraphrase (per `hqq2`'s fallback).

## Terminology

Plain-language gloss required for "Gate 1/2," "exact-revision handoffs,"
and "improvement-proposal loop" before first use in the top-level list;
gate numbers may follow the plain-language explanation parenthetically
(per `aqjp`), not lead the sentence.

## P5 — deterministic acceptance criteria (replacing non-mechanical wording)

Dropping `r5mm`'s "above the fold on a standard viewport" and "without
re-reading twice" (not reproducible per `aqjp`). Replaced with:

- the six-bullet block's canonical wording is byte-identical between README
  and docs landing page (or generated from one source) — checked by diff or
  grep, not visual inspection;
- no third public page contains the full six-bullet block — checked by
  grep for the block's opening phrase across `docs/` and `README.md`;
- every capability phrase in the block maps to a released artifact at the
  selected baseline (`hqq2`'s acceptance criterion, unchanged);
- every benefit claim carries an evidence-status/maturity-level check
  (`hqq2`, unchanged);
- gate references are followed by a plain-language restatement in the same
  sentence or the immediately following one (mechanical: reviewer checks
  each gate mention has an adjacent gloss).
- "Above the fold" and "readable without re-reading" are retained only as
  **human-review prompts** during Gate 2 visual acceptance
  (`visible-artifact-verification-rule`), not as automated acceptance
  criteria.

## Claim constraints (from `hqq2`, unchanged, carried into this synthesis)

No "continuous/automatic self-improvement" language; no implying CI/`agov`
approves a gate; no "complete audit trail" or "guaranteed compliance"
phrasing; distinguish shipped mechanism from organizational-benefit
maturity; no counts in the top block; keep `agov` visibly optional;
preserve the "does not run the assistant" boundary.

## Independence status disclosure (required by `trra`)

This synthesis is built from one non-blind Pedagogist pass (waived per
governor decision) and one blind Marketing pass. Any published artifact
derived from this synthesis, and any future audit of it, must state this
plainly rather than describe both inputs as blind-independent.

## Requesting

Gate 1 review/approval of this content-and-placement plan. No copy drafting
or page edits begin until Gate 1 clears.
