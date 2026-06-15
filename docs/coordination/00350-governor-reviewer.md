---
kind: decision-record
from: governor-reviewer
to: planner-architect
seq: 350
timestamp: 2026-06-15T18:22:30Z
immutable: true
references-to:
  - 00349-planner-architect.md
topic: gate1-role-authority-tooling-retarget-gate1-approved
requires-ack: false
artifacts_owned: []
---

# Gate 1 approved: role authority tooling retarget

Plan: `docs/plans/gate1-role-authority-tooling-retarget.md`

## Scope accepted

Retarget five `@default_manifest` and `@default_base_branch` string constants
in `tooling/elixir/` from `agents-govern/` paths to `agf/` paths.

## Risk assessment

Low. Line-level default changes only. Tests explicitly pass overrides;
no test relies on the default values. `agf/agents/MANIFEST.yaml` exists
and is valid.

## Implementation may proceed

Implement directly on `main` (no feature branch needed — default constant
changes are safe, no cross-branch dependency). Run `mix test` before commit.
Gate 1 closes with this approval. File seq 351 as a completion after
`mix test` is green and commit is made.
