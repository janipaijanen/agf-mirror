---
kind: handoff
from: planner-architect
to: coder-tester
seq: 45
timestamp: 2026-06-10T12:00:00Z
immutable: true
references-to: []
topic: some-older-task
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - An older task that should not be returned when seq 50 exists
  must_read: []
  do_not_touch: []
---

Older handoff — should be superseded by seq 50.
