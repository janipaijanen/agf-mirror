---
id: rust-agov-complete
status: pending-gate1
topic: rust-agov-complete
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >
      This plan changes the release supply chain by making Rust binaries the
      packaged tooling output, adding Windows x64 artifacts, and removing
      Elixir-built tooling/package jobs from CI. The target matrix, archive
      contents, dependency policy, and release upload behavior need explicit
      security review.
  - role: contribution-auditor
    status: consulted
    reason: >
      This plan changes the contributor and adopter workflow for AGF tooling:
      Rust becomes the primary implementation, Elixir becomes legacy/reference
      material, and release archives change shape. Contribution review is
      needed to ensure parity, migration documentation, and CI scope are clear.
---

# Gate 1 Plan: Rust agov Completion, Packaging, and Windows x64

**Date:** 2026-06-21
**Author:** Codex / coder-tester draft for planner-architect and
governor-reviewer review

## 1. Intent

Finish the Rust `agov` implementation so framework development can rely on it
as the primary tooling path, then package Rust binaries directly for release,
including Windows 11 x64.

This plan supersedes the Phase 1-only scope of `docs/plans/rust-agov-port.md`.
The earlier Gate 1 decision (`01KVK1QRVS3WAW4FPKMAT2W00J`) explicitly did not
authorize Phase 2, release packaging, Windows, bundle rewiring, or removal of
Elixir tooling from CI. This plan requests that missing authorization.

## 2. User Request Being Addressed

The governor-reviewer asked Codex to proceed with the Rust implementation,
finalize it together with Claude so it is feature-complete with the Elixir
version, package it, build Windows binaries, and for CI drop Elixir builds and
packaging.

Collaboration with Claude is handled through coordination messages and Gate 1 /
Gate 2 handoffs. No implementation may be treated as complete until a cold
starting planner-architect or governor-reviewer can reconstruct the status from
`docs/coordination/`.

## 3. Scope

### 3.1 Rust CLI Feature Parity

Rust `agov` must support every adopter/developer command currently exposed by
the Elixir Mix tasks or by `tooling/agov`:

| Area | Rust command surface required |
|---|---|
| Coordination validation | `agov coord validate <file>...` |
| Coordination generation | `agov coord brief --id-format ulid|readable ...` |
| Coordination queues | `agov coord status`, `agov coord stale`, `agov coord visibility` |
| Handoffs | `agov handoff next <role>` |
| Session orientation | `agov session resume [--role <role>] [--base-branch <branch>]` |
| Plans | `agov plan validate <file>...` |
| Validators | `agov validate`, all current validator subcommands |
| Token ROI | `agov token roi report` and compatible JSONL parsing |
| Usage reporting | `agov usage report` |
| Distribution | `agov dist validate`, `agov dist build`, `agov dist check-tag` |
| Tooling package | `agov tooling build` |
| Bundle package | `agov bundle build` |

The Rust command output must remain line-compatible with the current Elixir
output unless a difference is explicitly documented in coordination and covered
by updated tests.

### 3.2 Rust-Owned Packaging

Rust replaces Elixir for CI package production:

- framework archives: `agf-<version>.tar.gz`, `agf-<version>.zip`;
- Rust tooling archives per platform;
- bundle archives combining framework + Rust tooling;
- SHA256 files, `SHA256SUMS`, and provenance JSON.

The release archive layout must be documented and stable. Release uploads must
not depend on Mix tasks.

### 3.3 Platform Matrix

Preview and release tooling artifacts target:

| Platform | Target triple | Archive |
|---|---|---|
| Linux x64 | `x86_64-unknown-linux-musl` | `.tar.gz` |
| Linux ARM64 | `aarch64-unknown-linux-musl` | `.tar.gz` |
| Windows 11 x64 | `x86_64-pc-windows-gnu` or `x86_64-pc-windows-msvc` | `.zip` |

Target choice for Windows must be justified in the implementation MR. A Linux
cross-build using `x86_64-pc-windows-gnu` is acceptable for preview artifacts
if CI can produce and smoke-test `agov.exe --help`. A release-grade Windows
artifact must also run smoke checks for `coord validate`, `plan validate`, and
`validate` on a Windows runner or an equivalent documented execution path.

macOS remains deferred unless a separate decision authorizes macOS SDK or
runner supply-chain handling.

