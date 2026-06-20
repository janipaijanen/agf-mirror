# Gate 1 plan: Rust port of agov CLI tooling

## Summary

- Adds `docs/plans/rust-agov-port.md` — architecture and phased scope for porting the `agov` CLI from Elixir/OTP to Rust
- Adds `docs/coordination/01KVJZBMN8FRGPZZYPAQDAW3MF-planner-architect.md` — Gate 1 handoff to governor-reviewer

## What the plan proposes

The current `agov` CLI ships a bundled ERTS runtime (~50–70 MB, Linux x86_64 only). The Rust port produces statically-linked binaries of 5–12 MB for Linux x86_64/ARM64 and macOS x86_64/ARM64, with no runtime dependency on Elixir or OTP.

Key architectural decisions in the plan:

- **Workspace layout**: `tooling/rust/` with four crates — `agov-core`, `agov-coord`, `agov-validators`, `agov` (binary)
- **Concurrency**: `rayon` for parallel file validation; no async runtime needed for a CLI
- **Output compatibility**: byte-for-byte parity with the Elixir CLI output; parity test suite is a blocking CI check
- **Saved-token evidence**: Phase 1 Rust validation commands append local
  JSONL token ROI records compatible with existing Elixir reporting
- **Phasing**: Phase 1 = all validation commands; Phase 2 = channel analysis commands; Elixir tooling untouched throughout
- **Distribution**: `cross`-compiled via CI for four platform targets; archive layout mirrors current Elixir format

## What Gate 1 approval authorises

- Creation of `feat/rust-agov-phase1` branch
- Implementation of `tooling/rust/` workspace covering Phase 1 validation commands
- Local saved-token logging for successful Phase 1 validation commands, using
  `tmp/agov-token-roi.jsonl` by default and `AGOV_TOKEN_ROI_LOG` as override
- Addition of `rust:tooling:test`, `rust:tooling:clippy`, `rust:tooling:build:preview` CI jobs

It does not authorise Phase 2 implementation, bundle rewiring, or removal of the Elixir tooling.

## Advisory consultations

- `infosec-sentinel`: crate supply chain, cross toolchain, static linking, CI secrets — consulted
- `contribution-auditor`: new `tooling/rust/` subtree, workspace layout — consulted
- `build-engineer`: suspended; CI concerns documented in §10 of the plan; reviewed by governor-reviewer at Gate 2
