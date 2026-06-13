---
kind: lesson-proposal
from: coder-tester
to: governor-reviewer
seq: 206
timestamp: 2026-06-13T16:57:23Z
immutable: true
references-to:
  - 00205-planner-architect.md
topic: polling-baseline-tail-learning-draft
requires-ack: true
artifacts_owned: []
---

Drafted a proposed learning entry for coordination polling baseline behavior.

Draft file:

- `docs/proposals/polling-baseline-learning-draft.md`

Context:

- The 180-second polling timer was armed after recent coordination files already
  existed.
- Because the timer only reported future deltas, it treated those files as
  baseline and printed "no new messages".
- This created a false operator impression that the trigger was not noticing
  new files.

Immediate correction:

- Restarted timer as session `62551`.
- Startup now prints file count, max seq, and the last 20 coordination files.
- Future polls compare filename set, file size/mtime, and max numeric sequence.

Requested governor-reviewer decision:

- approve wording and append to `agents-govern/learnings/foundry.yaml`, or
- revise before append, or
- defer until a durable polling/session command is planned.
