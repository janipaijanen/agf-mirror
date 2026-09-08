# Roadmap

*Audience: evaluators, adopters, contributors, and maintainers or AI
assistants looking for current release and development status.*

This page is the public projection of the governed internal
[foundry roadmap](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/docs/roadmap.md).
It summarizes current state without creating release scope or authority of its
own.

| Field | Value |
|---|---|
| Status | v2.10.0 is published and independently verified. Post-v2.10 work is proceeding through separately gated, review-sized batches. |
| Observed at | 2026-09-08T19:02:54Z |
| Source revision | `13fe97382ee338cba8f049bb138ef2e3a65169a1` (`main`) |
| Governed source | `docs/roadmap.md`, the approved queue-convergence plan, and the governor decisions cited there |
| Authority | Informational projection. The internal roadmap, binding plans and decisions, exact tags, and release evidence are authoritative. |

## Current released baseline

**Latest fully published stable release: v2.10.0.** Its annotated tag,
mandatory tag pipeline, GitLab Release, generic package, OCI image, checksums,
and anonymous downloads were independently verified. Use the
[v2.10.0 GitLab Release](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases/v2.10.0)
for public artifacts.

No later version or candidate scope is committed merely because it appears in
an issue label, proposal, or advisory horizon. Each later release still needs
its own governed plan, Gate 1 approval, exact-head Gate 2 decision, and explicit
tag/publication authorization.

## Active post-v2.10 work

| Workstream | Current public status |
|---|---|
| Roadmap and queue convergence | The roadmap owner and this projection are the first approved batch. TODO, issue, MR, and intake-bound batches follow separately. |
| Plan lifecycle (`qbka`) | A separate Gate 1 work order will define lifecycle states, supersession provenance, validation, and legacy-plan migration. |
| Linked-data/evidence assessment (#129) | Assessment precedes the plan-lifecycle provenance freeze; it does not authorize linked-data implementation. |
| Coordination semantics (#121, #122, !2027) | Requires a separate Gate 1 workstream aligned with the plan-lifecycle provenance model. |
| Collision detection (#123) | Requires its own Gate 1 plan. |
| Improvement Propeller activation (#131) | Requires its own Stage 4 Gate 1 plan and a genuine scheduled-wake proof. |
| Improvement Propeller skill export (#132) | Blocked until #131 reaches genuine Level 5 wake evidence. |

The internal roadmap records the complete sequencing and collision boundaries.
The issue and MR disposition ledgers remain evidence-bearing projections for
individual backlog items; absence from this summary is not a terminal
disposition.

## How this projection changes

Every public update must be backed by the internal roadmap and the binding plan
or governor decision that changed it. The observation timestamp and source
revision are refreshed together. If this page diverges from
`docs/roadmap.md`, the mismatch is a documentation defect and the internal
owner prevails until this projection is corrected.
