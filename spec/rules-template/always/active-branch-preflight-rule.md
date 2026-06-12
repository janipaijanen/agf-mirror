---
id: active-branch-preflight-rule
scope: always
applies_to: [llm-agent]
evidence: L007
upstream_candidate: true
strength: MUST
status: active
see_also: [branch-and-worktree-rule]
---

# Verify Active Branch Before Durable Writes

Treat the active git branch and worktree state as volatile. Do not rely on the
branch seen earlier in the session, in a previous turn, or before another tool
operation.

Immediately before any durable write, run:

```bash
git branch --show-current
git status --short --branch
```

Durable writes include:

- `git commit`
- `git push`
- merge, tag, rebase, reset, branch deletion, or branch rewrite operations
- coordination, learning, or plan files intended for asynchronous consumption
- any generated artifact that another agent, reviewer, or CI job is expected to
  consume

## Required decision

After the preflight, compare the current branch and dirty state to the intended
task:

- If the branch is the intended branch and the dirty state matches the task,
  proceed.
- If the branch differs from the intended branch, stop before writing.
- If the branch is shared, claimed by another agent, or would mix unrelated
  artifacts into the current review question, stop before writing.
- If the working tree contains unrelated user or agent changes, leave them
  untouched and scope the write so they are not staged or committed.

## Recovery

When the preflight fails, do not "fix" it with destructive commands. Report the
observed branch, intended branch, and affected artifacts. Then either:

- switch or create the correct branch without touching unrelated changes;
- ask the governor/reviewer for disposition if branch ownership is ambiguous;
- write a coordination note if another agent must decide what happens next.

## Why this exists

In a shared checkout, branch identity can change between tool calls. IDEs,
resumed sessions, concurrent agents, and stale context can all invalidate an
agent's memory of where it is working. A wrong-branch commit is not merely a git
mistake; it contaminates review scope, confuses coordination sequence history,
and can make unrelated governance decisions travel together.
