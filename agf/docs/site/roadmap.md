# Roadmap

*Audience: evaluators, adopters, contributors, and maintainers or AI
assistants looking for current release and development status.*

This page is the public projection of the governed internal
[foundry roadmap](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/docs/roadmap.md).
It summarizes current state without creating release scope or authority of its
own. That owner is foundry repository governance and is not included in the
adopter archive; adopters maintain their own project roadmap rather than
inheriting foundry's development commitments.

| Field | Value |
|---|---|
| Status | v2.10.1 is the pending reviewed release from current `main`; v2.10.0 remains the latest published stable release until v2.10.1's tag pipeline and anonymous artifact verification complete. Foundry's Improvement Propeller is active at Level 4 Validated; Level 5 awaits a genuine unattended scheduled wake. |
| Observed at | 2026-09-12T18:50:00Z |
| Observation baseline | `main` at `90bb57a2f57c899b8ceacefb6a104d8bc80b0f58`, the merged v2.10.1 candidate before release-metadata preparation |
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
| v2.10.1 generic artifact/graph validation | Publication pending. Adds an opt-in `artifact_sets` registry and `agov artifact validate`/`index` commands so adopters can govern their own project-specific artifact types (schemas, dependency graphs) with the same tooling, without changing existing coordination record behavior. Merged to `main`; release metadata, exact-head Gate 2, the tag, and public artifact verification remain outstanding. The v2.11/RUW work below is separately paused while this ships. |
| Roadmap and queue convergence | The roadmap owner and this projection are the first approved batch. TODO, issue, MR, and intake-bound batches follow separately. |
| Plan lifecycle (`qbka`) | A separate Gate 1 work order will define lifecycle states, supersession provenance, validation, and legacy-plan migration. |
| Linked-data/evidence assessment (#129) | Assessment precedes the plan-lifecycle provenance freeze; it does not authorize linked-data implementation. |
| Coordination semantics (#121, #122, !2027) | Requires a separate Gate 1 workstream aligned with the plan-lifecycle provenance model. |
| Collision detection (#123) | Requires its own Gate 1 plan. |
| Improvement Propeller activation (#131) | Foundry activation is approved and active at Level 4 Validated. After the tracked activation merges, one guarded Monday 09:00 UTC heartbeat is initialized. A genuine unattended conforming wake, recorded separately, is required before any Level 5 claim. Adopters remain inactive unless their own governor activates the optional role. |
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
