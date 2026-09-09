---
kind: decision-record
from: governor-reviewer
to: all
id: 20260909T184019Z-governor-reviewer-st83
timestamp: 2026-09-09T18:40:19Z
model: human-via-codex
immutable: true
references-to:
  - 20260909T171028Z-planner-architect-8d8e
  - 20260909T174246Z-coder-tester-nd3b
  - 20260909T174013Z-coder-tester-4hbp
topic: gate-approvals-2628-2629
requires-ack: false
artifacts_owned: []
---

# Gate approvals for !2628 and !2629

The governor-reviewer instructed: “proceed. After RUW is ready, then focus on
closing those open MRs.” This decision records the two gate requests presented
immediately before that instruction.

1. Gate 1 is approved for MR !2628 at exact head
   `b70c153d6eeaf82fd4a6b0ef0a18f3533b382d74`. This authorizes CP-B-1
   implementation strictly within the approved plan. It does not authorize
   CP-B-2, CP-B-3, release publication, tagging, or merge of the resulting
   implementation.
2. Gate 2 is approved for MR !2629 at exact head
   `49bdfe391b9e3d4c389b07c782a55e9ef67e49fd`. This authorizes merging the
   B8 queue-convergence intake-bound change after exact-head and target-state
   revalidation.

After RUW v2.11 is published and verified, the active priority is to dispose of
the remaining open merge requests to terminal states using their ledgers,
current gates, exact-head evidence, and ownership. This is not authority for
indiscriminate bulk closure or for bypassing required gates.

The standing direction not to pursue semantic-data work remains unchanged.
