---
id: public-governance-philosophies-page
status: approved
topic: public-governance-philosophies-consultative-page
gate1_approved_by: governor-reviewer
gate1_approved_at: "2026-07-20T21:56:01Z"
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: documentation-only change with no code, schema, credential, or
      access-control surface; nothing for infosec-sentinel to review.
  - role: contribution-auditor
    status: waived
    reason: single-agent Gate 1 planning session; no contribution-auditor
      available to consult directly. The citation-accuracy and
      normative/informative boundary claims this plan makes are exactly the
      kind of claim contribution-auditor review should verify — flagged here
      for Gate 2 review rather than skipped silently.
---

# Plan: publish "Philosophical lenses on AI governance" as a Concepts page

**Date:** 2026-07-20T21:44:00Z
**Status:** GATE 1 APPROVED (governor-reviewer, 2026-07-20T21:56:01Z — MR !268
merge, following confirmation of the `r2sr`→`gmrh` routing correction).
Gate 2 implementation (page content, per-lens citation verification per the
table below, nav/cross-link wiring) is open work, not yet started.
**Triggering handoff:**
`docs/coordination/20260720T213845Z-coder-tester-hhh6.md` — Codex relaying
a governor-surfaced suggestion to publish the `## Philosophies` section of
`docs/sources.md` as a public page, for both human and AI readers evaluating
AI governance generally.

---

## Intent

`docs/sources.md` is a working research notebook (mixed verification status,
not meant for external readers) that has accumulated a `## Philosophies`
section: six philosophical/organizational-theory lenses (legibility critique,
accountability sinks, automation-bias/de-skilling, Goodhart's law,
psychological safety vs. surveillance, proceduralism vs. outcome-based
regulation) that each expose a governance failure mode AGF's own design
responds to. These are currently useful only to whoever reads the raw
notebook file.

Publish a new page, **Concepts → Philosophical lenses on AI governance**,
presenting each lens as a compact, repeatable unit: the idea, the failure
mode it names, the design question it prompts of *any* governance system
(including AGF), and an evidence-status label. The page is explicitly
**consultative, not normative** — it gives readers (human or AI) review
prompts to challenge a governance design; it creates no obligations, grants
no blocking authority, and does not silently amend `agf/rules/`.

## Why this is worth a Gate 1 plan, not a fast-track edit

Per `simplicity-rule`'s calibration table, a single-file doc typo/update is
fast-track-eligible; this is not that. It touches: a new page, the nav
config (`mkdocs.yml`), and cross-links from at least three existing pages
(`should-you-adopt.md`, `position.md`, `ai-evaluation-dossier.md`) — more
than one module by `development-workflow-rule`'s own trigger ("touches...
more than one file"). It also makes source-verification and
normative-vs-informative boundary judgment calls that `hhh6` explicitly asks
be settled at Gate 1, not improvised during Gate 2.

## Audience-tone note (flagged, not silently resolved)

`docs/documentation-audience-rule.md` states the public site targets
"business users... and system administrators." The site's actual existing
Concepts pages (`position.md`, `should-you-adopt.md`) are written for
developers and evaluators in a direct, factual, technical register —
matching `marketing-tone-rule`'s "Developers (README, plans)" row, not its
business-user row. This plan follows the site's **demonstrated** convention
(the existing Concepts section) rather than the rule's stated generic
audience, per `uncertainty-and-scope-rule`: *"If a rule conflicts with what
the code already does: point out the conflict explicitly rather than
silently resolving it in either direction."* Flagged here for the governor;
not something this plan proposes to fix — that would be a separate,
unrelated rule-text change.

## Affected files

- **New:** `agf/docs/site/docs/concepts/philosophical-lenses.md` (the page).
- **`agf/docs/build/mkdocs.yml`** — add the new page under the existing
  `Concepts` nav section, after `evidence-and-learnings.md`.
- **`agf/docs/site/docs/start/should-you-adopt.md`** — one cross-link, near
  the existing "wait if" / risk-acknowledgment material, not a new section.
- **`agf/docs/site/docs/concepts/position.md`** — one cross-link, near
  scope/guarantee-boundary language.
- **`agf/docs/site/ai-evaluation-dossier.md`** — one cross-link, since this
  page is explicitly written for external evaluators who need challenge
  material.
- **Not touched:** `docs/sources.md` stays as the internal research
  notebook; it is not published, only used as the source-verification
  starting point. `.claude/settings.local.json` and
  `tooling/elixir/mix.lock` are out of scope per `hhh6`'s `do_not_touch`
  and untouched by this plan regardless.

Migration needed: **no** — pure documentation addition, no schema or code
change, `downstream-impact-rule` classification: **EDITORIAL** if judged as
"new optional page" — actually **ADDITIVE** (new capability: a new public
page with citable content that did not exist before), not BREAKING and not
EDITORIAL, since it adds a new documented surface rather than only changing
wording of an existing one.

## Content design — per-lens unit structure

Each of the six lenses gets exactly this shape (matches `hhh6`'s recommended
structure):

