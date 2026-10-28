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
| Status | v2.12.0 is the latest published, independently verified stable release. Foundry's Improvement Propeller is Level 5 Deployable. |
| Observed at | 2026-10-28T16:13:58Z |
| Observation baseline | Annotated tag `v2.12.0` (`008ec0c3c633db62ff7e593aa9fb12c405cf9b27`) at `e045d20b073ce31341cd0d460469fac60cdbab5a` |
| Governed source | `docs/roadmap.md`, the approved queue-convergence plan, and the governor decisions cited there |
| Authority | Informational projection. The internal roadmap, binding plans and decisions, exact tags, and release evidence are authoritative. |

## Current released baseline

**Latest fully published stable release: v2.12.0.** Published 2026-10-28 from
the verified annotated tag. Its 23-job tag pipeline, GitLab Release, generic
package, OCI image, checksums, provenance, packaged executable, and anonymous
downloads were independently verified. Use the
[v2.12.0 GitLab Release](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases/v2.12.0)
for public artifacts.

No later version or candidate scope is committed merely because it appears in
an issue label, proposal, or advisory horizon. Each later release still needs
its own governed plan, Gate 1 approval, exact-head Gate 2 decision, and explicit
tag/publication authorization.

## Active post-v2.11 work

| Workstream | Current public status |
|---|---|
| v2.10.1 generic artifact/graph validation | Published (v2.10.1). Adds an opt-in `artifact_sets` registry and `agov artifact validate`/`index` commands so adopters can govern their own project-specific artifact types (schemas, dependency graphs) with the same tooling, without changing existing coordination record behavior. |
| Roadmap and queue convergence | The roadmap owner and this projection are the first approved batch. TODO, issue, MR, and intake-bound batches follow separately. |
| Plan lifecycle (`qbka`) | Shipped: lifecycle states, supersession provenance, validation, and legacy-plan migration are merged. |
| Linked-data/evidence assessment (#129) | Withdrawn by governor decision `20260909T135843Z-governor-reviewer-6sj1`; historical backlog only, not actionable and not a v2.11 blocker. |
| Coordination semantics (#122) | Shipped: superseded coordination records now self-announce (Row 5 S2). Issue closed after verified v2.11.0 publication. |
| Coordination semantics (#121) | Session-level addressing shipped. Issue closed after verified v2.11.0 publication; any future execution-lane work needs separate scope and authority. |
| MR !2027 | Closed (2026-09-13). |
| Collision detection (#123) | Shipped and MR-blocking for plan-vs-plan overlaps. Issue closed after verified v2.11.0 publication; a post-implementation drift check remains a separate, unclaimed follow-up. |
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
