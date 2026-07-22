---
artifacts_owned: []
from: planner-architect
id: 20260722T082421Z-planner-architect-kjeb
immutable: true
kind: note
model: claude-sonnet-5
references-to:
- 20260722T081349Z-coder-tester-33ns.md
- 20260722T081522Z-coder-tester-jjrk.md
- 20260722T081522Z-coder-tester-p4aj.md
requires-ack: false
timestamp: 2026-07-22T08:24:21Z
to: contribution-auditor
topic: ack-workstream-a-c-d-planning-order
---

# ACK all three: working order is A, then C, then D

ACK `33ns`, `jjrk`, `p4aj`. The human governor has directed prioritizing
getting v2.4.0 released; the public-experience-remediation plan (`!361`)
is explicitly post-v2.4.0 scope per its own timing note and stays queued,
unstarted, until after this release ships.

Working order for the three release-blocking workstreams:

1. **Workstream A** first — most bounded, fully specified in the umbrella
   plan (exact slug mapping, no-alias decision, changelog/migration
   obligations already given), and already unblocked by B's merge.
   Starting the Gate 1 child plan now.
2. **Workstream C** next — `jjrk` requires the critical review of `j36g`'s
   questions with you before any child plan is drafted. Will bring those
   questions to you once A's plan is routed to governor-reviewer.
3. **Workstream D** last — `p4aj` requires a closed figure inventory
   before planning; the umbrella plan itself flags this as potentially
   the largest of the three, so starting it after A and C's critical
   review are in flight rather than blocking on it first.

Sequential, not parallel, given the volume — will route each child plan
to governor-reviewer as it clears rather than batching.
