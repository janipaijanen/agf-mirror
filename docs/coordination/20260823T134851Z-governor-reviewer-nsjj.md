---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260823T134851Z-governor-reviewer-nsjj
timestamp: 2026-08-23T13:48:51Z
model: human-governor
immutable: true
references-to:
  - 20260823T134321Z-coder-tester-djjk.md
  - 20260823T132803Z-contribution-auditor-rtqp.md
  - 20260823T090334Z-governor-reviewer-g4h3.md
topic: issue110-recovery-batch-gate2-approved
requires-ack: false
artifacts_owned: []
---

# Gate 2 approved: issue #110 coordination recovery and conditional closure

This record transcribes the human governor's binding instruction in the active
session: "Requested Gate 2 is approved. proceed."

## Binding decision

The combined Gate 2 request in `djjk` is approved exactly as presented:

1. merge recovery MR !1808 only at exact head
   `5dd6bec423fa809e46791cdbc9e2ef71324292bf`; and
2. only after that merge is verified on `main`, execute the plan's Step 4
   conditional close operation for the 51 surviving carrier MRs.

Before each close, coder-tester must confirm that the MR remains open and
unchanged and that every record attributed to it is present byte-for-byte on
landed `main`. Any drift causes that MR to be skipped and recorded, not forced.
Each matching MR receives the standardized closing note naming the recovery MR
and merge commit plus its exact landed paths before it is closed.

No source branch may be deleted. No MR outside the approved 51-MR set may be
closed. Carrier closure remains bookkeeping and does not resolve the recovered
coordination message's underlying obligation.

The disclosed `pending-gate1` plan metadata does not supersede the binding Gate
1 approval in `g4h3`; this Gate 2 decision is based on that authority and the
independent COMPLETE/CLEAR exact-head review in `rtqp`.
