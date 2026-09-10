---
kind: note
from: planner-architect
to: governor-reviewer
id: 20260910T052328Z-planner-architect-771h
timestamp: 2026-09-10T05:23:28Z
model: claude-sonnet-5
immutable: true
topic: v211-gate1-approvals-2646-2644-2643-bridged
references-to:
  - 20260909T213740Z-planner-architect-kaqg.md
  - 20260909T200733Z-planner-architect-emrq.md
  - 20260909T195300Z-planner-architect-bs6g.md
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

<!--
Filed by the claude-sonnet-5 planner-architect session as a bridge for a
binding in-session governor decision (2026-09-10). Per AGENTS.md, a plan
metadata transition to `approved` on a Claude-authored plan must be paired
with an actual governor-reviewer coordination record — this is that record.
Governor: please ack or correct so the three Gate 1 approvals are binding
on-channel.
-->

# Governor: pending v2.11 Gate 1 items approved

## Decision (governor, 2026-09-10, in-session)

> "Pending Gate 1 items are approved."

Applied to the three plans that were pending a governor Gate 1 decision:

| MR | Plan | Exact head | Handoff |
|---|---|---|---|
| **!2646** | `20260909T203636Z-v211-release-publication` (v2.11 release-publication child plan) | `46940f39` + the `2pgh`/`g5p3` temp-index fix at the current branch tip | `kaqg` (supersedes `khm3`) |
| **!2644** | `20260909T200622Z-cpb1-copied-record-contract-clarification` | `c38464e8` (`1r5a` addressed) | `emrq` |
| **!2643** | `20260909T195116Z-post-v211-queue-closeout-work-order` | branch tip | `bs6g` |

## What each approval authorises

- **!2646** — the v2.11 release-publication procedure (Step 2–7). It grants
  **no** version bump, metadata merge, tag, release, or deploy authority;
  the tag remains a separate post-merge governor decision (Step 6). Cut
  execution still gated by the plan's own precondition table.
- **!2644** — the CP-B-1 copied-record contract clarification. Per the
  amended `j8bq`, this **unblocks CP-B-1 implementation MR !2636** — the
  one rule sentence, the byte-identity detector, and the four gate-self-
  tests fold into that unmerged MR, which then re-runs its exact-head
  contribution / InfoSec / build reviews.
- **!2643** — the post-v2.11 queue-closeout work order (design +
  decomposition). It grants **no** MR-close, issue-close, or ledger
  authority; each execution batch returns for its own governor decision,
  and execution runs after v2.11 is published (`j8bq`). Note: `bs6g` had no
  independent Codex review round before this approval; the governor
  approved it directly.

## Metadata transitions

Each plan's front matter is being updated on its own branch — `status:
approved`, `gate1_approved_by: 20260910T052328Z-planner-architect-771h`,
`gate1_approved_at: 2026-09-10T05:23:28Z` — in the same push, per
AGENTS.md's "prefer updating a plan's metadata in the same review cycle".
Those three commits and this record are one set; if the governor's ack
names a different record or head, the metadata commits are corrected to
match before merge.

## Still open (not covered by this approval)

- `jh3g` — the PMM / documentation-estate capability decision (its own
  ack).
- The v2.11 parent-scope amendment adding the PMM IN row + precondition
  (S4) — a separate governor Gate 1 decision, not yet drafted.
- The Gate 1 plans still unstarted: coordination-semantics
  (#121/#122/!2027/CP-A), #123/CP-C, `qbka` A/B, CP-B-2.
- The two pre-cut governor dispositions
  (`release-preflight-learning-gate2`, `capability-developer-startup-sync`).
