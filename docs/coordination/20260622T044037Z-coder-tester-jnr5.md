---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260622T044037Z-coder-tester-jnr5
timestamp: 2026-06-22T04:40:37Z
model: gpt-5-codex
immutable: true
references-to:
  - 20260622T020409Z-coder-tester-rpp7.md
topic: "Gate 2 handoff: release 2.1.4"
requires-ack: false
artifacts_owned:
  - agf/VERSION
  - agf/CHANGELOG.md
  - docs/coordination/20260622T044037Z-coder-tester-jnr5.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review release 2.1.4 version bump and changelog.
    - If approved, merge the release MR, tag the merge commit as v2.1.4, and push the tag.
  must_read:
    - agf/VERSION
    - agf/CHANGELOG.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 2 handoff: release 2.1.4

Branch: `release/v2.1.4`

Patch release for the Rust source-package migration merged in !105.

## Changed

- `agf/VERSION`: `2.1.3` -> `2.1.4`
- `agf/CHANGELOG.md`: added `2.1.4` entry dated 2026-06-22.

## Release theme

AGF framework source archives are now built by Rust `agov dist ...` commands,
matching the Rust tooling release direction and removing Mix-backed source
packaging from the release CI path.

## Verification

- `cargo run --quiet -p agov -- dist validate` — pass; package `agf 2.1.4`, 177 files.
- `cargo run --quiet -p agov -- dist build` — pass; built `dist/agf-2.1.4.tar.gz`,
  `dist/agf-2.1.4.zip`, checksum files, and provenance JSON.
- `cargo run --quiet -p agov -- validate` — pass.
- `CI_COMMIT_TAG=v9.99.99 cargo run --quiet -p agov -- dist check-tag` — exits 1
  before packaging as expected.
- `mkdocs build -f agf/docs/build/mkdocs.yml --strict` via `uvx` — pass.

## Requested Gate 2 action

1. Review the release-only diff.
2. Merge the MR if CI is green.
3. Confirm `agf/VERSION` is `2.1.4` on the merge commit.
4. Create immutable tag `v2.1.4` on the merge commit.
5. Push `v2.1.4` immediately so release CI publishes the artifacts.
