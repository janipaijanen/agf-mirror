---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260824T142545Z-governor-reviewer-kqm3
timestamp: 2026-08-24T14:25:45Z
model: human-delegated-via-gpt-5.6
immutable: true
references-to:
  - 20260824T140945Z-planner-architect-2b65.md
  - 20260824T071103Z-governor-reviewer-ekns.md
topic: foundry-historical-version-gap-disposition
requires-ack: true
artifacts_owned: []
---

# Accept the foundry historical framework-version gap as disclosed and unreviewed

The governor-reviewer approves the first disposition proposed in escalation
`20260824T140945Z-planner-architect-2b65`.

The mismatch between `agf/learnings/foundry.yaml`'s recorded
`adoption.framework_version: "2.0.0-provisional"` and the current
`agf/VERSION` value `2.6.1` is accepted as a historical, explicitly disclosed,
unreviewed gap. No retroactive line-by-line changelog review is required. This
decision does not represent or imply that such a review occurred.

Within the already-approved Gate 1 scope for MR !1856, the implementation may
now correct the foundry adoption record, add `checkout_role: live`, and make the
other plan-defined schema, validator, upgrade-skill, template, and documentation
changes. The correcting commit and resulting evidence must reference this
decision so the prior stale value is not silently erased from the audit trail.

This decision disposes only the one-time historical-gap question. It is not
Gate 2 approval, implementation merge approval, release/tag/deploy authority,
or permission to expand the approved plan.
