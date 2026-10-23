---
kind: decision-record
from: governor-reviewer
to: planner-architect,coder-tester
id: 20261023T140318Z-governor-reviewer-7rp6
timestamp: 2026-10-23T14:03:18Z
model: human-governor
immutable: true
references-to:
  - 20261023T135540Z-planner-architect-b2pm
  - 20261023T135456Z-coder-tester-pm7t
  - 20261023T132439Z-coder-tester-6hag
  - 20261023T112836Z-governor-reviewer-dhdh
topic: ratify-mr3160-amendment-gate2-and-plan-merges
requires-ack: false
artifacts_owned:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3144
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3146
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3160
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3163
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Governor decisions: MR !3160 amendment and Gate 2; plan-MR merges

The Human Governor replied in a live session to five numbered decisions:
"accept. approved - merge. merge. approved. proceed." This record is filed on
the Governor's behalf as previously instructed. It ratifies the mapping and
facts in bridge record `20261023T135540Z-planner-architect-b2pm` and the joint
measurement recommendation in handoff
`20261023T135456Z-coder-tester-pm7t`.

## 1. MR !3160 measurement amendment accepted

For approved plan `20261023T080431Z-instruction-context-followup` and MR !3160
exact head `fe94fa8adbfaca2622295a0de01ff418e5a4303e`, the unavailable measured
processed-character report is replaced by all five conditions in `pm7t`:

1. the expected exact 27-file loaded set is confirmed;
2. the exact raw loaded-set sum is at most 140,000 characters;
3. controlled startup input-token total falls by at least 6,500 tokens and at
   least 8% against the exact main baseline, using the recorded formula;
4. three runs per side are stable and startup has no combined-limit warning;
5. the required trigger replays and exact-head pipeline pass.

Accepted evidence: 136,404 loaded-set characters; 81,179 to 74,125 startup
input tokens, down 7,054 and 8.69%, stable over three runs per side; no warning;
replays pass; exact-head pipeline `2981300258` succeeded. This resolves the sole
open finding in review `20261023T132439Z-coder-tester-6hag` for this exact head.

## 2. MR !3160 Gate 2 approved and merge authorized

Gate 2 is approved and merge to `main` is authorized for MR !3160 only at exact
head `fe94fa8adbfaca2622295a0de01ff418e5a4303e`. A new source head voids this
authorization.

## 3. Approved plan MRs !3144 and !3146 may merge

Merge to `main` is authorized for:

- MR !3144 only at `04ab395d911ab7536a5776b242e5ec260e9c5d31`;
- MR !3146 only at `e9b346b56d2a09afc32f2c130f4a56f571cef497`.

Both plans are approved under decision `20261023T112836Z-governor-reviewer-dhdh`.
A new source head voids the corresponding authorization.

## 4. Split plan MR !3163 stays shelved Draft

MR !3163 is not approved and is not closed. It stays Draft and pending Gate 1.
No implementation may start from it. Its failed collision job is not retried.

## 5. Recall-suite proposal may proceed to Gate 1 drafting

The planner may draft a separate Gate 1 plan for a committed declarative
scenario corpus, manual protocol, and result schema. Raw model transcripts stay
MR evidence; no runner is added before the corpus is used twice; paid or
nondeterministic model calls do not enter blocking CI. This is drafting
authority only, not Gate 1 approval or implementation authority.

No release, tag, deploy, or additional scope is authorized. The expired `qpda`
delegation is not used.
