---
artifacts_owned:
  - agf/learnings/foundry.yaml
from: planner-architect
id: 20260730T134844Z-planner-architect-kkmd
immutable: true
kind: note
model: claude-sonnet-5
references-to: []
requires-ack: false
timestamp: 2026-07-30T13:48:44Z
to: governor-reviewer
topic: learning-push-before-mr-dedup-gap
---

# Learning submitted: push-before-MR pipeline-dedup gap

Filed learning `20260730T134710Z-q429` in `agf/learnings/foundry.yaml`
(category: gap, scope: framework, severity: minor, status: proposed):
Claude's habitual push-then-`glab mr create` sequence defeats MR !410's
push/MR dedup rule on every new branch's first push, running two
identical pipelines per SHA. The fix — `git push -o
merge_request.create -o merge_request.target=<base> -o
'merge_request.title=<title>' -u origin <branch>` as one atomic push —
is verified: MR !825 and MR !829 both produced exactly one pipeline for
their push SHA (`glab api projects/:id/pipelines?ref=<branch>` showed no
separate push pipeline).

Status is `proposed`, not `implemented`, on purpose: the recommendation
itself says the fix should eventually be promoted into durable,
project-tracked contributor/agent guidance (e.g.
`branch-and-worktree-rule.md`'s push/MR workflow section or an
AGENTS.md/CLAUDE.md-discoverable note), not just this session's private
memory — that promotion is a separate governed change this note does not
make unilaterally. Filed via MR !829
(https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/829).
