---
kind: handoff
from: coder-tester
to: governor-reviewer
model: gpt-5-codex
id: 20260622T001623Z-coder-tester-4tss
timestamp: 2026-06-22T00:16:23Z
immutable: true
references-to:
  - 20260622T001327Z-governor-reviewer-7r84.md
  - 20260622T001016Z-coder-tester-8a17.md
  - 20260621T235800Z-planner-architect-g1hf.md
topic: principal-delegation-v2x-docs-gate2
requires-ack: false
artifacts_owned:
  - agf/docs/site/docs/concepts/roles-and-gates.md
  - agf/docs/site/docs/start/adopter-faq.md
  - agf/docs/site/docs/start/should-you-adopt.md
  - docs/coordination/20260622T001623Z-coder-tester-4tss.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Review the docs-only principal-bound AI delegation convention implementation for Gate 2."
    - "Confirm the implementation stays within the approved three-page scope."
  must_read:
    - docs/plans/principal-delegation-v2x-docs.md
    - docs/coordination/20260622T001327Z-governor-reviewer-7r84.md
    - agf/docs/site/docs/concepts/roles-and-gates.md
    - agf/docs/site/docs/start/adopter-faq.md
    - agf/docs/site/docs/start/should-you-adopt.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 2 handoff: principal-bound AI delegation docs

Implementation is ready for governor-reviewer Gate 2 review on branch
`plan/principal-delegation-v2x-docs`.

Changes made:

- Added a `Delegated AI work` section to `roles-and-gates.md`.
- Added an FAQ entry, `Can an assistant work on behalf of a human?`.
- Added a short residual-risk paragraph to `should-you-adopt.md`.

Scope notes:

- Docs-only implementation.
- No schema changes.
- No new coordination `kind` values.
- No tooling changes.
- Example identifiers are synthetic (`@example-member`, `@docs-team`,
  `@on-call-reviewer`, and illustrative `@someone` phrasing only).
- The docs state that v2.x delegation and contestation objects are not
  semantically enforced by tooling.
- The docs keep Gate 1, Gate 2, release approval, and binding risk acceptance
  separate from delegated preparation work.
- Anti-surveillance language is intentionally explicit: AGF does not ship
  native productivity reports, principal ranking, review-depth scores, or
  HR-exportable datasets, but repository-access correlation risk remains.

Verification performed:

```bash
AGOV_SCHEMA_DIR="$PWD/agf/schemas" agov plan validate docs/plans/principal-delegation-v2x-docs.md
AGOV_SCHEMA_DIR="$PWD/agf/schemas" agov coord validate docs/coordination/20260622T001327Z-governor-reviewer-7r84.md
uvx --from 'mkdocs>=1.6,<2' \
  --with 'mkdocs-material>=9.5,<10' \
  --with 'mkdocs-awesome-pages-plugin>=2.9,<3' \
  --with 'mkdocs-gen-files>=0.5,<1' \
  mkdocs build -f agf/docs/build/mkdocs.yml --strict
```

Results:

- Plan validation: blocking OK.
- Gate 1 decision coordination validation: blocking OK; advisory warning only
  that the governor-authored note has no `model` field.
- MkDocs strict build: exit 0. Existing informational notes reported that
  `reference/figures.md` and `reference/palette.md` are present but not in nav.

Known local noise:

- Commands in this linked worktree print the existing `mise.toml` trust warning.
  It did not affect validation or the MkDocs build result.
