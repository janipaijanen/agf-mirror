---
kind: note
from: planner-architect
to: governor-reviewer
id: 20260910T061534Z-planner-architect-k8bh
timestamp: 2026-09-10T06:15:34Z
model: claude-sonnet-5
immutable: true
topic: vacuum-mrs-j8bq-resequence-and-v211-queue-hygiene-precondition
references-to:
  - 20260910T052328Z-planner-architect-771h.md
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

<!--
Filed by the claude-sonnet-5 planner-architect session as a bridge for a
binding in-session governor decision (2026-09-10, "Vacuum MRs into this
release. Its gonna be a major one. Not good to have MRs hanging around?").
Per AGENTS.md, a plan metadata transition and a re-sequencing of a prior
binding decision (`j8bq`) must be paired with an actual governor-reviewer
coordination record — this is that record. Governor: please ack or correct.
-->

# Governor: vacuum open MRs into v2.11 — `j8bq` re-sequenced, queue-hygiene precondition added

## Decision (governor, 2026-09-10, in-session)

Across three exchanges:

> 1. "approved" (re-sequence `j8bq` so queue-closeout runs concurrent with the RUW drive)
> 2. "approved your proposal" (queue-hygiene as a *bounded* cut precondition — decision gates, execution trails; carve-outs named)
> 3. "Triage and drafts and proposed work is approved. Proceed."

## A. `j8bq` / `st83` re-sequencing

The queue-closeout work order
`docs/plans/20260909T195116Z-post-v211-queue-closeout-work-order.md`
(MR !2643) — **both** its triage phase **and** its per-item batch execution
— now runs **concurrent with the RUW / v2.11 Gate-2 drive**.

This supersedes, **for that work order only**:
- `j8bq`: "bulk forge close/park/reconstruct execution is not a v2.11 cut
  precondition, and happens only after v2.11 is published and verified."
- `st83`: post-publication ordering of remaining-MR terminal disposition.

Unchanged: the per-item execution controls (per-item SHA-256 digest →
non-author contribution review → **governor** authorizing decision → step-4
digest re-read invalidates changed items → idempotent per-item execution →
stop-on-first-failure → completion record pinning IIDs+SHAs → separate
Gate 2 ledger-reconciliation MR). The `6sj1` semantic-data carve-out is
unchanged.

## B. v2.11 parent-scope amendment

`docs/plans/20260909T021841Z-v211-release-scope.md` gains a new subsection
**"Post-approval scope amendment — open-MR queue disposition hygiene"** and
a **condition 3** in "Release-cut precondition (stated precisely)":

> Every open merge request **eligible for disposition** carries a
> governor-recorded terminal disposition (one of `20260903T043752Z` §3b:
> `ship-in`, `merge-now`, `reconstruct`, `close-superseded`,
> `close-invalid`, `park-radar`, `awaiting-governor`). The disposition
> **decision** gates the cut; per-item **execution** may trail publication.

Carve-outs (no recorded disposition required before the cut): semantic-data
MRs under `6sj1`/`j8bq` (currently MR !2460); `reconstruct`-disposition MRs;
coordination-only MRs (uncounted per §3c). Advisory, not gating: the board
reaching the §3c intake bound (≤5 impl, ≤3 Gate-1-plan) — residual gap
reported at the cut, governor decides acceptability.

Per the pattern the #129-withdrawal amendment established, `status` returns
to `pending-gate1`. This record is the paired governor coordination record;
the one outstanding gate item is a **non-author (Codex) exact-head review**
of the amendment — governor to run that round or waive it as `!2643` was
waived.

## C. Triage digest approved

The 62-MR triage digest presented in-session is approved as the disposition
proposal. Summary of disposition verbs (full per-MR table in the session
transcript, to be formalised into the `!2643` MR-disposition ledger once
`!2643` merges):

| Verb | Count | MRs |
|---|---|---|
| ship-in (active v2.11 workstream) | 8 | !2646 !2644 !2643 !2636 !2027 !2587 !2592 !2593 |
| merge-now | 18 | !2651 !2655 !2652 !2653 !2654 !2582 !2574 !2368 !2394 !2455 !2452 !2412 !2596 !2590 !2410 !2437 !2409 !1444 |
| close-superseded | 9 | !1492 !1657 !1617 !2375 !1446 !973 !965 !1691 !952 |
| park-radar | 20 | !459 !745 !800 !810 !918 !923 !954 !967 !998 !1000 !1451 !1718 !1722 !2275 !1659 !1527 !1061 !2306 !2390 !2408 |
| awaiting-governor / carve-out | 7 | !2460 (6sj1 carve-out) !1723 !1750 (commercial-licensing) !915 (external eval) !654 !2038 (AGENTS.md collision pair) !1817 (planned normative owner) |

61/62 receive a disposition verb now; only MR !2460 is excluded by `6sj1`.
`awaiting-governor` counts as "carries a recorded disposition" for the
condition-3 gate.

## Vehicle

One MR on branch `plan/v211-scope-queue-hygiene-amendment` carries the
scope-plan amendment (parts A + B) and this record. Disposition **execution**
is a separate downstream cycle through the `!2643` batch mechanism, and
begins after `!2643` merges (the MR-disposition ledger it defines does not
exist yet).

## Ask

Governor: ack or correct. On ack, transition
`20260909T021841Z-v211-release-scope` `status` → `approved` with
`gate1_approved_by: 20260910T061534Z-planner-architect-k8bh` (or name the
non-author review round you want first).
