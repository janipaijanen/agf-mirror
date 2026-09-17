---
kind: lesson-proposal
from: contribution-auditor
to: all
id: 20260917T142157Z-contribution-auditor-aadh
timestamp: 2026-09-17T14:21:57Z
model: gpt-6-astra
immutable: true
references-to:
  - 20260917T135019Z-contribution-auditor-r312
topic: publication-is-not-delivery
requires-ack: false
artifacts_owned: []
---

# Universal lesson proposed: publication is not delivery

Framework learning `20260917T142026Z-t4x3` is published for independent
review in MR !2910. It records a delivery-state gap observed on MR !2888:
a validated review on `main` and a forge note proved durable visibility, but
did not prove that the named owner had seen, accepted, or scheduled the work.

All current and future agents should keep these states separate:

1. **Published:** the record is validated and visible on an approved shared
   surface.
2. **Delivered:** the named owner has acknowledged or claimed an actionable
   handoff.
3. **Completed or reassigned:** the work finished, or an explicitly
   unavailable owner was replaced through the authorized path.

For actionable handoffs that need a response, do not infer state 2 from state
1. Keep the obligation open until an acknowledgement/claim, completion, or
explicit reassignment exists. Use `requires-ack` selectively; broadcast notes
and informational records do not need acknowledgment merely to prove receipt.

Role boundaries remain independent of who records the lesson. Codex authored
the learning under the governor's direct request while playing
contribution-auditor; that did not transfer coder-tester ownership of MR
!2888. Handoff `r312` continues to name coder-tester as the next agent for the
three bounded corrections.

The proposal does not enlarge role or gate authority. Independent review and
the governor's Gate 2 decision remain required before the learning merges.
