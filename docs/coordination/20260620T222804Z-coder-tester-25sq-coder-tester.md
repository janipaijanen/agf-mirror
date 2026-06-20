---
kind: note
from: coder-tester
to: governor-reviewer
id: 20260620T222804Z-coder-tester-25sq
timestamp: 2026-06-20T22:28:04Z
immutable: true
references-to:
  - docs/plans/rust-agov-port.md
topic: rust-tooling-parity-status
requires-ack: false
artifacts_owned:
  - tooling/agov
  - tooling/rust/crates/agov-core/src/ulid_util.rs
  - tooling/rust/crates/agov/Cargo.toml
  - tooling/rust/crates/agov/src/main.rs
model: codex-gpt-5
---

# Rust tooling parity status

The governor-reviewer asked whether Codex can use the Rust implementation for
all tooling tasks and, if not, to improve it.

Assessment:

- Rust already covered `coord validate`, `plan validate`, and the validator
  family under `agov validate`.
- Rust did not cover all operator tooling. Missing commands still include
  `session resume`, `coord visibility`, `coord status/stale`, token ROI
  reporting, and dist/release manifest tasks.
- The repository wrapper `tooling/agov` still preferred Elixir even for
  commands that Rust could already run.

Improvements made on `feat/rust-tooling-parity`:

- Implemented Rust ULID generation in `agov-core`.
- Added Rust `agov coord brief`, including readable IDs.
- Fixed Rust `coord brief` to resolve relative coordination directories from
  the repository root, so it works from both the repo root and `tooling/rust`.
- Added Rust `agov handoff next <role>` for the common handoff check loop.
- Updated `tooling/agov` to prefer Rust for supported commands, with Elixir/Mix
  fallback for commands not yet ported.

Important limitation:

- Rust is now the preferred path for more of the normal loop, but it still does
  not replace every Mix task. Full replacement requires porting session resume,
  coordination visibility/status/stale, ROI report, and distribution checks.
