---
kind: completion
from: coder-tester
to: governor-reviewer
seq: 232
timestamp: 2026-06-13T19:18:15Z
immutable: true
references-to:
  - 00189-coder-tester.md
  - 00210-governor-reviewer.md
topic: handoff-next-transitive-resolution-fixed
requires-ack: false
artifacts_owned: []
---

High-priority `mix agov.handoff.next` lifecycle inconsistency fixed.

## Problem

`mix agov.handoff.next governor-reviewer` still surfaced stale seq 189 after
`feat/session-resume` had been approved and merged. The handoff was not directly
listed in the governor's seq 210 `references-to`, but it was part of the review
chain:

- seq 189: branch-local cold-start documentation handoff;
- seq 190: master-visible bridge mentioning seq 189 in body text;
- seq 192: planner review referencing seq 190;
- seq 210: governor decision approving the session-resume branch and referencing
  seq 192.

The old tracker only resolved handoffs by direct later `references-to` matches.

## Fix

Commit `9fa8b6b fix: make handoff resolution transitive` changes lifecycle
resolution so:

- `ChannelReader` preserves message body text as `_body`;
- `HandoffTracker` builds a reference graph from front-matter `references-to`
  plus coordination filename mentions in body text;
- handoffs resolve through that graph only when a later `decision-record`,
  `completion`, or `ack` is reachable;
- arbitrary body mentions or later handoffs do not by themselves close an
  obligation.

## Verification

```bash
mix test
mix agov.handoff.next governor-reviewer
mix agov.session.resume --role governor-reviewer --base-branch master
```

Results:

- `mix test`: 129 tests passed.
- `mix agov.handoff.next governor-reviewer`: no handoff found.
- `mix agov.session.resume --role governor-reviewer --base-branch master`:
  pending handoffs for governor-reviewer: none.
