---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260912T094217Z-governor-reviewer-3rha
timestamp: 2026-09-12T09:42:17Z
model: gpt-6-astra
immutable: true
references-to:
  - 20260912T091257Z-planner-architect-h5hr
topic: des6-rewrite-disposition
requires-ack: true
artifacts_owned: []
---

# Governor disposition: rewrite DES-6

Transcribed by Codex from Jani's current task message, "proceed as proposed",
following the explicit recommendation to rewrite the stale governance document
to reflect current authority rather than archive it. The decision is the human's;
the transcribing model has no independent gate authority.

For DES-6, rewrite `agf/docs/governance/governance.md` to describe the current
governed role, ownership, coordination and gate model. Use current
`agf/agents/MANIFEST.yaml`, role definitions, governed rules and decisions as
authority; reconcile the current loader against them rather than copying a stale
role table. Correct the old model-specific phase framing, sequence-keyed channel,
nonexistent charter path and obsolete framework footer. Do not change role
activation, gate powers, schemas, or policy as part of this documentation rewrite.
Record the bounded disposition and implementation evidence in the existing
estate work/plan trail; preserve exact-head review and Gate 2 before merge.

Claude retains estate correction/completion and release-precondition ownership
under d5mq/6rs6. Codex continues CP-B and the requested independent DES-5 review.
Please acknowledge and advance the remaining Pages inventory and release
precondition assessment while CP-B CI runs. Publish further coordination on an
existing shared coordination branch with bridge notes; avoid additional
status-only MRs. Existing unique work must not be discarded to reduce MR count.

This message does not approve CP-B-2 Gate 1, any new Gate 2 candidate, or a
release/tag. The newly proposed DES-6 disposition is the binding decision here.
