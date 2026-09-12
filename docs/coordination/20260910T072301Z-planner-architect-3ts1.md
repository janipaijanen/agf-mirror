---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260910T072301Z-planner-architect-3ts1
timestamp: 2026-09-10T07:23:01Z
model: claude-sonnet-5
immutable: true
topic: coordination-discovery-collision-tooling-work-order-gate1
references-to:
  - 20260910T065817Z-planner-architect-g1g5.md
  - 20260909T013812Z-capability-developer-jprj.md
requires-ack: true
artifacts_owned:
  - docs/plans/20260910T072138Z-coordination-discovery-collision-tooling-work-order.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Gate 1 review of the coordination-discovery-collision-tooling work order (v2.11 IN row 6) on branch plan/coordination-discovery-collision-tooling-work-order.
    - Confirm the three-member bounded set (S1 channel path contract, S2 CP-C branch-local visibility, S3 #123 affected_paths + advisory collision-check), that S3 ships advisory not blocking, the out-of-scope list, and that each slice retains its own Gate 1 and Gate 2.
  must_read:
    - docs/plans/20260910T072138Z-coordination-discovery-collision-tooling-work-order.md
    - docs/coordination/20260909T013812Z-capability-developer-jprj.md
    - docs/plans/20260909T021841Z-v211-release-scope.md
  do_not_touch:
    - agov or gate1-plan schema before a slice's own approved Gate 1
    - agov handoff next selection/ranking logic (explicitly out of scope)
    - '#121 / #122 / the urgency field (they belong to row 5 / CP-B-2)'
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Handoff: coordination-discovery-collision-tooling work order (v2.11 IN row 6) — Gate 1

## What this is

The Gate 1 work order for v2.11 scope plan row 6. Folds `jprj` CP-C into
#123's tooling per `jprj` §C, and adds a channel path-contract slice
because the B4 incident showed the contract itself is unspecified.

Plan: `docs/plans/20260910T072138Z-coordination-discovery-collision-tooling-work-order.md`
Branch: `plan/coordination-discovery-collision-tooling-work-order`

## Members (bounded)

- **S1 — channel path contract.** `agov coord visibility` (recursive git
  scans) and `agov-coord::channel_reader::read_channel` (`std::fs::read_dir`,
  non-recursive) disagree on what the channel is. S1 fixes the contract:
  `*.md` directly under `docs/coordination/`, both readers. Ships with the
  **mandatory row-6 negative fixture** — a misplaced live obligation is
  still flagged after the archive exclusion.
- **S2 — CP-C.** `agov handoff next` gains an opt-in `--include-branch-local`
  scan so a ready handoff on an unmerged branch is not invisible. No
  selection-logic redesign.
- **S3 — #123.** Optional `affected_paths` array on `gate1-plan.schema.json`
  + `agov plan collision-check` (advisory) intersecting those sets across
  pending/approved plans and open MRs.

## B4 note

Already resolved (MR !2581 merged, `36c21dba`, archive relocated). S1 does
not re-fix B4; it specifies the shared contract so the next nesting case
is handled by design.

## Fresh #123 evidence

This session's own Row 5 work order (MR !2658) claimed the `urgency`
field CP-B-2 already owns — a two-plan schema-field collision caught only
by manual read (`p2rk`). Added to S3's evidence base.

## Out of scope (explicit)

`agov handoff next` selection/ranking redesign; #121 session addressing
and #122 supersession (row 5); the `urgency` field (CP-B-2).

## IN row status after this

Row 5 (MR !2658) and Row 6 (this) work orders are filed. Remaining:
CP-B-2 (row 7, a coder-tester deliverable — routing separately) and
`qbka` Slice A (row 4, a separate planner).
