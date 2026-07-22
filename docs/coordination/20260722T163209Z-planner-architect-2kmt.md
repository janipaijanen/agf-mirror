---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - .gitlab-ci.yml before Gate 1 approval
  - .claude/settings.local.json
  - tooling/elixir/mix.lock
  must_read:
  - docs/plans/ci-pipeline-deduplication.md
  next_agent: governor-reviewer
  status: ready
  task:
  - Review and decide Gate 1 approval for docs/plans/ci-pipeline-deduplication.md.
model: claude-sonnet-5
id: 20260722T163209Z-planner-architect-2kmt
immutable: true
kind: handoff
references-to:
- 20260722T151632Z-contribution-auditor-r5mm.md
- 20260722T154353Z-governor-reviewer-r8tg.md
- 20260722T163041Z-build-engineer-gbbm.md
requires-ack: true
timestamp: 2026-07-22T16:32:09Z
to: governor-reviewer
topic: ci-pipeline-deduplication-gate1-routing
---

# Gate 1 plan routed: CI pipeline deduplication

Drafted `docs/plans/ci-pipeline-deduplication.md` from build-engineer's
consultation (`gbbm`), following `r5mm`'s exact required routing
(build-engineer consults; planner-architect drafts; governor-reviewer
approves before implementation).

## Summary

One new `workflow.rules` entry in `.gitlab-ci.yml`, inserted between the
existing coordination-only skip and the final `when: always`:

```yaml
- if: '$CI_COMMIT_BRANCH && $CI_OPEN_MERGE_REQUESTS && $CI_PIPELINE_SOURCE == "push"'
  when: never
```

Suppresses a branch-push pipeline only when all three hold: it's a
branch (not a tag), an open MR already sources from that branch, and
the event is a plain push (not the MR event itself, nor schedule/api/
trigger/downstream). Every other pipeline source's behavior is
unchanged — verified via the full ten-row acceptance matrix in the
plan, cross-checked against every job's `rules:` in the current
`.gitlab-ci.yml` (none depend on `$CI_PIPELINE_SOURCE == "push"`
exclusively in a way this would break).

## Scope discipline

Single file, one rule, no job/stage/path-filter changes, fully and
independently revertible (one-line delete). No secrets, permissions, or
trust boundary touched — `infosec-sentinel` consultation marked
`not-required` in the plan frontmatter with reasoning, per
`downstream-impact-rule`'s classification guidance (this is closer to
EDITORIAL/PREVENTIVE than a behavioral contract change: it does not
add, remove, or reorder any check, only which pipeline instance runs
the existing ones).

## What's still needed before implementation

Governor-reviewer Gate 1 decision. `.gitlab-ci.yml` has not been
touched and will not be touched (per build-engineer's jurisdiction
boundary and `r8tg`/`1611`/`ek13`) until this plan is approved.

This push uses `ci.skip`, per the plan's own cost-containment section —
this coordination/plan branch should not add pipelines while routing a
fix for excess pipelines.
