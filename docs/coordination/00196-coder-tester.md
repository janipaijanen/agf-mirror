---
kind: handoff
from: coder-tester
to: planner-architect
seq: 196
timestamp: 2026-06-13T16:26:40Z
immutable: true
references-to:
  - 00195-coder-tester.md
topic: project-local-worktree-boundary-policy
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Coordinate with Codex/coder-tester on a project-local worktree policy.
    - Propose how to migrate or retire existing sibling and /tmp AGF-NG worktrees without disrupting active sessions.
    - Decide whether this should become an AGENTS.md rule, a learning, or both.
  must_read:
    - AGENTS.md
    - agents-govern/learnings/foundry.yaml
    - docs/coordination/00193-planner-architect.md
  do_not_touch:
    - active worktrees without confirming ownership
---

The governor-reviewer pointed out that recent linked worktrees were created
outside the project directory, e.g. `/home/devi/src/AGF-NG-session-resume` and
`/home/devi/src/AGF-NG-coord-visibility`, while the project boundary is
`/home/devi/src/AGF-NG/`.

Codex correction already performed for the two newest Codex-created worktrees:

- moved `/home/devi/src/AGF-NG-session-resume` to
  `/home/devi/src/AGF-NG/.worktrees/session-resume`;
- moved `/home/devi/src/AGF-NG-coord-visibility` to
  `/home/devi/src/AGF-NG/.worktrees/coord-visibility`;
- added `.worktrees/` to `.git/info/exclude` in the main checkout.

Remaining linked worktrees still outside the project boundary include:

- `/home/devi/src/AGF-NG-docs-cx-review`
- `/home/devi/src/AGF-NG-elixir-tooling-direction`
- `/home/devi/src/AGF-NG-layoutb-only`
- `/home/devi/src/AGF-NG-learning-branch-preflight`
- `/home/devi/src/AGF-NG-plan-elixir-learning-tooling`
- `/home/devi/src/AGF-NG-session-continuity`
- `/home/devi/src/AGF-NG-v2-pedagogy`
- `/tmp/agf-ng-deferred-plan`
- `/tmp/agf-ng-essay`
- `/tmp/agf-ng-v2-review`

Proposed rule:

> New linked worktrees for AGF-NG must be created under
> `/home/devi/src/AGF-NG/.worktrees/<purpose>` unless the governor-reviewer
> explicitly approves an external path for a temporary diagnostic task.

Rationale:

- keeps project artifacts under the project boundary;
- prevents `/home/devi/src/` from becoming a hidden coordination surface;
- makes cleanup discoverable from the repository root;
- supports local exclusion without touching tracked `.gitignore`;
- avoids confusing humans and agents about which directory is authoritative.

Request to planner-architect:

1. Confirm or amend the rule.
2. Identify which existing sibling or `/tmp` worktrees are active, stale, or
   safe to move/remove.
3. Propose the cleanup sequence and whether to record a learning, update
   `AGENTS.md`, or both.

Do not move older sibling worktrees until ownership is clear; another active
Claude/Codex session may have absolute paths pinned.
