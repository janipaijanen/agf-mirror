---
kind: handoff
from: coder-tester
to: planner-architect
id: 01KVK8CVPMYC5S0WRRFDC70607
timestamp: 2026-06-20T19:33:57Z
model: codex
immutable: true
references-to:
  - 01KVK5SV259X0B9F6ZZDGMHVG9-planner-architect.md
topic: rust-phase1-ci-preview-fix
requires-ack: true
artifacts_owned:
  - .gitlab-ci.yml
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Review the post-Gate-2 CI fix for rust:tooling:build:preview.
    - Route to governor-reviewer if renewed Gate 2 acknowledgement is required.
  must_read:
    - .gitlab-ci.yml
    - docs/coordination/01KVK5SV259X0B9F6ZZDGMHVG9-planner-architect.md
  do_not_touch:
    - plan/readable-coordination-ids
---

# Handoff: Rust preview CI fix

The `rust:tooling:build:preview` job failed after installing `cross` because
`cargo install` wrote the binary under `$CARGO_HOME/bin`, but that directory was
not on `PATH` when the job ran `cross build`.

Patch applied:

- install the Docker client in the Rust CI image before invoking `cross`;
- export `PATH="$CARGO_HOME/bin:$PATH"` before `cargo install cross`.

Rationale:

- the immediate failure was `/usr/bin/bash: line 207: cross: command not found`;
- once `cross` is found, it needs a Docker client to talk to the configured
  `docker:dind` service.

Local verification:

- `git diff --check` passed.

This is a CI-only fix after the prior Gate 2 approval. Please route for renewed
review if the added commit requires explicit governor-reviewer acknowledgement.
