---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260622T050749Z-coder-tester-nr2j
timestamp: 2026-06-22T05:07:49Z
model: gpt-5-codex
immutable: true
references-to:
  - 20260622T044037Z-coder-tester-jnr5.md
topic: windows-release-artifacts
requires-ack: false
artifacts_owned:
  - .gitlab-ci.yml
  - agf/docs/site/reference/release-artifacts.md
  - docs/coordination/20260622T050749Z-coder-tester-nr2j.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review Windows x64 release artifact wiring.
    - Confirm whether Windows should be included in the next patch release.
  must_read:
    - .gitlab-ci.yml
    - agf/docs/site/reference/release-artifacts.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 2 handoff: Windows release artifacts

Branch: `ci/windows-release-artifacts`

This MR turns the existing manual Windows preview build into a tag-release
artifact path for `x86_64-pc-windows-gnu`.

## Implemented

- `rust:tooling:release` now builds `x86_64-pc-windows-gnu` on tag pipelines.
- Windows release output is packaged as:
  - `dist/agov-X.Y.Z-x86_64-pc-windows-gnu.zip`
  - `dist/agov-X.Y.Z-x86_64-pc-windows-gnu.zip.sha256`
  - `dist/agov-X.Y.Z-x86_64-pc-windows-gnu-provenance.json`
- `release:create` uploads `dist/agov-*.zip` and adds matching GitLab release
  asset links.
- Release artifact docs now list the Windows zip and explain that Linux uses
  `.tar.gz` while Windows uses `.zip`.

## Scope notes

- The Windows build remains release-only in the default packaging job so normal
  branch/MR preview cost does not grow.
- The existing manual `rust:tooling:build:windows:preview` job remains available
  for ad hoc pre-release checks.
- No version bump or tag is included in this MR.

## Verification

- `cargo run --quiet -p agov -- validate` — pass.
- `mkdocs build -f agf/docs/build/mkdocs.yml --strict` via `uvx` — pass.
- `git diff --check` — pass.

Local note: while freeing disk for Rust validation, generated `tooling/rust/target`,
`agf/site-html`, and root `dist` outputs were removed from old worktrees. A too-broad
cleanup briefly removed tracked `agf/dist/*` in this worktree; those files were
immediately restored before commit. The final diff is limited to CI, docs, and this
coordination handoff.

## Expected CI proof

MR CI should validate the YAML and docs. The full Windows release artifact is only
proven on the next tag pipeline, where `rust:tooling:release` runs with
`CI_COMMIT_TAG` set and the new Windows packaging branch executes.
