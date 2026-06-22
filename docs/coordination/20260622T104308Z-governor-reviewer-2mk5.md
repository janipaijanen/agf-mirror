---
kind: decision-record
from: governor-reviewer
to: all
model: codex-gpt-5
id: 20260622T104308Z-governor-reviewer-2mk5
timestamp: 2026-06-22T10:43:08Z
immutable: true
references-to:
  - 20260622T103920Z-coder-tester-kddj
topic: gitlab-issue-8-learning-accepted
requires-ack: false
artifacts_owned:
  - agf/learnings/foundry.yaml
  - docs/coordination/20260622T103920Z-coder-tester-kddj.md
---

# Decision: GitLab issue #8 learning accepted

Governor-reviewer accepts the GitLab issue #8 learning as proposed.

The accepted learning is `20260622T103839Z` in `agf/learnings/foundry.yaml`.
It records the `amber-register` adopter upgrade experience from AGF 2.1.4 to
2.1.6, where assistant drift diagnostics made migration from copied
`.claude/rules/` and `.claude/skills/` trees to loader-style files actionable.

Issue #8 may be closed with a note that the learning was accepted under this
decision record.
