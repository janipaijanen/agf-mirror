---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260730T061322Z-governor-reviewer-gsgs
timestamp: 2026-07-30T06:13:22Z
model: human
immutable: true
references-to:
  - 20260729T200957Z-contribution-auditor-j5pk
  - 20260729T195431Z-planner-architect-msqs
  - 20260729T185725Z-governor-reviewer-m5qp
topic: mr795-interruptibility-gate1-approval
requires-ack: true
artifacts_owned:
  - docs/plans/interruptible-superseded-pipelines.md
---

# Gate 1 approved: interruptible superseded pipelines

The human governor-reviewer explicitly approved Gate 1 for MR !795 at exact
source head `c1f95749993a2f52c12739e848f4381b2ae00ccb` in the current session.
Codex transcribes that binding decision into the durable coordination channel;
it does not infer or expand the governor's authority.

**Decision: APPROVE Gate 1.** Exact-head pipeline `2716544482` passed. The
contribution-auditor's final exact-head record `j5pk` is CLEAR with no open
design, factual, safety, or gate-boundary finding. Review-publication pipeline
`2716575200` also passed.

The approved scope is the explicit per-job opt-in design in
`docs/plans/interruptible-superseded-pipelines.md`: add
`interruptible: true` to exactly the 18 classified restart-safe jobs and leave
the 6 protected default-branch/tag publication jobs implicitly
non-interruptible. Preserve the approved mandatory pre-merge Gate 2 evidence
and the separately tracked post-merge monitoring boundary.

Planner-architect may mechanically change the plan status to `approved`, sync
MR !795's stale description to the approved head and `j5pk`/this decision,
and land the plan-only MR through the governed merge path.

Per the still-controlling sequencing decision `m5qp` and the approved plan,
implementation does **not** begin until MR !792 receives its final Gate 2
evidence disposition and is merged or otherwise finally resolved, because
both streams touch `.gitlab-ci.yml`. Once that condition is satisfied,
build-engineer may implement the frozen !795 scope on a separate branch and
must return with the approved pre-merge evidence for exact-head Gate 2.

This Gate 1 decision does not approve an implementation merge, alter MR !792's
evidence disposition, unpark the v2.4.6 release, authorize a release/tag, or
adopt Candidate D (`workflow:auto_cancel:on_new_commit`). Those remain
separate decisions and scopes.
