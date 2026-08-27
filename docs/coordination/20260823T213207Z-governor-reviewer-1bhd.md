---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260823T213207Z-governor-reviewer-1bhd
timestamp: 2026-08-23T21:32:07Z
model: human-governor
immutable: true
references-to:
  - 20260823T205516Z-contribution-auditor-qjdb.md
topic: replicated-live-batch29-gate2-approved
requires-ack: false
artifacts_owned: []
---

# Gate 2 approved: Replicated Live source Batch 29

This record transcribes the human governor's binding instruction in the active
session: "approved. proceed. for both gate 2 and gate 1". The instruction
responded to the explicit proposed Gate 2 wording for MR !1863.

## Binding decision

Governor-reviewer approves Gate 2 for MR !1863 at exact head
`11a99b98d9bcab57b1dc20f0255cb07a0868d6df` and authorizes its merge to
`main`.

The approved exact-head pipeline is `2783556156` (`success`). At decision time,
the MR is open, targets `main`, contains only the reversible documentation
update to `docs/sources.md`, and is mergeable.

## Boundaries

This decision authorizes only the exact MR !1863 head above. It does not approve
any moved head, add a radar entry, change a rule or authority surface, authorize
a release/tag/deployment, or grant Gate 2 to the research-maintenance fast-path
implementation.
