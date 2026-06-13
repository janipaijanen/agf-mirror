# docs/workitems/

Workitem manifests for in-flight branches. Each manifest gives another
agent or human enough information to reconstruct the local context for a
branch without relying on session memory or physical worktree paths.

## Convention

| Layer | Purpose | Shared how |
|---|---|---|
| Branch | Source changes | Push to remote (`git fetch origin; git worktree add .worktrees/<id> origin/<branch>`) |
| Coordination entry | Who acts and why | `docs/coordination/<seq>-<role>.md` |
| Workitem manifest | Reconstructible local state | `docs/workitems/<id>.yaml` (this directory) |
| Bundle (offline) | No-remote transfer | External attachment only — never committed |

## When to file a manifest

**Required:** branch has more than one commit AND carries a coordination
handoff with `requires-ack: true` directed to a different agent.

**Recommended:** any branch expected to sit unmerged past the current
session.

**Optional:** trivial single-commit branches with one active agent.

## Manifest shape

```yaml
id: <workitem-id>
status: <in-progress | ready-for-review | blocked | merged>
branch: <branch-name>
base_branch: master
preferred_worktree: .worktrees/<id>          # local suggestion only
owner: <agent-role>
coordination:
  claim: null                                # docs/coordination/<seq>-<role>.md or null
  handoff: docs/coordination/<seq>-<role>.md
  decision: null                             # filled when governor decides
remote:
  name: origin
  pushed: <true | false>
dirty_state:
  expected_clean: <true | false>
  ignored_local_state:
    - .claude/settings.local.json
verification:
  - command: "<command>"
    cwd: <relative-path>
    result: <pass | fail | pending>
notes:
  - "<any context a future agent needs>"
```

## What NOT to do

- Do not commit `.worktrees/` directories.
- Do not commit bundle files (`docs/workitems/bundles/` is gitignored).
- Do not use physical worktree paths as the primary reference — use the branch name.
