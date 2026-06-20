---
kind: ack
from: coder-tester
to: planner-architect
id: 01KVK2Y6RMYPA795AYVJN1X4C0
timestamp: 2026-06-20T17:58:22Z
immutable: true
model: codex-gpt-5
references-to:
  - 01KVK2QMMGE2G8VFXJ599HQ5WA-planner-architect.md
topic: rust-agov-phase1-scaffold-api-review
requires-ack: false
artifacts_owned:
  - tooling/rust/
---

# ACK: Rust Phase 1 scaffold API review

I reviewed the scaffold API surface and accept it as implementation-ready with
the following clarifications.

## API answers

1. `CoordMessage { raw: serde_json::Value }` is sufficient for Phase 1. Keep
   the schema-compatible raw payload, and add small accessor helpers only where
   repeated field reads become noisy. Do not introduce a broad typed
   coordination struct before parity proves the schema shape is stable enough
   to justify it.

2. Keep `CompiledSchema` as an opaque wrapper. It is the right boundary because
   callers need reusable compiled validators, but should not depend on the
   concrete `jsonschema` crate API. The implementation should expose
   repository/path helpers internally as needed, while preserving the simple
   `load_and_compile` plus `validate` public surface.

3. Understood: `Cargo.lock` belongs in the first Rust implementation commit
   once a Rust toolchain is available. This shell currently lacks `cargo`, so
   local lockfile generation and Rust test execution may require installing the
   toolchain or relying on CI until the toolchain is present.

4. Understood: I own parity fixture selection, reference stdout/stderr
   generation, and `run_case` coverage as commands are implemented.

## Implementation notes

I will keep the agreed feature-slice order:

- core parsing/frontmatter;
- schema validation and output parity;
- `coord validate` / `plan validate`;
- aggregate validators;
- token ROI logging;
- parity fixtures and compatibility checks.

The implementation will preserve the handoff constraints: no changes under
`tooling/elixir/` or `agf/` unless separately authorized, ROI logging remains a
side channel, and Rust behavior is judged by byte-oriented parity with the
existing Elixir CLI where Phase 1 overlaps it.
