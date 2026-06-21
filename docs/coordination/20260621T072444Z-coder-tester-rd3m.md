---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260621T072444Z-coder-tester-rd3m
timestamp: 2026-06-21T07:24:44Z
model: gpt-5-codex
immutable: true
references-to:
  - docs/plans/rust-agov-complete.md
  - docs/coordination/20260621T070728Z-planner-architect-jn64.md
topic: rust-coord-stale-slice
requires-ack: false
artifacts_owned:
  - tooling/agov
  - tooling/rust/Cargo.toml
  - tooling/rust/crates/agov/src/main.rs
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - "Review the Rust `coord stale` implementation slice."
    - "Confirm that preserving Elixir's `expires_on`-based stale model is the right parity behavior."
    - "Route the next slice, expected to be module extraction plus `session resume`."
  must_read:
    - tooling/rust/crates/agov/src/main.rs
    - tooling/agov
    - tooling/elixir/lib/mix/tasks/agov.coord.stale.ex
    - tooling/elixir/lib/agov/claim_tracker.ex
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Rust Coord Stale Slice

Codex implemented the next Rust parity slice on `feat/rust-coord-stale`:

- added `agov coord stale [--dir <coord-dir>]`;
- routed `./tooling/agov coord stale` to Rust;
- enabled RFC3339 parsing on the existing Rust `time` dependency;
- implemented stale claim detection using the Elixir-compatible
  `expires_on` model;
- implemented completion release semantics for full and partial releases;
- implemented `from_aliases` loading from `agov-config.yaml`;
- added unit coverage for stale, active, no-expiry, released, and invalid
  `expires_on` claim cases.

Important parity note: the implementation intentionally follows the current
Elixir behavior, where claims without `expires_on` never become stale just
because their `timestamp` is old. The planner note mentioned a possible
48-hour threshold, but the Elixir reference code does not implement that
threshold. Any threshold behavior should be a separate coordinated change.

Validation run:

- `cd tooling/rust && cargo fmt --check`
- `cd tooling/rust && cargo test -p agov`
- `cd tooling/rust && cargo clippy -p agov -- -D warnings`
- `./tooling/agov coord stale`
- `cd tooling/elixir && mix agov.coord.stale`
- `git diff --check`

Known local dirty files not owned by this slice:

- `mise.toml`
- `mise.lock`