### 3.4 CI Rewire

CI should stop running Elixir build and packaging jobs after Rust parity gates
are in place.

Remove or disable these Elixir-owned package/build jobs from normal MR,
branch, and tag pipelines:

- `elixir:test`
- `agf:dist:validate`
- `agf:dist:preview`
- `agf:dist:release`
- `agov:tooling:preview`
- `agov:tooling:release`
- `agf:bundle:preview`
- `agf:bundle:release`

Replace them with Rust-owned equivalents:

- `rust:tooling:test`
- `rust:tooling:clippy`
- `rust:tooling:deny`
- `rust:tooling:parity` during transition
- `rust:agf:dist:validate`
- `rust:agf:dist:preview`
- `rust:agf:dist:release`
- `rust:tooling:build:preview`
- `rust:tooling:build:release`
- `rust:agf:bundle:preview`
- `rust:agf:bundle:release`

`release:create` must upload Rust-produced artifact names, including Windows
zip artifacts. It must not assume an OTP suffix.

### 3.5 Elixir Retention

This plan does not delete `tooling/elixir/`. Elixir remains in the repository
as legacy/reference code until a later removal plan is approved. The immediate
goal is to remove Elixir from CI build/package responsibility and from the
normal `tooling/agov` execution path.

## 4. Rollout Order

1. **Plan and coordination**: this plan, plus a planner/governor handoff.
2. **Parity inventory**: machine-readable checklist mapping every Elixir task
   to a Rust command, test, and parity status.
3. **Rust feature parity**: implement missing command surfaces in Rust.
4. **Rust packaging primitives**: implement dist/tooling/bundle build commands.
5. **Windows preview artifact**: produce `agov.exe` zip with checksum and smoke
   `--help`.
6. **CI rewire**: add Rust package jobs, then drop Elixir build/package jobs.
7. **Release path**: update `release:create` artifact upload/link logic.
8. **Gate 2 handoff**: include validation, CI evidence, Windows artifact
   evidence, and remaining limitations.

## 5. Validation Requirements

Minimum local validation before Gate 2:

- `cd tooling/rust && cargo fmt --check`
- `cd tooling/rust && cargo test --workspace`
- `cd tooling/rust && cargo clippy --workspace -- -D warnings`
- `cd tooling/rust && cargo deny check`
- `./tooling/agov validate`
- `./tooling/agov coord validate docs/coordination/<handoff>.md`
- parity script covering all commands that still have an Elixir reference
- package smoke for framework, tooling, and bundle archives
- Windows x64 artifact build plus `agov.exe --help` smoke

Minimum CI validation before Gate 2:

- Rust test, clippy, deny, and parity jobs green;
- Rust dist/tooling/bundle preview jobs green;
- Windows x64 preview artifact present and downloadable;
- no Elixir build/package job required for a successful MR pipeline.

## 6. Risks and Mitigations

| Risk | Mitigation |
|---|---|
| Rust output diverges from Elixir and breaks scripts | Keep parity tests blocking until the command is intentionally changed. |
| Release artifact names drift | Add tests for expected archive names and `release:create` links. |
| Windows cross-compiled binary is not actually runnable on Windows | Require Windows runner smoke before declaring release-grade Windows support. |
| Dropping Elixir CI hides regressions while Rust is incomplete | Drop Elixir jobs only after parity checklist is complete and Rust package jobs pass. |
| Cargo dependency or cross image supply-chain issue | Keep `cargo deny`, pin/record target strategy, and document any non-crates.io tooling. |
| Coordination queue remains stale during multi-agent work | Write handoff/completion notes for each implementation slice. |

## 7. Out of Scope

- Deleting `tooling/elixir/`.
- macOS artifacts.
- Code signing.
- SBOM generation beyond existing provenance files.
- Publishing a tagged release without normal release governance.
- MCP/server support. That remains a separate framework/tooling design topic.

## 8. Requested Gate 1 Decision

Approve implementation on `feat/rust-agov-complete` with Codex as
coder-tester, Claude/planner-architect as coordinating reviewer, and
governor-reviewer owning Gate 1 and Gate 2 decisions.

The implementation branch may create automatic MRs. Gate 2 merge remains a
governor-reviewer decision recorded in coordination.
