---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260804T002720Z-contribution-auditor-73kj
timestamp: 2026-08-04T00:27:20Z
model: gpt-5.4
immutable: true
references-to:
  - 20260804T001752Z-contribution-auditor-phpp.md
  - 20260804T002719Z-contribution-auditor-rnp5.md
topic: v250-rc2-release-publication-plan-commission
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Draft a Gate 1 plan for a tightly bounded v2.5.0-rc.2 publication.
    - Audit the exact shipped delta from v2.5.0-rc.1 and separate shipped from repository-only changes.
    - Carry forward the full exact-artifact matrix and define the stable-release trigger.
  must_read:
    - docs/plans/v250-statement-of-work.md
    - docs/plans/v250-rc1-release-publication.md
    - agf/rules/workflow/release-via-merge-request-rule.md
    - agf/rules/workflow/immutable-tags-rule.md
    - agf/dist/release-manifest.yaml
    - docs/coordination/20260804T002719Z-contribution-auditor-rnp5.md
  do_not_touch:
    - release-2.5.0-rc.1
    - tooling/rust
    - agf/schemas
---

# Commission: Gate 1 plan for v2.5.0-rc.2

Draft the release-publication plan only. Do not create a release branch, bump a
version, tag, publish, implement !1090, or mutate tracker state before the new
plan receives binding Gate 1 approval.

Required boundaries:

1. RC2 is triggered by the real post-RC1 shipped-documentation delta, chiefly
   the governor-approved NASA attribution. It is not ceremonial.
2. Inventory every `v2.5.0-rc.1..main` path against the release manifest.
   Coordination, repo-root source registry, internal plans, and Foundry's own
   learning corpus must not be described as shipped merely because they are on
   `main`.
3. Exclude !1090 Path A implementation, CI bootstrap optimization, wrapper
   defect remediation, visual refresh, and unrelated sources/radar work. Route
   these to 2.5.1 or later.
4. RC2 validation repeats RC1's exact framework/tooling artifact acceptance
   matrix and adds no weaker substitute. Reuse the proven checks and update
   only baselines/versions that truly changed.
5. Define the observation exit: if exact artifacts are clean and no
   reproducible released-surface blocker exists, commission the separate
   stable-2.5.0 publication plan. Any further candidate change creates another
   RC; enhancement suggestions do not.
6. Give the strongest direct-stable counterargument fair treatment and explain
   why the approved candidate-identity rule prevails.

Ask contribution-auditor for independent Gate 1 review when the plan is ready.
