---
artifacts_owned: []
from: build-engineer
id: 20260722T163041Z-build-engineer-gbbm
immutable: true
kind: ack
model: claude-sonnet-5
references-to:
- 20260722T154353Z-governor-reviewer-r8tg.md
- 20260722T154432Z-governor-reviewer-1611.md
- 20260722T162730Z-governor-reviewer-ek13.md
requires-ack: false
timestamp: 2026-07-22T16:30:41Z
to: governor-reviewer
topic: build-engineer-activation-and-first-assignment-ack
---

# ACK: build-engineer activation (`r8tg`) and first assignment (`1611`)

Acknowledging activation and the CI-deduplication first assignment.
Treating it as the highest immediate priority now, ahead of resuming
Workstream D Class 1 (`ph8p`), per the human governor's direct
instruction in this session.

Per `build-engineer.yaml`'s jurisdiction boundary ("cannot_touch: Plan
documents or ADRs — owned by Planner-Architect"), this role supplies
the pipeline-source acceptance matrix and candidate mechanism as
consultation input; the Gate 1 plan document itself is drafted under
the planner-architect hat in a separate coordination record, per
`r5mm`'s required routing. `.gitlab-ci.yml` will not be touched before
Gate 1 approval.

## Pipeline-source acceptance matrix (consultation input for the plan)

Verified current `.gitlab-ci.yml` state: the `workflow.rules` block has
exactly one rule today (coordination-only commit title skip) before an
unconditional `when: always` — every other event source, including a
branch push when that branch already has an open MR, runs the full
pipeline. This is the confirmed root cause of `rsam`'s duplicate-
pipeline observation on commit `dd2e5e6`.

| Event | Open MR on this branch? | Current behavior | Required behavior |
|---|---:|---|---|
| Feature-branch push | yes | Branch pipeline runs (duplicate) | No branch pipeline; MR pipeline only |
| Feature-branch push | no | Branch pipeline runs | Unchanged: branch pipeline runs |
| Merge-request event | yes | MR pipeline runs | Unchanged: MR pipeline runs |
| Default-branch (`main`) push/merge | n/a | Pipeline runs | Unchanged |
| Tag push | n/a | Tag pipeline runs | Unchanged |
| Scheduled pipeline | n/a | Runs | Unchanged |
| API-triggered pipeline | n/a | Runs | Unchanged |
| External trigger | n/a | Runs | Unchanged |
| Downstream (multi-project) pipeline | n/a | Runs | Unchanged |
| Coordination-only commit (title `^coordination:/`) | any | Skipped (existing rule) | Unchanged |

## Candidate mechanism (matches `r5mm`'s recommendation independently re-derived)

Insert one rule between the existing coordination-only skip and the
final `when: always`:

```yaml
workflow:
  rules:
    - if: '$CI_COMMIT_TITLE =~ /^coordination:/'
      when: never
    - if: '$CI_COMMIT_BRANCH && $CI_OPEN_MERGE_REQUESTS && $CI_PIPELINE_SOURCE == "push"'
      when: never
    - when: always
```

All three conditions must hold for suppression: a branch-scoped commit
(`$CI_COMMIT_BRANCH`, excludes tags), an open MR sourcing from that
branch (`$CI_OPEN_MERGE_REQUESTS`, GitLab-populated), and a plain push
event specifically (`$CI_PIPELINE_SOURCE == "push"`, excludes
`merge_request_event`, `schedule`, `api`, `trigger`, `pipeline`, and
`web`). No path filters, no stage-level changes — the smallest rule
that closes the observed defect.

## Cost containment until Gate 1 lands

This coordination push and the subsequent plan-branch push will use
GitLab's `ci.skip` push option, so only the eventual MR's pipeline
provides evidence — matching `r5mm`'s interim cost-containment
instruction.
