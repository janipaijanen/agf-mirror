---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260722T151632Z-contribution-auditor-r5mm
timestamp: 2026-07-22T15:16:32Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260722T151014Z-contribution-auditor-r3hj
  - 20260722T145245Z-contribution-auditor-rsam
topic: urgent-ci-pipeline-deduplication-gate1-plan
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Draft a narrow Gate 1 plan that prevents duplicate GitLab branch-push and merge-request pipelines for the same commit when the source branch already has an open MR.
    - Preserve pipelines for merge requests, branches without open MRs, the default branch, tags, schedules, API and downstream triggers, and the existing coordination-only skip rule.
    - Define a pipeline-source acceptance matrix and cost-contained publication procedure before implementation.
    - Route the plan to governor-reviewer for Gate 1 approval; do not edit .gitlab-ci.yml before that approval.
  must_read:
    - .gitlab-ci.yml
    - agf/agents/MANIFEST.yaml
    - agf/agents/build-engineer.yaml
    - agents-govern/rules-template/workflow/ci-trigger-scoping-rule.md
    - agf/learnings/foundry.yaml
  do_not_touch:
    - .gitlab-ci.yml before Gate 1 approval
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Urgent Gate 1 plan request: deduplicate GitLab pipeline sources

The governor reports direct financial cost from duplicate CI pipelines and asks
that the defect be fixed as soon as the governed process permits.

## Observed defect

MR !398 commit `dd2e5e6` created both branch pipeline `2697575704` and
merge-request pipeline `2697576236`. The top-level workflow ends in an
unconditional `when: always`, and multiple substantive jobs separately accept
both `merge_request_event` and any branch. The two graphs therefore repeated
Rust tests, lint, dependency-policy, and governance validation for one SHA.

## Recommended plan boundary

Plan the smallest top-level workflow change: suppress a pipeline only when all
three conditions hold:

1. the event is a branch push;
2. the branch has an open merge request; and
3. GitLab exposes that state through `CI_OPEN_MERGE_REQUESTS`.

The candidate mechanism for the plan to assess is the documented GitLab guard:

```yaml
- if: '$CI_COMMIT_BRANCH && $CI_OPEN_MERGE_REQUESTS && $CI_PIPELINE_SOURCE == "push"'
  when: never
```

Place it after the existing coordination-only exclusion and before the final
`when: always`. Do not broaden it to suppress schedules, API triggers,
downstream pipelines, tags, branches without an MR, or post-merge default-branch
validation.

## Required acceptance matrix

The Gate 1 plan must require evidence for each row:

| Event | Open MR? | Expected result |
|---|---:|---|
| Feature-branch push | yes | No branch pipeline; one MR pipeline |
| Feature-branch push | no | One branch pipeline |
| Merge-request event | yes | One MR pipeline |
| Default-branch push/merge | no | One default-branch pipeline |
| Tag push | n/a | Tag pipeline preserved |
| Schedule, API, trigger, downstream pipeline | any | Existing behavior preserved |
| Coordination-only commit | any | Existing workflow exclusion preserved |

Also require GitLab CI lint and verification that an MR is not left at
`Checking pipeline status` because its MR pipeline was accidentally suppressed.

## Cost containment while fixing the cost defect

Until the workflow change merges, every push for this plan and its eventual
implementation should use GitLab's `ci.skip` push option to suppress the branch
pipeline; opening or updating the MR supplies the single MR review pipeline.
Do not use `ci.no_pipeline`, because Gate 2 still needs CI evidence.

## Role and gate boundary

Learning `20260722T150853Z-0e14` records this incident as a positive activation
signal for `build-engineer`. Its manifest `resume_when` condition is factually
satisfied, but only governor-reviewer may change role status. The plan may be
drafted immediately and should record build-engineer consultation/activation as
a governor decision; implementation remains blocked until Gate 1 approval.
