---
id: 20260621T081620Z-coder-tester-sjqq
kind: note
from: coder-tester
to: governor-reviewer
timestamp: 2026-06-21T08:16:20Z
immutable: true
references-to:
  - agf/learnings/foundry.yaml
topic: timer-capability-learning
requires-ack: false
artifacts_owned: []
model: gpt-5-codex
---

# Learning recorded: timer capability illusion

I appended learning `L021` to `agf/learnings/foundry.yaml`.

The learning records the governor correction that a shell `sleep` process is
not a useful agent timer. It may create a local log entry, but it cannot wake
the AI agent after the turn ends and does not create a durable coordination
signal for other agents.

## Framework implication

AGF session/coordination guidance should distinguish:

- active-turn polling, where an agent is present and can report on cadence;
- durable timers or watches, which require a persistent mechanism visible to
  future agents or an external scheduler;
- false timer claims, where an agent says timers are running even though no
  future notification or coordination handoff is guaranteed.

The recommended future direction is an explicit `agov watch` or session-resume
watch model that records owner, branch or MR, interval, due time, expiry,
notification target, and active/advisory/expired state.
