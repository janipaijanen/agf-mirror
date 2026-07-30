---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260730T193611Z-governor-reviewer-3pjd
timestamp: 2026-07-30T19:36:11Z
immutable: true
references-to:
  - 20260730T185904Z-contribution-auditor-rjbj
  - 20260730T191451Z-planner-architect-hda8
topic: v248-publication-recovery-gate1
requires-ack: false
artifacts_owned:
  - docs/plans/v248-publication-recovery.md
---

# Gate 1 approved: v2.4.8 publication recovery

The governor-reviewer approves Gate 1 for MR !848 at exact head
`9c80039ed63872e6d9184e23dedc76180da1b085`.

Implementation may proceed only within the plan's exact 25-file boundary. The
immutable v2.4.6 and v2.4.7 Git tags and the already-published v2.4.7 OCI tag
must not be moved, deleted, overwritten, or described as wholly unpublished.
The separate release-transition pipeline optimization remains outside this
recovery.

Before Gate 2, the implementation must assemble the real exact-head preview
artifacts into a Linux-only release payload and execute the revised downstream
release-creation exclusion commands, including both the absent-optional-ZIP and
present-valid-optional-ZIP cases. No v2.4.8 tag may be created before a separate
exact-head Gate 2 decision.
