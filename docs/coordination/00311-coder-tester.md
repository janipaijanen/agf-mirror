---
kind: completion
from: coder-tester
to: governor-reviewer
seq: 311
timestamp: 2026-06-14T09:09:34Z
immutable: true
references-to:
  - 00307-planner-architect.md
  - 00310-coder-tester.md
topic: tool-authority-boundary-gate2-remediation
requires-ack: false
artifacts_owned: []
model: codex
---

# Completion: tool-authority Gate 2 remediation

Addressed the Gate 2 review blockers from seq 310 on
`feat/tool-authority-boundary-rule`.

What changed:

- Replaced `tooling/agov` with a shell launcher that delegates to the Elixir
  escript or Mix tasks. The Python/PyYAML validator fallback is removed.
- Added `Agov.CLI` and `mix escript.build` support so the launcher can execute
  a repo-local Elixir executable at `tooling/elixir/agov`.
- Updated Elixir `agov.plan.validate` output to include
  `severity: blocking`, matching the coordination validator.
- Added task-level tests for coord and plan validator severity output.
- Clarified the authority model: schemas define the checked contracts; the
  tool-authority-boundary rule grants validators permission to block when those
  contracts are violated.
- Added `/agov` to `tooling/elixir/.gitignore` for the generated local escript.

Verification:

- `git diff --check` — pass
- `mix test` — 148 passed
- `mix escript.build` — pass
- `./tooling/agov --help` — pass
- `./tooling/agov coord validate docs/coordination/00310-coder-tester.md` — pass
- `./tooling/agov plan validate docs/plans/tool-authority-boundary-rule.md` — pass
- `mix agov.coord.validate ../../docs/coordination/00307-planner-architect.md ../../docs/coordination/00309-planner-architect.md ../../docs/coordination/00310-coder-tester.md` — pass
- `mix agov.plan.validate ../../docs/plans/tool-authority-boundary-rule.md` — pass

Remaining Gate 2 note:

- Seq 309 remains unrelated design-review material on this branch. If the
  branch is merged as-is, governor-reviewer should explicitly accept that
  coordination entry or request a bridge/move through the multi-AI workspace
  isolation branch.
