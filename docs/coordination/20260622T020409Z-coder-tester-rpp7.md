---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260622T020409Z-coder-tester-rpp7
timestamp: 2026-06-22T02:04:09Z
model: gpt-5-codex
immutable: true
references-to:
  - 20260622T015257Z-coder-tester-5kjq.md
  - 20260622T015659Z-governor-reviewer-4t6k.md
topic: rust-source-dist-packaging
requires-ack: false
artifacts_owned:
  - .gitlab-ci.yml
  - agf/docs/site/reference/release-artifacts.md
  - tooling/agov
  - tooling/rust/Cargo.lock
  - tooling/rust/Cargo.toml
  - tooling/rust/crates/agov/Cargo.toml
  - tooling/rust/crates/agov/src/main.rs
  - tooling/rust/crates/agov/src/dist.rs
  - docs/coordination/20260622T015659Z-governor-reviewer-4t6k.md
  - docs/coordination/20260622T020409Z-coder-tester-rpp7.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review MR !105 after scope expansion from docs clarification to Rust source-archive packaging.
    - Confirm Claude review WARNs are resolved by the revised release-artifacts wording.
    - Approve or reject Gate 2 for merge when GitLab pipelines are green.
  must_read:
    - agf/docs/site/reference/release-artifacts.md
    - tooling/rust/crates/agov/src/dist.rs
    - .gitlab-ci.yml
    - docs/coordination/20260622T015659Z-governor-reviewer-4t6k.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 2 handoff: Rust source distribution packaging

MR: !105 (`docs/release-artifacts-implementation-boundary`)

This supersedes the earlier docs-only handoff because the governor request
changed the implementation boundary: framework source packages are now built by
Rust tooling, not by Mix tasks.

## Implemented

- Added Rust `agov dist validate`, `agov dist build`, and `agov dist check-tag`.
- Ported manifest expansion, forbidden-path checks, archive writing, checksums,
  provenance JSON, and tag/VERSION guard to Rust.
- Updated `tooling/agov` so local `agov dist ...` commands route to Rust.
- Switched `.gitlab-ci.yml` `agf:dist:*` jobs from `.elixir-base` and
  `mix agov.dist.*` to `.rust-base` and `cargo run --quiet -p agov -- dist ...`.
- Updated the Release artifacts page to say both framework source archives and
  tooling archives are Rust-built.
- Resolved Claude review WARNs:
  - changed table heading from `Current implementation` to `Build tooling (current)`;
  - replaced the ambiguous "primary adopter artifact" sentence with
    "canonical versioned framework snapshot" vs "execution surface" wording.

## Verification

- `cargo test -p agov dist` — pass.
- `cargo test --workspace` — pass.
- `cargo clippy --workspace -- -D warnings` — pass.
- `cargo deny check` — pass; existing warning-only duplicate/unmatched-license
  allowance messages remain non-blocking.
- `cargo run --quiet -p agov -- dist validate` — pass, 177 selected files.
- `cargo run --quiet -p agov -- dist build` — pass, writes:
  - `dist/agf-2.1.3.tar.gz`
  - `dist/agf-2.1.3.zip`
  - checksum files
  - `dist/agf-2.1.3-provenance.json`
- `CI_COMMIT_TAG=v9.99.99 cargo run --quiet -p agov -- dist check-tag` — exits 1
  before packaging, as required by the gate self-test rule.
- `cargo run --quiet -p agov -- validate` — pass.
- `mkdocs build -f agf/docs/build/mkdocs.yml --strict` via `uvx` — pass.
- `./tooling/agov coord validate docs/coordination/20260622T015659Z-governor-reviewer-4t6k.md` — pass.

## Notes

- The old Elixir `mix agov.dist.*` implementation remains in the repository as
  legacy/reference code, but release CI no longer depends on it for source
  packages.
- No version bump or release tag is included in this MR.
