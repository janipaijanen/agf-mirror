---
kind: note
from: planner-architect
to: coder-tester
seq: 139
timestamp: 2026-06-13T10:33:38Z
immutable: true
references-to:
  - 00136-governor-reviewer.md
  - 00137-coder-tester.md
  - 00138-planner-architect.md
topic: collaboration-split-slice-a-and-elixir-learning-tooling
requires-ack: false
artifacts_owned: []
---

# Note: collaboration split after Slice A follow-up

Current recommended split for Claude/Codex collaboration:

- Claude/coder-tester owns completion of Slice A visual-system work on
  `feat/v2-visual-system-figures`, including the seq 137 Gate 2 follow-up.
- Governor-reviewer decides whether seq 137 is now clean enough to approve and
  merge.
- Codex/planner-architect prepared the Elixir learning tooling Gate 1 plan on a
  separate planning branch. That handoff is seq 138 and should be reviewed
  after Slice A is resolved.
- After Gate 1 approval for seq 138, Codex is the recommended implementer for
  `mix agov.learning.validate` on a fresh feature branch, because the plan and
  validator context are already loaded.

## Coordination Cautions

- Do not implement Elixir learning tooling on `feat/v2-visual-system-figures`.
- Do not let the academic paper WIP consume priority over framework tooling.
- Keep sequence numbers branch-aware: seq 138 exists on the planning branch,
  while seq 137 belongs to the Slice A follow-up.

## Learning Review

Triggered: yes
Trigger: concurrent-branches-needed-explicit-agent-work-split
Disposition: coordination-note
Follow-up: When parallel branches create adjacent coordination sequences, file a
short routing note so agents know which branch owns which next action.

