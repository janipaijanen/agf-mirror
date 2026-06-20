---
kind: handoff
from: coder-tester
to: planner-architect
id: 01KVK9CH15S9R2957ACQKMB1KH
timestamp: 2026-06-20T19:51:23Z
model: codex
immutable: true
references-to:
  - 01KVK8PSX6393G16ES5PWCZ966-coder-tester.md
topic: rust-phase1-linux-preview-scope-correction
requires-ack: true
artifacts_owned:
  - .gitlab-ci.yml
  - docs/plans/rust-agov-port.md
  - MR_DESCRIPTION.md
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Review the Phase 1 preview build scope correction.
    - Route to governor-reviewer for explicit acknowledgement if required.
  must_read:
    - .gitlab-ci.yml
    - docs/plans/rust-agov-port.md
    - MR_DESCRIPTION.md
    - docs/coordination/01KVK8PSX6393G16ES5PWCZ966-coder-tester.md
  do_not_touch:
    - plan/readable-coordination-ids
---

# Handoff: Linux preview scope correction

The remote-Docker `cross` fix allowed the MR !31 preview job to build both
Linux musl targets successfully:

- `x86_64-unknown-linux-musl`
- `aarch64-unknown-linux-musl`

The job then failed at `x86_64-apple-darwin` because upstream `cross` does not
provide a Docker image for Darwin targets. Its guidance points to custom
toolchains for additional targets, and macOS builds require a macOS SDK/custom
image supply-chain decision.

Patch applied:

- `rust:tooling:build:preview` now builds the two Phase 1 Linux musl preview
  targets only.
- `docs/plans/rust-agov-port.md` now documents macOS preview builds as deferred
  until a separate macOS SDK/custom-image decision.
- `MR_DESCRIPTION.md` now describes the Linux preview lane and the macOS
  deferral.

Local verification:

- `git diff --check` passed.

This is a post-Gate-2 scope correction caused by CI evidence. Please route for
renewed governor-reviewer acknowledgement if needed before merge.
