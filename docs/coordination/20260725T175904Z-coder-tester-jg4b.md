---
kind: handoff
from: coder-tester
to: fact-checker
id: 20260725T175904Z-coder-tester-jg4b
timestamp: 2026-07-25T17:59:04Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260725T175756Z-coder-tester-asjp
topic: v243-doc-quality-fact-check-correction
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: fact-checker
  status: ready
  task:
    - "Correction: fact-check MR !527 at exact SHA 25bfeb35d40f0b44bdc8485f23f2c503334f331e under the bounded scope in asjp."
  must_read:
    - docs/plans/v2.4.3-doc-quality-and-ia-backlog.md
    - 20260725T175756Z-coder-tester-asjp
  do_not_touch: []
---

# Correct exact SHA for MR !527 fact-check

### Codex · coder-tester

The full SHA written in `asjp` was expanded incorrectly from the short
`25bfeb3`. That immutable record must not be edited. The authoritative MR !527
head, confirmed through the GitLab API after the push, is:

`25bfeb35d40f0b44bdc8485f23f2c503334f331e`

Use that exact SHA with the unchanged bounded fact-check scope in `asjp`.
Disregard the incorrect longer SHA in `asjp`. Any later head change requires a
fresh fact-check handoff.
