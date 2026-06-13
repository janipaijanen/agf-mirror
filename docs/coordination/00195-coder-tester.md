---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 195
timestamp: 2026-06-13T16:24:33Z
immutable: true
references-to:
  - 00193-planner-architect.md
topic: coordination-visibility-scanner-s1-gate2-bridge
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review branch feat/coord-visibility-scanner for Gate 2.
    - Treat branch-local seq 194 as the detailed Gate 2 handoff.
    - Decide whether to merge commit a80b110 into master.
  must_read:
    - docs/plans/coordination-visibility-scanner-s1.md
    - docs/coordination/00193-planner-architect.md
  do_not_touch:
    - agents-govern/
---

Visibility bridge for Coordination Visibility Scanner S1 Gate 2.

Branch-local detailed handoff:

- `docs/coordination/00194-coder-tester.md`

Feature branch:

- `feat/coord-visibility-scanner`

Feature commit:

- `a80b110 feat(tooling): add coordination visibility scanner`

Summary:

- Adds `mix agov.coord.visibility --base-branch master`.
- Scans local branches for branch-local coordination obligations hidden from
  the base branch.
- Reports unbridged obligations and sequence collisions.
- Provides JSON output for future CI.
- Adds `docs/reference/coord-visibility.md` and `AGENTS.md` quick reference.

Verification reported by branch-local seq 194:

- `mise exec elixir -- mix test` passed, 118 tests.
- Negative gate-self-test: targeted scan of `feat/session-resume` exits `2`
  because base branch contains known duplicate seq values.
- JSON smoke output includes stable fields and exit code.
