---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260725T060120Z-contribution-auditor-qtad
timestamp: 2026-07-25T06:01:20Z
model: gpt-5-codex
immutable: true
references-to:
  - 20260725T055605Z-planner-architect-smch
topic: progressive-disclosure-standing-scope-review
requires-ack: true
artifacts_owned: []
---

# Review: progressive-disclosure standing scope

Scope: MR !491 at `30ee96ea2e5e638c9245c7ed1dfdc7bf28f7c4e9`
Coverage: COMPLETE
Recommendation: CLEAR
Blocking findings: 0
Advisory findings: 3
Open findings with accepted risk: 0
Authority: recommendation only; the authorized decision-maker decides the gate

### Codex · contribution-auditor

The proposed standing shape is sound with the refinements below. Keep
progressive disclosure as an explicit **applicability check in each release
plan**, not a standing instruction to rewrite pages. A one-line
`not-applicable` reason is enough when a release does not change a public
reader journey, evaluator-facing claim, or adoption decision surface. This
preserves the v2.4.2 boundary against manufactured scope.

## Advisory 1 — classify by audience and task, not directory

Do not make every `docs/adopt/*` or `docs/advanced/*` page a permanent
non-candidate merely from its path. The documentation-audience rule assigns
the public site broadly to business users and administrators; it does not
declare path-based exclusions. Some adoption pages, especially downstream
impact and exit, are part of an evaluator's decision journey even though they
also serve administrators.

Use this test instead: does a page help a reader decide, orient, or take a
first action? If yes, plain-language framing may be useful while its technical
details retain the specialist register. The AI dossier remains a good default
non-candidate because its review precision is intentional. Advanced contract
pages likewise remain default non-candidates, but not an irreversible class.

## Advisory 2 — revise the four candidate dispositions

| Candidate | Disposition | Boundary |
|---|---|---|
| `first-governed-change.md` | ADOPT, targeted | Improve the opening, responsibility transitions, and next action; preserve commands and exact procedure. It is a developer tutorial, not evaluator marketing copy. |
| `framework-map.md` | VERIFY FIRST | It already leads with ordinary-language purpose, gives a concept table, and ends with next actions. Include only if a page-level check finds a concrete comprehension gap; do not rewrite for quota. |
| `evaluate.md` | ADOPT, coordinated | It is a genuine evaluator entry point. Coordinate with the release that changes its competing-entry structure so one pass owns both structure and prose; do not create a parallel edit. |
| `future/evolution.md` | ADAPT, narrow | Preserve hypothesis, maturity, and evidence-trigger precision. Apply plain language to the page lead and to newly changed hypotheses, not as a blanket rewrite of the roadmap register. |

## Advisory 3 — make the trigger observable

A release plan should name progressive-disclosure scope when it adds or
materially changes at least one of:

- a public evaluator/adopter entry point;
- a feature, maturity, evidence, cost, privacy, or enforcement claim;
- the reader's next action or navigation between decision stages; or
- a new AGF term introduced on a beginner-facing page.

For each included page, retain v2.4.2's evidence bar: reader question, next
action, ordinary language before the exact term, a concrete example where it
helps, and explicit preservation of technical and evidence boundaries.

No change is requested to MR !488. Its exact-text fact-check remains the only
open v2.4.2 consultation; this review is planning input for later releases.
