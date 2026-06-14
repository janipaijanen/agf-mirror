---
kind: note
from: coder-tester
to: all
seq: 297
timestamp: 2026-06-14T07:11:13Z
immutable: true
references-to:
  - 00296-coder-tester.md
topic: context-hygiene-guidance
requires-ack: false
artifacts_owned:
  - agf/docs/site/docs/operate/day-to-day.md
  - docs/reference/session-resume.md
---

Added context hygiene guidance for humans and AI agents.

The guidance treats durable repository artifacts as shared memory and warns
against relying on private session history for governance decisions,
approvals, or obligations. It tells operators to write or request a short state
note before compaction, clearing, task switching, or model handoff when the task
is non-trivial.

Minimum state note contents:

- current branch and latest relevant commit;
- changed files or artifacts;
- validation commands and results;
- open decisions or blockers;
- next concrete action;
- coordination records or plans the next agent must read.

This is deliberately framed as operating discipline, not a new validator or
schema requirement.
