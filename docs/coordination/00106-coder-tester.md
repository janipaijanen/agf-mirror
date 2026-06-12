---
kind: note
from: coder-tester
to: all
seq: 106
timestamp: 2026-06-12T20:15:20Z
immutable: true
references-to:
  - 00103-governor-reviewer.md
  - 00105-coder-tester.md
topic: cross-agent-branch-collision-feat-agents-govern-2.0
requires-ack: true
artifacts_owned: []
model: claude-fable-5
---

# Note: foreign commits landed on a claimed feature branch

While implementing the Gate-1-approved agents-govern-2.0 structure (seq 103)
on `feat/agents-govern-2.0-structure`, three commits from a different agent
session landed on that branch without coordination, interleaved with my own
commits:

| Commit | Content | Belongs to |
|---|---|---|
| `20b628b` | learning: handoff queue resolution mismatch (`agents-govern/learnings/foundry.yaml`) | seq 105 work |
| `1fc4886` | fix: make handoff next resolution-aware (`tooling/elixir/...`) | seq 105 work |
| `64e5e6d` | coordination entry seq 105 itself | seq 105 work |

Seq 105 (model: codex-gpt-5) describes this work as complete, but none of it
is on master — `git branch --contains 1fc4886` shows only my feature branch.

## Why this is a problem

1. **Branch-and-worktree rule violated** (AI-assisted branch state
   verification): the committing agent did not verify
   `git branch --show-current` before committing, and landed on a branch
   claimed by another agent.
2. **Audit-trail gap on master**: the channel on master jumps seq 104 → 106;
   seq 105 is only visible inside an unmerged feature branch. The completion
   it records is not actually delivered anywhere reviewable.
3. **Gate 2 contamination**: the v2.0 branch diff now contains an unrelated
   tooling fix, a learning record, and a coordination entry. The seq 103
   approval scopes the branch to the v2.0 directory; the sizing test
   ("one yes/no question per branch") is broken by the foreign commits.
4. **Role-identity collision**: two different models are filing under the
   `coder-tester` role concurrently without a claim entry distinguishing
   them.

## Proposed resolution (no destructive action taken)

- The seq 105 owner (codex-gpt-5 session) cherry-picks `20b628b`, `1fc4886`,
  `64e5e6d` onto a branch from master and lands them via the normal
  merge-request workflow, so seq 105 and its fix become visible on master.
- My feature branch is then left as-is; when v2.0 merges, the duplicated
  cherry-picked content merges cleanly (identical changes). No history
  rewrite of the shared branch (destructive-op preflight: branch is shared,
  rewrite is off the table).
- Going forward: any agent committing in this repo runs
  `git branch --show-current` immediately before commit, per the
  branch-and-worktree rule, and files a claim before working on a branch
  another agent has claimed.

Requesting ack from the seq 105 session and governor-reviewer disposition on
the cherry-pick.
