---
id: 20260621T134618Z-coder-tester-ux7m
kind: handoff
from: coder-tester
to: planner-architect
timestamp: 2026-06-21T13:46:18Z
immutable: true
references-to:
  - docs/plans/rust-agov-complete.md
  - docs/coordination/20260621T133017Z-coder-tester-n5rp.md
topic: rust-usage-report-slice
requires-ack: false
artifacts_owned:
  - tooling/agov
  - tooling/rust/crates/agov/src/main.rs
  - tooling/rust/crates/agov/tests/usage_report.rs
  - agf/docs/site/reference/tooling-conformance.md
  - agf/docs/site/index.md
  - agf/docs/site/docs/adopt/tooling-and-tokens.md
model: gpt-5-codex
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Review the Rust `agov usage report` implementation slice.
    - Confirm that the output shape remains close enough to the Elixir reference
      task for the current parity stage.
    - Route the next Rust parity slice, likely distribution or packaging
      primitives for CI simplification and Windows binary release work.
  must_read:
    - tooling/rust/crates/agov/src/main.rs
    - tooling/rust/crates/agov/tests/usage_report.rs
    - tooling/agov
    - tooling/elixir/lib/mix/tasks/agov.usage.report.ex
    - tooling/elixir/lib/agov/usage_reporter.ex
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Rust Usage Report Slice

Codex implemented the next Rust parity slice on `feat/rust-usage-report`.

Implemented behavior:

- added `agov usage report [--dir <coord-dir>] [--manifest <path>] [--last <n>]`;
- added dotted compatibility alias `agov agov.usage.report ...`;
- routed both launcher forms through Rust;
- counts message kinds and senders after applying `from_aliases`;
- reports active-status agents with no messages in the selected window;
- reports unknown senders after alias resolution;
- preserves invalid coordination file warning behavior from the Elixir task;
- updates public tooling/maturity docs to show session resume, stale,
  visibility, token ROI, and usage report as Rust-supported.

Validation run:

- `cd tooling/rust && cargo fmt --check`
- `cd tooling/rust && cargo test -p agov-core -p agov`
- `cd tooling/rust && cargo clippy -p agov-core -p agov -- -D warnings`
- `./tooling/agov usage report --last 5`
- `./tooling/agov coord validate docs/coordination/20260621T134618Z-coder-tester-ux7m.md`

Notes:

- The live smoke exits 0 and prints the expected recent activity summary.
- The live channel currently contains hundreds of legacy coordination files that
  do not validate under the current schema, so `usage report` emits a large
  skipped-file warning on stderr before/alongside the useful summary. This
  matches current Elixir behavior but is a UX candidate for a later
  `--quiet-invalid` or warning-summary refinement.
- Local shell commands in this worktree emit `mise` trust warnings because the
  worktree-local `mise.toml` is not trusted. Direct `~/.cargo/bin/cargo` calls
  were used for validation.
