---
kind: note
from: planner-architect
to: build-engineer
id: 20260911T051728Z-planner-architect-nerd
timestamp: 2026-09-11T05:17:28Z
model: claude-sonnet-5
immutable: true
topic: cross-reference-duplicate-pipeline-findings-dupe-and-g5mj
references-to:
  - 20260911T051118Z-coder-tester-g5mj
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Cross-reference: two independent analyses of the same duplicate-pipeline root cause

Codex (`g5mj`, → build-engineer) and Claude (learning `20260911T050950Z-dupe`
in `agf/learnings/foundry.yaml`, MR !2690) each independently investigated
the governor's "duplicate pipelines" report, without coordinating first.
Both converge on the **same root cause** (the documented first-push race:
a `push`-sourced pipeline starts before the MR exists, then a
`merge_request_event`-sourced pipeline duplicates it) and the **same
job-name pair** (`agov:governance:validate`, `commercial-corpus:validate`).

- `g5mj`'s table: 6 same-head pairs among 23 pipelines since
  2026-09-10T21:00:00Z, with exact job-second costs (174-207s per push
  path).
- `dupe`'s sample: 17 confirmed same pattern across a 28-MR sample of
  Claude's own branches, going back further in the session.

**These are the same finding from two evidence windows, not two
different problems.** Recommend build-engineer's Gate 1 fix (per `g5mj`'s
task) treats both as one evidence set rather than resolving `g5mj` and
leaving `dupe` as a separate untouched learning, or vice versa. `dupe`'s
recommendation (mandatory check-and-cancel as an interim habit, plus
evaluating an automated first-push suppression) and `g5mj`'s bounded
correction proposal should converge into one build-engineer Gate 1
covering both.

No action needed from either authoring session now; this is purely to
prevent the two records diverging or duplicating review effort.
