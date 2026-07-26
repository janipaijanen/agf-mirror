---
kind: review
from: planner-architect
to: planner-architect
id: 20260726T064928Z-planner-architect-r5mm
timestamp: 2026-07-26T06:49:28Z
immutable: true
references-to:
  - 20260725T215315Z-planner-architect-7mhg
  - 20260725T215315Z-governor-reviewer-h4hm
  - 20260725T214402Z-planner-architect-3krp
topic: public-feature-summary-pedagogist-review
requires-ack: true
artifacts_owned: []
---

# Pedagogist-perspective first pass: public feature summary

Requested by `20260725T215315Z-planner-architect-7mhg`, per the designation
in `20260725T215315Z-governor-reviewer-h4hm` (Codex independently reached the
same "designate existing roles" decision I was about to record — this pass
supersedes my own draft of that decision, which is not published).

**Independence note:** the analysis and conclusions below were drafted before
I read the marketing-perspective pass. Before publishing this file I did read
`20260725T215458Z-fact-checker-hqq2` in full, in order to align references and
confirm no direct conflict — I did not revise my own conclusions based on its
content. Where the two passes agree (canonical single-source placement,
README + docs-landing-page as the two surfaces, jargon must be glossed) that
is convergence between independent first passes, not one pass copying the
other. Where they differ (bullet count: 5 here vs. 6 there; explicit ordering
scheme) both are recorded below for the synthesis step to reconcile, not
silently merged into one.

## Cognitive load

The candidate envelope lists 9 items. For a scanning, first-contact list
(README top, docs landing page), that exceeds what a novice reader retains
in one pass — working-memory research on list comprehension generally puts
the practical ceiling for an unstructured bullet list nearer 4-6 items before
readers stop reading each one individually and start skimming. Recommend
compressing to a **canonical 5-item list**, folding related items together:

1. Gate 1/Gate 2 human decisions before planning and before merge
2. A durable, append-only record of who decided what and why
3. Named roles with explicit authority boundaries
4. Evidence and maturity labels that keep claims honest
5. A learning loop that turns operational evidence into governed rule changes

This folds "structured roles" (item 3 of the original 9) together as-is,
merges "durable coordination and exact-revision handoffs" into one clause,
and drops "deterministic validation," "adoption/upgrade/removal paths," and
"optional tooling" from the *top-level* list — not because they're
unimportant, but because they read as implementation detail relative to the
other five, and belong one click away rather than in the first-contact list
(see Placement, below).

## Ordering

The original envelope interleaves outcome-level claims (gate decisions,
auditability) with mechanism-level claims (deterministic validation,
tooling posture) without signaling the hierarchy. A first-time reader needs
the outcome first ("what does this actually guarantee me") before the
mechanism ("how is that guarantee implemented"). Recommend ordering
outcome-level items first, always, in any version of this list — mechanism
items are candidates for a secondary, linked list, not the first five lines
a reader sees.

## Terminology

"Gate 1"/"Gate 2," "exact-revision handoffs," and "the improvement-proposal
loop" are AGF-internal vocabulary a first-time reader has not yet learned.
Per `documentation-audience-rule`, the README audience is developers
evaluating the project — not yet AGF practitioners. Every jargon term in the
top-level list needs either a plain-language paraphrase in the same clause
(no linked glossary lookup required to parse the sentence) or must be
dropped from the top-level list entirely. "Gate 1 planning and Gate 2 merge
decisions" should read as "a human approves the plan before work starts, and
approves the change again before it merges" — the gate numbers can appear
in the linked detail page, not the first-contact sentence.

## Progressive disclosure and repetition risk

Repeating the full 9-item (or even 5-item) list on every public page creates
noise: a reader who has already seen the list once does not need to re-read
it identically on the next page, and per-page context (what this
specific page is for) gets crowded out. Recommend:

- **One canonical location** carrying the full list: the root README.
- **One additional canonical location** for the documentation site itself:
  the docs landing page (`agf/docs/site/docs/index.md`), since it is the
  first page a reader reaches independently of the README (a different
  entry point, not a repeat visit).
- **Everywhere else**: a one-sentence contextual pointer plus a link back to
  whichever of the two canonical lists is nearer, not a repeated list.

This matches the existing `marketing-tone-rule`'s message-integrity
principle (state each claim once, canonically; other pages link or quote,
they do not paraphrase) — the same discipline that already governs claim
consistency should also govern list placement, for the same reason: drift
between near-duplicate lists on different pages is a credibility risk once
a careful reader compares them.

## Distinguishing gates, coordination, learning, and improvement

The original envelope risks reading as one vague "feedback loop" unless each
concept keeps a distinct, concrete verb:

- **Gates** — a human *approves* (plan, then merge). Point-in-time decision.
- **Coordination** — the record *persists* (who decided what, referenced by
  ID, never edited after the fact). Ongoing artifact, not a decision.
- **Learning** — operational evidence *becomes a reviewed record* (with its
  own provenance and privacy boundary) — not a decision and not a raw log.
- **Improvement** — a learning record *may motivate a proposal*, and that
  proposal still goes through the same gates above. It is explicitly not
  automatic: nothing changes a rule, role, or scope without a new Gate 1/2
  decision.

Each of these four needs its own one-clause verb in the summary (approves /
records / learns / proposes) so a reader can tell them apart on first read,
rather than one blended noun phrase ("governance and improvement loop") that
implies they are the same mechanism.

## Acceptance criteria for comprehension and progressive disclosure

- A first-time reader can restate, from the top-level list alone and without
  following any link, what a "gate" is in one sentence.
- No top-level bullet requires the reader to already know an AGF-specific
  term to parse it (glossed inline or reworded in plain language).
- The list fits above the fold on a standard viewport at the README's
  existing heading level, without introducing a new nested heading level.
- A reader can distinguish "a human decided this" (gates) from "this was
  recorded" (coordination) from "this could change AGF later" (learning/
  improvement) without re-reading the list twice.
- The list appears in exactly two places (README, docs landing page) with
  byte-identical wording; every other public page's reference to it is a
  link, verifiable by grep for the canonical sentence across `agf/docs/site/`.

## Recommendation

Smallest useful placement pattern: root README + docs landing page only,
each carrying the same 5-item canonical list in the same order and wording,
with every other public page linking to one of those two rather than
repeating the list. Final wording and reconciling the 5-vs-6-item count
against the marketing pass is a synthesis task, not this pass's job — this
pass scopes structure, count, ordering, placement, and comprehension
acceptance criteria only.
