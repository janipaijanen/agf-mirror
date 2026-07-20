# docs: add audience-taxonomy callout and documentation map (MR-1)

Branch: `docs/audience-taxonomy-callout-mr1`

## Status

- **Pending:** Gate 2 review by governor-reviewer
  (`docs/coordination/20260720T145316Z-coder-tester-ejsk.md`).
- **Codex collaboration:** branch rebased on current `main` after !228 merged;
  advisory cross-check recorded in
  `docs/coordination/20260720T145632Z-coder-tester-mbtr.md`.
- **Blocking Claude:** nothing. Next action (if any) will arrive as a
  pushed coordination handoff.

## Summary

MR-1 of 6 from the approved documentation audience-boundary plan
(`docs/plans/docs-audience-boundary-proposal.md`, Gate 1 approved in
`20260720T143255Z-governor-reviewer-s6hg.md`), assigned to Claude via
`docs/coordination/20260720T144305Z-coder-tester-hpsg.md`.

Adds the concept-heavy first slice: a "How to read this site" section on
`agf/docs/site/docs/index.md` (before the existing stage diagram) stating
the plan's four-bucket taxonomy — Human decision / AI-operator
instructions / Shared concept / Reference — plus a new
`docs-audience-map` diagram mapping example pages to each bucket.

## Files changed

| File | Change |
|---|---|
| `agf/docs/site/docs/index.md` | New "How to read this site" section, ahead of "## The path" |
| `agf/docs/figures/src/docs-audience-map.dot` | New DOT source, styled consistent with `operating-responsibility-map.dot` |
| `agf/docs/figures/docs-audience-map.svg` | Rendered SVG (build-output location) |
| `agf/docs/site/reference/figures/docs-audience-map.svg` | Rendered SVG (site source-of-truth location, per the existing dual-copy convention — confirmed byte-identical to the other copy) |
| `docs/coordination/20260720T145316Z-coder-tester-ejsk.md` | Gate 2 handoff to governor-reviewer |
| `docs/coordination/20260720T145632Z-coder-tester-mbtr.md` | Codex advisory review after rebasing on top of merged MR-4 |

## Scope boundary

Per the assigning handoff: does not touch `install-and-layouts.md` (Codex's
parallel MR-4 claim) and does not relabel other pages (MR-2's scope, which
depends on this MR landing first).

## Review checklist

- [x] `mkdocs build -f agf/docs/build/mkdocs.yml --strict` passes (only
      pre-existing, unrelated warnings)
- [x] `git diff --check` passes
- [x] New diagram rendered to PNG and visually inspected before finalizing
      (per `visible-artifact-verification-rule`) — one cosmetic issue found
      and fixed (stray legend placeholder)
- [x] Both SVG copies confirmed byte-identical
- [x] No `Co-authored-by` trailers in commit
- [x] Rebased after !228; diff no longer touches `install-and-layouts.md` or
      MR-4 coordination files
