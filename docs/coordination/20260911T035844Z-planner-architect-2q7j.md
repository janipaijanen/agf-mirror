---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260911T035844Z-planner-architect-2q7j
timestamp: 2026-09-11T03:58:44Z
model: claude-sonnet-5
immutable: true
topic: row6-s2-cpc-branch-local-visibility-gate1
references-to:
  - 20260910T072301Z-planner-architect-3ts1.md
requires-ack: true
artifacts_owned:
  - docs/plans/20260911T035752Z-row6-s2-cpc-branch-local-visibility.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Gate 1 decision on Row 6 S2 (CP-C branch-local visibility) on branch plan/row6-s2-cpc-branch-local-visibility.
    - Independent of the Codex in-progress branches (row5-s4, row6-s1); no overlap.
    - Confirm the opt-in flag, byte-identical default behaviour, and the 4 fixtures.
  must_read:
    - docs/plans/20260911T035752Z-row6-s2-cpc-branch-local-visibility.md
    - docs/plans/20260910T072138Z-coordination-discovery-collision-tooling-work-order.md
  do_not_touch:
    - agov handoff next selection/ranking logic (explicitly out of scope)
    - the in-progress Codex branches for row5 S4 / row6 S1
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Handoff: Row 6 S2 — CP-C branch-local visibility (Gate 1)

4th of the ~9 rows 4-7 implementation-slice details (Row 6 S1 = MR !2675;
Row 5 S1+S4 = MR !2676, S1 now merged as !2682). Independent of the two
Codex branches currently mid-rebase (row5 S4 / row6 S1) — no file overlap,
drafted while those are Codex's to finish.

`agov handoff next <role>` reads only the checked-out base branch's
`docs/coordination` tree; a handoff on an unmerged branch is invisible to
it. S2 adds an **opt-in** `--include-branch-local` flag reusing
`agov coord visibility`'s existing branch-enumeration/read path. Default
(no-flag) behaviour is byte-identical. `agov plan validate` OK.

On Gate 1 approval this goes to coder-tester (Codex, when it resumes) for
implementation and its own binding Gate 2.
