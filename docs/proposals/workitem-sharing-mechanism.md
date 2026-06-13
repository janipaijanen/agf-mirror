# Proposal: Shareable Workitems

Status: draft for planner-architect critique. Motivation: AGF development uses
local worktrees heavily, but `.worktrees/` itself is local workspace state and
is now intentionally ignored. Complex work still needs a shareable shape.

## Problem

For simple work, a branch plus a coordination handoff is enough. For complex
agent work, that is not enough:

- the branch name does not say which coordination handoff owns it;
- local worktrees carry context that colleagues cannot see;
- unpushed WIP may be important when an agent pauses or runs out of tokens;
- generated artifacts, dirty files, and verification status are easy to lose;
- copied assistant configuration such as `.claude/` appears in every worktree,
  which makes people wonder what is authority and what is merely checkout-local.

We need a mechanism to share the **workitem**, whether the local checkout happens
to live under `.worktrees/` or another directory name.

## Non-goals

- Do not commit `.worktrees/` directories.
- Do not share machine-local assistant state.
- Do not treat physical worktree paths as portable.
- Do not require everyone to use the same local directory layout.

## Proposed Model

Share workitems as a tracked manifest plus normal Git transport.

| Layer | Purpose | Shared how |
|---|---|---|
| Branch | Source changes and commits | Push to remote |
| Coordination entry | Who needs to act and why | `docs/coordination/<seq>-<role>.md` |
| Workitem manifest | Reconstructible local state | `docs/workitems/<id>.yaml` |
| Optional bundle | Offline/no-remote transfer | `docs/workitems/bundles/<id>.bundle` or external attachment |

The worktree directory is disposable. The workitem is not.

## Manifest Shape

Example:

```yaml
id: agf-public-docs-polish
status: ready-for-review
branch: docs/agf-public-docs-polish
base_branch: master
preferred_worktree: .worktrees/agf-public-docs-polish
owner: coder-tester
coordination:
  claim: null
  handoff: docs/coordination/00227-coder-tester.md
  decision: docs/coordination/00229-governor-reviewer.md
remote:
  name: origin
  pushed: true
dirty_state:
  expected_clean: true
  ignored_local_state:
    - .claude/settings.local.json
verification:
  - command: "mise exec -- mix test"
    cwd: tooling/elixir
    result: pass
  - command: "mise exec -- uvx --from 'mkdocs>=1.6,<2' mkdocs build -f agf/docs/build/mkdocs.yml --strict"
    cwd: .
    result: pass
notes:
  - "Physical worktree path is a suggestion only; reconstruct from branch."
```

## Restore Flow

With a shared remote:

```bash
git fetch origin
git worktree add .worktrees/<id> origin/<branch>
```

Without a shared remote:

```bash
git bundle create docs/workitems/bundles/<id>.bundle <base_branch>..<branch>
git clone docs/workitems/bundles/<id>.bundle <id>
```

The bundle path should normally be short-lived or attached outside the repo; a
bundle may be too large or too transient to keep in Git history. The manifest can
record where the bundle was placed.

## `.claude/` Rule

Each Git worktree is a full checkout root. If `.claude/rules/` and
`.claude/commands/` are tracked project files, they will appear inside each
worktree. That is expected.

Authority remains branch content plus coordination records, not the parent
directory. Local state such as `.claude/settings.local.json` should not be used
as shared workitem state. The project should consider untracking
`.claude/settings.local.json` and replacing it with a template.

## Tooling Direction

Add Elixir tasks later:

- `mix agov.workitem.new <id> --branch <branch>`
- `mix agov.workitem.status <id>`
- `mix agov.workitem.restore <id>`
- `mix agov.workitem.bundle <id>`
- `mix agov.workitem.validate docs/workitems/*.yaml`

Validation should check:

- branch exists locally or remotely;
- referenced coordination files exist;
- sequence references are not hidden only on a feature branch;
- status is one of a small fixed set;
- dirty-state expectations are explicit;
- verification commands have cwd, command, and result.

## Open Questions For Claude / Planner-Architect

- Should manifests live in `docs/workitems/`, `agf/workitems/`, or another
  project-control directory?
- Should bundle files ever be committed, or should the manifest only point to an
  external transfer location?
- Should workitem manifests be required for every feature branch, or only for
  long-lived/complex/agent-handoff work?
- Should a Gate 2 handoff require a workitem manifest when the branch has more
  than one commit or one active agent?
- Should `.claude/settings.local.json` be untracked as a separate hygiene slice?

## Codex Recommendation

Make workitem manifests required for complex or cross-agent work, but optional
for trivial single-commit branches. Do not share `.worktrees/` directly. Share a
branch, a coordination handoff, and a small manifest that tells another human or
AI exactly how to reconstruct the local context.
