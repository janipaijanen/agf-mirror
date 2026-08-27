---
id: 20260728T093814Z-planner-architect-rhk6
kind: handoff
immutable: true
from: planner-architect
to: governor-reviewer
timestamp: 2026-07-28T09:38:14Z
model: claude-sonnet-5
references-to:
  - 20260727T154845Z-coder-tester-bsg4.md
  - 20260727T161905Z-coder-tester-8dap.md
topic: issue75-issue76-gate1-plans-ready
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Review Gate 1 plan for issue #75 (issue-board workflow): docs/plans/issue75-issue-board-workflow.md, MR !745."
    - "Review Gate 1 plan for issue #76 (coord visibility performance): docs/plans/issue76-coord-visibility-performance.md, MR !746."
    - Approve, request changes, or block each independently -- they are unrelated and should not be batched into one decision.
  must_read:
    - docs/plans/issue75-issue-board-workflow.md
    - docs/plans/issue76-coord-visibility-performance.md
  do_not_touch:
    - agf/VERSION
    - any release branch or tag
    - GitLab labels or board lists (issue #75 plan)
    - tooling/elixir (issue #76 plan)
---

# Two Gate 1 plans ready for review: issue #75 and issue #76

Both requested via coordination handoffs from an earlier coder-tester
session; picked up as planner-architect per your explicit choice earlier
this session to finish MR !719 first, then take these up.

## Issue #75 — issue-board workflow

`docs/plans/issue75-issue-board-workflow.md` (MR !745). Seven mutually
exclusive states (six `workflow::*` scoped labels + GitLab closed).
Notable decisions: no `Tested`/`Deployed` column per your explicit
guidance in the requesting handoff; radar and portfolio-discovery issues
excluded from the board entirely (Level 1 Planned signals, not governed
work in the pipeline); automation is advisory-only, since no rule grants
board-label automation blocking authority under
`tool-authority-boundary-rule.md`; migration of the ~30 currently open
issues is documented as a procedure but not executed by this plan.

## Issue #76 — `agov coord visibility` performance

`docs/plans/issue76-coord-visibility-performance.md` (MR !746). Root
cause traced directly to `load_git_messages`
(`tooling/rust/crates/agov/src/main.rs:3436`): one `git show` subprocess
per coordination file, unbounded default branch enumeration. Plan:
`git cat-file --batch` to collapse per-file spawns to one subprocess per
branch; bound the default scan (no more silent all-local-branches scan);
surface stale-local-vs-fetched-remote divergence instead of silently
scanning stale state (the exact failure mode observed in your evidence:
6.1s/168KB misleading vs 5.6s/2.3KB accurate); update assistant-facing
docs to fetch-first and prefer explicit branch targeting. Deterministic
output, exit codes, and token-ROI telemetry preserved by requirement.

Neither plan authorizes any implementation, label, board, version, or
release action before your Gate 1 approval.
