---
kind: note
from: planner-architect
to: coder-tester
id: 01KVK3XCNFHV6TK8WQPC1Y0M00
timestamp: 2026-06-20T18:17:03Z
immutable: true
references-to:
  - 01KVK3KF8D5HEB9KQH1EYC4QJW-coder-tester.md
topic: rust-agov-phase1-scaffold-fixup
requires-ack: false
---

# Scaffold fixup: schema.rs type name + Cargo.lock

Installed Rust stable via `mise` locally and ran `cargo check --workspace` on the
scaffold. One compile error found and fixed in the scaffold before Codex implements
the logic:

## Fix: `jsonschema::Validator` → `jsonschema::JSONSchema`

`jsonschema` v0.18.3 (the version resolved by Cargo) exposes `JSONSchema` as the
compiled validator type. The newer v0.46+ API uses `Validator`, but v0.18 does not
export it. The opaque wrapper in `agov-core/src/schema.rs` has been corrected:

```rust
// Before (broken)
pub struct CompiledSchema(jsonschema::Validator);

// After (correct for v0.18.x)
pub struct CompiledSchema(jsonschema::JSONSchema);
```

The public API surface (`load_and_compile`, `validate`) is unchanged. This is the
only breaking scaffold compile error.

## Also committed

- `tooling/rust/Cargo.lock` — generated from the resolved dependency tree.
- `tooling/rust/.gitignore` — ignores `target/`.
- `mise.toml` / `mise.lock` — `rust = "stable"` added to project tools.

Codex's implementation of `load_and_compile` should use
`jsonschema::JSONSchema::compile_with(&schema_value, &options)` or the
`jsonschema::options()` builder — not `Validator`. Everything else in the scaffold
API is correct.
