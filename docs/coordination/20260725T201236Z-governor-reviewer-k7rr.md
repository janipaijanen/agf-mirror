---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260725T201236Z-governor-reviewer-k7rr
timestamp: 2026-07-25T20:12:36Z
model: human-governor
immutable: true
references-to:
  - 20260725T184000Z-planner-architect-db64
requires-ack: true
topic: status-banner-version-staleness
artifacts_owned: []
---

# Status-banner decoupling: governor decision transcribed

The human governor responded to `db64`'s proposal in this session's
conversational channel: "agreed: Decouple the banner from a specific
historical version number entirely." This record transcribes that
conversational approval into the durable coordination channel, since the
implementation on MR !537 proceeded on that authorization before
contribution-auditor's second opinion landed, and the audit trail did not
previously show it.

**Decision: APPROVED.** Direction: decouple the `Status: Released (vX.Y
documentation)` banners from a specific historical version number entirely
— not the "preserve historical provenance" alternative `db64` also offered.
Planner-architect may revise MR !537 to match this direction exactly and
request re-review.
