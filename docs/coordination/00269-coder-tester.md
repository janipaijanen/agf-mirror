---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 269
timestamp: 2026-06-14T02:55:00+03:00
immutable: true
references-to:
  - 00268-coder-tester.md
topic: env-var-reference-and-root-guard-gate2
requires-ack: true
artifacts_owned:
  - .worktrees/env-var-reference
model: codex
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review branch `docs/env-var-reference`.
    - Decide Gate 2 for the canonical AGF environment variable reference page.
    - Decide Gate 2 for blocking `mix agov.*` task execution as root unless explicitly overridden.
  must_read:
    - agf/docs/site/reference/environment-variables.md
    - agf/docs/site/docs/advanced/contracts-and-validators.md
    - tooling/elixir/lib/agov/cli_safety.ex
    - tooling/elixir/test/agov/cli_safety_test.exs
  do_not_touch:
    - agents-govern/
    - agents-govern/agents/MANIFEST.yaml
    - agents-govern/learnings/foundry.yaml
---

Gate 2 handoff for the environment variable reference and root-execution guard.

Branch: `docs/env-var-reference`

Summary:

- Added `agf/docs/site/reference/environment-variables.md` as the canonical
  page for AGF-controlled `AGOV_*` and `AGF_*` environment variables.
- Documented the current code-level variables:
  `AGOV_ALLOW_ROOT`, `AGOV_MANIFEST_PATH`, `AGOV_SCHEMA_DIR`,
  `AGOV_TOKEN_ROI`, and `AGOV_TOKEN_ROI_LOG`.
- Added the page to the public MkDocs Reference navigation.
- Updated token ROI and local orientation docs to point at the canonical page.
- Added `Agov.CliSafety` and wired all ten `mix agov.*` tasks to refuse UID 0
  by default.
- Added `AGOV_ALLOW_ROOT=1` as the explicit override for disposable CI/container
  cases where root execution is intentional.

Validation:

```bash
cd tooling/elixir
mise exec -- mix compile --warnings-as-errors
rm -f /tmp/agov_token_roi_*.jsonl && rm -rf /tmp/agov_session_resume_* && mise exec -- mix test
cd ../..
mise exec -- uvx --from 'mkdocs>=1.6,<2' mkdocs build -f agf/docs/build/mkdocs.yml --strict
git diff --check
```

Results: OK. Full suite passed with 138 tests after clearing stale generated
`/tmp/agov_*` test artifacts. MkDocs strict build passed.

Environment variable coverage check:

```bash
rg --no-filename -o '\bAGOV_[A-Z0-9_]+\b|\bAGF_[A-Z0-9_]+\b' tooling/elixir/lib | sort -u
rg --no-filename -o '\bAGOV_[A-Z0-9_]+\b|\bAGF_[A-Z0-9_]+\b' agf/docs/site/reference/environment-variables.md | sort -u
```

Both lists contain:

- `AGOV_ALLOW_ROOT`
- `AGOV_MANIFEST_PATH`
- `AGOV_SCHEMA_DIR`
- `AGOV_TOKEN_ROI`
- `AGOV_TOKEN_ROI_LOG`
