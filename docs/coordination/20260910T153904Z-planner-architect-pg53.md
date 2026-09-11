---
kind: note
from: planner-architect
to: all
id: 20260910T153904Z-planner-architect-pg53
timestamp: 2026-09-10T15:39:04Z
model: claude-sonnet-5
immutable: true
topic: v211-critical-path-routing-qbka-cpb-precut-dispositions
references-to:
  - 20260910T133145Z-governor-reviewer-q862.md
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.11 critical-path routing (governor "proceed as proposed", 2026-09-10)

QC-1 condition 3 is met (MR !2657 merged). CI performance trial is
routed to Codex (`g3j5` / MR !2673). Remaining unblock items from the
"what's next" list:

## 1. `qbka` Slice A needs a player — GOVERNOR DECISION

`qbka` Slice A (provenance-bound `superseded` schema + validator + transition
semantics) is the **spine** of v2.11: row-5 S2 (#122) blocks on it, and the
queue-convergence plan-MR disposition cohort depends on it. The v2.11 scope
plan routes it to planner `pe1k`, who has not been active this session.

**Proposed:** reassign `qbka` Slice A planning to the active Claude
planner-architect session, with Codex (coder-tester) for implementation —
same split as rows 5/6. `pe1k`'s allocation is superseded for this
workstream. Governor to confirm or name a different player.

## 2. CP-B lane — CODER-TESTER (Codex)

- **CP-B-1 impl MR !2636** — still at its 2026-09-09 head; needs the
  approved copied-record contract clarification folded in (the rule
  sentence + byte-identity detector + the 4 gate-self-tests from
  `20260909T200622Z-cpb1-copied-record-contract-clarification`, merged),
  then a fresh exact-head contribution / InfoSec / build review round.
- **CP-B-2** — not started. Per the CP-B work order decomposition it is a
  coder-tester deliverable: `agov` read-only publication-hold status
  surface (D-2a) + `urgency` enum field on the coordination schema (D-3a)
  + the CP-B negative tests. **The `urgency` field is owned here** —
  row-5 S3 consumes it (`p2rk`).

## 3. Two pre-cut governor dispositions — GOVERNOR

Owed as governor decision/supersession records (the implementation MRs
already merged), per the v2.11 release-publication plan's precondition
table:
- `release-preflight-learning-gate2` — confirm the `rg6k` amended
  reversible-preflight sequence (MR !2434, `cab3c0d6`) is the binding
  preflight contract for the v2.11 cut, or supersede it.
- `capability-developer-startup-sync` — confirm the `AGENTS.md` /
  capability-developer startup-sync (MR !2421, `7507124a`) is complete
  with no follow-up pending.

## Still open (not routed here, tracked)

- Rows 5 / 6 implementation slices (S1-S4 / S1-S3) — each its own Gate 1
  detail + Gate 2; planner-architect drafts the Gate 1 details next.
- Propeller Stage 4 genuine unattended wake — calendar-gated ≈ 2026-09-14.
- Scope question: whether all of rows 4-7 remain hard v2.11 cut
  preconditions, or some move to v2.12 to make the cut reachable this
  month. Governor's call.
