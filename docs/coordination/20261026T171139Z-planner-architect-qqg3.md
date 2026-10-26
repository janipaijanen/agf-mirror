---
kind: note
from: planner-architect
to: all
id: 20261026T171139Z-planner-architect-qqg3
timestamp: 2026-10-26T17:11:39Z
model: claude-sonnet-5-5
immutable: true
references-to:
  - 20261026T160359Z-planner-architect-75qt
  - 20261026T161155Z-planner-architect-q65r
  - 20261026T165855Z-planner-architect-gnqr
topic: crossapproval-delegation-bridge
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Bridge record: Human Governor's cross-approval statement

**Provisional.** This note transcribes live statements. It is not binding on its own and
stays provisional until a `governor-reviewer` record by the Human Governor references it.

## What the Human Governor said

In the live room and session on 2026-10-26, after planner-architect proposed delegation
text in ACS seq 687:

> So, if I delegate so that you can cross-approve each-others plans, MRs, etc once the
> other party has signalled CLEAR with exact head etc. info attached?  (ACS seq 686)

> referring to seq 687, you have authority to approve codex work  (live session)

## How I read it, and the limits I keep

I read the second statement as the Human Governor's delegation to planner-architect to
decide Gate 1, Gate 2 and merge for work authored by codex-foundry, never for work I
authored, on the terms of the text I proposed in ACS seq 687:

1. Only after the author-side review is CLEAR at the exact head, with a green required
   pipeline and every advisory row consulted by someone other than the author.
2. A decision-record citing that CLEAR is published before any irreversible action.
3. A new head cancels a CLEAR.
4. Excluded unless the Human Governor adds them by name: release, tag and deploy; rule,
   role-definition and schema changes; security boundaries; changes to this delegation.
5. Items of mixed authorship go to the Human Governor.
6. Expiry: `2026-10-27T18:00:00Z`.

The statement is about my authority over Codex's work. It does not say what authority
codex-foundry has over mine, and I do not assume it. I take no decision on any item I
authored, including MRs !3243, !3246 and !3248.

## State when written

No open MR is authored by codex-foundry alone and awaiting a decision. MR !3066 has mixed
authorship, and MR !2679 is a Draft proposal.

## Ratification requested

A `governor-reviewer` record from the Human Governor referencing this note, confirming
this reading, and stating whether codex-foundry holds the reciprocal authority over
Claude-authored work.
