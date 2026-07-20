# plan: documentation audience-boundary proposal (Gate 1)

Branch: `plan/docs-audience-boundary-proposal`

## Status (as of 20260720T143255Z-governor-reviewer-s6hg)

- **Gate 1:** approved by governor-reviewer, Codex-scribed in
  `docs/coordination/20260720T143255Z-governor-reviewer-s6hg.md`.
- **Fact-checker:** approved in
  `docs/coordination/20260720T143237Z-fact-checker-kqkg.md`.
- **Blocking Claude:** nothing right now. If Claude needs to act next,
  expect a pushed `kind: handoff` coordination file addressed to
  `planner-architect`; after this plan MR lands, implementation work should
  follow the recorded Claude/Codex split.
- **Advisory cycle so far:** contribution-auditor approved (`prbd`);
  fact-checker withheld twice on evidence-precision grounds, both fixed
  (`ag3k`→`75sq`, `qr8m`→`j2sq`), then approved (`kqkg`).

## Summary

Gate 1 proposal responding to the handoff at
`docs/coordination/20260720T125518Z-coder-tester-kpmn.md`: the docs site
currently implies human adopters must personally learn every AGF ceremony
and CLI command. Proposes a four-bucket audience taxonomy (Human-decision /
AI-operator / Shared-concept / Reference), backed by a concrete finding
corrected through advisory review before Gate 1 — planner-architect
self-review, contribution-auditor review, and independent fact-checker passes
(see coordination notes `3rs1`, `prbd`, `ag3k`, `75sq`, `qr8m`, `j2sq`,
`kqkg`): of the site's 34 nav pages, 22 carry an explicit
`Audience:` clause; 20 of those name only human roles, and the 2 that
mention "AI assistants" still list the human role first and don't reframe
who primarily executes the page.

## What's in this branch

- **`docs/plans/docs-audience-boundary-proposal.md`**: the Gate 1 plan.
  Contains the audience-line evidence table, a responsibility-boundary
  matrix (human/AI/tooling/artifact), a full per-page classification against
  the four buckets, a reconciliation with two prior unresolved docs plans
  (`agents-govern-2.0-pedagogical-docs-restructure.md`, pending-gate1 since
  2026-06-13; `docs-assessment-2026-06-21.md`, active since 2026-06-21 —
  both largely superseded by already-landed work but never formally
  closed), a diagram plan that reuses two existing figures and proposes two
  new ones, a 6-MR implementation split, an HTML-comment acceptance-
  criteria coverage-marker convention (grep-checked, no CI strip step
  needed — comments are already invisible to readers), a conditional
  Claude/Codex work split, and 5 Gate 1 questions with current
  governor-reviewer direction recorded.
- **`docs/coordination/20260720T130837Z-planner-architect-phm5.md`**: the
  Gate 1 handoff to governor-reviewer.
- **`docs/coordination/20260720T134651Z-planner-architect-3rs1.md`**:
  planner-architect's self-correction (34 pages not 32; 2 pages already
  mention AI assistants) plus an open fact-checker-gap flag — governor's
  work-split note excludes the same planner session from certifying
  fact-checker for this plan.
- **`docs/coordination/20260720T134835Z-contribution-auditor-prbd.md`**:
  contribution-auditor advisory pass — found and fixed 3 unhedged
  superlatives, otherwise no blocking findings; fact-checker still open.
- **`docs/coordination/20260720T135454Z-fact-checker-ag3k.md`**: the
  independent fact-check (Codex, separate session, per governor's same-
  session restriction on this role) — caught that the first self-correction
  conflated "has a `*Status:*` line" with "has an `Audience:` clause" and
  undercounted by 2 pages (22 actual vs. 20 claimed). Withheld approval
  pending the fix.
- **`docs/coordination/20260720T141116Z-planner-architect-75sq.md`**: the
  fix, independently re-verified rather than taken on trust, routed back
  for fact-check confirmation.
- **`docs/coordination/20260720T141554Z-fact-checker-qr8m.md`**: formal
  handoff for the stale phrase-count table (developers/administrators/
  governors/evaluators/adopters rows still showed the pre-22-page tally).
  Fact-checker's independently-extracted numbers (12/7/7/4/4/2) matched
  Claude's own manual recount exactly before either side saw the other's
  numbers.
- **`docs/coordination/20260720T142703Z-planner-architect-j2sq.md`**: third
  fix applied — developers 12 (was 9), administrators 7 (was 3), governors
  7 (was 3), evaluators 4 (was 5, now excludes "developers evaluating"-style
  verb-form false positives), adopters 4 (unchanged), AI assistants 2
  (unchanged), plus 2 pages with non-standard phrasing ("people
  deciding...", "anyone evaluating...") not previously tabulated. Routed
  back for final fact-check confirmation.
- **`docs/coordination/20260720T143237Z-fact-checker-kqkg.md`**:
  fact-checker approval after independently confirming the final counts.
- **`docs/coordination/20260720T143255Z-governor-reviewer-s6hg.md`**:
  Gate 1 approval decision record.

## Non-goal

Does not rewrite the documentation site. Six follow-on MRs are proposed,
each gated on this plan's approval.

## Review checklist

- [x] `agov plan validate` passes (blocking)
- [x] `agov coord validate` passes (blocking)
- [x] `mkdocs build --strict` unaffected (plan/coordination files only, no
      docs-site content changed yet)
- [x] Four-bucket taxonomy agreed
- [x] Disposition of the two prior open docs plans agreed
- [x] Existing gate lifecycle diagram extension selected as first attempt
- [x] Claude/Codex work split recorded conditionally on final Gate 1
- [x] No `Co-authored-by` trailers in commit
