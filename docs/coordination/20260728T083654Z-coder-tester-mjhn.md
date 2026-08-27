---
id: 20260728T083654Z-coder-tester-mjhn
kind: handoff
immutable: true
from: coder-tester
to: fact-checker
timestamp: 2026-07-28T08:36:54Z
model: claude-sonnet-5
references-to: []
topic: ci-skip-substring-learning-factcheck
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: fact-checker
  status: ready
  task:
    - "Verify the incident account in learning record 20260728T082825Z-m969 (agf/learnings/foundry.yaml) and the corresponding rule clarification in agf/rules/workflow/coordination-publication-rule.md (MR !739, branch learning/ci-skip-substring-in-commit-message): does the claimed sequence of GitLab pipeline IDs (2709894713, 2709895485, 2709896093, 2709903266 -- all zero-job/skipped -- followed by 2711130008 succeeding after the commit message was reworded) actually support the claimed root cause (literal [ci skip]/[skip ci] substring in commit message body text, not directive use, not a platform flake)?"
    - Return CLEAR, ADAPT, or BLOCK with reasoning.
  must_read:
    - agf/learnings/foundry.yaml
    - agf/rules/workflow/coordination-publication-rule.md
  do_not_touch:
    - agf/VERSION
    - any release branch or tag
---

# Fact-check requested: CI-skip-substring learning and rule clarification

MR !739 adds a learning record and a rule-doc clarification after
diagnosing (with the human governor's direct catch) a real incident on
MR !737: the release-prep commit message's own prose, describing the
`[ci skip]`/`[skip ci]` prohibition using the literal bracketed marker
text, self-triggered GitLab's platform-level pipeline-creation skip --
mistaken for a transient flake across four retries before the actual
cause was identified. Fix was rewording the commit message and
force-pushing with lease; a real pipeline followed immediately.

Requesting your independent verification of this account before it is
considered for governor-reviewer review/merge, per this session's
established review discipline. I did not self-merge or self-close this;
it is a proposal awaiting review like everything else this session.
