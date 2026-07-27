# Pages/runbook normative-language baseline inventory

Baseline audit required by `agf/rules/workflow/documentation-rule-projection-rule.md`
§4 ("Baseline-audit existing Pages and runbooks"), produced as part of issue
#73's implementation. This is **prospective evidence** — it classifies the
existing corpus, it does not retroactively invalidate or rewrite historical
plans, coordination records, or already-published releases.

## Method

Reproducible command (corrected per fact-checker finding F2 — the first
version of this document used a non-reproducible ad-hoc script and reported
inaccurate counts):

```bash
git grep -c -E -o 'MUST|REQUIRED|blocking|approve(s|d)|prohibited' HEAD -- 'agf/docs/site/docs/**/*.md'
```

Per the rule, **keyword detection is advisory candidate discovery only** —
it is not a semantic judgment about whether the surrounding prose is
actually normative, duplicated, or correctly deferring to a named rule.
Every occurrence below was read in context (not counted blind) to classify
its owner.

## Inventory and classification

| File | Occurrences | Owner classification |
|---|---|---|
| `advanced/contracts-and-validators.md` | 12 | `tool-authority-boundary-rule.md` — page's subject *is* the advisory/blocking distinction; consistently defers to "a named rule" rather than asserting one |
| `concepts/coordination-protocol.md` | 7 | `coordination-publication-rule.md` (rewritten section, this implementation) + `roles-and-gates.md` (existing gate-chain description) |
| `concepts/roles-and-gates.md` | 5 | Self-owning — this is the canonical page defining the role/gate authority split; not a projection target for another rule |
| `operate/review-and-release.md` | 5 | `tool-authority-boundary-rule.md` (blocking-findings language) + `roles-and-gates.md` (approval language) |
| `start/should-you-adopt.md` | 3 | `roles-and-gates.md` — describes the human-governor role generically for prospective adopters |
| `operate/run-agf-in-real-situations.md` | 3 | `roles-and-gates.md` (Gate 1/Gate 2 language) — situational examples, not new rules |
| `adopt/assistant-loader-configuration.md` | 3 | `roles-and-gates.md` + `tool-authority-boundary-rule.md` |
| `adopt/install-and-layouts.md` | 2 | `tool-authority-boundary-rule.md` — explicit "the rule that grants it blocking authority" |
| `adopt/tooling-and-tokens.md` | 2 | `tool-authority-boundary-rule.md` — CI wiring guide pointer |
| `operate/runbook.md` | 2 | `roles-and-gates.md` (approved-scope language) + `always/coordination-message-rule.md` (visible-bridge language) |
| `start/adopter-faq.md` | 2 | `roles-and-gates.md` — describes governor-reviewer approval authority, does not redefine it |
| `concepts/philosophical-lenses.md` | 2 | `tool-authority-boundary-rule.md` (blocking authority) + `roles-and-gates.md` (accountability) |
| `operate/day-to-day.md` | 1 | `roles-and-gates.md` — anti-pattern example ("gates approved in seconds") |
| `operate/poll-and-coordinate.md` | 1 | `coordination-publication-rule.md` (this implementation's added pointer) |
| `operate/session-continuity.md` | 1 | `always/coordination-message-rule.md` — visible-bridge language |
| `start/first-governed-change.md` | 1 | `roles-and-gates.md` — "governor reviews and approves" |
| `concepts/evidence-and-learnings.md` | 1 | `always/uncertainty-and-scope-rule.md`-adjacent — "required coordination entry" in a learning-category table |

17 files, 53 matching lines (git-grep-verified, replacing the earlier
inconsistent 20-file/~79-occurrence estimate).

## Disposition

Per `documentation-rule-projection-rule.md`'s bounded baseline-audit scope,
this implementation:

- **Fixes:** issue #72/#73's own projections (see
  `docs/reviews/rule-projection/coordination-independent-publication.yaml`)
  and confirms no broken pointer was introduced (validated by the Gate 2
  strict Pages build and internal-link check).
- **Classifies:** every one of the 17 candidate files above to a plausible
  owning rule, based on a contextual read of each occurrence, not keyword
  count alone. No file was found asserting a new or competing rule; every
  occurrence describes or points at an existing authority (the role/gate
  split in `roles-and-gates.md`, or the advisory/blocking framework in
  `tool-authority-boundary-rule.md`).
- **Routes forward:** a more rigorous per-matching-line classification (all 53
  occurrences individually, with an explicit owner path added where one is
  currently only implied) is tracked as
  [issue #74](https://gitlab.com/janipaijanen/agents-govern-framework/-/work_items/74)
  — a separately prioritized, non-release-blocking follow-up, not folded
  into this release.
