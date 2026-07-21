---
id: philosophical-lenses-learning-additions
status: pending-gate1
topic: philosophical-lenses-learning-additions
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: documentation-only change with no code, schema, credential, or
      access-control surface; nothing for infosec-sentinel to review.
  - role: contribution-auditor
    status: waived
    reason: single-agent Gate 1 planning session; no contribution-auditor
      available to consult directly. The citation-accuracy claims this plan
      makes are exactly the kind of claim contribution-auditor review should
      verify — flagged here for Gate 2 review rather than skipped silently.
---

# Plan: add six learning/knowledge-focused lenses to the Philosophical lenses page

**Date:** 2026-07-20T23:56:00Z
**Status:** PENDING GATE 1 APPROVAL
**Triggering context:** user-requested addition, drafted through several
rounds of question-then-verify-then-draft in the same session that
produced `docs/plans/public-governance-philosophies-page.md` (the
six-lens page, MR !278). That plan named six specific lenses. This
addition was implemented directly onto the same branch without a
preceding Gate 1 pass — Codex's review
(`docs/coordination/20260720T234823Z-coder-tester-8qsh.md`) correctly
blocked that as an unapproved scope expansion mid-Gate-2-review. Agreed
(`docs/coordination/20260720T235247Z-coder-tester-tdhr.md`) and reverted
the content from MR !278 (commit `55374d6`). This plan is the corrected,
proper process for the same content — Gate 1 first this time, not after.

## Intent

Add six more lenses to `agf/docs/site/docs/concepts/philosophical-lenses.md`
(once MR !278's six-lens version has merged), forming a second, explicitly
labeled group focused specifically on how governance systems learn — or
fail to learn — from their own evidence, distinct from the first group's
oversight/accountability focus:

1. Observe-Orient-Decide-Act (OODA) — Boyd
2. Plan-Do-Check-Act (PDCA) — Shewhart/Deming
3. Single-loop versus double-loop learning — Argyris & Schön
4. Espoused theory versus theory-in-use — Argyris & Schön
5. The Cynefin framework — Snowden & Boone
6. Knowledge creation and the concept of *ba* — Nonaka, Takeuchi, Konno

Same structure, same boundary statement, same evidence-status vocabulary
as the approved six-lens page — this plan does not propose changing the
page's format, only adding to its content.

## Why this needs its own Gate 1 plan, not an addendum to the existing one

Per Codex's review (`8qsh`), accepted in full: these six lenses are not
in `docs/sources.md`'s Philosophies inventory (the source material the
original plan was built from), they roughly double the page, and they
form a coherent second topic that can and should be reviewed
independently of the already-cleared six-lens implementation. Splitting
avoids re-litigating settled review findings and keeps each MR answerable
with a single yes/no question, per this project's own branch-sizing
discipline.

## What changed from the reverted attempt — content fixes, committed up front

The reverted commit (`f545df5`, no longer live) had five real gaps,
found by Codex's review. This plan commits to fixing all five before
implementation, not after:

1. **OODA's historical claim needs a citation.** The Korean War F-86
   versus MiG-15 analysis that grounds Boyd's model is real (verified via
   search) but was previously asserted without a citable source. Add:
   Coram, Robert, *Boyd: The Fighter Pilot Who Changed the Art of War*,
   Little, Brown, 2002 — the standard biographical source for this
   specific historical claim.
2. **PDCA's historical claims need citations.** Add: Shewhart, W.A.,
   *Economic Control of Quality of Manufactured Product*, D. Van
   Nostrand, 1931 (origin of the cycle) and Deming, W.E., *The New
   Economics for Industry, Government, Education*, MIT Press, 1993
   (source for the PDCA-to-PDSA "Check" to "Study" rename).
3. **Cynefin's uncited "Academy of Management recognition" detail is
   dropped.** Not load-bearing to the lens's point; removed rather than
   sourced.
4. **"Well established" / "widely cited" popularity language is removed
   or replaced.** Bibliographic existence is not evidence of empirical
   validation. Evidence-status paragraphs state what is cited and what
   is not, without asserting how well-known a source is unless that
   claim is itself sourced (matching the fix already applied to the
   automation-bias lens on the six-lens page).
5. **The *ba* section's "one-way" mischaracterization is corrected.**
   This and any coordination channel this framework's own append-only
   design produces is asynchronous, not one-way — participants reply and
   cross-reference constantly (as this exact multi-message review thread
   demonstrates). The lens will state the critique as weak tacit-to-tacit
   transfer specifically, not directionality.

## Downstream impact (three-circle statement)

```
Impact:
  Foundry:          No. Pure documentation addition to a page not yet on
                     main (depends on MR !278 merging first).
  Direct adopters:  ADDITIVE. New content on an already-additive page;
                     no existing claim, link, or contract changes.
  Downstream:       None. No schema, code, or tooling surface touched.
  Migration:        None required.
  Evidence:         docs/coordination/20260720T234823Z-coder-tester-8qsh.md
                     (the review that blocked the unapproved version and
                     named the required fixes).
```

## Test strategy

Same as the approved six-lens plan: `mkdocs build -f
agf/docs/build/mkdocs.yml --strict` must pass; live citation verification
for all new sources before the page ships (already done for this plan —
all six citations confirmed via search during planning, listed above with
publisher/year); Gate 2 review checks the same three criteria as before
(boundary statement holds across all twelve lenses, no lens reads as an
AGF-specific claim, evidence-status labels are vocabulary-valid) plus the
five fixes above are actually present in the diff, not just promised in
this plan.

## Sequencing

Depends on MR !278 (the six-lens page) merging to main first — this
plan's diff target doesn't exist on main yet. Implementation branch will
be created off `main` post-merge, not stacked on the still-open !278.

## Risks

- **Same overclaiming risk as the base page**, mitigated the same way:
  explicit boundary statement, evidence-status label per lens, no lens
  claims more than its cited source supports.
- **Repeat-mistake risk (self-identified):** this is the second draft of
  this exact content after the first was reverted for insufficient
  citation rigor. The fixes above are specific and checkable at Gate 2 —
  reviewers should verify the five fixes are actually in the diff, not
  accept a repeated claim that they were applied.

## Rollback

Six-section addition to an already-additive page; trivial single-purpose
revert if the governor declines, following the same pattern as the
reversion already executed once in this same episode.
