---
kind: decision-record
from: governor-reviewer
to: all
id: 20260729T092303Z-governor-reviewer-bn8g
timestamp: 2026-07-29T09:23:03Z
model: human
immutable: true
references-to:
  - 20260729T070455Z-contribution-auditor-72sb
  - 20260729T083528Z-contribution-auditor-gpea
  - 20260729T091518Z-contribution-auditor-mq4r
topic: v250-statement-of-work-gate1-approved
requires-ack: false
artifacts_owned:
  - docs/plans/v250-statement-of-work.md
---

# Gate 1 approved: v2.5.0 Statement of Work

The governor-reviewer approves Gate 1 for MR !755 at exact source head
`e8c2fcb3b9ceda9f18f11a378b70b69df5b6edbf`.

The instruction was given directly in the current human/AI session:

> I as governor approve
> - Gate 1 for MR !755 at e8c2fcb

Codex transcribed that direct human decision into the durable coordination
channel; it did not infer or expand the governor's authority.

The approved plan is the externally reprioritized revision:

- v2.4.6 recovery is the first release deliverable;
- internal board/archive/backlog hygiene does not block it unless a specific
  dependency is proved;
- v2.5's adopter outcome is upgrade-safe role authority and actionable
  acknowledgement obligations;
- rc.1 is an exact-artifact acceptance checkpoint, not an unsupported claim of
  adopter feedback;
- immature trust-chain expansion remains parked outside this SoW.

This decision authorizes the mechanical plan-status update and governed child
planning, beginning with the v2.4.6 recovery plan. It does not authorize
implementation, an issue/MR closure, a board or archive migration, a version
bump, a release branch, a tag, or a GitLab Release. Each implementation child
requires its own approved Gate 1 plan and later Gate 2 decision.

Any substantive MR !755 source change beyond recording `status: approved`
invalidates this exact-head approval and must return to the governor.

MR !772 rule formalization is not a prerequisite and remains behind the
release train.
