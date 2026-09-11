---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260910T155355Z-planner-architect-jhmp
timestamp: 2026-09-10T15:53:55Z
model: claude-sonnet-5
immutable: true
topic: row5-s1-s4-slice-details-gate1
references-to:
  - 20260910T065817Z-planner-architect-g1g5.md
requires-ack: true
artifacts_owned:
  - docs/plans/20260910T155224Z-row5-s1-session-level-addressing.md
  - docs/plans/20260910T155225Z-row5-s4-cpa-answered-ack-pending.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Gate 1 decision on Row 5 slices S1 (#121 session-level addressing) and S4 (CP-A answered-vs-ack-pending display), on branch plan/row5-s1-s4-slice-details.
    - Both are independent slices (no predecessor). One branch/MR for review efficiency; each plan keeps its own Gate 1 decision and its own Gate 2 at implementation.
    - Governor "Keep all in v2.11" (2026-09-10) confirmed - row 5 stays a hard cut precondition, so all four slices proceed.
  must_read:
    - docs/plans/20260910T155224Z-row5-s1-session-level-addressing.md
    - docs/plans/20260910T155225Z-row5-s4-cpa-answered-ack-pending.md
    - docs/plans/20260910T065635Z-coordination-lifecycle-semantics-work-order.md
  do_not_touch:
    - the coordination schema or agov before these Gate 1s (coder-tester implements after)
    - Row 5 S2 (#122, blocks on qbka Slice A) and S3 (!2027, blocks on CP-B-2) - drafted next
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Handoff: Row 5 S1 + S4 slice-detail plans (Gate 1)

Two of the ~9 remaining rows 4-7 implementation-slice details (Row 6 S1
was MR !2675). Both independent, both implementable immediately on Gate 1.

- **S1 (#121)** — the coordination schema **already has** an optional
  `session` field marked "audit only"; S1 promotes it to an advisory
  routing key and adds optional `handoff.for_session`, plus a `--session`
  filter on `agov handoff next` / `coord visibility`. A session-scoped
  item is **never hidden**, only re-labelled `addressed elsewhere`.
- **S4 (CP-A)** — `agov`-display-only: a `requires-ack` handoff with a
  later authorized-role substantive answer shows as `answered — ack
  pending` (distinct from `ready`, sorted below it). **Auto-ack deferred**
  per `5jde`. No schema change.

Both `agov plan validate` OK. On Gate 1 each goes to coder-tester (Codex)
for implementation and its own Gate 2.
