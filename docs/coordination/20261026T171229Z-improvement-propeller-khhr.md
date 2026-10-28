---
kind: note
from: improvement-propeller
to: governor-reviewer
id: 20261026T171229Z-improvement-propeller-khhr
timestamp: 2026-10-26T17:12:29Z
model: codex
immutable: true
references-to:
  - 20261026T160359Z-planner-architect-75qt
  - 20261026T161155Z-planner-architect-q65r
  - 20261026T165855Z-planner-architect-gnqr
topic: cross-approval-delegation-bridge
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Bridge record: cross-approval delegation for Claude- and Codex-authored work

**This record is provisional and is not binding authority on its own.** It transcribes a
live decision of the Human Governor. It remains provisional until a Human Governor
`governor-reviewer` record references it. No action in an excluded category may rely on
this bridge.

## Live decision

In the live session on 2026-10-26, the Human Governor asked whether delegating authority
so that Claude and Codex could cross-approve each other's plans and merge requests after
an exact-head `CLEAR` would resolve the self-approval problem. Codex answered with the
terms below. The Human Governor then wrote, referring to those terms:

> you have authoritity to approve claude work

I record that statement as adoption of the cross-approval terms, including the expiry and
limits stated in the referenced answer. The typo in the quotation is preserved; it does
not change the meaning.

## Transcribed terms

From now until `2026-10-27T18:00:00Z`:

- Codex may decide Gate 1, Gate 2, and merge Claude-authored non-red-list work.
- Claude may decide Gate 1, Gate 2, and merge Codex-authored non-red-list work.
- Neither may approve or merge its own work. Mixed-authorship work returns to the Human
  Governor.
- The cross-approver must have an independent `CLEAR` for the exact current head, all
  required advisory consultations must be completed by someone other than the author,
  and the required exact-head pipeline must be green.
- A new head voids the prior `CLEAR`. Any `OTHER`, disagreement, unresolved blocker, or
  ambiguous authorship returns the decision to the Human Governor.
- Before a merge or other irreversible action, a decision record must cite the exact-head
  `CLEAR`, pipeline, consultations, and this bridge. Every use is logged.

## Exclusions

This delegation does not cover releases, tags, deploys, rule changes, role-definition
changes, schema changes, security-boundary changes, changes to delegation scope, or any
other category that current governance marks as never delegable. Those matters remain
with the Human Governor unless the governing authority is changed through its own valid
process.

## No retroactive ratification

This prospective cross-approval delegation does not ratify earlier bridge records or
actions. In particular, it does not by itself ratify `75qt`, `q65r`, `gnqr`, the
governor-reviewer record `86ps` that is not part of this branch baseline, or the already
executed merges of MRs !3245 and !3244. Those records and actions require a separate
Human Governor ratification that names them explicitly.

## Ratification requested

A Human Governor `governor-reviewer` record should reference this file and confirm the
transcribed scope, conditions, exclusions, and expiry before the delegation is used as
durable binding authority.
