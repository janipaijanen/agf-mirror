---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20261027T152503Z-governor-reviewer-1rdd
timestamp: 2026-10-27T15:25:03Z
model: codex
immutable: true
references-to:
  - 20261027T061034Z-planner-architect-3mts
  - 20260910T052328Z-planner-architect-771h
  - 20260910T133145Z-governor-reviewer-q862
  - 20261026T171229Z-improvement-propeller-khhr
  - 20261026T173119Z-governor-reviewer-3rrh
topic: mr-3281-scope-and-exact-head-gate2
requires-ack: false
artifacts_owned:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3281
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Clarify scope and approve exact-head merge of MR !3281

This record durably transcribes the applicable part of the Human Governor's
standing decision bridged by `20261027T061034Z-planner-architect-3mts` on MR
!3284 at exact head `b088880a98d143417107525a1fdc444406a52bc6`.

The Human explicitly named MR !3281, the correction of the MR !459 radar
inventory, as work that must be independently CLEAR and landed before the
standing decisions can take effect. That supplies the governed scope
clarification: this two-line correction is necessary execution evidence under
the approved post-v2.11 queue-closeout work order ratified in `q862`.

MR !3281 therefore follows the ordinary approved-plan and exact-head Gate 2
path. It does **not** rely on the narrow `docs/radar.md` research-maintenance
exception, and no fast-track commit-message waiver or research collision
inventory is claimed as its authority.

Under the Human Governor's prospective ratification of `khhr` in `3rrh`,
Codex approves Claude-authored MR !3281 at exact head
`ba28a3e7c9f6d50eb4d482a928f141e6ed3348b6`.

Independent exact-head review verified that MR !459 leaves exactly thirteen
`docs/coordination/` records plus proposed learning
`20260723T231109Z-w7k2` in `agf/learnings/foundry.yaml` only on its retained
branch. The MR changes only the two corresponding lines in `docs/radar.md`;
it changes no retained question, trigger, review date, source branch, closure
state, or other radar entry.

Canonical pipeline `2992880942` passed both required jobs at this exact head:
`agov:governance:validate` `17492404504` and
`commercial-corpus:validate` `17492404505`. The MR is mergeable with
discussions resolved. A fresh current-target merge-tree check produced tree
`eb2b7b9ee361bdb1a1af08e5195631b7610a0bb3`; it passes `git diff --check`
and changes only `docs/radar.md`.

Merge is authorized only while the source head remains exact, using no squash
and with the resulting parents and one-file diff verified. This decision does
not close any old merge request, delete any branch, approve MR !3282, trigger
the standing Gate 1 decisions, admit work into a release, choose a version, or
authorize a freeze.
