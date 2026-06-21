# Session Resume Reference

`agov session resume` gives a returning human or AI agent a compact,
role-specific starting point before it acts on the repository.

It is read-only. It does not approve work, resolve handoffs, release claims,
write coordination messages, or enforce a gate. It is a visibility tool for
reducing false context at session boundaries.

## One Command, Two Steps

S1 intentionally keeps cold start and role-specific resume in the same command
instead of adding another tool.

For humans and AI agents, the startup path is:

1. Run `agov session resume --base-branch main` without `--role` to get the
   project map.
2. Choose the role you are actually playing from that output.
3. Rerun the same command with the selected role, for example
   `agov session resume --role coder-tester --base-branch main`.

The first run answers “where do I start?” The second run answers “what must I
do for this role before acting?”

## When To Run It

Run this at the start of a new or resumed working session, before making a
durable change such as editing governed files, committing, filing a handoff, or
approving a gate.

Also run it after:

- switching branches or worktrees;
- returning from a long pause;
- model or tool handoff;
- context compaction;
- receiving a coordination message that says the queue changed.

Before deliberately compacting, clearing, or handing a non-trivial task to
another model, externalize state first. A durable note, handoff, review, or
completion should name the branch, latest relevant commit, changed files,
validation results, open blockers, next concrete action, and must-read
coordination or plan files. Session memory is never the authority for a
governance decision or obligation.

Claude Code users can run `/compact-agf` before `/compact`; Codex and other AI
agents should treat the same phrase as an instruction to write the same durable
closeout note before compaction. The project command definition lives at
`.claude/commands/compact-agf.md`, but the AGF contract is tool-neutral:
externalize the state to `docs/coordination/`, validate it, then compact.

## Command

Run session resume through the Rust `agov` command:

```bash
agov session resume --role coder-tester --base-branch main
```

For a true cold start, when you do not yet know which role you are playing, run
the command without `--role`:

```bash
agov session resume --base-branch main
```

That prints:

- active roles from the manifest;
- exact follow-up commands for each role;
- pending handoffs grouped by role;
- recent decisions;
- branch and dirty-tree warnings.

Options:

| Option | Use |
|---|---|
| `--role <agent-role>` | Role whose queue should be shown. Overrides `current_role` in `agov-config.yaml`. |
| `--base-branch <branch>` | Branch used to detect coordination files present only on the current branch. Use the repository's active integration branch, normally `main`. |
| `--dir <path>` | Coordination directory. Defaults to `docs/coordination` from the repository root. |

If `--role` is omitted, the command uses `current_role` from
`docs/coordination/agov-config.yaml` when present. If neither is available, it
prints the cold-start bootstrap view instead of failing.

Example config:

```yaml
current_role: coder-tester
from_aliases:
  claude: planner-architect
```

## How AI Agents Should Use The Output

Treat the output as a pre-flight briefing, not as permission to proceed.

1. If the output says `Session bootstrap`, choose the role you are actually
   playing and rerun the displayed role-specific command.
2. Confirm the reported branch is the branch you intend to use.
3. Read any pending handoff and `Must read` files before editing.
4. Resolve dirty-tree warnings before writing durable governance records.
5. Treat stale claims as a coordination risk. Do not take ownership silently.
6. Check recent decisions for approvals, rejections, or changed constraints.
7. If the command reports branch-local coordination files, make them visible on
   the branch where the receiving role will read them before relying on them.

If the output conflicts with session memory, trust the files and git state over
memory.

For long sessions, preserve context deliberately. Prefer reading named files to
carrying large excerpts in prompt history. When context is high, write a short
state note before compaction. When switching to an unrelated task, clear the
session only after the current task's state is durable enough for a cold-starting
agent to resume.

### Coordination Visibility And L009

L009 is the governing lesson for branch-local coordination warnings:
coordination messages that create an obligation for another agent or human must
be visible on the branch the recipient is expected to poll, or immediately
paired with a bridge note on that branch.

That means:

- a Gate 2 handoff written only on a feature branch is not enough when the
  governor-reviewer is polling `master`;
- a cross-agent review request written only on a side branch is not enough when
  the receiving agent is polling `master`;
- branch-local coordination is acceptable only when its audience is limited to
  reviewers of that branch and an already-visible message names where to look.

When `session.resume` reports coordination files present only on the current
branch, decide whether they create an obligation. If they do, file a visible
bridge note before waiting for the recipient to act.

## How Humans Should Use The Output

Use the command when reviewing an agent's claim about what is pending. It gives
you a quick answer to:

- what branch the working session is on;
- whether there are uncommitted changes;
- whether the role has pending handoffs;
- whether it owns active or stale claims;
- which recent decisions may affect the next action;
- whether coordination records may be hidden on the current branch.

For Gate 1 or Gate 2 review, this command is useful context, but the binding
decision still comes from the plan, diff, tests, and coordination record.

## What The Sections Mean

| Section | Meaning |
|---|---|
| `Role` | The role used for filtering handoffs and claims. |
| `Branch` | Current git branch, or detached state if applicable. |
| `Warnings` | Dirty tree, skipped base-branch check, or branch-local coordination risk. |
| `Invalid coordination files ignored` | Markdown files that `ChannelReader` skipped because they do not validate as coordination messages. |
| `Pending handoffs` | Unresolved handoffs addressed to the selected role. |
| `Open claims` | Active artifact ownership claims owned by the selected role. |
| `Stale claims` | Expired claims owned by the selected role. |
| `Recent decisions` | Latest decision-record messages in the coordination channel. |
| `Must read` | Files listed by currently pending handoffs for the selected role. |

## Failure Modes

If the command says no role is configured, pass `--role` explicitly or add
`current_role` to `agov-config.yaml`.

If the base branch is unavailable, the command continues and reports that the
branch-local coordination check was skipped. The rest of the resume output is
still usable, but hidden coordination risk has not been ruled out.

If the working tree is dirty, the command reports changed paths. This is
expected while implementing work, but it is a warning before creating
coordination messages, approvals, or merge decisions.

## Scope

This command is the first session-continuity slice. It does not replace:

- `agov handoff next`, when you only need the newest handoff for one role;
- `agov coord status`, when you need the wider channel overview;
- human governor-reviewer approval;
- closeout discipline before shutdown or handoff.

Future slices may add closeout or guard commands if repeated incidents show
that read-only resume is not enough.
