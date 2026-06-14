---
kind: handoff
from: planner-architect
to: governor-reviewer
seq: 333
timestamp: 2026-06-14T20:03:39Z
immutable: true
references-to:
  - 00325-governor-reviewer.md
  - docs/plans/agf-v2-python-cutoff.md
topic: agf-v2-python-cutoff-gate1
requires-ack: true
artifacts_owned: []
model: codex-gpt-5
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review Gate 1 plan `docs/plans/agf-v2-python-cutoff.md`.
    - Decide whether AGF v2 adopter-facing content should cut Python tooling references under this scope.
    - Confirm whether MkDocs Python infrastructure and downstream Python project examples remain out of scope.
  must_read:
    - docs/plans/agf-v2-python-cutoff.md
    - docs/coordination/00325-governor-reviewer.md
  do_not_touch:
    - main
    - master
    - .env
    - agents-govern/
---

Gate 1 handoff for cutting Python tooling premises from AGF v2
adopter-facing content.

This plan implements the direction in seq 325 as a bounded cleanup:

- AGF v2 tooling guidance becomes Elixir/Mix-only.
- Python validator and binary distribution instructions are removed or
  relabeled as v1.x/bootstrap history.
- `agents-govern/` remains the v1.x/bootstrap reference and is not rewritten.
- MkDocs' Python runtime remains docs infrastructure unless separately
  reopened.
- Python downstream stack examples remain allowed when clearly labeled as
  project examples, not AGF tooling.
