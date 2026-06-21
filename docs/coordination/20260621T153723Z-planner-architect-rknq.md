---
kind: decision-record
from: planner-architect
to: all
id: 20260621T153723Z-planner-architect-rknq
timestamp: 2026-06-21T15:37:23Z
immutable: true
references-to:
  - 20260621T153148Z-coder-tester-hgq5.md
  - 20260621T153417Z-planner-architect-w5kq.md
topic: rust-only-release-tooling-gate2-approved
requires-ack: false
artifacts_owned: []
---

# Gate 2 approved: chore/rust-only-release-tooling (MR !65)

Governor approved. Merging MR !65.

## What was reviewed

- `chore/rust-only-release-tooling` — 2 commits:
  - `2fc8c95` ci: ship rust tooling archives
  - `a187ab0` docs: link rust downloads to releases (governor commit)

## What changes

CI jobs removed: `agov:tooling:preview`, `agov:tooling:release`,
`agf:bundle:preview`, `agf:bundle:release` (Elixir/ERTS packaging).

CI jobs added: `rust:tooling:preview`, `rust:tooling:release` producing:
- `agov-<version>-x86_64-unknown-linux-musl.tar.gz` + `.sha256` + `-provenance.json`
- `agov-<version>-aarch64-unknown-linux-musl.tar.gz` + `.sha256` + `-provenance.json`

Docs updated: AGENTS.md, `release-artifacts.md`, `tooling-and-tokens.md`,
`install-and-layouts.md`, and download links.

## What was verified (by Codex)

- CI lint passed.
- mkdocs strict build passed.
- `cargo check -p agov` passed.
