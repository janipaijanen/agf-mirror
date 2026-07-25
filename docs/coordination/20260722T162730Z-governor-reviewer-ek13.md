---
kind: handoff
from: governor-reviewer
to: build-engineer
id: 20260722T162730Z-governor-reviewer-ek13
timestamp: 2026-07-22T16:27:30Z
model: human-governor
immutable: true
references-to:
  - 20260722T151632Z-contribution-auditor-r5mm
  - 20260722T154353Z-governor-reviewer-r8tg
  - 20260722T154432Z-governor-reviewer-1611
topic: ci-deduplication-gate1-plan-priority
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: build-engineer
  status: ready
  task:
    - Acknowledge build-engineer activation and first assignment `1611`.
    - Treat CI deduplication as the highest immediate cost-saving priority and supply the pipeline-source matrix and workflow-guard consultation to planner-architect now.
    - Ensure planner-architect drafts and routes the narrow Gate 1 plan before any `.gitlab-ci.yml` implementation.
    - Record exact expected behavior for MR, open-MR branch push, branch without MR, main, tag, schedule, API, trigger, downstream, and coordination-only events.
  must_read:
    - agf/agents/MANIFEST.yaml
    - agf/agents/build-engineer.yaml
    - .gitlab-ci.yml
    - docs/coordination/20260722T151632Z-contribution-auditor-r5mm.md
    - docs/coordination/20260722T154432Z-governor-reviewer-1611.md
    - MR !399
  do_not_touch:
    - .gitlab-ci.yml before Gate 1 approval
    - deployment targets, tag authority, or rollback procedures
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Governor priority confirmation: route CI deduplication plan now

Claude, acting as `build-engineer`, must acknowledge activation decision
`r8tg` and first assignment `1611`. Eliminating duplicate paid branch/MR job
execution is the highest immediate cost-saving priority.

Consult with `planner-architect` now on the narrow Gate 1 plan requested by
`r5mm`. The plan must assess the `CI_OPEN_MERGE_REQUESTS` workflow guard and
carry the complete pipeline-source acceptance matrix. It must preserve MR,
branch-without-open-MR, default-branch, tag, schedule, API, trigger,
downstream, and coordination-only behavior while suppressing only a redundant
branch-push pipeline when an open MR exists.

Do not edit `.gitlab-ci.yml` before the governor approves Gate 1. Until the
fix is merged, use the established cost-contained publication procedure so a
single MR pipeline provides implementation evidence. Release-build efficiency
is in scope; deployment, tag, release approval, and rollback authority are not.
