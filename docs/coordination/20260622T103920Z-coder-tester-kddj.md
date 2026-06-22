---
kind: handoff
from: coder-tester
to: governor-reviewer
model: codex-gpt-5
id: 20260622T103920Z-coder-tester-kddj
timestamp: 2026-06-22T10:39:20Z
immutable: true
references-to: []
topic: gitlab-issue-8-learning-recorded
requires-ack: false
artifacts_owned:
  - agf/learnings/foundry.yaml
  - docs/coordination/20260622T103920Z-coder-tester-kddj.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the learning appended from GitLab issue #8.
    - Decide whether the wording is acceptable for merge and whether issue #8 should close after the learning lands.
  must_read:
    - agf/learnings/foundry.yaml
  do_not_touch: []
---

# Gate 2 handoff: GitLab issue #8 learning recorded

GitLab issue #8 contains an adopter learning from `amber-register` about AGF
2.1.4 to 2.1.6 upgrade friction. The useful finding is positive: AGF 2.1.6
assistant drift diagnostics made copied `.claude/rules/` and `.claude/skills/`
cleanup actionable and clarified that plan and coordination records require
their dedicated validators.

I appended this to `agf/learnings/foundry.yaml` as learning
`20260622T103839Z`.

## Validation

Run from `.worktrees/mr-parity-repair` before transfer to the clean learning
branch, then repeated on `.worktrees/issue-8-learning` before commit:

- `./tooling/agov validate learnings` — passed
- `./tooling/agov validate infosec` — passed
- `git diff --check -- agf/learnings/foundry.yaml` — passed

The shell printed a `mise` trust warning in `.worktrees/mr-parity-repair`
before command output, but each command exited successfully and reported
OK/pass. The clean learning branch did not include that unrelated dirty
worktree state.

## Issue disposition

Recommendation: keep GitLab issue #8 open until this learning is merged or
otherwise accepted by governor-reviewer. Then close #8 with a note naming
learning ID `20260622T103839Z`.
