---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260909T015949Z-governor-reviewer-sgbh
timestamp: 2026-09-09T01:59:49Z
model: human
immutable: true
references-to:
  - 20260908T201001Z-coder-tester-2mp6.md
  - 20260908T185629Z-governor-reviewer-j2gs.md
  - 20260908T183242Z-planner-architect-ds4k.md
topic: b1-gate2-and-handled-mr-cleanup-approved
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# B1 Gate 2 and staged handled-MR cleanup approved

Transcribes the governor's binding in-session direction on 2026-09-09:
"learning first, then proceed as proposed." The proposal followed a complete
poll of 72 open merge requests and separated handled non-plan carriers from
plan-lifecycle items and unique work.

The learning-first condition is satisfied by filing learning
`20260909T015659Z-aebe` in MR !2574 after both learning and InfoSec validation.
That learning MR still follows its own exact-head pipeline and Gate 2; filing it
first does not pre-approve its later merge.

## Binding decisions

1. **Gate 2 is approved for queue-convergence B1 MR !2562** at exact head
   `161c9281a2a6bfb4d5dc0122843169e208b6051b`, whose canonical MR pipeline
   `2830800994` succeeded. The governor, who did not author `ds4k`, confirms
   the `ds4k`-derived roadmap rows and sequencing as part of this approval.
   Merge is authorized with exact-SHA protection.
2. After B1 merges, coder-tester may implement the already-Gate-1-approved
   queue-convergence **non-plan MR disposition cohort**: one evidence-bearing,
   review-sized ledger batch covering handled/superseded carriers. Each row
   must cite its canonical replacement or falsifying fact; age alone is never
   evidence. The batch returns for its own exact-head Gate 2 before forge
   closures execute.
3. **Plan-MR closures remain held.** They wait for `qbka` Slice A's canonical
   supersession-provenance contract, or a later explicit governor override,
   exactly as `ds4k` Finding 1 requires.
4. Unique evidence and still-live work are advanced, reconstructed, or parked;
   they are not closed merely to reduce the count.

This decision does not approve the future disposition-ledger batch's Gate 2,
the learning MR's Gate 2, `qbka`, #129 implementation, a release, a tag, or
Propeller activation work outside separately published authority.
