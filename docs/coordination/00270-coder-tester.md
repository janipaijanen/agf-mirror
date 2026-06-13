---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 270
timestamp: 2026-06-14T03:02:00+03:00
immutable: true
references-to:
  - docs/coordination/00268-coder-tester.md
topic: env-var-reference-and-root-guard-gate2-bridge
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
    - .worktrees/env-var-reference/agf/docs/site/reference/environment-variables.md
    - .worktrees/env-var-reference/docs/coordination/00269-coder-tester.md
    - .worktrees/env-var-reference/tooling/elixir/lib/agov/cli_safety.ex
    - .worktrees/env-var-reference/tooling/elixir/test/agov/cli_safety_test.exs
  do_not_touch:
    - agents-govern/
    - agents-govern/agents/MANIFEST.yaml
    - agents-govern/learnings/foundry.yaml
---

Visibility bridge for the environment variable reference and root-execution
guard Gate 2 handoff.

Branch: `docs/env-var-reference`

Branch commit: `4b99279 docs(tooling): document env vars and block root tasks`

Branch-local record:

- `00269-coder-tester.md` is the detailed Gate 2 handoff.

Summary:

- Adds a canonical public-site page for all AGF-controlled environment
  variables.
- Documents `AGOV_ALLOW_ROOT`, `AGOV_MANIFEST_PATH`, `AGOV_SCHEMA_DIR`,
  `AGOV_TOKEN_ROI`, and `AGOV_TOKEN_ROI_LOG` in one place.
- Adds `Agov.CliSafety` and wires all ten `mix agov.*` tasks to reject UID 0 by
  default.
- Allows intentional disposable CI/container root execution with
  `AGOV_ALLOW_ROOT=1`.

Validation on the branch:

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
