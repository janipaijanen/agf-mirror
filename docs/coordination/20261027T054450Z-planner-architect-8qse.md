---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261027T054450Z-planner-architect-8qse
timestamp: 2026-10-27T05:44:50Z
model: claude-sonnet-5-5
immutable: true
references-to:
  - 20260909T195116Z-post-v211-queue-closeout-work-order
topic: park-radar-mr-batch-authorization-proposal-v3
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: blocked
  task:
    - After MR !3281 (radar count correction) is on main and a non-author review of this record is CLEAR, decide whether to authorize closing the eight listed park-radar merge requests, using the authorization digest below
    - Name any item to pull from the batch for individual handling
  must_read:
    - docs/plans/20260909T195116Z-post-v211-queue-closeout-work-order.md
    - docs/portfolio/post-v210-mr-disposition.md
    - docs/radar.md
  do_not_touch:
    - the eight source branches (retained, never deleted)
    - MR !2679 and MR !3066
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Batch authorization proposal v3: close eight park-radar merge requests

Status of this handoff is **blocked** until MR !3281 (a one-line radar correction, below) is on `main`. This replaces proposals
v1 (MR !3270) and v2 (MR !3280), both never merged. It closes nothing. All eight merge requests were authored by the Human
Governor, so the cross-approval delegation does not cover them; the Human Governor decides, after the non-author review the
work order requires.

## What review changed (v2 to v3)

- The authorization digest now covers exactly `{iid, state, disposition, note}` as the work order requires. A separate
  full-baseline digest covers heads, branches and per-MR discussion state.
- Discussion state is a per-MR digest over every non-system note's `[id, updated_at]` pair, so an edit to an earlier note is
  detected, not only the latest one.
- The leftover count for !459 was wrong. It is **thirteen coordination records and one proposed learning record**
  (`20260723T231109Z-w7k2` in `agf/learnings/foundry.yaml`) that stay only on the retained branch. `docs/radar.md` on `main`
  repeated the wrong count; MR !3281 corrects it and must land before closure.

## Authorization lines (digest 1)

Eight JSON objects, keys sorted, compact separators, UTF-8, one per line in this order, one trailing newline. SHA-256 over the
exact text including that final newline.

```jsonl
{"disposition":"park-radar","iid":459,"note":"Closed as park-radar. Its retained question and trigger are on main in docs/radar.md with review-by 2027-01-31, carried verbatim under docs/proposals/radar/459-issue-53-feedback-latency-measurement.md (merge 208f1d5a1031d49ac5b6056738a928c39dd0cb88). The source branch plan/issue53-blinded-ci-followup (head 09b5147f2095364cc238b837777de70ba5d1edac) is retained and nothing is deleted. Reopen if the radar trigger fires.","state":"opened"}
{"disposition":"park-radar","iid":923,"note":"Closed as park-radar. Its retained question and trigger are on main in docs/radar.md with review-by 2027-01-31, carried verbatim under docs/proposals/radar/923-expectation-signals-consensus.md (merge 208f1d5a1031d49ac5b6056738a928c39dd0cb88). The source branch proposal/expectation-signals-consensus (head 8bb1efc2660c3be387aca63d91abcb2d931f5d58) is retained and nothing is deleted. Reopen if the radar trigger fires.","state":"opened"}
{"disposition":"park-radar","iid":967,"note":"Closed as park-radar. Its retained question and trigger are on main in docs/radar.md with review-by 2027-01-31, carried verbatim under docs/proposals/radar/967-ai-regulation-requirements-gap-analysis.md (merge 208f1d5a1031d49ac5b6056738a928c39dd0cb88). The source branch docs/ai-regulation-gap-analysis (head ae29daf8128d1cfb8809d7383b40379bb5aefc62) is retained and nothing is deleted. Reopen if the radar trigger fires.","state":"opened"}
{"disposition":"park-radar","iid":998,"note":"Closed as park-radar. Its retained question and trigger are on main in docs/radar.md with review-by 2027-01-31, carried verbatim under docs/proposals/radar/998-learning-field-enforcement-audit.md (merge 208f1d5a1031d49ac5b6056738a928c39dd0cb88). The source branch plan/learning-field-enforcement-audit (head 945c335f8738be8d90029cc8e42001c85f231f74) is retained and nothing is deleted. Reopen if the radar trigger fires.","state":"opened"}
{"disposition":"park-radar","iid":1000,"note":"Closed as park-radar. Its retained question and trigger are on main in docs/radar.md with review-by 2027-01-31, carried verbatim under docs/proposals/radar/1000-llis-deferred-learning-candidates.md (merge 208f1d5a1031d49ac5b6056738a928c39dd0cb88). The source branch docs/llis-schema-learnings-deferred-todo (head ab9e36b9d48a1ac5051a37df4f5bb45aad7e7e59) is retained and nothing is deleted. Reopen if the radar trigger fires.","state":"opened"}
{"disposition":"park-radar","iid":1451,"note":"Closed as park-radar. Its retained question and trigger are on main in docs/radar.md with review-by 2027-01-31, carried verbatim under docs/proposals/radar/1451-strategic-candidate-comparison.md (merge 208f1d5a1031d49ac5b6056738a928c39dd0cb88). The source branch plan/strategic-comparison-queued (head 947be2ea99dc0be14375ab2b558fcc62a9a15ce5) is retained and nothing is deleted. Reopen if the radar trigger fires.","state":"opened"}
{"disposition":"park-radar","iid":1718,"note":"Closed as park-radar. Its retained question and trigger are on main in docs/radar.md with review-by 2027-01-31, carried verbatim under docs/proposals/radar/1718-operational-debt-phase0-measurement.md (merge 208f1d5a1031d49ac5b6056738a928c39dd0cb88). The source branch docs/learning-dd-operational-debt-backlog (head a5e690e2877c8a5a49555062b94c193ab20af79a) is retained and nothing is deleted. Reopen if the radar trigger fires.","state":"opened"}
{"disposition":"park-radar","iid":1722,"note":"Closed as park-radar. Its retained question and trigger are on main in docs/radar.md with review-by 2027-01-31, carried verbatim under docs/proposals/radar/1722-supply-chain-signing-learning-entry.txt (merge 208f1d5a1031d49ac5b6056738a928c39dd0cb88). The source branch docs/learning-dd-supply-chain-signing (head 95d95c5fe06d7c20d9b6c5f06d9790af1d85240b) is retained and nothing is deleted. Reopen if the radar trigger fires.","state":"opened"}
```

