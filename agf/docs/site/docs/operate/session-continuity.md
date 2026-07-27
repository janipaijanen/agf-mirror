# Session continuity

*Status: Released. Audience: humans and AI assistants pausing, resuming, or recovering governed work. This procedure preserves uncertainty instead of turning session memory into project state.*

## Start or resume

Begin in the intended checkout. Before editing, run the Rust operator path in
two steps:

```bash
agov session resume --base-branch main
agov session resume --role <role> --base-branch main
```

Read the resulting handoffs, decisions, warnings, and must-read files. Confirm
the branch and worktree are the intended ones. Fetching remote refs and
checking forge activity are separate steps; use [poll and coordinate](poll-and-coordinate.md)
when shared state may have changed.

## Graceful checkpoint

Use this before deliberate context cleanup or `/compact`, an IDE reload or
update, planned reboot/shutdown, model/tool switch, long pause, or cross-agent
handoff.

1. Stop new edits. Identify the active role, task, branch, worktree, UTC time,
   and exact commit.
2. Record dirty and untracked paths, completed changes, validations, open
   blockers and decisions, next authorized action, must-read references, and
   facts the next session must not assume.
3. Generate a readable coordination ID with
   `agov coord brief --from <role>`.
4. Write a `note` for continuity with no new recipient obligation. Use a
   `handoff` when a specific role must act next.
5. Validate the record with `agov coord validate <path>`.
6. If another machine, person, or agent may resume, commit and push the record
   or publish an approved visible bridge naming its branch or merge request.
7. Report all remaining dirty, uncommitted, local-only, and unpushed state.
   Only then compact, update, reboot, or hand off.

Validation alone does not publish a record. A pushed checkpoint does not make
unsaved editor buffers or uncommitted files available elsewhere; name those
gaps explicitly.

## Abrupt-loss recovery

Use this after automatic compaction, an IDE or process crash, killed terminal,
power or network loss, unexpected reboot, missing session, or resumption on
another machine. Start read-only. Do not reset, overwrite, clean, or treat a
stash as a shared handoff.

```bash
git worktree list
git status --short --branch
git branch --show-current
git log --oneline --decorate -5
git reflog -10
```

When transport is available, run `git fetch --prune origin` and compare the
exact local and remote heads. Then run both `session resume` steps, inspect
coordination status, stale claims, visibility, graph state and handoffs, and
inspect the forge surfaces described in [poll and coordinate](poll-and-coordinate.md).

Classify each fact as:

- **local** — recoverable only in this checkout;
- **shared** — visible on a remote branch, coordination record, MR, issue, or
  pipeline; or
- **unknown** — absent, ambiguous, or remembered but unsupported.

Publish a recovery note naming the evidence, gaps, ownership ambiguity, and
next already-authorized action. Escalate unclear approval, ownership, security,
or destructive recovery choices to the appropriate role.

## Recovery cases

| Recovery input | Classification | Expected action |
|---|---|---|
| No checkpoint exists | Unknown | Publish a recovery note that names the evidence gap; do not reconstruct completion or approval from memory |
| Dirty persistent checkout | Local | Preserve it; identify changed/untracked paths and ownership before editing |
| Local-only commit not pushed | Local | Verify its parent and content; push only if already authorized and safe |
| Pushed branch without handoff | Shared branch, unknown completion | Inspect exact head and tests; publish a validated handoff or recovery note rather than inferring completion |
| Lost or unsaved IDE buffer | Unknown | Mark the contents unknown; do not claim the missing work existed |
| Base branch missing or network unavailable | Unknown shared state | Continue local read-only inspection; state that shared-state comparison is incomplete |
| Fresh clone on another machine | Unknown local state | Assume no local commits, stashes, worktrees, `tmp/`, overlays, terminal history, or unsaved buffers survived |

If a workstation incident may have exposed credentials, stop credential use
and route the evidence to the local infosec-sentinel/security process. Do not
publish secrets in the coordination record.

## Claude adapter: `/compact-agf`

The optional Claude command can collect and validate the checkpoint. Before it
declares readiness it must also report publication state and any remaining
local-only work. The canonical contract is this page, so removing the Claude
adapter does not remove continuity support.
