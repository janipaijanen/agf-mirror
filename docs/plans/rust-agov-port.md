---
id: a9a5187a-83bc-4c16-9b81-5404053319d9
status: pending-gate1
topic: rust-agov-port
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >
      New binary supply chain: Rust crate registry (crates.io), cross-compilation
      toolchains, static linking strategy, and CI secrets handling for
      multi-platform release uploads all require InfoSec review before
      implementation begins.
  - role: contribution-auditor
    status: consulted
    reason: >
      New tooling/rust/ subtree introduces a new technology surface. Contribution
      auditor must confirm crate selection and workspace layout do not create
      governance or supply-chain drift.
---

# Gate 1 Plan: Rust port of agov CLI tooling

**Date:** 2026-06-20
**Author:** planner-architect

## 1. Intent

Port the `agov` CLI from Elixir/OTP to Rust to eliminate the bundled ERTS
runtime (~50–70 MB), enable true static binaries for Linux and macOS, and
unblock adopters who cannot or do not want to run an Elixir/OTP stack.
Windows support is a future goal deferred beyond Phase 2.

The Rust port must also make its own value measurable. Phase 1 records local,
append-only saved-token evidence for successful Rust validation commands so the
project can justify Rust usage with data instead of anecdote.

The Elixir tooling remains in place during the transition. Both tooling
implementations coexist under `tooling/elixir/` and `tooling/rust/`
respectively until the Rust port reaches Phase 2 feature parity, at which
point the bundle CI jobs are rewired to use the Rust binary.

This plan does not cover:
- Removing the Elixir tooling (a separate future decision)
- Porting the Mix release/bundle/dist tasks (packaging stays Elixir for now)
- Registry publishing, code signing, or SBOMs (out of scope for this slice)

## 2. Motivation

| Problem | Current state | After port |
|---------|--------------|------------|
| Binary size | ~50–70 MB (ERTS bundled) | ~5–12 MB (static) |
| Platform support | Linux x86_64 only | Linux x86_64+ARM64, macOS x86_64+ARM64 (Windows: future) |
| Runtime dependency | Elixir + OTP required on dev machine for source build | `cargo` only; binary adopters need nothing |
| Parallel validation | OTP processes (startup cost per file) | `rayon` data parallelism, zero per-task overhead |
| Installation | Mix release unpack + PATH config | Single binary, drop anywhere |
| Token ROI evidence | Existing Elixir-only local JSONL telemetry | Rust validation commands append compatible saved-token records |

## 3. Workspace layout

```
tooling/rust/
├── Cargo.toml            # workspace manifest
├── Cargo.lock            # committed
├── .cargo/
│   └── config.toml       # target aliases, musl linker config
├── crates/
│   ├── agov-core/        # types, YAML/JSON parsing, JSON-schema validation
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── yaml.rs          # YAML frontmatter extraction + parse
│   │       ├── schema.rs        # JSON schema loading + validation
│   │       ├── infosec.rs       # regex pattern scanner
│   │       └── ulid.rs          # ULID generation helpers
│   ├── agov-coord/       # coordination channel domain
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── coord_validator.rs
│   │       ├── plan_validator.rs
│   │       ├── channel_reader.rs
│   │       ├── handoff_tracker.rs
│   │       ├── claim_tracker.rs
│   │       ├── claim_fsm.rs
│   │       ├── coord_identity.rs
│   │       └── coord_config.rs
│   ├── agov-validators/  # adoption, agents, learnings, infosec, contributions
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── adoption.rs
│   │       ├── agents.rs
│   │       ├── learnings.rs
│   │       ├── infosec.rs
│   │       ├── contributions.rs
│   │       └── framework_config.rs
│   └── agov/             # binary crate: CLI entry point
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
```

## 4. Key crate dependencies

