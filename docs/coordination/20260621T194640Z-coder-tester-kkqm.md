---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260621T194640Z-coder-tester-kkqm
timestamp: 2026-06-21T19:46:40Z
immutable: true
model: codex-gpt-5
references-to:
  - 20260621T193926Z-planner-architect-ah1n
  - 20260621T192432Z-coder-tester-4sh2
topic: docs-pre-release-blockers-gate2
requires-ack: true
artifacts_owned:
  - docs/coordination/20260621T194640Z-coder-tester-kkqm.md
  - agf/docs/site/index.md
  - agf/docs/site/reference/tooling-conformance.md
  - agf/docs/site/reference/environment-variables.md
  - agf/docs/site/docs/start/should-you-adopt.md
  - agf/docs/site/docs/adopt/install-and-layouts.md
  - agf/docs/site/docs/adopt/tooling-and-tokens.md
  - agf/docs/build/mkdocs.yml
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review Slice A pre-release documentation blockers.
    - Confirm the release-facing Rust tooling wording is accurate enough for v2.0 provisional docs.
    - Merge if approved.
  must_read:
    - docs/coordination/20260621T193926Z-planner-architect-ah1n.md
    - docs/plans/docs-assessment-2026-06-21.md
    - agf/docs/site/reference/tooling-conformance.md
    - agf/docs/site/reference/environment-variables.md
    - agf/docs/site/index.md
  do_not_touch:
    - Slice B nav/file-location decisions
    - Slice C session-continuity page draft
    - Slice D deferred benefits/CTA refinements
---

# Gate 2 handoff: docs pre-release blockers, Slice A

Branch: `docs/fix-pre-release-blockers`

This implements Slice A from planner-architect handoff
`20260621T193926Z-planner-architect-ah1n`.

## Summary

- Removed the false automatic Elixir fallback claim from the public home page.
- Reframed Tooling conformance around the release-facing Rust `agov` binary,
  with Mix scoped to source-checkout development use.
- Reworked Environment variables so Mix-relative defaults are not presented as
  adopter defaults.
- Deleted the jargon-heavy "More precisely" paragraph from `should-you-adopt`.
- Removed orphan "gateway assistants", "chat bridges", "policy engine", and
  "budget caps" wording from operational pages.
- Removed the duplicate "six months later" audit-trail claim from the home page.
- Added first-use glosses for fast track and active claim.
- Removed Visual palette and Figures from the public Reference nav while
  leaving the source pages/files in place.
- Marked the "one to two orders of magnitude" token-savings statement as a
  foundry estimate.

## Verification

- `git diff --check`: passed.
- `mkdocs build -f agf/docs/build/mkdocs.yml --strict`: passed.
- `agov coord validate docs/coordination/20260621T193926Z-planner-architect-ah1n.md`:
  blocking validation passed; advisory warning noted that the planner handoff
  lacks `model`.

MkDocs reports `reference/figures.md` and `reference/palette.md` as existing
outside nav. That is expected for this slice: the files remain available for
contributors, but are no longer primary adopter navigation.

## Review focus

- [ ] Public pages no longer imply release adopters have automatic Elixir/Mix
      fallback.
- [ ] Tooling conformance is framed as an adopter-facing Rust command surface,
      with development Mix use clearly scoped.
- [ ] Environment variable defaults no longer expose `tooling/elixir/` relative
      paths as adopter defaults.
- [ ] Prose removals do not erase needed meaning.
- [ ] Palette/Figures nav removal is acceptable while files remain in the docs
      tree.
