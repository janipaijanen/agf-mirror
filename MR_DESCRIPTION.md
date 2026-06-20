# feat(rust): Phase 1 Rust agov CLI port

## Status

In progress. Scaffold merged; Codex API review pending before logic implementation begins.

## What this MR adds

### Workspace scaffold (planner-architect)

- `tooling/rust/` — 4-crate Cargo workspace: `agov-core`, `agov-coord`, `agov-validators`, `agov`
- Public API surface for all Phase 1 validation commands (all logic stubs use `todo!()`)
- CI jobs: `rust:tooling:test`, `rust:tooling:clippy`, `rust:tooling:parity`, `rust:tooling:build:preview`
- Parity test harness skeleton: `tooling/rust/tests/parity/run.sh`
- `.cargo/config.toml`: musl linker config for Linux cross-compilation targets

### Phase 1 implementation (coder-tester — in progress)

Feature slices to follow per agreed sequencing:
1. Core parsing and frontmatter extraction
2. Schema validation and output formatting
3. Coordination and plan validation end-to-end
4. Aggregate validators
5. Token ROI JSONL logging
6. Parity fixtures and final compatibility fixes

### Governance trail

- Gate 1 approval: `docs/coordination/01KVK1QRVS3WAW4FPKMAT2W00J-governor-reviewer.md`
- Collaboration protocol: `docs/coordination/01KVK1V68PYXVJ45ST8ENMBVVY-planner-architect.md`
- Codex protocol ack: `docs/coordination/01KVK1YT6KZNVN0WVFNV6728JT-coder-tester.md`
- Scaffold API review request: `docs/coordination/01KVK2QMMGE2G8VFXJ599HQ5WA-planner-architect.md`

## What is NOT in scope

- `rust:tooling:build:release` CI job (requires separate Gate 2 decision)
- Phase 2 channel analysis commands (`coord status`, `session resume`, etc.)
- Bundle rewiring or removal of Elixir tooling
