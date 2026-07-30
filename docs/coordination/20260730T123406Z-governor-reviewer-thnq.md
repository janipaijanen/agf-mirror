---
kind: handoff
from: governor-reviewer
to: planner-architect
id: 20260730T123406Z-governor-reviewer-thnq
timestamp: 2026-07-30T12:34:06Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260729T152543Z-governor-reviewer-hapn.md
  - 20260730T084823Z-governor-reviewer-8rph.md
topic: v246-release-plan-explicit-reactivation
requires-ack: true
artifacts_owned:
  - docs/plans/v246-release.md
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Draft the release-only v2.4.6 Gate 1 plan already assigned by hapn.
    - Reconcile the release contents against current origin/main and freeze the exact planning base.
    - Route the plan through contribution-auditor and infosec-sentinel before requesting Gate 1.
  must_read:
    - docs/coordination/20260729T152543Z-governor-reviewer-hapn.md
    - docs/coordination/20260730T084823Z-governor-reviewer-8rph.md
    - docs/plans/v246-recovery-patch.md
    - agf/rules/workflow/release-via-merge-request-rule.md
    - agf/rules/workflow/immutable-tags-rule.md
    - agf/CHANGELOG.md
  do_not_touch:
    - agf/VERSION
    - release tags
---

# Explicitly reactivate the v2.4.6 release-plan assignment

The planner-architect is explicitly authorized and assigned to draft
`docs/plans/v246-release.md` now. This is not speculative work.

The original governor-to-planner handoff `hapn` remains the complete task
contract. It is visible on MR !788 and requires acknowledgment. The binding
governor decision `8rph`, already published on `main`, resumed that release
work in parallel after MR !804 merged. The visible !788 bridge dated
2026-07-30T08:51:21Z also says to resume `hapn`.

Treat this record as an explicit reactivation and context-gap repair, not a
scope expansion. Prepare the narrow release-only Gate 1 plan, reconcile again
against current `origin/main`, and request the required consultations. Do not
bump the version, create the release branch, or tag before Gate 1.

The future tooling plans in MRs !805–!807 and the longitudinal CI assessment
remain outside v2.4.6 and are not blockers for this drafting assignment.
