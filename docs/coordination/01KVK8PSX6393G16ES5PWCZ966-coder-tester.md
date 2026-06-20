---
kind: handoff
from: coder-tester
to: planner-architect
id: 01KVK8PSX6393G16ES5PWCZ966
timestamp: 2026-06-20T19:39:25Z
model: codex
immutable: true
references-to:
  - 01KVK8CVPMYC5S0WRRFDC70607-coder-tester.md
topic: rust-phase1-ci-cross-remote-fix
requires-ack: true
artifacts_owned:
  - .gitlab-ci.yml
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Review the follow-up rust:tooling:build:preview GitLab remote Docker fix.
    - Route to governor-reviewer if renewed Gate 2 acknowledgement is required.
  must_read:
    - .gitlab-ci.yml
    - docs/coordination/01KVK8CVPMYC5S0WRRFDC70607-coder-tester.md
  do_not_touch:
    - plan/readable-coordination-ids
---

# Handoff: Rust preview CI remote Docker fix

The first CI fix made `cross` discoverable on `PATH`, and the job progressed to
the first `cross build` invocation. The next failure was:

```text
sh: 1: cargo: not found
```

This happened after `cross` pulled the `x86_64-unknown-linux-musl` image. The
job runs in GitLab CI with a remote `docker:dind` service, so `cross` must use
remote-Docker mode instead of assuming local bind mounts can expose the host
Rust toolchain to the build container.

Patch applied:

- set `DOCKER_HOST` to `tcp://docker:2375/`;
- set `DOCKER_DRIVER=overlay2`;
- set `CROSS_REMOTE=1`.

Reference checked: the upstream `cross` GitLab CI guidance documents
`DOCKER_HOST`, `DOCKER_DRIVER`, and `CROSS_REMOTE=1` for GitLab's remote Docker
build setup.

Local verification:

- `git diff --check` passed.

Next step is to let the MR !31 pipeline run again and inspect the next concrete
result.
