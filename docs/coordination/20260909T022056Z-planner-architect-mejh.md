---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260909T022056Z-planner-architect-mejh
timestamp: 2026-09-09T02:20:56Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260908T183242Z-planner-architect-ds4k.md
  - 20260909T013812Z-capability-developer-jprj.md
  - 20260909T014426Z-capability-developer-drtp.md
  - 20260909T020642Z-governor-reviewer-gasg.md
topic: v211-release-scope-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/20260909T021841Z-v211-release-scope.md
  - docs/reviews/rule-projection/20260909T021841Z-v211-release-scope.yaml
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 1 review of the v2.11 release scope and sequence plan. Confirm/revise the IN / CANDIDATE / DEFER lines and the lifecycle-freeze ordering."
    - "Before Gate 1: route the independent contribution audit to a NON-author session, plus a build-engineer review of the release-cut section. Two advisory_consultation rows are waived-pending; infosec is not-applicable (scope plan, no code/schema/CI change)."
  must_read:
    - docs/plans/20260909T021841Z-v211-release-scope.md
    - docs/roadmap.md
    - docs/coordination/20260908T183242Z-planner-architect-ds4k.md
  do_not_touch:
    - "Any workstream's own Gate 1/Gate 2 — this plan re-approves none of them"
    - "agf/VERSION, changelogs, tags, release metadata — the v2.11 cut is a section of this plan, not authorized by its Gate 1"
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.11 release scope — ready for Gate 1

Authored per governor directive "plan refined contents for next release"
(2026-09-09). Derived from and consistent with the merged roadmap owner
`docs/roadmap.md` and the `ds4k` collision disposition.

## Shape

A **scope/sequence plan** (like the v2.10 RUW plan) — it bounds v2.11
membership and ordering; it re-approves no workstream's own Gate 1/Gate 2.

- **Spine:** the lifecycle/provenance freeze — #129 freeze slice → `qbka`
  Slice A → (#122 provenance shape + queue-convergence plan-MR cohort).
  Propeller Stage 4 (#131) runs in parallel, independent of the freeze.
- **IN:** queue-convergence B1(merged)/B2/B3/B8 + non-plan-MR B4–B7
  cohorts; Propeller Stage 4 activation (#131, Gate 1 approved `gasg`) +
  genuine first wake + Level 5 follow-up; #129 freeze-blocking
  compatibility slice; `qbka` Slice A + Slice B; the
  coordination-protocol-semantics work order (#121 + #122 + !2027, with
  `jprj` CP-A considered for inclusion).
- **CANDIDATE (slip to v2.12 if not clean):** #123 collision-detection
  tooling; `jprj` CP-B (coordination-MR ceremony / `main`-churn); #132
  skill export (hard-blocked on #131 Level 5); #116; #128.
- **DEFER:** linked-data *implementation* (v2.13 per #129); advisory
  horizon !2459; !2460/!2462 (inputs); `release-v2.7.0-rc.2`
  (`awaiting-governor`).
- **Pre-cut:** dispose `release-preflight-learning-gate2` and
  `capability-developer-startup-sync` before the v2.11 metadata MR.
- **Release cut:** `rg6k` reversible preflight → v2.11 metadata MR (the
  first release to exercise issue #120's `release:namespace-check` gate on
  the standard path) → exact-head Gate 2 → tag `v2.11.0` → publish →
  anonymous verify → completion record.

## Validation

`agov plan validate` OK; `agov plan projection-check --allow-planned
--base-ref origin/main` OK; `git diff --check` clean.

## Boundary

This plan authorizes no implementation, no version bump, no
schema/CI/rule/tag change. Each admitted workstream keeps its own gates.
The non-author contribution audit is required before Gate 1.