1. **The idea** — one to two sentences, plain language.
2. **The governance failure mode it exposes** — stated as a risk, not a
   AGF-specific claim.
3. **The practical design question it prompts** — phrased as a question a
   reviewer could ask of *any* governance system, AGF included.
4. **Evidence-status label** — per `reference/maturity-vocabulary.md`'s
   Evidence Status table (`Theoretical`, `Empirically informed`, or
   `Empirically validated`). Verified per-lens below, not assumed from
   `sources.md`'s existing informal notes.

## Required authority and evidence boundary (opening section of the page)

The page opens with an explicit boundary statement, before any lens content:

> These lenses are consultative review prompts, not AGF rules. They help
> any reader — human or AI — challenge a governance design, including this
> one. They create no obligations and do not grant blocking authority.
> Binding authority lives in named rules (`agf/rules/`) and human gate
> decisions, never in this page.

This directly satisfies `hhh6`'s bright-line requirement and
`implementation-honesty-rule`'s prohibition on overclaiming — a structured
page is more likely to be read as instruction by an AI reader specifically,
so the boundary needs to be stated, not implied.

## Source verification — per lens, done before publication (Gate 2 task)

Per `verifiable-claims-rule`, every citation must be independently
verifiable before the page ships. This plan does **not** complete
verification — it names the exact check each lens needs, as Gate 2
implementation work:

| Lens | Citation to verify | Status in `sources.md` today | Proposed evidence-status |
|---|---|---|---|
| Legibility critique | Scott, *Seeing Like a State* (Yale UP, 1998) | Already labeled "theoretical" | Theoretical |
| Accountability sinks | Davies, *The Unaccountability Machine* (2024) | No existing label — needs one added | Theoretical (single-source concept, not independently corroborated in AGF's own use yet) |
| Automation bias / de-skilling | Bainbridge, "Ironies of Automation" (*Automatica*, 1983) + automation-bias literature | Noted "evidence-backed, cite" | Empirically informed (peer-reviewed literature exists, but AGF's own gate-design application of it is not independently corroborated across projects) |
| Goodhart's law | Goodhart (1975 origin; widely restated) | No formal citation in `sources.md` | Theoretical (a well-known heuristic, not a single empirical study) |
| Psychological safety vs. surveillance | Edmondson (Harvard Business School, psychological safety research program) | Marked "[theoretical]" | Theoretical |
| Proceduralism vs. outcome-based regulation | Comparative regulatory-theory framing, not a single citation | No citation, framing only | State as framing/comparison, not a citation — use hedged language ("many regulatory regimes trend outcome/risk-based"), per `verifiable-claims-rule`'s hedging option rather than inventing a citation |

Each row's citation must resolve to a real, checkable publication (author,
title, year, publisher/venue) before the page ships — a broken or
unverifiable citation on a public page is exactly the credibility risk
`verifiable-claims-rule` exists to prevent. This table is the Gate 2
implementer's verification checklist, not a claim that verification is
already done.

## Test strategy

- `mkdocs build -f agf/docs/build/mkdocs.yml --strict` must pass (the
  project's existing docs quality gate — broken internal links fail the
  build by design, per `docs/build/README.md`).
- Manual citation-verification pass against the table above — each citation
  resolves to a real, checkable source. Record the verification (source
  found, date checked) either in the commit message or a short note at the
  bottom of the new page's Gate 2 review, not silently assumed.
- Gate 2 human review specifically checks: (a) the boundary statement is
  present and unambiguous, (b) no lens content reads as a new AGF rule or
  guarantee, (c) evidence-status labels match the table above or are
  corrected with reasoning if changed.

## Risks

- **Overclaiming risk (the central risk this plan exists to manage):** a
  structured, citation-bearing public page inherently reads with more
  authority than the working notebook it's drawn from. Mitigated by the
  explicit boundary statement (required, not optional) and the
  per-lens evidence-status labels (no lens ships unlabeled).
- **Citation drift:** `sources.md`'s existing notes are informal working
  annotations, not verified citations. Treating them as already-verified
  would be exactly the shortcut `verifiable-claims-rule` forbids. The
  verification table above exists to prevent that shortcut, not to paper
  over it.
- **Scope creep into `sources.md`'s other material:** `sources.md` also
  contains "Pre-identified concepts" (a numbered roadmap list) and "New
  issue candidates" — unrelated to the Philosophies section and explicitly
  out of scope per `hhh6`'s own instruction ("should not bundle the
  separate roadmap concepts... or turn every lens into a new framework
  mechanism"). This plan does not touch that material.
- **Nav placement risk:** placing this under Concepts (developer/evaluator
  register) rather than a more general-audience section is a judgment call
  — see the audience-tone note above — and is the governor's to confirm,
  not this plan's to decide unilaterally.

## Rollback

New page + nav entry + three cross-link additions; a straightforward
single-purpose revert if the governor decides against publication after
implementation, or if Gate 2 citation verification surfaces a lens that
cannot be adequately sourced (in which case that lens is dropped from the
page rather than blocking the other five).
