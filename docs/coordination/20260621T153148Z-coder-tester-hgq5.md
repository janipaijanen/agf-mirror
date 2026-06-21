---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260621T153148Z-coder-tester-hgq5
timestamp: 2026-06-21T15:31:48Z
immutable: true
references-to: []
topic: rust-only-release-tooling
requires-ack: false
artifacts_owned: []
model: codex-gpt-5
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the CI/release switch from Elixir/ERTS tooling packages to Rust platform-specific tooling archives.
    - Confirm that framework source archives may still be built by the current Mix dist helpers until those helpers are ported.
  must_read:
    - .gitlab-ci.yml
    - AGENTS.md
    - agf/docs/site/reference/release-artifacts.md
    - agf/docs/site/docs/adopt/tooling-and-tokens.md
    - agf/docs/site/docs/adopt/install-and-layouts.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 2 handoff: Rust-only release tooling

## What changed

The GitLab release tooling lane no longer builds or publishes the Elixir/ERTS
`agov` tooling package.

Removed from CI:

- `agov:tooling:preview`
- `agov:tooling:release`
- `agf:bundle:preview`
- `agf:bundle:release`
- release-page lookup/linking for `agov-*-linux-x86_64-otp-*.tar.gz`
- release-page linking for `agf-*-bundle.tar.gz`

Added in CI:

- `rust:tooling:preview`
- `rust:tooling:release`

The Rust jobs build platform-specific archives:

- `agov-<version>-x86_64-unknown-linux-musl.tar.gz`
- `agov-<version>-aarch64-unknown-linux-musl.tar.gz`

Each archive contains the Rust `agov` executable and a short README, plus a
`.tar.gz.sha256` checksum and `-provenance.json` file. The release job uploads
those Rust artifacts and dynamically adds a release-page link for each Rust
`agov-<version>-*.tar.gz` it receives.

## Documentation updates

- `AGENTS.md` now tells agents not to use Elixir/ERTS package helpers for
  GitLab CI or release-page downloads.
- `release-artifacts.md` lists Rust platform artifacts instead of OTP/ERTS
  archives and removes the bundle artifact claims.
- `tooling-and-tokens.md` tells framework-only adopters to download the Rust
  archive for their platform or build Rust locally.
- `install-and-layouts.md` points users at optional Rust tooling archives, not
  bundles.

## Scope note

Framework source archives are still built by `mix agov.dist.*` in this branch.
This change removes Elixir/ERTS `agov` tooling packages from CI/release
shipping; it does not port the framework source archive builder itself.

## Validation

- GitLab CI lint: `valid: true`, no errors or warnings.
- `uv run --project agents-govern --group docs mkdocs build -f agf/docs/build/mkdocs.yml --strict`: passed.
- `cargo check -p agov`: passed.
- `git diff --check`: passed.

Environment note: fresh worktree shells print the local `mise.toml` trust
warning. Commands still completed.

## Review focus

- Confirm the shipped tooling artifact set is Rust-only and platform-specific.
- Confirm removing the convenience bundle is acceptable because it previously
  depended on the Elixir tooling package path.
- Confirm the release page should link framework source archives plus Rust
  `agov` archives only.
