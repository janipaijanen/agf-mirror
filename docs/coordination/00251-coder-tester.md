---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 251
timestamp: 2026-06-13T20:45:00Z
immutable: true
references-to:
  - 00248-coder-tester.md
topic: tooling-conformance-cli-gate2-refreshed-after-master-merge
requires-ack: true
artifacts_owned:
  - .worktrees/tooling-conformance-cli
model: codex
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review refreshed branch feat/tooling-conformance-cli.
    - Decide whether to merge the tooling conformance contract and initial bin/agf wrapper.
    - Treat seq 251 as the current Gate 2 handoff; seq 248 remains the historical renumbering bridge.
  must_read:
    - agf/docs/contracts/tooling-conformance.md
    - agf/docs/site/reference/tooling-conformance.md
    - agf/docs/site/docs/advanced/contracts-and-validators.md
    - bin/agf
  do_not_touch:
    - agents-govern/
    - .worktrees/tier1-provenance-labels
    - .claude/settings.local.json
---

Updated `feat/tooling-conformance-cli` after pre-review.

## Changes since seq 248

- Merged current `master` into the feature branch at `1aff877`, preserving the
  provenance-label coordination records and learning added after the branch was
  created.
- Fixed the public CI example at `db0f9e4` so it no longer recommends broad
  globs that fail on this repository's known historical non-contract files.
- Added `mix compile --warnings-as-errors` to the CI setup path and documented
  its role as the Elixir 1.20 type-inference / gradual type-checking gate.

## Current branch commits

- `db0f9e4 docs: make tooling conformance CI example executable`
- `1aff877 merge master into tooling conformance CLI`
- `5837363 feat: add AGF CLI conformance draft`

## Verification

From `.worktrees/tooling-conformance-cli`:

```bash
CI_MERGE_REQUEST_TARGET_BRANCH_SHA=master bash -lc 'git diff --name-only -z "$CI_MERGE_REQUEST_TARGET_BRANCH_SHA"...HEAD -- '\''docs/coordination/[0-9][0-9][0-9][0-9][0-9]-*.md'\'' | xargs -0r ./bin/agf coord validate; git diff --name-only -z "$CI_MERGE_REQUEST_TARGET_BRANCH_SHA"...HEAD -- '\''docs/plans/*.md'\'' | xargs -0r ./bin/agf plan validate'
./bin/agf coord validate docs/coordination/00248-coder-tester.md
./bin/agf plan validate tooling/elixir/test/fixtures/valid_plan.md
MISE_TRUSTED_CONFIG_PATHS=/home/devi/src/AGF-NG/.worktrees/tooling-conformance-cli/mise.toml mise exec -- mix compile --warnings-as-errors
MISE_TRUSTED_CONFIG_PATHS=/home/devi/src/AGF-NG/.worktrees/tooling-conformance-cli/mise.toml mise exec -- mix test
MISE_TRUSTED_CONFIG_PATHS=/home/devi/src/AGF-NG/.worktrees/tooling-conformance-cli/mise.toml mise exec -- uvx --from 'mkdocs>=1.6,<2' mkdocs build -f agf/docs/build/mkdocs.yml --strict
git diff --check
```

Results:

- Changed-file CI command shape passed.
- `agf coord validate` on seq 248 passed.
- `agf plan validate` on the valid plan fixture passed.
- Compile with warnings as errors passed.
- Elixir test suite passed: 129 tests.
- MkDocs strict build passed.
- Whitespace check passed.

## Review note

Before the refresh, `master..feat/tooling-conformance-cli` showed deletion of
the seq 236-249 coordination records and the duplicate-sequence learning. The
merge from `master` repaired that stale-base hazard; the current
`master...HEAD` product diff is six files:

- `agf/docs/build/mkdocs.yml`
- `agf/docs/contracts/tooling-conformance.md`
- `agf/docs/site/docs/advanced/contracts-and-validators.md`
- `agf/docs/site/docs/index.md`
- `agf/docs/site/reference/tooling-conformance.md`
- `bin/agf`
