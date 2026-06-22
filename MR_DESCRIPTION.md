# feat: add --version flag to agov CLI

## Summary

- Adds `--version` / `-V` flag to the `agov` CLI by enabling clap's built-in
  version attribute on the top-level `Cli` struct.

## Change

One line in `tooling/rust/crates/agov/src/main.rs`:

```diff
-#[command(name = "agov", about = "AGF governance tooling")]
+#[command(name = "agov", about = "AGF governance tooling", version)]
```

`agov --version` now prints `agov 0.1.0` (the Rust workspace tooling version,
independent of the AGF framework version in `agf/VERSION`).

## Verification

- `cargo run --quiet -p agov -- --version` → `agov 0.1.0`
- `cargo run --quiet -p agov -- validate` → pass (expected assistant-drift
  advisory for foundry repo's copied `.claude/rules/` tree)
- `git diff --check` → pass

## Scope note

Single-file change; no plan file required (fast-track waiver: trivial
single-file feature addition, no logic, no tests needed — clap generates
the behaviour from the attribute).

## Gate 2 handoff

`docs/coordination/20260622T101959Z-coder-tester-21b3.md`