Authorization digest: `fcb0776461780c88633183eb4e3d5be0497a32f23a9a0cb39f4cd67e4b1727ab`

## Baseline lines (digest 2, pre-execution re-read)

Same serialization. `notes_digest` is the SHA-256 of the compact JSON array of `[id, updated_at]` for every non-system note in
ascending order, followed by a newline. `user_note_count` counts those notes.

```jsonl
{"branch":"plan/issue53-blinded-ci-followup","head":"09b5147f2095364cc238b837777de70ba5d1edac","iid":459,"notes_digest":"5c6e00968fc5c74018b3c9f7a372223359c5c9a904ce7195ca76cebe7d84606e","state":"opened","user_note_count":1}
{"branch":"proposal/expectation-signals-consensus","head":"8bb1efc2660c3be387aca63d91abcb2d931f5d58","iid":923,"notes_digest":"9c5d1c917a68641c5068d7b2dbe45d3f80c9e473110bfa176ef2ed0a0b7434fc","state":"opened","user_note_count":5}
{"branch":"docs/ai-regulation-gap-analysis","head":"ae29daf8128d1cfb8809d7383b40379bb5aefc62","iid":967,"notes_digest":"510e5a4e1a2595f0cd81bfbe4d344ba8df9c3c64f3e1e538977ef6e8d3625974","state":"opened","user_note_count":12}
{"branch":"plan/learning-field-enforcement-audit","head":"945c335f8738be8d90029cc8e42001c85f231f74","iid":998,"notes_digest":"37517e5f3dc66819f61f5a7bb8ace1921282415f10551d2defa5c3eb0985b570","state":"opened","user_note_count":0}
{"branch":"docs/llis-schema-learnings-deferred-todo","head":"ab9e36b9d48a1ac5051a37df4f5bb45aad7e7e59","iid":1000,"notes_digest":"37517e5f3dc66819f61f5a7bb8ace1921282415f10551d2defa5c3eb0985b570","state":"opened","user_note_count":0}
{"branch":"plan/strategic-comparison-queued","head":"947be2ea99dc0be14375ab2b558fcc62a9a15ce5","iid":1451,"notes_digest":"4aa5fcf495d484cb9bb1f40c3621f1261a14d83fda5243cda6714369ebb4241e","state":"opened","user_note_count":1}
{"branch":"docs/learning-dd-operational-debt-backlog","head":"a5e690e2877c8a5a49555062b94c193ab20af79a","iid":1718,"notes_digest":"863e4e184e288f1f07bbff536bb22ece9a7b956ef75230d70f9852dcf9b963ed","state":"opened","user_note_count":11}
{"branch":"docs/learning-dd-supply-chain-signing","head":"95d95c5fe06d7c20d9b6c5f06d9790af1d85240b","iid":1722,"notes_digest":"100b2c8c92594d45e9eba3aa89c6a1e99399c0306143c9df9190cf147178c620","state":"opened","user_note_count":1}
```

Baseline digest: `ec8fa8f2f2ffad509a49becb672974a5f41dd80f989800032a229a0e59c74d19`

Pre-execution: re-read each item; any item whose state, head, branch, note count or notes digest differs is dropped from the
batch (work order step 4).

## What closing leaves only on the retained branches

Per the radar entries on `main`: the thirteen coordination records and the proposed learning record of !459 (after MR !3281),
the 554-line `branches.csv` of !1718, and the `docs/sources.md` edit of !967. The proposed learning records of !1718 and !1722
are carried as radar text and are not filed in `agf/learnings/foundry.yaml`. Accepting these is part of the decision.

## Points for the Human Governor

1. Authorize closing all eight, or name items to pull. Items with real discussion: !923 (5 notes), !967 (12), !1718 (11).
2. Accept the branch-only leftovers above.
3. The Human said old merge requests should be merged or closed and not left open, if Codex agrees. I recommend closing, not
   merging: the eight carry content no one has reviewed.
