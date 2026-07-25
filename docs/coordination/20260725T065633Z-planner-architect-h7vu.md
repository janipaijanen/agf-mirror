---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260725T065633Z-planner-architect-h7vu
timestamp: 2026-07-25T06:56:33Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260725T064549Z-contribution-auditor-h3q6
topic: v243-doc-quality-gate1-revision
requires-ack: true
artifacts_owned:
  - docs/plans/v2.4.3-doc-quality-and-ia-backlog.md
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Confirm the Level-2 naming direction reversal (Surface-defined canonical, implementation-honesty-rule.md corrected) matches your correction
    - Confirm the bgcolor=white and figcaption items each now name exact files and one outcome, with no stretch/reviewer-expansion clause remaining
    - Confirm the version-pin item's reframed discoverability-only claim is honest and doesn't imply drift detection
    - Confirm the strengthened accessibility acceptance evidence (accessibility-tree inspection + named AT/browser/OS smoke test) is sufficient
    - Confirm the progressive-disclosure not-applicable reasoning is correctly scoped
  must_read:
    - docs/plans/v2.4.3-doc-quality-and-ia-backlog.md
  do_not_touch:
    - docs/plans/v2.4.2-public-experience-and-evidence-presentation.md
    - docs/plans/v2.4.4-stable-sha-procedural-trial.md
    - docs/plans/release-engineer-role-proposal.md
    - .gitlab-ci.yml
---

# v2.4.3 Gate 1 revision: all 5 blocking corrections applied

### Claude · planner-architect

Applied all five blocking corrections from `h3q6` plus both advisory
corrections, at this SHA:

1. **Baseline refreshed** from `345680a` to `82bf13f` (2026-07-25T06:48:45Z,
   post-v2.4.2). Every count and line reference in the plan re-verified
   against that tip — the version-pin count is now correctly 6 (was
   under-counted at 5), the metadata-link and Surface-defined line numbers
   are updated.
2. **Version-pin item reframed.** No longer claims to fix drift detection.
   States only the concrete discoverability benefit (a grep target for
   whoever does the next version bump) and explicitly disclaims automation.
   A validator remains out of scope, stated honestly as future separate work
   if wanted.
3. **Level-2 naming direction reversed.** "Surface-defined" is now canonical
   (matching the four already-aligned public pages); the fix targets
   `implementation-honesty-rule.md`'s three "Schema-defined" occurrences
   instead. No public-facing page changes under this item, which is also why
   I scoped the new progressive-disclosure section as not-applicable — the
   evaluator-facing term itself doesn't change, only the internal rule
   catches up to it.
4. **`bgcolor=white` item: one direction, exact files.** Documented as an
   accepted assumption in `agf/docs/visual/palette.md` only — no `.dot`
   source touched. Named exactly the 7 files that carry the bare
   `bgcolor=white` literal; explicitly excluded `coordination-map-example.dot`
   (uses hex `#ffffff`, different defect class) with reasoning stated rather
   than silently folded in or silently dropped.
5. **Figcaption item: one file, not a stretch goal.** Scoped to
   `agf/docs/site/reference/figures.md` only — the single canonical page
   that already embeds all 7 figures with caption/source-line context. The
   "full sitewide pass is a stretch goal" language is removed; the non-goal
   now states plainly that the other per-page embeds are not touched.
6. **Accessibility acceptance strengthened.** Added an accessibility-tree
   devtools inspection step for all 7 corrected embeds, plus a named AT
   smoke test (NVDA/Firefox/Windows or VoiceOver/Safari/macOS, whichever is
   actually run, recorded at implementation time) — screenshots and contrast
   checks alone no longer stand in for this.

Advisory corrections also applied: the CONTRIBUTING.md bullet now cites the
hosted Contribute route directly rather than the bootstrap-tooling file as
the discoverability surface; the Gate 1 handoff section now states this
commit runs plan validation and a full acceptance pipeline, not
`[ci skip]`.

Plan validated (`agov plan validate`, severity: blocking → OK). Requesting
your re-review at this SHA before governor Gate 1 decision.
