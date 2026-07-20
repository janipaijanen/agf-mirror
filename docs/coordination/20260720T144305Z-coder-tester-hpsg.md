---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260720T144305Z-coder-tester-hpsg
timestamp: 2026-07-20T14:43:05Z
model: codex-gpt-5
immutable: true
references-to:
  - docs/plans/docs-audience-boundary-proposal.md
  - 20260720T143255Z-governor-reviewer-s6hg.md
topic: docs-audience-implementation-claude-mr1
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Route or pick up MR-1 from the approved documentation audience-boundary plan.
    - Implement the "How to read this site" taxonomy callout in agf/docs/site/docs/index.md.
    - Add the new documentation-audience map DOT source and rendered SVG next to the taxonomy callout.
    - Keep MR-1 concept-focused; do not relabel the full page set for MR-2 and do not rewrite install-and-layouts.md while Codex's MR-4 claim is active.
    - File normal Gate 2 evidence when the MR-1 implementation is ready.
  must_read:
    - docs/plans/docs-audience-boundary-proposal.md
    - docs/coordination/20260720T143255Z-governor-reviewer-s6hg.md
    - docs/coordination/20260720T125518Z-coder-tester-kpmn.md
    - agf/docs/site/docs/index.md
  do_not_touch:
    - agf/docs/site/docs/adopt/install-and-layouts.md
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Handoff: start MR-1 for the docs audience-boundary implementation

Gate 1 is approved for the documentation audience-boundary plan. The agreed
implementation split gives Claude the concept-heavy first slice:

- add the early "How to read this site" taxonomy callout to
  `agf/docs/site/docs/index.md`;
- add the documentation-audience map figure (`docs-audience-map`) beside that
  callout;
- keep the change small enough for independent Gate 2 review.

Codex is starting MR-4 on
`agf/docs/site/docs/adopt/install-and-layouts.md` in parallel. MR-2 should
wait until MR-1 lands because it depends on the taxonomy being present.
