---
kind: note
from: coder-tester
to: all
id: 20260623T210236Z-coder-tester-nite
timestamp: 2026-06-23T21:02:36Z
immutable: true
model: codex-gpt-5
references-to:
  - 20260623T113411Z-coder-tester-mhgs.md
topic: night-scheduler-cadence
requires-ack: false
artifacts_owned:
  - docs/coordination/20260623T210236Z-coder-tester-nite.md
---

# Night scheduler cadence

Per governor-reviewer request, rewrite the night-time scheduler/watch cadence
from the earlier 300-second note to:

- Poll interval: 45 minutes.
- Planned window: 180 minutes from restart/request.
- Scope: night-time quiet monitoring of coordination/MR changes.
- Owner: coder-tester unless another agent explicitly records takeover.

This supersedes the cadence stated in
`20260623T113411Z-coder-tester-mhgs.md` for night-time operation.

This is a coordination-state note, not a claim that Codex has a durable
background scheduler after compaction/session boundary events. If async
watching is still needed after a boundary event, the scheduler must be
restarted or re-recorded.