| Function | Elixir equivalent | Rust crate | Notes |
|----------|-------------------|------------|-------|
| YAML parsing | yaml_elixir | `serde_yml` | maintained community fork of `serde_yaml`; drop-in API compatible |
| JSON parsing | Jason | `serde_json` | Industry standard |
| JSON schema validation | ex_json_schema | `jsonschema` | draft-07 support matches current schemas |
| ULID | ulid | `ulid` | Same semantics |
| Regex | Erlang re (PCRE) | `regex` | RE2 syntax; current patterns are RE2-compatible |
| CLI framework | mix tasks + escript | `clap` | derive macro for subcommand dispatch |
| Parallel iteration | OTP processes | `rayon` | Data parallelism for ValidateAll file lists |
| Git operations | System.cmd("git", ...) | `std::process::Command` | Shell out; avoids `git2` C dependency |
| Snapshot tests | — | `insta` | CLI output regression testing |

No `tokio` / async runtime. Validation is CPU+I/O bound per file with no
shared mutable state between files; `rayon` covers this without the overhead
of an async executor in a CLI context.

## 5. Concurrency model

### 5.1 File validation (hot path)

```
agov coord validate file1.md file2.md ... fileN.md
```

```rust
files.par_iter()
    .map(|path| validate_coord_file(path))
    .collect::<Vec<_>>()
```

Each file is independent. `rayon` distributes them across worker threads
automatically. No locks, no channels — each validation returns a
`Result<ValidCoordMessage, Vec<Finding>>`.

### 5.2 Channel reader + handoff tracker

Sequential in-memory graph walk after parallel file reads:

1. `rayon::par_iter()` reads and parses all `.md` files in `docs/coordination/`
2. Single-threaded graph construction: `HashMap<filename, MessageNode>`
3. Single-threaded reachability walk: DFS from each handoff to find resolvers

The graph phase is O(N²) in the worst case but N is bounded by the channel
size (typically < 500 messages). No parallelism needed.

### 5.3 Validate umbrella command

```
agov validate
```

Runs all sub-validators in parallel:

```rust
rayon::join(
    || validate_adoption(root),
    || rayon::join(
        || validate_agents(agents_dir),
        || rayon::join(
            || validate_learnings(learnings_dir),
            || rayon::join(
                || validate_infosec(target_dir),
                || validate_contributions(contributions_dir),
            ),
        ),
    ),
)
```

Each sub-validator is independent. This mirrors what the Elixir umbrella
runs sequentially today but exploits available cores.

## 6. Output format compatibility

The Rust binary MUST produce identical output line-for-line to the Elixir
binary for all Phase 1 and Phase 2 commands. This enables:

- Parity testing: run both on the same fixture set, `diff` the output
- CI gate: parity test is a blocking CI check during the transition period
- Adopter confidence: existing scripts that parse `agov` output continue to work

**Ordering and stream requirements (applies even under parallel execution):**

- File-level results MUST appear in the same order as the Elixir CLI regardless
  of which rayon thread completes first. The implementation collects parallel
  results and sorts them by input argument order before writing to stdout/stderr.
- `OK` lines go to stdout; `ERR` lines go to stderr — same as Elixir.
- Exit code semantics are identical: 0 if all files pass, 1 if any file fails.
- Parity tests run with a fixed multi-file argument list and verify both
  stdout and stderr byte-for-byte against the Elixir reference output.

Output format is owned by `agov-core`. Any format change requires a
coordination message noting the deviation and a CHANGELOG entry.

## 7. Phased scope

### Phase 1 — Validation MVP

Commands:
- `agov coord validate <file>...`
- `agov plan validate <file>...`
- `agov validate` (umbrella)
- `agov validate adoption`
- `agov validate agents`
- `agov validate learnings`
- `agov validate infosec`
- `agov validate contributions`
- `agov validate framework-config`

Gate: parity test suite passes for all Phase 1 commands on the AGF fixture set.

