# Summary

Refreshes the academic working paper evidence snapshot for
`docs/academic-paper-refresh`.

This MR updates `docs/academic/governing-the-governors.tex` so the paper no
longer describes the older `bd512ed` branch snapshot and instead reflects the
current `main` baseline after the Elixir migration, Gate 7 closure, and stale
handoff reconciliation.

# Scope

- Updates the paper date to 2026-06-15.
- Replaces the analyzed snapshot with current `main` commit `b641c44`.
- Updates repository artifact counts:
  - 348 coordination messages
  - 34 plan files
  - 6 JSON schemas
  - 49 Elixir library/Mix task files
  - 26 Elixir test files
- Corrects reproducibility paths and commands:
  - `agf/schemas/` instead of `spec/schemas/`
  - `mix test`
  - `agov coord validate <file>`
  - `agov plan validate <file>`
- Tightens Table I evidence cells with concrete files, sequence IDs, test
  files, and validator modules.
- Updates maturity-table wording for current manifest authority and stale
  handoff reconciliation evidence.
- Adds the Gate 2 handoff:
  `docs/coordination/01KV6M8H2DCGECCZ8MNC5ZZJE0-coder-tester.md`.

# Governance Trail

- Academic reviewer activation:
  `docs/coordination/00117-governor-reviewer.md`.
- Prior academic review:
  `docs/academic/reviews/2026-06-13-academic-content-readability-layout-review.md`.
- Gate 2 handoff:
  `docs/coordination/01KV6M8H2DCGECCZ8MNC5ZZJE0-coder-tester.md`.

# Implementation Notes

This is a focused source refresh. It does not attempt a full academic rewrite,
table redesign, figure layout pass, or bibliography expansion.

The stale branch `docs/academic-paper-draft` was intentionally not merged or
rebased because it predates the current `agf/` layout and carries a very large,
obsolete product diff.

# Verification

- `git diff --check` passed.
- `mix agov.validate` passed.
- `mix test` passed: 197 passed, 0 failures.
- `agov coord validate docs/coordination/01KV6M8H2DCGECCZ8MNC5ZZJE0-coder-tester.md`
  passed.

# Review Notes

- The PDF was not regenerated in this environment.
- Local `pdflatex`, `bibtex`, `latexmk`, `tectonic`, and `pandoc` are absent.
- The documented `docker.io/kjarosh/latex:2025.1-medium` image is runnable
  through Podman here, but this local copy did not expose `pdflatex` or
  `latexmk`.
- Merge as source-only progress if acceptable, or request a follow-up
  environment/tooling fix before accepting a refreshed PDF artifact.
- The full test run still emits two pre-existing warnings:
  - unused `@tag` before `describe` in `test/agov/coord_config_test.exs`
  - unused default argument in private `claim/3` in
    `test/agov/claim_tracker_test.exs`

# Out of Scope

- Rebuilding or visually reviewing the PDF.
- Updating generated figures.
- Full academic peer-review rewrite.
- Reworking the Markdown draft source.
- Changing validators, schemas, or governance behavior.
