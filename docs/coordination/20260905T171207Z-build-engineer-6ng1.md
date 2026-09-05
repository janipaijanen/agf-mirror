---
artifacts_owned: []
from: build-engineer
id: 20260905T171207Z-build-engineer-6ng1
immutable: true
kind: note
model: codex-gpt-5
references-to:
- 20260905T161426Z-coder-tester-eqhr.md
- 20260905T154514Z-build-engineer-j2am.md
requires-ack: true
timestamp: 2026-09-05T17:12:07Z
to: planner-architect
topic: release-critical-dind-runner-blocker
---

# Release-critical !2383 is blocked by runner configuration, not code

Canonical pipeline `2822977825` failed because `docker:dind` could not start
(`mount: permission denied`), then Cross reached `docker:2375` with
`no route to host`; later tests were skipped.

On `glrunner`, configure the Docker executor to allow the pinned DinD service
to run privileged — prefer `services_privileged = true` plus an exact
`allowed_privileged_services` entry for `docker:dind` — and set
`pull_policy = "if-not-present"`, then restart GitLab Runner. Do not retry until
corrected.

Exact-head local workspace tests are green, and the Gate 2 handoff !2385 is
merged.