Successful Phase 1 validation commands also record local token ROI evidence as
defined in section 8. Logging is part of Phase 1 because validation commands
are the first Rust surface whose deterministic output can replace repeated AI
context reads.

### Phase 2 — Channel analysis

Commands:
- `agov coord status`
- `agov coord stale`
- `agov handoff next`
- `agov session resume [--role <role>] [--base-branch <branch>]`

Gate: parity test suite passes for all Phase 2 commands.

### Phase 3 — Generation and reporting (future, not planned here)

- `agov coord brief`
- `agov coord visibility`
- `agov token roi`
- `agov usage report`

### Out of scope (stays in Elixir)

- `agov dist build` / `agov tooling build` / `agov bundle build`
- Mix release packaging tasks

## 8. Saved-token evidence log

Phase 1 Rust validation commands append local token ROI records compatible with
the existing Elixir `Agov.TokenRoi` JSONL format.

### 8.1 Path and switches

- Default path: repository-root `tmp/agov-token-roi.jsonl`.
- Override: `AGOV_TOKEN_ROI_LOG=<path>`.
- Disable: `AGOV_TOKEN_ROI=0`, `AGOV_TOKEN_ROI=false`, or
  `AGOV_TOKEN_ROI=off`.
- Format: append-only JSONL, one record per successful Rust command run.
- Consumer: existing Elixir `mix agov.token.roi report`; no Rust-native
  `agov token roi report` command is authorized in Phase 1.

The log remains local and gitignored through the repository's existing `tmp/`
ignore rule. It must not store prompts, command output bodies, model responses,
secrets, billing data, or remote telemetry.

### 8.2 Record shape

Each Rust-generated record must include at minimum:

- `event: "tool_output"`;
- `timestamp`;
- `implementation: "rust-agov"`;
- `command`;
- `args`;
- `workflow`;
- `chars`;
- `approx_tokens`;
- `prompt_overhead_tokens`;
- `baseline_source` when a workflow baseline is known;
- `approx_saved_min_tokens` and `approx_saved_max_tokens` when a workflow
  baseline is known.

Token estimates use the existing approximation:

```text
approx_tokens = ceil(character_count / 4)
```

Savings estimates use the documented manual-baseline ranges in
`docs/reference/tooling-token-roi.md`. Unknown workflows may record measured
usage without inventing saved-token fields.

### 8.3 Phase 1 workflow labels

Rust Phase 1 commands use these workflow labels:

| Command | Workflow label |
|---|---|
| `agov coord validate <file>...` | `coord-validate` |
| `agov plan validate <file>...` | `plan-validate` |
| `agov validate` | `validate:umbrella` |
| `agov validate <subcommand>` | `validate:<subcommand>` |

### 8.4 Compatibility requirements

Saved-token logging must not alter command stdout, stderr, output ordering, or
exit codes. Logging failure is non-fatal by default. Strict telemetry failure
handling is not part of this plan unless a future Gate 1 decision explicitly
adds it.

## 9. Distribution strategy

Phase 1 CI preview builds the Linux platform targets using `cross`:

| Target triple | Archive name | Notes |
|---------------|-------------|-------|
| `x86_64-unknown-linux-musl` | `agov-<version>-linux-x86_64.tar.gz` | fully static |
| `aarch64-unknown-linux-musl` | `agov-<version>-linux-aarch64.tar.gz` | fully static |

macOS (`x86_64-apple-darwin`, `aarch64-apple-darwin`) is deferred until a
separate decision covers the required macOS SDK/custom-image supply chain.
Windows (`x86_64-pc-windows-gnu`) remains deferred to Phase 3.

Archive layout (mirrors current Elixir tooling format):
```
agov-<version>-<platform>/
├── bin/
│   └── agov
└── agov-<version>-<platform>.sha256
```

Bundle job (`agf:bundle:release`) is rewired after Phase 2 completion to
pick up Rust binaries instead of the Elixir Mix release.

## 10. CI pipeline changes

