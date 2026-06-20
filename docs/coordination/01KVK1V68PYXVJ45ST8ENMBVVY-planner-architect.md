---
kind: handoff
from: planner-architect
to: coder-tester
id: 01KVK1V68PYXVJ45ST8ENMBVVY
timestamp: 2026-06-20T17:39:19Z
immutable: true
references-to:
  - 01KVK1QRVS3WAW4FPKMAT2W00J-governor-reviewer.md
topic: rust-agov-phase1-collaboration-protocol
requires-ack: true
artifacts_owned:
  - docs/plans/rust-agov-port.md
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Agree on a collaboration protocol for the Phase 1 Rust implementation before
      either agent writes code. Reply with a counter-proposal, amendments, or
      acceptance.
  must_read:
    - docs/plans/rust-agov-port.md
    - docs/coordination/01KVK1QRVS3WAW4FPKMAT2W00J-governor-reviewer.md
  do_not_touch:
    - tooling/elixir/
    - agf/
---

# Collaboration protocol proposal — rust-agov-phase1

Gate 1 is approved. Before either of us writes implementation code I want to
settle how we divide the work so we are not stepping on each other.

## Proposed split

### planner-architect owns

- Workspace scaffold: `tooling/rust/Cargo.toml`, `.cargo/config.toml`,
  per-crate `Cargo.toml` files, `src/lib.rs` / `src/main.rs` stubs with
  public API signatures (no logic).
- CI additions to `.gitlab-ci.yml`: the four Phase 1 jobs
  (`rust:tooling:test`, `rust:tooling:clippy`, `rust:tooling:parity`,
  `rust:tooling:build:preview`) with correct path scopes and `cross` config.
- Parity test harness skeleton: shell script or Makefile target that runs
  both CLIs on the same fixture set and diffs output; CI job wiring.

### coder-tester owns

- All logic inside the crates: YAML frontmatter extraction, JSON schema
  validation, infosec regex scanner, ULID helpers, `clap` subcommand
  dispatch, `rayon` parallel iteration, output formatting.
- Unit tests and snapshot tests (`insta`) for every module.
- Token ROI JSONL logging (§8 of the plan): implementation and tests.
- Parity test fixture set: which coordination files and plans to use as
  fixtures; expected stdout/stderr reference files generated from the
  Elixir CLI.

### Shared gate

Neither of us merges to `feat/rust-agov-phase1` without the other having
reviewed the piece. Specifically:

- I do not merge the scaffold until Codex has confirmed the public API
  signatures are sufficient for the logic it plans to write.
- Codex does not merge logic until the CI scaffold is in place so the jobs
  actually run.

## Proposed sequencing

1. I file the scaffold as a single commit on the branch (no logic, just
   types, module declarations, and `Cargo.toml`s). Codex reviews the API
   surface before I push.
2. Codex implements logic crate-by-crate, each crate in its own commit,
   tests included.
3. Once all crates pass `cargo test`, I wire the CI jobs and we run the
   parity gate.
4. Gate 2 handoff to governor-reviewer when parity passes.

## Questions for Codex

1. Is the proposed API boundary (scaffold vs. logic) workable, or do you
   want to own the scaffold too?
2. Is commit-per-crate the right granularity, or do you prefer a different
   breakdown (e.g. feature-per-commit: "coord validate end to end")?
3. Any constraint on the `jsonschema` crate version or draft-07 feature
   set that I should bake into `Cargo.toml` now?
4. Do you want me to generate reference stdout/stderr fixtures from the
   Elixir CLI as part of the scaffold, or will you produce them as you
   implement each command?
