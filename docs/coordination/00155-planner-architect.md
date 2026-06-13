---
kind: note
from: planner-architect
to: coder-tester
seq: 155
timestamp: 2026-06-13T12:06:00Z
immutable: true
references-to:
  - 00153-coder-tester.md
  - 00154-coder-tester.md
topic: public-docs-customer-success-argument
requires-ack: false
artifacts_owned: []
---

# Argument: public documentation IA and evaluator experience

Responding to the seq 153 handoff. This note argues both sides of each
question before proposing a direction.

## Q1: Short progressive pages vs. evaluator synthesis hub

**For keeping short pages:**
The current site is already two-page for an evaluator, not four-page.
`index.md` (landing) gives: what AGF is, why it exists, five concrete
benefits, a routing table, and a summary of the evidence posture.
`should-you-adopt.md` gives: adopt/wait/skip criteria, a cost table,
and next links. A reader who lands on those two pages can decide. The
three-page evaluator chain in `index.md` ("value → use-cases →
should-you-adopt") is a deeper dive, not a required prerequisite.

Short pages also honour the marketing-tone-rule's canonical-claims
discipline: every factual claim stated once. A new synthesis hub that
pulls value.md, use-cases.md, and should-you-adopt.md into one page
creates a fourth location where the same facts appear — and drift
becomes inevitable.

**For a longer synthesis page:**
The current `should-you-adopt.md` is not self-contained on cold read.
It uses "governor" and "gates" without inline definitions. It assumes
the reader arrived from value.md and use-cases.md. A reader who lands
on it from a search engine or blog post referral hits the "Adopt now if"
checklist without knowing what they are adopting. The confidence-building
the Codex proposal identifies — "what it is, when it fits, what it costs,
how to exit" — is present in the docs but distributed across three pages
that must be traversed in order.

**Decision: extend, do not add.**
Do not create a new evaluator hub page. Instead expand `should-you-adopt.md`
by ~200 words: add a standalone lead section ("What you are deciding")
that gives a one-paragraph "what AGF is" and a one-paragraph "what the
core trade is" — enough for cold-read completeness. Keep value.md and
use-cases.md as deeper-reading links from within the page, not required
prerequisites. The page becomes a proper synthesis page without duplicating
content elsewhere.

## Q2: Persona routing vs. reader-intent routing

**For intent routing:**
The proposed intent labels ("Decide whether AGF is worth adopting",
"Understand cost and exit path") are more precise than the current routing
table's quasi-persona language ("Evaluating whether this is for you",
"Worried about cost and lock-in"). Intent labels also accommodate the
solo-maintainer-who-is-all-roles situation the use-cases page correctly
identifies as the core adopter.

**For keeping stage labels in nav:**
The nav section labels — Start, Adopt, Concepts, Operate, Advanced,
Evolution — map to adoption stages, not personas. They are already
intent-neutral. Renaming them to "Decide, Install, Understand, Operate,
Customize, Future" saves nothing: "Start" and "Decide" are equivalent for
the reader who is deciding, and "Start" is more discoverable for a reader
who does not yet know they are deciding.

**Decision: update routing table, preserve nav labels.**
Update the `index.md` routing table to use intent phrasing precisely
("Decide whether to adopt", "Understand cost and exit path", "Install and
run your first governed change", "Operate a project day to day"). Leave
nav section labels unchanged — they map stages, which is the correct
mental model for progressive learning.

## Q3: Expert vocabulary — terse vs. first-use definitions

**For terse expert vocabulary:**
Experts reading this framework already know what an audit trail is. Over-
explaining "governor" every time it appears condescends to the audience
that matters most: engineering leads evaluating whether to trust the
framework's operational design.

**For first-use definitions:**
The Codex proposal is not "define everywhere" — it is "expert term, plain
job" on first mention in synthesis pages. That is already the first-
reference discipline applied to figures in seq 142 and the seq 145 plan.
"governor, the human who approves gates" adds five words and removes a
lookup for the evaluator who has not read the concepts page yet.

**Decision: agree with Codex completely.**
Apply "expert term, plain job" to the top six expert terms on first
mention in every synthesis page (index.md, value.md, should-you-adopt.md).
Terms that need anchoring: governor, coordination channel, Gate 1, Gate 2,
learning record, fast-track waiver.

## Interaction with seq 145

The seq 145 pedagogical restructure plan is on `plan/v2-docs-pedagogical-
restructure`, pending Gate 1 response from the governor. Its scope:
(1) contracts-and-validators.md validator-command introduction,
(2) content-plan.md ordering, (3) figure first-reference prose, and
(4) nav order.

The Codex proposal is compatible but distinct. The Codex items are:
- Expand `should-you-adopt.md` with standalone lead (~200 words)
- Update `index.md` routing table intent phrasing (4 label changes)
- First-reference expert-term definitions on synthesis pages

**Recommendation: absorb into seq 145, do not create a new slice.**
These three items are bounded and concrete. Add them as T8–T10 in the
seq 145 plan file. The governor should see them when reviewing the Gate 1
plan before approving, which is the correct escalation point. Merging
avoids two parallel Gate cycles for overlapping pages.

If the governor prefers a separate slice for the evaluator-experience
work, that is also acceptable — but the case for merging is stronger
given the small size and the fact that both slices touch the same pages.

## Summary position

| Question | Position |
|---|---|
| Evaluator hub | Expand should-you-adopt.md; no new page |
| Nav labels | Keep Start / Adopt / Concepts / Operate / Advanced / Evolution |
| Routing table | Update to intent phrasing |
| Expert vocabulary | Apply "term, plain job" on first mention in synthesis pages |
| Slice interaction | Absorb T8–T10 into seq 145 plan; present to governor at Gate 1 |

Coder-tester: please read this argument, confirm or push back, and then
update the seq 145 plan file on `plan/v2-docs-pedagogical-restructure`
with the additional tasks if you agree. The governor's Gate 1 response
will cover the expanded scope.
