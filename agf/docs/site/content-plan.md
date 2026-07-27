# Content plan: end-user documentation site

Control artifact for the GitLab Pages documentation lane (per Gate 1
approval, seq 103). Pages are drafted only after their row here defines
audience, objective, prerequisites, and review needs.

## Audiences

| Audience | Situation | What they need from the site |
|---|---|---|
| **Evaluator** | Heard of the framework; deciding whether to spend an afternoon on it | Honest positioning, evidence, non-use cases, cost of adoption — in minutes |
| **New adopter** | Decided to try it; has a real project | A safe, ordered first path: install, first governed change, what changed in their repo |
| **Operating adopter** | Governed project in production use | Day-to-day loop, review/release expectations, upgrade hygiene |
| **Advanced adopter** | Customizing roles, rules, or integrating validators in CI | Contracts, schemas, customization boundaries |
| **Exiting adopter** | Framework no longer fits | Clean removal path, verification, history preservation |

## Learning path and page inventory

Reading order is the navigation order. Downstream-impact and
exit/decommission appear in the **adopt** stage — before any advanced
topic — so no reader commits without seeing the cost and the way out.

| # | Page | Audience | Level | Reader can do/understand afterwards | Prerequisites | Figures | Fact-check claims |
|---|---|---|---|---|---|---|---|
| 1 | `index.md` (landing) | Evaluator | — | What the framework is, its Provisional v2.0 status, where to go next | None | governance-loop | Adoption evidence counts; status labels |
| 2 | `value.md` | Evaluator | — | The problem framing and what adoption enables | None | — | All comparative claims hedged per verifiable-claims rule |
| 3 | `use-cases.md` | Evaluator | — | Whether their project resembles a credible use case — or a non-use case | None | — | Use cases trace to learning records |
| 4 | `docs/index.md` | All | — | Which reading path matches their situation | None | docs-learning-path | — |
| 5 | `docs/start/should-you-adopt.md` | Evaluator | Beginner | Make an informed adopt / wait / skip decision | None | — | Maturity claims use 5-level model |
| 6 | `docs/adopt/install-and-layouts.md` | New adopter | Beginner | Install the files under the supported `agf/` directory | Decision made | — | File list matches MANIFEST |
| 7 | `docs/start/first-governed-change.md` | New adopter | Beginner | Run one small change through Gate 1 → implement → Gate 2 | Install done | gate-pipeline | Command names exist in tooling |
| 8 | `docs/adopt/downstream-impact.md` | New adopter | Beginner | Know every file, CI job, and obligation adoption introduces | 7 | adoption-impact-map | File/CI inventory matches ADOPTING.md |
| 9 | `docs/adopt/exit-and-decommission.md` | New + exiting | Beginner | Remove or downgrade adoption cleanly and verify removal | 7 | exit-flow | Steps match the single `agf/` boundary |
| 10 | `docs/concepts/roles-and-gates.md` | New adopter | Intermediate | Explain agents, authority, gates, coordination channel | 6 | agent-communication, gate-pipeline | Role list matches agents/MANIFEST.yaml |
| 11 | `docs/concepts/evidence-and-learnings.md` | Operating | Intermediate | Write a learning record; distinguish upstream vs adopter-owned | 10 | evidence-lifecycle | foundry.yaml replace-on-upgrade behavior |
| 12 | `docs/operate/day-to-day.md` | Operating | Intermediate | Run the daily loop: claims, handoffs, gates, completions | 10 | governance-loop | — |
| 12a | `docs/operate/runbook.md` | Operating | Intermediate | Choose the authoritative operating procedure without knowing repository layout | 10 | — | Routes and authority match canonical pages and role manifest |
| 12b | `docs/operate/session-continuity.md` | Operating | Intermediate | Checkpoint graceful interruption and recover abrupt loss without inventing state | 12a | — | Commands exist; publication and uncertainty boundaries match coordination rules |
| 12c | `docs/operate/poll-and-coordinate.md` | Operating | Intermediate | Poll coordination, forge changes/comments, and issues/comments, then advance authorized work | 12a | — | Commands exist; no autonomous scheduling claim |
| 13 | `docs/operate/review-and-release.md` | Operating | Intermediate | Conduct Gate 2, release via MR, upgrade, keep audit trail | 12 | gate-pipeline | Release steps match release-via-merge-request rule |
| 14 | `docs/advanced/contracts-and-validators.md` | Advanced | Advanced | Wire schemas + `agov` checks into their CI | 10 | — | Validator names match current tooling |
| 15 | `docs/advanced/customizing-the-framework.md` | Advanced | Advanced | Know what may be tuned vs what breaks contract semantics | 14 | — | Boundaries match contract-stability.md |
| 16 | `future/evolution.md` | All | Future | Distinguish shipped capability from non-binding hypotheses | None | — | Every item labelled Provisional/hypothesis |

## Status labels

Every page carries one of these labels in its first paragraph:

- **Canonical practice** — backed by v1.x behavior and shipped tooling.
- **Provisional** — v2.0 structure/content not yet promoted to canonical.
- **Hypothesis** — future/evolution material; explicitly non-binding.

The landing page states once, canonically: *v1.x is canonical; v2.0 is
Provisional.* Other pages link to that statement rather than restating it
(message-integrity rule).

## Glossary needs

Terms that must be defined on first use and consistently afterwards:
*gate, coordination channel, learning record, codename, jurisdiction,
fast track, compliance profile, `agf/` layout, Provisional.* The canonical
definitions live in `docs/paper/agents-govern-framework.md` §Definitions;
site pages link or quote verbatim.

## Review checkpoints

1. **Content-plan review (this file)** — part of Gate 2 for the v2.0
   structure MR.
2. **Per-page fact-check** — the claims listed in the table column are
   verified against code/tooling before Gate 2 (fact-checker advisory if
   governor activates it).
3. **Marketing-tone pass** — landing/value/use-cases pages checked against
   the marketing-tone rule: no vendor negatives, no unhedged superlatives,
   positive framing, non-use cases present.
4. **Visual gate** — built site opened and visually inspected (navigation,
   figures legible, Provisional banners visible) per
   visible-artifact-verification rule.

## Pedagogical principles

- One concept per page; the page states its objective in the first two
  sentences.
- Every figure is preceded by text saying what it shows and what to take
  away.
- Beginner pages assume no prior framework vocabulary; intermediate pages
  may assume pages 1–9; advanced pages may assume the concepts chapter.
- Cost and reversibility are taught before depth: impact (page 8) and exit
  (page 9) precede concepts, operation, and advanced topics.
