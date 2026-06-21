---
id: 20260621T133017Z-coder-tester-n5rp
kind: handoff
from: coder-tester
to: planner-architect
timestamp: 2026-06-21T13:30:17Z
immutable: true
references-to:
  - docs/plans/rust-agov-complete.md
  - docs/coordination/20260621T130024Z-coder-tester-v3bm.md
topic: rust-token-roi-report-slice
requires-ack: false
artifacts_owned:
  - tooling/agov
  - tooling/rust/crates/agov/src/main.rs
  - tooling/rust/crates/agov-core/src/roi.rs
  - tooling/rust/crates/agov/tests/token_roi.rs
model: gpt-5-codex
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Review the Rust `agov token roi report` implementation slice.
    - Confirm that parsing both Elixir-style and Rust-style ROI JSONL records is
      sufficient for this parity step.
    - Route the next Rust parity slice, likely `agov usage report` or the first
      packaging primitive.
  must_read:
    - tooling/rust/crates/agov-core/src/roi.rs
    - tooling/rust/crates/agov/src/main.rs
    - tooling/rust/crates/agov/tests/token_roi.rs
    - tooling/agov
    - tooling/elixir/lib/agov/token_roi.ex
    - tooling/elixir/test/agov/token_roi_test.exs
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Rust Token ROI Report Slice

Codex implemented the next Rust parity slice on `feat/rust-token-roi-report`.

Implemented behavior:

- added `agov token roi report [--log <path>]`;
- added dotted compatibility alias `agov agov.token.roi report [--log <path>]`;
- routed both launcher forms through Rust;
- added JSONL loading that tolerates missing logs and skips invalid lines;
- added summary grouping by command and workflow;
- added report formatting compatible with the Elixir report shape;
- added integration tests for normal and dotted command forms.

Validation run:

- `cd tooling/rust && cargo fmt --check`
- `cd tooling/rust && cargo test -p agov-core -p agov`
- `cd tooling/rust && cargo clippy -p agov-core -p agov -- -D warnings`
- `cargo build --manifest-path tooling/rust/Cargo.toml -p agov`
- `./tooling/agov token roi report --log tmp/agov-token-roi.jsonl`

Note:

- Total approximate tokens are estimated from total characters, while per-command
  and per-workflow rows sum record-level token estimates. A one-token difference
  can appear because both paths use ceil(chars / 4), matching the current Elixir
  style.
