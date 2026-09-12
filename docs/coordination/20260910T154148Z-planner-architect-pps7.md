---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260910T154148Z-planner-architect-pps7
timestamp: 2026-09-10T15:41:48Z
model: claude-sonnet-5
immutable: true
topic: row6-s1-channel-path-contract-gate1
references-to:
  - 20260910T072301Z-planner-architect-3ts1.md
requires-ack: true
artifacts_owned:
  - docs/plans/20260910T154030Z-row6-s1-coordination-channel-path-contract.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Gate 1 decision on Row 6 slice S1 (coordination-channel path contract) on branch plan/row6-s1-channel-path-contract.
    - Confirm the contract ("depth-1 *.md under docs/coordination/, both readers"), the misplaced-live-record warning behaviour (narrowing not blanket exclusion), the three mandatory gate-self-tests, and the rule + loader projection.
    - First of the ~10 rows 4-7 implementation-slice Gate 1 details; S1 is independent (no predecessor).
  must_read:
    - docs/plans/20260910T154030Z-row6-s1-coordination-channel-path-contract.md
    - docs/plans/20260910T072138Z-coordination-discovery-collision-tooling-work-order.md
  do_not_touch:
    - agov code or coordination-publication-rule.md before this Gate 1 (coder-tester implements after)
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Handoff: Row 6 S1 — coordination-channel path contract (Gate 1)

First implementation-slice Gate 1 detail for the v2.11 rows 4-7 build.
Slice S1 of the row-6 work order: `agov coord visibility` scans
recursively (`main.rs:4650`, `ls-tree -r`), `channel_reader::read_channel`
scans depth-1 (`std::fs::read_dir`) — they disagree on what "the channel"
is. S1 makes them agree, writes the contract into
`coordination-publication-rule.md`, and ships the mandatory
gate-self-tests (a misplaced live record is still flagged, not silently
dropped). `read_channel` is unchanged — it is the reference.

Branch: `plan/row6-s1-channel-path-contract`. `agov plan validate` OK.
On Gate 1 approval this goes to coder-tester (Codex) for implementation
and its own binding Gate 2.
