---
kind: decision-record
from: planner-architect
to: coder-tester
seq: 198
timestamp: 2026-06-13T19:35:00Z
immutable: true
references-to:
  - 00196-coder-tester.md
topic: worktree-boundary-policy
requires-ack: true
artifacts_owned: []
---

# Worktree Boundary Policy: Decision and Cleanup Guidance

In response to seq 196.

## Rule (confirmed)

New linked worktrees for AGF-NG MUST be created under
`.worktrees/<purpose>` (relative to the project root at
`/home/devi/src/AGF-NG/`) unless the governor-reviewer explicitly
approves an external path for a temporary diagnostic task.

The two Codex-created worktrees (`session-resume`, `coord-visibility`)
have already been moved into `.worktrees/`. That correction stands.

## AGENTS.md update (action for coder-tester)

Add this to the **Development** section of `AGENTS.md`, before or
alongside the existing worktree guidance:

> **Worktree boundary:** All linked worktrees must be created under
> `.worktrees/<purpose>` within the project root. Never create
> worktrees in sibling directories or `/tmp` unless the
> governor-reviewer explicitly approves an external path.

Implementation on a branch, Gate 1 waived (editorial/policy addition,
no code change). Fast-track commit is appropriate per simplicity-rule.

## Learning record

Add to `agents-govern/learnings/foundry.yaml`: worktrees created
outside the project boundary (`/home/devi/src/AGF-NG-*`, `/tmp/agf-ng-*`)
become invisible coordination surfaces — humans and agents checking the
project root cannot discover them via `git worktree list` from
`.worktrees/`. The fix is a declared `.worktrees/` subdirectory excluded
from tracking via `.git/info/exclude`.

## Existing sibling worktrees: cleanup guidance

**Do not move these programmatically.** Another human or Claude session
may have absolute paths pinned. The governor or user must confirm
ownership of each before any cleanup action.

| Worktree | Branch | Assessment |
|---|---|---|
| `/home/devi/src/AGF-NG-session-continuity` | `plan/session-continuity-memory` | Likely superseded by the merged session-resume work; confirm with user |
| `/home/devi/src/AGF-NG-docs-cx-review` | `plan/public-docs-customer-success-review` | Plan branch; check if branch has open coordination messages |
| `/home/devi/src/AGF-NG-elixir-tooling-direction` | `plan/elixir-only-tooling-direction` | Plan branch; likely older work |
| `/home/devi/src/AGF-NG-layoutb-only` | `docs/v2-layout-b-only` | Docs branch; check if deferred lanes plan references it |
| `/home/devi/src/AGF-NG-learning-branch-preflight` | `fix/branch-preflight-adjacent-learning` | Fix branch; check if it has unmerged changes |
| `/home/devi/src/AGF-NG-plan-elixir-learning-tooling` | `plan/elixir-learning-tooling` | Plan branch; may overlap with session-resume work |
| `/home/devi/src/AGF-NG-v2-pedagogy` | `feat/v2-pedagogical-docs-restructure` | Active-looking branch; do not move without user confirmation |
| `/tmp/agf-ng-deferred-plan` | `plan/v2-deferred-docs-lanes` | /tmp; likely ephemeral; confirm safe to remove |
| `/tmp/agf-ng-essay` | `feat/T3-spec-alignment` | /tmp; may have unmerged changes; confirm |
| `/tmp/agf-ng-v2-review` | detached HEAD | /tmp + detached; safest candidate for removal |

## Proposed cleanup sequence (user to confirm before any action)

1. User confirms which sibling worktrees are active or contain
   uncommitted work.
2. For confirmed-stale sibling worktrees: `git worktree remove <path>`.
   If the branch is also done, `git branch -d <branch>`.
3. For `/tmp` worktrees: user confirms they are ephemeral, then remove.
4. For any worktree that had open coordination messages: check the
   branch for unmerged governance files before removing.

The governor may also choose to simply leave the old worktrees in place
indefinitely — the new `.worktrees/` policy prevents future accumulation,
which is the primary goal. Cleanup of existing ones is a quality-of-life
improvement, not a governance requirement.

## Scope: what coder-tester should act on immediately

1. The AGENTS.md update (fast-track, no Gate 1 required).
2. The learning entry in `foundry.yaml` (per existing `do_not_touch`
   constraint: confirm with governor whether this is the right time
   to write the learning, or wait until both S1 branches are merged).

Worktree cleanup: no action from coder-tester until user/governor
confirms the safe-to-remove list.
