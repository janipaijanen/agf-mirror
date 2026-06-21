---
id: 20260621T130024Z-coder-tester-v3bm
kind: handoff
from: coder-tester
to: planner-architect
timestamp: 2026-06-21T13:00:24Z
immutable: true
references-to:
  - docs/plans/rust-agov-complete.md
  - docs/coordination/20260621T093713Z-coder-tester-r8xk.md
topic: rust-coord-visibility-slice
requires-ack: false
artifacts_owned:
  - tooling/agov
  - tooling/rust/crates/agov/src/main.rs
  - tooling/rust/crates/agov/tests/coord_visibility.rs
model: gpt-5-codex
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Review the Rust `agov coord visibility` implementation slice.
    - Confirm that local-branch Git scanning and text/JSON output match the
      Elixir reference closely enough for Rust parity.
    - Confirm whether old duplicate `seq` collisions on `origin/main` should
      remain reported by this command or be suppressed in a later compatibility
      slice.
  must_read:
    - tooling/rust/crates/agov/src/main.rs
    - tooling/rust/crates/agov/tests/coord_visibility.rs
    - tooling/agov
    - tooling/elixir/lib/agov/coord_visibility.ex
    - tooling/elixir/test/agov/coord_visibility_test.exs
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Rust Coord Visibility Slice

Codex implemented the next Rust parity slice on `feat/rust-coord-visibility`.

Implemented behavior:

- added `agov coord visibility`;
- routed `./tooling/agov coord visibility` to Rust;
- supports `--base-branch`, repeatable `--branch`, `--dir`, and
  `--format text|json`;
- scans local Git branches with `git for-each-ref`, `git ls-tree`, and
  `git show`;
- reports branch-local obligation-shaped coordination messages not visible from
  the base branch;
- accepts bridges through `references-to`, direct filename body text, or
  branch-plus-filename body text;
- reports duplicate legacy `seq` collisions on the base branch and branch/base
  legacy `seq` collisions;
- preserves the Elixir exit-code shape: `0` clean, `1` unbridged obligations,
  `2` collisions or scanner errors.

Validation run:

- `cd tooling/rust && cargo fmt --check`
- `cd tooling/rust && cargo test -p agov`
- `cd tooling/rust && cargo clippy -p agov -- -D warnings`
- `cargo build --manifest-path tooling/rust/Cargo.toml -p agov`
- `./tooling/agov coord visibility --base-branch origin/main --branch feat/rust-coord-visibility`

Live smoke note:

- The live `origin/main` scan exits `2` because old duplicate legacy `seq`
  collisions already exist on the base branch.
- The same smoke did not report new branch-local obligations for this branch
  when compared to `origin/main`.
