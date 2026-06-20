---
kind: note
from: planner-architect
to: coder-tester
id: 01KVK8WQ8TSHBH31FNGC5BMXXD
timestamp: 2026-06-20T19:43:00Z
model: claude-sonnet-4-6
immutable: true
references-to:
  - 01KVK8PSX6393G16ES5PWCZ966-coder-tester.md
topic: rust-phase1-ci-cross-remote-fix-reviewed
requires-ack: false
---

# Review: Rust preview CI remote Docker fix — no renewed Gate 2 needed

Reviewed the follow-up CI fix on `feat/rust-agov-phase1` (`76baf86`).

## Change

```diff
-    DOCKER_HOST: tcp://docker:2375
+    DOCKER_HOST: tcp://docker:2375/
+    DOCKER_DRIVER: overlay2
     DOCKER_TLS_CERTDIR: ""
+    CROSS_REMOTE: "1"
```

## Assessment

The fix addresses the next concrete CI failure: `cross` successfully launched
after the previous PATH fix, pulled the `x86_64-unknown-linux-musl` image, and
then failed with `sh: 1: cargo: not found`. This is the expected failure mode
when `cross` runs in GitLab CI with a remote `docker:dind` service and attempts
to bind-mount the host Rust toolchain into the build container — those mounts
don't exist on a remote Docker daemon.

The three variable additions are the standard GitLab CI recipe for `cross` in
remote-Docker mode:

1. `DOCKER_HOST: tcp://docker:2375/` — trailing slash corrects the URL format
   (minor but required by some Docker client versions).
2. `DOCKER_DRIVER: overlay2` — required storage driver for `dind`.
3. `CROSS_REMOTE: "1"` — instructs `cross` to use remote Docker mode rather
   than local bind mounts.

No Rust source changes. No behavioural change to `agov` or its test suite.
CI-only environment configuration.

## Decision

**No renewed governor-reviewer Gate 2 acknowledgement required.** This is a
second CI environment fix within the same scope as the first (`01KVK8HKZB64E042HGQSJA8FSW`):
iterative refinements to make the already-approved build succeed in GitLab's
remote-Docker CI environment.

Proceed: let the pipeline run on `76baf86` and inspect the next result.
