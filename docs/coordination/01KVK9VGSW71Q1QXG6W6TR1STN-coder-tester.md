---
kind: handoff
from: coder-tester
to: planner-architect
id: 01KVK9VGSW71Q1QXG6W6TR1STN
timestamp: 2026-06-20T19:59:28Z
model: codex
immutable: true
references-to:
  - 01KVK9CH15S9R2957ACQKMB1KH-coder-tester.md
topic: rust-phase1-ci-harness-fixes
requires-ack: true
artifacts_owned:
  - .gitlab-ci.yml
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Review the final CI harness fixes for MR !31.
    - Route to governor-reviewer if renewed Gate 2 acknowledgement is required.
  must_read:
    - .gitlab-ci.yml
    - docs/coordination/01KVK9CH15S9R2957ACQKMB1KH-coder-tester.md
  do_not_touch:
    - plan/readable-coordination-ids
---

# Handoff: CI harness fixes

After the Linux preview scope correction, pipeline `2616449130` reached the
test stage:

- `rust:tooling:build:preview` passed.
- `rust:tooling:test` passed.
- `rust:tooling:parity`, `rust:tooling:clippy`, and `rust:tooling:deny`
  failed due to CI harness setup issues.

Patch applied:

- `rust:tooling:deny` now uses `cargo install cargo-deny ... --force` so a
  cache-restored binary does not fail the job.
- `rust:tooling:clippy` now runs `rustup component add clippy` before invoking
  `cargo clippy`.
- `rust:tooling:parity` now installs `build-essential` in the Elixir image and
  runs the Rust build in a subshell so the subsequent parity script path remains
  repo-root-relative.

Local verification:

- `git diff --check` passed.

Next step is to let the MR !31 pipeline rerun on this commit.
