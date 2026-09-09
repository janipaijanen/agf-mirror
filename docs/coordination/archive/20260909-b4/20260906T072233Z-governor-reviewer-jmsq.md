---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260906T072233Z-governor-reviewer-jmsq
timestamp: 2026-09-06T07:22:33Z
model: human
immutable: true
references-to:
  - 20260905T215114Z-planner-architect-gpss
  - 20260905T214541Z-contribution-auditor-p37b
topic: v290-atomic-release-unit-adopted
requires-ack: true
artifacts_owned:
  - docs/plans/20260905T145109Z-route-all-jobs-gate-accompaniment.md
---

# Adopt the atomic release unit and assign its plan revision to Codex

The governor directly instructs: “Advance this” in response to the stated
v2.9.0 unblock choice between approving the current MR !2373 plan and adopting
the `gpss` atomic-unit approach with a Codex plan revision.

The binding disposition is to adopt
`20260905T215114Z-planner-architect-gpss` and assign Codex
planner-architect to revise the release plan. The atomic Gate 2 candidate must
combine:

1. MR !2362's ordinary-runner defaults;
2. MR !2392's native mandatory Linux packaging and its bounded Windows DinD
   path;
3. MR !2373's six remaining genuinely nested-container `[dind]` overrides;
   and
4. both gate-accompaniment test families, including RED-then-GREEN evidence.

This resolves `p37b` B1/B2 and A2 by forbidding either invalid intermediate
merge order. The plan must refresh the live runner evidence: runner 55975189
is online, tagged only `dind`, and does not run untagged; runner 55908035 is
online, retains `podman/linux/amd64`, and runs untagged. The operator reports
the duplicate `/cache` configuration fixed, but the first tagged nested job
must still prove executor preparation and workload success.

Temporary allocation: Codex owns the plan revision and release-unit
integration lane until its Gate 1 handoff. The Claude session that currently
owns MR !2373 must acknowledge this binding allocation and stop mutating the
old plan head; it then acts only as independent contribution-auditor/infosec
reviewer for the revised exact head. Consensus cannot alter the governor's
decision, but the acknowledgement makes the execution lane unambiguous.

This decision authorizes the plan revision and review convergence. It does not
pre-approve Gate 1, implementation, Gate 2, merging the atomic implementation,
retrying unchanged failed pipelines, tagging, or publishing v2.9.0.
