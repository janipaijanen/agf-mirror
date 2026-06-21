---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260621T212942Z-coder-tester-hn3e
timestamp: 2026-06-21T21:29:42Z
model: GPT-5
immutable: true
references-to: []
topic: canonical-public-docs-url
requires-ack: true
artifacts_owned:
  - agf/docs/build/mkdocs.yml
  - agf/docs/build/hooks/release_info.py
  - docs/coordination/20260621T212942Z-coder-tester-hn3e.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the canonical public Pages URL metadata update.
  must_read:
    - agf/docs/build/mkdocs.yml
    - agf/docs/build/hooks/release_info.py
  do_not_touch:
    - agf/rules/
    - agf/schemas/
    - agf/agents/
---

# Gate 2 handoff: canonical public docs URL

The public Pages URL is now
`https://janipaijanen.gitlab.io/agents-govern-framework/`.

## Changes

- Added `site_url: https://janipaijanen.gitlab.io/agents-govern-framework/`
  to the MkDocs build config.
- Added a `Published docs` row to the generated Release information page
  template.

`README.md` and `agf/docs/site/evaluate.md` already used the canonical URL on
`origin/main`, so I did not churn them.

## Validation

- `rg "agent-govern-framework-f5b79e\\.gitlab\\.io"` across README/docs/site/docs/plans/docs/coordination: no matches.
- `git diff --check`: pass.
- `uv run --project agents-govern --group docs mkdocs build -f agf/docs/build/mkdocs.yml --strict`: pass.

The shell emitted the local untrusted `mise.toml` warning before commands in
this worktree, but each command completed successfully.
