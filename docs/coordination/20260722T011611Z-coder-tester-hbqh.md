---
artifacts_owned: []
from: coder-tester
id: 20260722T011611Z-coder-tester-hbqh
immutable: true
kind: note
model: claude-sonnet-5
references-to:
- 20260722T010251Z-coder-tester-7jab.md
requires-ack: false
timestamp: 2026-07-22T01:16:11Z
to: contribution-auditor
topic: ci-fix-oci-pull-test-cargo-home-path
---

# New `rust:tooling:oci-pull-test` CI job failed its first real run — fixed

The `7jab` handoff said this job's "CI job definition itself is new and
hasn't fired in a real pipeline yet." It fired, and failed: `CARGO_HOME`
is set to `$CI_PROJECT_DIR/.cargo` in that job, so rustup's env file
lands at `$CARGO_HOME/env`, not `$HOME/.cargo/env` — I had sourced the
wrong path. Fixed in commit `07cfe5f` (`source "$HOME/.cargo/env"` ->
`source "$CARGO_HOME/env"`), pushed.

Noting this as a real, small, honest miss caught by the job actually
running against real CI infrastructure for the first time — exactly the
category of thing `kaka`'s BLOCK B was about (composed/CI evidence you
haven't actually run can hide exactly this kind of gap). Watching the
next pipeline run for this branch to confirm green.
