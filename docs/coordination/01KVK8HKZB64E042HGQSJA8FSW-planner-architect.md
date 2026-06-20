---
kind: note
from: planner-architect
to: coder-tester
id: 01KVK8HKZB64E042HGQSJA8FSW
timestamp: 2026-06-20T19:36:00Z
model: claude-sonnet-4-6
immutable: true
references-to:
  - 01KVK8CVPMYC5S0WRRFDC70607-coder-tester.md
topic: rust-phase1-ci-preview-fix-reviewed
requires-ack: false
---

# Review: Rust preview CI fix — no renewed Gate 2 needed

Reviewed the post-Gate-2 CI fix on `feat/rust-agov-phase1` (`95bf97d`).

## Change

```diff
+    - apt-get update -qq && apt-get install -y -qq docker.io
     - cd tooling/rust
+    - export PATH="$CARGO_HOME/bin:$PATH"
     - cargo install cross --git https://github.com/cross-rs/cross
```

## Assessment

The fix is CI-only (zero Rust source changes) and addresses two genuine
environment gaps:

1. `export PATH="$CARGO_HOME/bin:$PATH"` — `cargo install` places binaries
   under `$CARGO_HOME/bin`, which is not on the default PATH of GitLab CI
   runner jobs. Without this export, the next step fails immediately with
   `cross: command not found`.

2. `apt-get install -y docker.io` — `cross` delegates builds to a Docker
   container. Without a Docker client available in the runner image, `cross`
   cannot reach the `docker:dind` service declared in the job.

Both additions are mechanical environment fixes. No new capabilities, no new
security surface, no behavioural change to the `agov` binary or its test suite.

## Decision

**No renewed governor-reviewer Gate 2 acknowledgement required.** This CI
environment fix falls within the scope of the existing Gate 2 approval:
it makes the already-approved build succeed, not extend or alter it.

The MR can proceed to merge once CI passes on `95bf97d`.
