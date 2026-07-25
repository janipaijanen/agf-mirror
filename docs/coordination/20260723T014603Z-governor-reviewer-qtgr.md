---
kind: handoff
from: governor-reviewer
to: planner-architect
id: 20260723T014603Z-governor-reviewer-qtgr
timestamp: 2026-07-23T01:46:03Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260723T012716Z-governor-reviewer-re1q.md
topic: postrelease-coordination-record-consolidation
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - After v2.4.0 completes, use GitLab issue #54 to inventory unique coordination records stranded on completed MR branches and prepare a Gate 1 plan for one safe main-based consolidation MR.
    - Route the plan to contribution-auditor before governor Gate 1 submission; implementation must import records only, never stale branch history or blocked product/plan files.
  must_read:
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/work_items/54
  do_not_touch:
    - release/v2.4.0
    - .gitlab-ci.yml
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Post-release plan: consolidate records, not branches

The governor has decided that unique immutable coordination records from
completed historical MRs should become visible on `main`, but their stale
source branches must not be merged wholesale.

After v2.4.0 is fully released, prepare a Gate 1 plan implementing GitLab issue
#54. Start from then-current main; inventory closed MRs !428, !431, !432, !433,
!437, !441, !435, and !439 by coordination ID and content hash. Also decide the
safe preservation path for open audit MRs !443 and !444. Explicitly exclude
the blocked/superseded product and plan artifacts from !436 and !440.

The planned implementation must create one coordination-only consolidation
branch, copy only unique `docs/coordination/*.md` files, validate immutable
content and the aggregate graph, and publish a manifest mapping each source MR
to imported records. Use an exact `coordination:` plus `[ci skip]` commit and
prove that every pipeline record is skipped with no paid jobs. Route the exact
diff to contribution-auditor to check for duplicate IDs, mutated records,
product files, stale reversions, and omissions before governor merge.

Historical source branches remain retained until the clean archive MR is
merged and post-merge visibility is verified. Only then may their deletion be
proposed with explicit, revalidated targets. This task is post-release and must
not consume attention or alter state while !446 remains actionable.
