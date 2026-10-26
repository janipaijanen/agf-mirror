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
| Status | v2.11.0 is the pending reviewed release from current `main`; v2.10.1 remains the latest published stable release until v2.11.0's tag pipeline and anonymous artifact verification complete. Foundry's Improvement Propeller is Level 5 Deployable. |
| Observed at | 2026-10-26T05:50:00Z |
| Observation baseline | `main` at `24d098a108ffe9cb227bd55c1c8208cac6506123`, the frozen v2.11.0 release base |
| Governed source | `docs/roadmap.md`, the approved queue-convergence plan, and the governor decisions cited there |
| Authority | Informational projection. The internal roadmap, binding plans and decisions, exact tags, and release evidence are authoritative. |

## Current released baseline

**Latest fully published stable release: v2.10.1.** Published 2026-09-12,
superseding v2.10.0 as latest. Its tag (lightweight, unlike v2.10.0's
annotated tag -- a disclosed process gap, not a functional defect),
mandatory tag pipeline, GitLab Release, generic package, OCI image,
checksums, and anonymous downloads were independently verified. Use the
[v2.10.1 GitLab Release](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases/v2.10.1)
for public artifacts.

No later version or candidate scope is committed merely because it appears in
an issue label, proposal, or advisory horizon. Each later release still needs
its own governed plan, Gate 1 approval, exact-head Gate 2 decision, and explicit
tag/publication authorization.

## Active post-v2.10 work

| Workstream | Current public status |
|---|---|
| v2.10.1 generic artifact/graph validation | Published (v2.10.1). Adds an opt-in `artifact_sets` registry and `agov artifact validate`/`index` commands so adopters can govern their own project-specific artifact types (schemas, dependency graphs) with the same tooling, without changing existing coordination record behavior. |
| Roadmap and queue convergence | The roadmap owner and this projection are the first approved batch. TODO, issue, MR, and intake-bound batches follow separately. |
| Plan lifecycle (`qbka`) | Shipped: lifecycle states, supersession provenance, validation, and legacy-plan migration are merged. |
| Linked-data/evidence assessment (#129) | Withdrawn by governor decision `20260909T135843Z-governor-reviewer-6sj1`; historical backlog only, not actionable and not a v2.11 blocker. |
| Coordination semantics (#122) | Implementation done: superseded coordination records now self-announce (Row 5 S2). Issue closure/reconciliation on the tracker is pending. |
| Coordination semantics (#121) | Session-level addressing shipped; whether it fully covers #121's scope remains under separate review. |
| MR !2027 | Closed (2026-09-13). |
| Collision detection (#123) | Shipped and MR-blocking for plan-vs-plan overlaps. A post-implementation drift check remains a separate, unclaimed follow-up. |
| Improvement Propeller activation (#131) | Level 5 Deployable after a genuine unattended wake, independent review, and binding governor confirmation (`20260918T162825Z-governor-reviewer-8hs2`). Adopters remain inactive unless their own governor activates the optional role. |
| Improvement Propeller skill export (#132) | Shipped as an optional skill (`32fd0134`); it activates nothing without adopter-local governor authority and wake configuration. |

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