New jobs added to `.gitlab-ci.yml` — Phase 1 only:

```
rust:tooling:test       # cargo test --workspace, every MR touching tooling/rust/
rust:tooling:clippy     # cargo clippy --workspace -- -D warnings
rust:tooling:parity     # parity test suite against AGF fixture set, blocking
rust:tooling:build:preview  # cross build for Linux preview targets, MR + default branch
```

`rust:tooling:build:release` (tag-triggered archive production) is **not
authorized in Phase 1**. It requires a separate explicit decision — either
as part of Phase 2 Gate 2 approval or a standalone coordination entry —
before it is added to the pipeline.

Path scoping per `ci-trigger-scoping-rule.md`:
- `rust:tooling:test`, `rust:tooling:clippy`, `rust:tooling:parity` trigger on `tooling/rust/**`
- `rust:tooling:build:preview` triggers on `tooling/rust/**` + MR + default branch

The Elixir CI jobs (`agov:tooling:*`) remain unchanged until Phase 2 gate passes.

## 11. Migration path

1. `tooling/rust/` subtree created on `feat/rust-agov-phase1` branch.
2. Phase 1 implemented; parity and saved-token evidence tests written and
   passing.
3. Gate 2 review: governor-reviewer owns the review and covers build-risk
   (CI pipeline changes, cross toolchain, archive layout). infosec-sentinel
   and contribution-auditor consulted as advisors. build-engineer is suspended
   and is not a required reviewer.
4. Merge Phase 1. Elixir tooling unchanged.
5. Phase 2 implemented on `feat/rust-agov-phase2`.
6. Gate 2: same reviewer structure as Phase 1.
7. Merge Phase 2. Bundle CI rewired. Elixir tooling enters maintenance mode.
8. Separate future decision: deprecate / remove Elixir tooling.

## 12. Affected files

**New:**
- `tooling/rust/` subtree (all files listed in §3)
- `docs/plans/rust-agov-port.md` (this file)

**Modified:**
- `.gitlab-ci.yml` — new rust build/test jobs (Phase 1+2 only; bundle rewiring deferred to Phase 2 completion)
- `agf/VERSION` — unchanged by this plan; version bump follows normal release workflow

**Unchanged:**
- `tooling/elixir/` — untouched throughout Phase 1 and Phase 2
- `agf/` framework sources — no changes

## 13. Risks

| Risk | Likelihood | Mitigation |
|------|-----------|------------|
| `serde_yml` diverges from yaml_elixir on edge cases | Low | Parity test suite catches divergence per file |
| `jsonschema` crate lacks draft-07 feature parity | Low | Evaluate against all AGF schemas before committing; crate supports draft-07 |
| `cross` Docker images unavailable in CI | Medium | Pin image digests; keep Elixir jobs active as fallback |
| Output format drift breaks adopter scripts | Low | Parity tests are blocking; format is tested per command |
| Saved-token logging changes command output or failure behavior | Low | Logging is side-channel JSONL only; parity tests run with logging enabled and disabled |
| Rust build time slows CI | Medium | `sccache` + CI caching; `cargo-chef` for Docker layer caching |

## 14. Test strategy

- **Unit tests**: per crate, `cargo test`
- **Parity tests**: run Elixir and Rust CLI on the same AGF fixture set, `diff` output; blocking CI check
- **Snapshot tests**: `insta` for CLI output stability across refactors
- **Integration tests**: run `agov validate` against the live `agf/` directory in CI
- **Token ROI tests**: cover token estimate math, JSONL serialization,
  `AGOV_TOKEN_ROI_LOG`, disable switch, default path resolution, compatible
  Elixir `Agov.TokenRoi.load/summarize` consumption, successful validation
  append behavior, invalid validation non-recording, and stdout/stderr parity
  with logging enabled

Coverage floor: no existing Elixir test scenario may be left without a
corresponding Rust test.
