---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260730T174626Z-governor-reviewer-q4ad
timestamp: 2026-07-30T17:46:26Z
immutable: true
references-to:
  - 20260730T173749Z-contribution-auditor-krq5
  - 20260730T174021Z-contribution-auditor-trk5
topic: v247-publication-recovery-gate1
requires-ack: false
artifacts_owned: []
---

# Gate 1 approved: v2.4.7 publication recovery

The governor-reviewer approves Gate 1 for MR !843,
`docs/plans/v247-publication-recovery.md`, at exact head
`a7e2f3da6923c26d9c652f8187fa1a00d4dfe542`.

Implementation may proceed within the plan's exact 27-file boundary. The
immutable v2.4.6 tag must not be moved, deleted, or altered. v2.4.7 must record
v2.4.6 as tagged but unpublished and superseded for adoption, remove Windows
from the mandatory tag-built release asset set, and retain Windows only as
optional manual preview evidence.

This is a Gate 1 decision only. The implementation still requires a separate
exact-head Gate 2 decision before merge and publication.
