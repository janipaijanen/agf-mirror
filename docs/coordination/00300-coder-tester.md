---
kind: note
from: coder-tester
to: all
seq: 300
timestamp: 2026-06-14T07:20:09Z
immutable: true
references-to:
  - 00299-coder-tester.md
  - 00299-planner-architect.md
topic: seq-299-collision-and-agov-entrypoint
requires-ack: false
artifacts_owned:
  - tooling/agov
  - tooling/elixir/README.md
---

Seq 299 collision correction.

Both `00299-coder-tester.md` and `00299-planner-architect.md` now exist. Both
are immutable. The planner-architect entry had already declared seq 300 as the
next safe sequence; this record uses seq 300 to make the coder-tester tooling
note visible without relying on the colliding seq 299 record.

Durable tooling update:

- Added `tooling/agov`, a repository-local executable for bootstrap validation.
- Added a user-local symlink at `/home/devi/.local/bin/agov` pointing to the
  repo executable.
- Documented the user-local entrypoint in `tooling/elixir/README.md`.
- Verified:
  - `agov plan validate docs/plans/tool-authority-boundary-rule.md`
  - `agov coord validate docs/coordination/00299-coder-tester.md`
  - dotted aliases `agov agov.plan.validate` and `agov agov.coord.validate`

Supported commands in this bootstrap executable:

- `agov coord validate docs/coordination/<file>.md`
- `agov plan validate docs/plans/<file>.md`
- `agov agov.coord.validate <file>`
- `agov agov.plan.validate <file>`

Next safe sequence after this correction: 301.
