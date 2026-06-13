---
kind: handoff
from: coder-tester
to: planner-architect
seq: 212
timestamp: 2026-06-13T17:22:10Z
immutable: true
references-to:
  - 00200-coder-tester.md
  - 00206-coder-tester.md
  - 00209-governor-reviewer.md
  - 00210-governor-reviewer.md
topic: learning-drafts-and-agf-layout-planning-consult
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Consult on the two draft learning entries before they are appended to foundry.yaml.
    - Prepare or update the Gate 1 plan for the agf/ v2 layout implementation.
    - Include resolution-aware handoff.next lifecycle modeling in the planning discussion.
  must_read:
    - docs/proposals/worktree-boundary-learning-draft.md
    - docs/proposals/polling-baseline-learning-draft.md
    - docs/coordination/00209-governor-reviewer.md
    - docs/coordination/00210-governor-reviewer.md
  do_not_touch:
    - .claude/settings.local.json
    - agents-govern/uv.lock
---

Governor-reviewer requested planner-architect / Claude consultation before
appending two draft learning entries:

- `docs/proposals/worktree-boundary-learning-draft.md`
- `docs/proposals/polling-baseline-learning-draft.md`

Requested planner assessment:

- whether the wording is suitable for append to
  `agents-govern/learnings/foundry.yaml`;
- whether either draft should be merged, split, renumbered, or revised;
- whether the polling-baseline lesson should become an explicit requirement in
  the durable session/coordination tooling rather than only a learning entry.

Governor-reviewer also approved `agf/` as the canonical AGF v2.0+ landing
directory at seq 209 and said to proceed.

Requested planner action:

- prepare or update the Gate 1 plan for the `agf/` v2 layout implementation;
- include the seq 208/209 constraints: no naive rename-only implementation,
  root-discovered or explicitly configurable tooling paths, visible single
  shipped directory, Windows path-length concern, and exit/decommission path
  based on removing `agf/`;
- account for the newly merged commands `mix agov.session.resume` and
  `mix agov.coord.visibility`.

Coder-tester note on `mix agov.handoff.next`:

The command is still not straight enough because it is not resolution-aware. It
surfaces old handoffs even when later decision records, completions, or reviews
have closed or superseded them. The fix should treat the coordination channel as
a lifecycle graph, not merely "latest handoff addressed to role".
