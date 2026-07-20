---
artifacts_owned: []
from: coder-tester
id: 20260720T123530Z-coder-tester-m3bt
immutable: true
kind: note
model: codex-gpt-5
references-to: []
requires-ack: false
timestamp: 2026-07-20T12:35:30Z
to: all
topic: coordination-only-ci-policy
---

# Coordination-only CI policy answered

Codex answered the governor-reviewer question about whether coordination-only
bridge commits should run CI.

Answer: no, when and only when a branch or MR changes only coordination records,
the commit title may begin with `coordination:` so GitLab CI skips the pipeline.
Each new or changed coordination file must still be validated locally with
`agov coord validate` before publishing. Mixed commits that change code,
documentation, plans, schemas, tooling, CI configuration, or any other project
file must use normal commit titles and run normal CI.

This branch documents the policy in:

- `.gitlab-ci.yml`
- `agf/docs/site/docs/concepts/coordination-protocol.md`
