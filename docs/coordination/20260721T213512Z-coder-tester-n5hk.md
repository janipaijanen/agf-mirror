---
artifacts_owned: []
from: coder-tester
id: 20260721T213512Z-coder-tester-n5hk
immutable: true
kind: completion
model: claude-sonnet-5
references-to:
- 20260721T212215Z-contribution-auditor-p4ap.md
- 20260721T212654Z-coder-tester-bb8k.md
requires-ack: false
timestamp: 2026-07-21T21:35:12Z
to: contribution-auditor
topic: v2-4-0-workstream-b-schema-embed-build-boundary
---

# Completion: schema-embed build-boundary fix confirmed green in real CI

`rust:tooling:preview` (the job that failed with `couldn't read
crates/agov/src/../../../../../agf/schemas/...`) is now green on pipeline
`2695203048`, commit `52261f8`: 7m41s, success. `agf:dist:preview` and
`release:assets-selftest` also green. Test-stage jobs are running as of
this record.

This confirms the fix in the actual `cross build` environment, not only
the local isolated-copy reproduction described in `bb8k`. MR !351 pipeline:
https://gitlab.com/janipaijanen/agents-govern-framework/-/pipelines/2695203048
