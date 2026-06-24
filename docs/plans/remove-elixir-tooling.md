# Plan: Remove Elixir Tooling (2.2.4)

**Date:** 2026-06-24T13:35:29Z  
**Target version:** 2.2.4 (patch)  
**Branch:** `chore/remove-elixir-tooling`

## Intent

Remove the Elixir reference implementation and all traces from CI,
docs, and distribution config. The Rust `agov` tooling is the sole
implementation going forward. The Elixir code has been superseded;
keeping it causes CI noise, cache bloat, and documentation confusion
about which implementation is authoritative.

## Affected files

### Delete entirely
- `tooling/elixir/` — the full directory (mix.exs, lib/, test/, deps/,
  _build/, mix.lock, rel/, tmp/, agov/, README.md, erl_crash.dump)

### `.gitlab-ci.yml`
- Remove `.elixir-base` template (lines 48–59)
- Remove `elixir:test` job (lines 200–216)
- Remove `rust:tooling:parity` job (lines 405–438) — exists solely to
  diff Rust vs Elixir output; purpose disappears with Elixir
- Remove `--skip-dirs tooling/elixir/_build` and `tooling/elixir/deps`
  from the `agov-scan` and `cargo-deny` vulnerability scan sections
  (lines ~97–112)
- Remove `tooling/elixir/mix.lock` and `tooling/elixir/mix.exs` from
  the cargo-deny path trigger list (lines ~129–139)
- Update header comment (line 13: `elixir:test — legacy Elixir...`)

### `agf/dist/release-manifest.yaml`
- Remove two exclude entries: `tooling/elixir/_build/**` and
  `tooling/elixir/deps/**` (lines 47–48)

### `agf/dist/README.md`
- Remove the "From `tooling/elixir/`:" block with Mix task instructions

### `agf/docs/contracts/tooling-conformance.md`
- Remove the Elixir reference implementation row from the
  implementations table
- Remove the "current Elixir reference implementation" framing;
  Rust is now the sole implementation

### `agf/docs/site/reference/tooling-conformance.md`
- Remove "Source-checkout contributors may also run development Mix
  tasks" note
- Remove the sentence "The table does not require the adopter to have
  Elixir installed" (already true, but confusing to keep)

### `agf/ADOPTING.md`
- Remove the footnote warning about Mix tasks / Elixir validators not
  yet shipped (lines 8–9 frontmatter note, lines 122–123)
- Remove the "Do not confuse with Elixir tooling package version"
  callout (line 305)
- Remove references to Mix tasks that "ship later" (lines 374, 421)
- Remove the Elixir licensing clause (line 396: "Elixir validation
  tools are MIT licensed")

### `agf/CHANGELOG.md`
- Add `[Unreleased]` entry documenting the removal

### Version bump (on release branch, not this branch)
- `agf/VERSION`: 2.2.3 → 2.2.4
- `tooling/rust/Cargo.toml`: all four version fields 2.2.3 → 2.2.4

## Files that mention Elixir but need only minor or no changes

- `agf/docs/adr/20260505-lean-agile-test-fly.md` — historical ADR,
  leave as-is (documents a decision made at that time)
- `agf/rules/workflow/visible-artifact-verification-rule.md` —
  uses Elixir as a generic example only; leave as-is
- `agf/rules/workflow/migration-generation-rule.md` — Elixir/Ecto
  is one of six migration examples; leave as-is
- `agf/rules/always/active-branch-preflight-rule.md` — check; likely
  incidental reference
- `agf/agents/build-engineer.yaml` — `mix.exs` is listed alongside
  other manifests as an example; leave as-is
- `agf/docs/site/docs/adopt/tooling-and-tokens.md` — review;
  one incidental reference ("mix streams" in a different context)

## Migration needed?

No. This is a removal of `tooling/elixir/` which is not part of the
`agf/` adopter distribution. Adopters never ran Elixir; the Rust
`agov` binary is unchanged. The CI change only removes jobs that
were already path-scoped to `tooling/elixir/**/*` (so they haven't
fired on non-Elixir MRs anyway).

The `rust:tooling:parity` job was manual-only; removing it requires
no migration.

## Test strategy

- `cargo test` in `tooling/rust/` — unchanged; verify green
- `agov dist validate` — must pass after release-manifest.yaml edit
- `agov validate` — must pass (no Elixir references in agf/)
- CI pipeline on the MR — all remaining jobs must pass

## Risks

- `agf/ADOPTING.md` is long (1200+ lines); Elixir references are
  scattered. Do a final `grep -n "elixir\|Elixir\|mix\b\|Mix\b"` after
  edits to catch any missed instances.
- `docs/contracts/tooling-conformance.md` reframes the "reference
  implementation" concept — review that the Rust tooling is described
  as the sole authoritative implementation, not just "the other one".

## Rollback

`git revert` the merge commit. No schema changes, no migration, no
external state to undo.

## Commit plan

Single commit on the feature branch with everything except the version
bump. Version bump goes on the release branch per the release workflow.

---

Awaiting confirmation before implementation.
