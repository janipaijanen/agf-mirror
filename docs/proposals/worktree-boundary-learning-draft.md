# Draft Learning Entry: L010 Worktree Boundary

Status: draft for governor-reviewer / planner-architect review.

This is a proposed entry for `agents-govern/learnings/foundry.yaml`. It has not
been appended to the learning record yet.

```yaml
  - id: L010
    date: "2026-06-13"
    category: gap
    agents_involved: [coder-tester, planner-architect, governor-reviewer]
    summary: >
      Linked worktrees created outside the project root became hidden working
      surfaces that were difficult for humans and agents to discover from the
      AGF-NG checkout.
    detail: >
      During session-continuity and coordination-visibility work, linked git
      worktrees were created as siblings of the project directory, such as
      `/home/devi/src/AGF-NG-session-resume` and
      `/home/devi/src/AGF-NG-coord-visibility`, rather than under the project
      root `/home/devi/src/AGF-NG/`.

      The same pattern already existed for several older AGF-NG worktrees under
      `/home/devi/src/AGF-NG-*` and `/tmp/agf-ng-*`. Although these are valid
      git worktrees, they violate the operator's mental model that AGF-NG work
      should be discoverable from the AGF-NG project directory. They also make
      cleanup, ownership checks, and cold-start orientation harder: a human or
      AI agent inspecting `/home/devi/src/AGF-NG/` does not naturally see sibling
      or `/tmp` worktrees.

      This differs from L007, which is about committing durable artifacts on
      the wrong branch, and from L009, which is about coordination messages
      hidden on side branches. Here the branch may be correct and the
      coordination may be visible, but the filesystem location itself becomes a
      hidden working surface outside the agreed project boundary.

      The immediate correction was to move the two newest Codex-created
      worktrees into `/home/devi/src/AGF-NG/.worktrees/` and to add `.worktrees/`
      to the local `.git/info/exclude`. Older sibling and `/tmp` worktrees were
      not moved automatically because another active human, Claude, or Codex
      session might have absolute paths pinned.
    framework_impact: medium
    status: proposed
    recommendation: >
      Add a worktree-boundary rule for AGF-NG and future adopter guidance: new
      linked worktrees must be created under `.worktrees/<purpose>` inside the
      project root unless the governor-reviewer explicitly approves an external
      path for a temporary diagnostic task. Do not move or remove existing
      external worktrees until ownership and active-session risk are confirmed.
      Keep `.worktrees/` locally ignored so linked worktrees are discoverable
      from the project root without becoming tracked project content.
```

## Notes

- `AGENTS.md` already contains the immediate operational rule from seq 198.
- Seq 199 records the fast-track completion for that `AGENTS.md` update.
- Existing external worktrees should be inventoried before cleanup; the draft
  learning intentionally does not recommend automatic migration.
