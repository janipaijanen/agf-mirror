# feat(rust): Phase 1 Rust agov CLI port

## Summary

Implements the approved Phase 1 Rust `agov` CLI port.

This MR adds a Rust workspace under `tooling/rust/` for Phase 1 validation
commands, keeps Elixir as the reference implementation, and wires CI preview,
test, clippy, dependency-policy, parity, and build jobs for the Rust tooling
lane.

## Scope

- Adds the Rust workspace:
  - `agov-core`
  - `agov-coord`
  - `agov-validators`
  - `agov`
- Implements:
  - `agov coord validate <file>...`
  - `agov plan validate <file>...`
  - `agov validate`
  - `agov validate agents`
  - `agov validate learnings`
  - `agov validate adoption`
  - `agov validate infosec`
  - `agov validate contributions`
  - `agov validate framework-config`
- Preserves dotted compatibility aliases:
  - `agov agov.coord.validate <file>...`
  - `agov agov.plan.validate <file>...`
- Adds local saved-token ROI logging for successful Rust validation commands:
  - default: `tmp/agov-token-roi.jsonl`
  - override: `AGOV_TOKEN_ROI_LOG`
  - disable: `AGOV_TOKEN_ROI=0|false|off`
- Adds live Rust-vs-Elixir parity cases in `tooling/rust/tests/parity/run.sh`.
- Adds Rust CI jobs:
  - `rust:tooling:test`
  - `rust:tooling:deny`
  - `rust:tooling:clippy`
  - `rust:tooling:parity`
  - `rust:tooling:build:preview` for Linux musl preview targets:
    - `x86_64-unknown-linux-musl`
    - `aarch64-unknown-linux-musl`
- Adds a CI workflow guard so `coordination:` audit-trail commits do not
  retrigger the full MR build matrix.
- Adds dependency policy via `tooling/rust/deny.toml`.
- Uses `serde_norway` instead of `serde_yml` after the RustSec advisory.
- Updates token ROI reference docs with validation workflow baselines.

## Governance Trail

- Gate 1 approval:
  `docs/coordination/01KVK1QRVS3WAW4FPKMAT2W00J-governor-reviewer.md`
- Collaboration protocol:
  `docs/coordination/01KVK1V68PYXVJ45ST8ENMBVVY-planner-architect.md`
- Codex protocol ack:
  `docs/coordination/01KVK1YT6KZNVN0WVFNV6728JT-coder-tester.md`
- Scaffold API review request:
  `docs/coordination/01KVK2QMMGE2G8VFXJ599HQ5WA-planner-architect.md`
- Codex API review ack:
  `docs/coordination/01KVK2Y6RMYPA795AYVJN1X4C0-coder-tester.md`
- Implementation cleared:
  `docs/coordination/01KVK3049XQPM1GE8WBJ5N2D00-planner-architect.md`
- Implementation start:
  `docs/coordination/01KVK3KF8D5HEB9KQH1EYC4QJW-coder-tester.md`
- Scaffold/dependency notes:
  `docs/coordination/01KVK3XCNFHV6TK8WQPC1Y0M00-planner-architect.md`
  `docs/coordination/01KVK4D2PH56CX8MXY3J7MNVR0-planner-architect.md`
- Gate 2 handoff and final CI evidence:
  `docs/coordination/01KVK58XFZ91W97F11EZE9BCEW-coder-tester.md`
  `docs/coordination/01KVKB0BS8GERB05Q5RVNX06P3-coder-tester.md`

## Verification

- `cargo test --workspace` passed.
- `cargo clippy --workspace -- -D warnings` passed.
- `bash tooling/rust/tests/parity/run.sh` passed: 13 passed, 0 failed.
- `agov plan validate docs/plans/rust-agov-port.md` passed.
- New coordination messages validated.
- Rust ROI JSONL smoke test passed.
- Elixir `mix agov.token.roi report` read a Rust-generated JSONL record.
- Logging-enabled vs logging-disabled stdout/stderr parity check passed.
- `git diff --check` passed.
- GitLab MR pipeline `2616471585` passed after the final Rust preview and
  cache-safety fixes, including `rust:tooling:build:preview`,
  `rust:tooling:deny`, `rust:tooling:test`, `rust:tooling:clippy`, and
  `rust:tooling:parity`.
- GitLab CI lint passed for the `coordination:` workflow skip guard.

## Review Notes

- `cargo deny check` was not run locally because `cargo-deny` is not installed
  in this shell. The CI job installs `cargo-deny` before running the check, and
  the MR pipeline passed the dependency policy job.
- Generated Rust `target/` output and local token ROI logs are not committed.
- Elixir tooling remains the reference implementation and is not removed.
- macOS preview builds are deferred because upstream `cross` does not provide
  Darwin images; those targets require a separately approved macOS SDK/custom
  image supply-chain path.
- The `coordination:` workflow skip is intentionally commit-title based. It
  preserves full CI for code, docs, and CI changes while avoiding repeated full
  MR rebuilds for plain governance coordination updates.

## Out of Scope

- Phase 2 channel analysis commands.
- Rust-native `agov token roi report`.
- Release build CI (`rust:tooling:build:release`).
- Bundle rewiring or Elixir deprecation.
- macOS/Windows binary or installer work beyond the Linux preview build lane.
