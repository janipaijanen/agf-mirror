---
kind: decision-record
from: governor-reviewer
to: all
id: 20260917T061354Z-governor-reviewer-7ppm
timestamp: 2026-09-17T06:13:54Z
model: human-governor-via-codex
immutable: true
references-to:
  - 20260912T220706Z-governor-reviewer-e7mr
  - 20260917T055825Z-governor-reviewer-h2jk
  - 20260913T092433Z-coder-tester-nbr1
  - 20260910T195332Z-planner-architect-khgr
topic: ruw-queue-learning-contract-otgr-priority-order
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Decision: RUW and queue convergence precede learning-contract and OTGR follow-ups

The governor approves the priority disposition proposed in the active
development session. This decision sequences already-approved directions; it
does not add a workstream to v2.11, approve a Gate 1 plan, authorize
implementation, or authorize a release/tag action.

## Active priority order

1. **P0 — v2.11 RUW.** Decision `e7mr` remains controlling: v2.11 is the
   active Remaining Useful Work programme. Existing authorized v2.11
   implementation, review, planning, and release-precondition work takes
   precedence over the new follow-up designs.
2. **P0 — merge-request queue convergence.** MR cleanup is part of the RUW,
   not competing work. Evidence-backed pre-release dispositioning and the
   approved stale-plan cohort may proceed now. Broader closure executes in the
   already-governed sequence and must preserve unique content before closing a
   source MR.
3. **P1 — issue closeout.** Existing issue-ledger dispositions remain the
   basis. Release-dependent closure waits for the applicable release condition;
   intentional radar, reconstruction, and governor-decision items remain open
   until their own disposition permits closure.
4. **P2 — versioned learning contracts.** Decision `h2jk` remains approved in
   substance, including the GitLab Pages namespace, versioned schema/lifecycle/
   semantic-profile contracts, and non-destructive legacy handling. It is a
   prepared follow-up and is not admitted into v2.11 by this record. Plan-only
   research may advance during genuine RUW waits when it neither delays an
   executable RUW obligation nor opens avoidable MRs. Implementation requires a
   separately approved Gate 1 plan.
5. **P3 — OTGR.** Preserve the approved design decisions, but retain `e7mr`'s
   sequencing: OTGR technical replanning and implementation follow v2.11. No
   active RUW capacity is diverted to OTGR. Closing or preserving stale OTGR
   carriers may proceed only through the queue-convergence evidence rules.
6. **After RUW and queue convergence — operating-model assessment.** The token,
   service, and consumer-GPU footprint assessment remains triggered only after
   v2.11 is published and verified and the MR queue has reached its deliberately
   retained terminal set. A non-zero queue is allowed when every remaining item
   has an intentional live disposition; arbitrary numerical zero is not the
   goal.

## Immediate execution focus

The next visible RUW planning obligation is Row 5 S3 from `nbr1`, now unblocked
by merged CP-B-2: define the bounded material challenge-response slice using the
existing urgency contract and explicitly dispose the superseded design in MR
!2027. In parallel, authorized agents should continue evidence-backed MR cohort
disposition and closure rather than opening unrelated planning MRs.

Waiting on one RUW pipeline, review, or binding decision is not a global stop.
Agents continue other dependency-independent RUW or queue-convergence work
within their roles. Local follow-up research is permissible during such waits;
publishing a new plan/MR or beginning implementation still follows the priority
and gate boundaries above.

## Supersession and compatibility

This record narrows the sequencing language in `h2jk` that called the
learning-contract plan the "next governed work": it remains the next governed
work for that follow-up topic, but it is not the next project-wide priority.
All substantive learning-contract decisions in `h2jk` remain in force.

This record reinforces, and does not supersede, `e7mr`'s v2.11/OTGR ordering.
Adding learning-contract implementation to v2.11 would require a separate,
explicit governor scope amendment; it cannot occur by inference from `h2jk` or
this record.

## Provenance

The human governor approved this priority order directly in the active
development session. Codex transcribed the decision into the coordination
schema without adding gate, merge, release, or implementation authority.
