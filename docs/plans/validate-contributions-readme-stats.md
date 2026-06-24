# Plan: Expand contributions validator + add readme_stats validator

**Timestamp:** 2026-06-24T19:49:22Z
**Branch:** feat/validate-contributions-readme-stats

## Intent

Port two Python v1.x validators that have no Rust equivalent:

1. **Adversarial contribution detection** — expand the stub `contributions.rs`
   to match the full Python pattern set (absence bias, governance relaxation,
   evidence quality / universal claims).

2. **README stats consistency check** — new `readme_stats.rs` validator that
   counts `agf/rules/**/*.md`, `agf/agents/*.yaml`, `agf/skills/*.md`, and
   `agf/learnings/*.yaml` and verifies the numbers match claims in `README.md`.
   Adds a stats line to `README.md` so the validator has something to check.

## Affected files

| File | Change |
|---|---|
| `tooling/rust/crates/agov-validators/src/contributions.rs` | Expand patterns; add universal-claims check |
| `tooling/rust/crates/agov-validators/src/lib.rs` | Extend `freetext()` target keys to include `notes`, `title`, `customization_notes`, `conclusion` |
| `tooling/rust/crates/agov-validators/src/readme_stats.rs` | New — README stats validator |
| `tooling/rust/crates/agov-validators/src/lib.rs` | Add `pub mod readme_stats` |
| `tooling/rust/crates/agov/src/main.rs` | Add `ReadmeStats` variant to `ValidateCommand`; wire into `run_validate` and `run_all_validators` |
| `README.md` | Add stats line with rule/agent/skill counts |

## Migration needed

No. New advisory-only validators; no existing contract surface changed.

## Test strategy

- Unit tests in `contributions.rs`: one positive (pattern fires) and one
  negative (clean text) per check type.
- Unit tests in `readme_stats.rs`: temp directory with known file counts and
  matching vs mismatched README claims; verify advisory vs ok output.

## Risks

- `freetext()` helper is shared by other validators. Extending its target keys
  (`notes`, `title`, `customization_notes`, `conclusion`) may surface new
  findings in `infosec.rs` or `learnings.rs` — must run full test suite.
- README stats regex patterns are coupled to README wording; if the README
  changes the phrasing, the validator warns. This is the intended behavior.

## Rollback

Remove `readme_stats.rs`, revert `contributions.rs` to stub, remove wiring.
No schema or coordination channel changes; rollback is a code-only revert.
