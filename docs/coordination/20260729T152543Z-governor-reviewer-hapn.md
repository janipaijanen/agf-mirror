---
kind: handoff
from: governor-reviewer
to: planner-architect
id: 20260729T152543Z-governor-reviewer-hapn
timestamp: 2026-07-29T15:25:43Z
immutable: true
references-to:
  - 20260729T134714Z-governor-reviewer-8jr7
  - 20260729T140021Z-coder-tester-rjhn
  - 20260729T142420Z-governor-reviewer-nmka
topic: v246-release-gate1-plan-request
requires-ack: true
artifacts_owned:
  - docs/plans/v246-release.md
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Prepare a narrowly scoped Gate 1 plan for cutting and publishing v2.4.6 from current main.
    - Route the plan through contribution-auditor and infosec-sentinel consultation before requesting the governor-reviewer's Gate 1 decision.
  must_read:
    - docs/plans/v246-recovery-patch.md
    - agf/rules/workflow/release-via-merge-request-rule.md
    - agf/rules/workflow/immutable-tags-rule.md
    - agf/CHANGELOG.md
  do_not_touch:
    - agf/VERSION
    - release tags
---

# Advance v2.4.6 through a separate release Gate 1 plan

The governor-reviewer directs the project to advance the v2.4.6 release. The
recovery implementation is merged by MR !784 at merge commit `7146101`, its
approved implementation plan is merged by MR !774, and the exact-head Gate 2
decision is merged by MR !785.

The recovery plan explicitly excluded the version bump, release branch/MR,
tag, and publication. Prepare the separate plan required to authorize those
actions. Keep it release-only and aligned with
`release-via-merge-request-rule.md`:

1. confirm the intended release base on current `main` and reconcile every
   commit since `v2.4.5` into the release contents or an explicit exclusion;
2. create `release-2.4.6` from the approved base;
3. limit the release MR diff to the `agf/VERSION` bump, finalized v2.4.6
   changelog entry, and strictly necessary release metadata;
4. validate version consistency, adopter-doc parity, manifest/distribution
   builds, clean tar/zip extraction tests, release paths, SPDX/license checks,
   and the full required CI matrix at the exact release-MR head;
5. require Gate 2 on the release MR; and
6. after merge, tag the resulting `main` merge commit as `v2.4.6` and push the
   tag immediately so CI publishes and verifies the immutable release.

The plan must state rollback/stop conditions before the tag, post-publication
verification after the tag, and what happens if any expected artifact or
provenance record is missing or inconsistent. It must not fold issue #53 CI
experiments, the parked branch-scope rule, or v2.5.0-rc.1 work into this patch.
