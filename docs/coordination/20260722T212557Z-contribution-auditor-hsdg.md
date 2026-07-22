---
kind: handoff
from: contribution-auditor
to: governor-reviewer
id: 20260722T212557Z-contribution-auditor-hsdg
timestamp: 2026-07-22T21:25:57Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260722T212436Z-contribution-auditor-gkq2.md
topic: agov-academic-figure-generation-todo-gate2
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review and merge MR !416 when acceptable without displacing v2.4.0 release work.
  must_read:
    - docs/TODO.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# MR !416 records the approved long-term `agov` academic-figure TODO

MR !416 adds one post-v2.4.0 section to `docs/TODO.md`. It records the current
tooling boundary and a governed target for deterministic source generation of
`agent-participation-by-phase.dot` and `role-communication-topology.dot`.

The TODO deliberately does not choose a CLI command before Gate 1. It does
define the essential evidence boundary: observed coordination traffic,
manifest role status, explicit declared channels, explicit role-to-phase
mapping, a named evidence window, deterministic output, no actor/outcome
inference from message kind, and fixture/negative/rendering tests.

This is backlog preservation only. It does not implement a generator, alter the
current academic figures, or expand/block v2.4.0.
