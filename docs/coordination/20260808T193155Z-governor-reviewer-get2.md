---
kind: decision-record
from: governor-reviewer
to: all
id: 20260808T193155Z-governor-reviewer-get2
timestamp: 2026-08-08T19:31:55Z
model: human
immutable: true
references-to:
  - 20260808T192437Z-contribution-auditor-egn1.md
  - 20260808T191033Z-contribution-auditor-rkb7.md
  - 20260808T191131Z-fact-checker-2qsp.md
  - 20260808T190912Z-fact-checker-eare.md
  - 20260808T192437Z-contribution-auditor-r8rd.md
topic: approved-reviewer-piyaz-beagle-sequence
requires-ack: false
artifacts_owned:
  - docs/plans/20260808T181423Z-independent-implementation-reviewer.md
  - docs/plans/20260808T175620Z-piyaz-sources-radar.md
  - docs/plans/20260808T183550Z-beagle-crdt-source-concurrency.md
---

# Governor decision: approve the proposed reviewer/Piyaz/Beagle sequence

Direct human decision in chat, transcribed: “ok. approved as proposed.
proceed with claude. collaborate and coordinate”. This accepts the specialist
sequence in `egn1`; it is not an unqualified approval of every open MR.

## 1. Independent implementation reviewer — Gate 1 approved

Gate 1 is **APPROVED** for the plan in MR !1244 at exact head
`192384331e7e0791f5f9dda640970b911b8a079a`.

Round-2 review `rkb7` is COMPLETE/CLEAR and exact-head pipeline `2744133093`
succeeded. Before implementation begins, the plan branch must record this
decision in `status`, `gate1_approved_by`, and `gate1_approved_at`, validate,
and publish the approved plan. Claude may then implement under `coder-tester`;
implementation still requires its own independent exact-head review and a
fresh binding Governor Gate 2 decision.

## 2. Piyaz — plan publication and Gate 2 approved in sequence

The already-approved Piyaz plan/authority MR !1240 may merge first. After that
merge, MR !1241 is **APPROVED for Gate 2** only while all of these conditions
remain true:

- source head is exactly
  `e6df2c3e5d641677d7230fa893e4fdcda50f31c2`;
- target is `main` and the MR is open, non-draft, conflict-free, and mergeable;
- exact-head pipeline `2744137144` remains successful; and
- refreshed fact-check `2qsp` remains applicable with no new discussion or
  source/radar change.

When those conditions are rechecked after !1240 merges, MR !1241 may merge.
Piyaz is deliberately sequenced before Beagle.

## 3. Beagle — bounded remediation approved; current Gate 2 is not

MR !1252 at
`75a83eeb47a8ed7eb72ee3034c38c80f4ed20c72` is **NOT Gate 2 approved**.
Its content evidence is CLEAR, but it conflicts with Piyaz because both heads
allocate Batch 20.

After Piyaz merges, this decision supplies the bounded Governor reaffirmation
requested in `egn1` to:

1. amend Beagle plan MR !1250 from literal Batch 20 to Batch 21 or the next
   available batch, without changing its substantive scope;
2. publish the amended approved plan and authority trail;
3. rebase MR !1252, change only the batch number/context required by the new
   base, and rerun exact-head CI; and
4. obtain a focused patch-equivalence fact-check before seeking a fresh
   binding Gate 2 decision.

No new source claim, radar entry, issue #33 implementation, CRDT backend, or
scope expansion is authorized by this reaffirmation.

## 4. Claude collaboration and Pages planning

Claude must use separate named role identities for the remaining lanes:

- `coder-tester` for the approved independent-reviewer role implementation;
- `planner-architect` for the separate broader Pages-rework Gate 1 plan already
  authorized by `r6hp` and unblocked in `r8rd`; and
- the applicable advisory identity for later independent/domain review.

The Pages lane remains planning only. It requires advisory consultation and a
separate binding Gate 1 decision before any broader Pages implementation.
