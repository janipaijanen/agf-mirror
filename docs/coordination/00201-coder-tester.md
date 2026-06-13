---
kind: completion
from: coder-tester
to: planner-architect
seq: 201
timestamp: 2026-06-13T16:40:28Z
immutable: true
references-to:
  - 00198-planner-architect.md
  - 00200-coder-tester.md
topic: escaped-worktrees-moved-under-project
requires-ack: false
artifacts_owned: []
---

Moved escaped linked worktrees under the AGF-NG project boundary.

All linked AGF-NG worktrees are now under:

```text
/home/devi/src/AGF-NG/.worktrees/
```

Moved from `/home/devi/src/AGF-NG-*`:

- `AGF-NG-docs-cx-review` -> `.worktrees/docs-cx-review`
- `AGF-NG-elixir-tooling-direction` -> `.worktrees/elixir-tooling-direction`
- `AGF-NG-layoutb-only` -> `.worktrees/layoutb-only`
- `AGF-NG-learning-branch-preflight` -> `.worktrees/learning-branch-preflight`
- `AGF-NG-plan-elixir-learning-tooling` -> `.worktrees/plan-elixir-learning-tooling`
- `AGF-NG-session-continuity` -> `.worktrees/session-continuity`
- `AGF-NG-v2-pedagogy` -> `.worktrees/v2-pedagogy`

Moved from `/tmp`:

- `/tmp/agf-ng-deferred-plan` -> `.worktrees/deferred-plan`
- `/tmp/agf-ng-essay` -> `.worktrees/essay`
- `/tmp/agf-ng-v2-review` -> `.worktrees/v2-review`

Previously moved Codex worktrees remain:

- `.worktrees/session-resume`
- `.worktrees/coord-visibility`

Verification:

- `git worktree list --porcelain` now shows all AGF-NG linked worktrees under
  `/home/devi/src/AGF-NG/.worktrees/`.
- The old sibling and `/tmp` paths no longer exist.
- Main `git status --short` did not gain `.worktrees/` noise because the local
  exclude already contains `.worktrees/`.
