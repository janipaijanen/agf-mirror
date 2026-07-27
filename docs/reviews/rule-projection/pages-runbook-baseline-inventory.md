# Pages/runbook normative-language baseline inventory

Baseline audit required by `agf/rules/workflow/documentation-rule-projection-rule.md`
§4 ("Baseline-audit existing Pages and runbooks"), produced as part of issue
#73's implementation. This is **prospective evidence** — it classifies the
existing corpus, it does not retroactively invalidate or rewrite historical
plans, coordination records, or already-published releases.

## Method

Scanned `agf/docs/site/docs/**/*.md` for candidate normative language:
`MUST`, `REQUIRED`, `blocking`, `approve(s|d)`, `prohibited`. Per the rule,
**keyword detection is advisory candidate discovery only** — it is not a
semantic judgment about whether the surrounding prose is actually normative,
duplicated, or correctly deferring to a named rule.

## Inventory

| File | Candidate occurrences |
|---|---|
| `advanced/contracts-and-validators.md` | 13 |
| `start/adopter-faq.md` | 8 |
| `concepts/coordination-protocol.md` | 8 |
| `operate/review-and-release.md` | 6 |
| `concepts/roles-and-gates.md` | 6 |
| `operate/runbook.md` | 5 |
| `start/should-you-adopt.md` | 4 |
| `operate/run-agf-in-real-situations.md` | 4 |
| `adopt/assistant-loader-configuration.md` | 4 |
| `start/first-governed-change.md` | 3 |
| `index.md` | 3 |
| `concepts/philosophical-lenses.md` | 3 |
| `adopt/install-and-layouts.md` | 3 |
| `operate/day-to-day.md` | 2 |
| `adopt/tooling-and-tokens.md` | 2 |
| `operate/session-continuity.md` | 1 |
| `operate/poll-and-coordinate.md` | 1 |
| `concepts/position.md` | 1 |
| `concepts/evidence-and-learnings.md` | 1 |
| `advanced/customizing-the-framework.md` | 1 |

20 files, ~79 candidate occurrences total.

## Spot-check findings (this implementation's bounded scope)

Spot-checked the two highest-density files plus the two files this
implementation directly edited:

- **`advanced/contracts-and-validators.md`** (13 occurrences) — the page's
  actual subject is the advisory-vs-blocking distinction itself
  ("A validator failure becomes a blocking gate only when a named AGF rule,
  contract, CI policy, or recorded governor decision says that check is
  blocking"). It consistently defers to "a named rule" as the real owner
  rather than asserting new rules. No competing-authority finding.
- **`concepts/coordination-protocol.md`** (8 occurrences, one section
  rewritten by this implementation) — the rewritten "Publication is
  independent of Gate 2" section points to
  `coordination-publication-rule.md` as the owner rather than restating its
  conditions. No competing-authority finding in the changed section.
- **`concepts/roles-and-gates.md`** and **`operate/runbook.md`** — not
  modified by issue #72/#73; not deep-audited beyond keyword detection in
  this pass.

## Disposition

Per `documentation-rule-projection-rule.md`'s bounded baseline-audit scope,
this implementation:

- **Fixes:** issue #72/#73's own projections (see
  `docs/reviews/rule-projection/coordination-independent-publication.yaml`)
  and confirms no broken pointer was introduced by this change (validated by
  the Gate 2 strict Pages build and internal-link check).
- **Does not fix:** any pre-existing normative-language occurrence outside
  this change's own scope. The 20-file inventory above is unclassified
  beyond the two spot-checked files and this implementation's own edits.
- **Routes forward:** a full per-occurrence ownership classification of the
  remaining ~18 files/~66 occurrences is out of this implementation's
  bounded scope and should be tracked as a separately prioritized issue, not
  folded into this release. Flagging this explicitly in the Gate 2 handoff
  rather than silently declaring the audit complete.
