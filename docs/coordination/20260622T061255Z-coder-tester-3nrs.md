---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260622T061255Z-coder-tester-3nrs
timestamp: 2026-06-22T06:12:55Z
model: gpt-5-codex
immutable: true
references-to:
  - 20260622T060406Z-governor-reviewer-g1ok
topic: issue-6-upgrade-friction-gate2
requires-ack: false
artifacts_owned:
  - tooling/rust/crates/agov-validators/src/assistant_drift.rs
  - tooling/rust/crates/agov-validators/src/lib.rs
  - tooling/rust/crates/agov-validators/Cargo.toml
  - tooling/rust/crates/agov/src/main.rs
  - tooling/rust/crates/agov/tests/validate.rs
  - tooling/rust/tests/parity/run.sh
  - agf/docs/site/docs/adopt/install-and-layouts.md
  - agf/docs/site/docs/adopt/tooling-and-tokens.md
  - agf/docs/site/docs/adopt/assistant-loader-configuration.md
  - agf/skills/upgrade/SKILL.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the issue #6 upgrade-friction implementation.
    - Confirm advisory drift warnings and optional-tooling docs wording.
  must_read:
    - tooling/rust/crates/agov-validators/src/assistant_drift.rs
    - tooling/rust/crates/agov/src/main.rs
    - agf/docs/site/docs/adopt/install-and-layouts.md
    - agf/docs/site/docs/adopt/tooling-and-tokens.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 2 handoff: issue #6 upgrade friction

Branch: `fix/issue-6-upgrade-friction`

Implements the Gate 1 approved first slice for issue #6.

## Implemented

- Added advisory `agov.validate.assistant_drift` coverage to umbrella
  `agov validate`.
- The drift check inspects only `.claude/rules/` and `.claude/skills/` when
  those copied trees exist, comparing them to canonical `agf/rules/` and
  `agf/skills/`.
- Findings report paths and remediation guidance, not file contents or hashes.
- Large copied-tree drift sets are summarized into one warning with counts and
  examples to avoid noisy output.
- `agov validate` now prints a compact coverage note: plan and coordination
  files need `agov plan validate` / `agov coord validate` or equivalent manual
  review.
- Adoption docs and the upgrade skill now preserve the optional-tooling
  boundary and explain what to do after drift warnings.

## Verification

- `cargo test -p agov-validators` — pass.
- `cargo test -p agov` — pass.
- `cargo clippy --workspace --all-targets -- -D warnings` — pass.
- `bash tooling/rust/tests/parity/run.sh` — pass, 13/13.
- `cargo deny check` — pass with existing non-blocking duplicate/license
  allowance warnings; advisories/bans/licenses/sources all OK.
- `cargo run --quiet -p agov -- validate` — pass; emits one summarized
  `.claude/rules` advisory drift warning in this repo.
- `uvx --from mkdocs-material mkdocs build -f agf/docs/build/mkdocs.yml --strict`
  — pass.
- `git diff --check` — pass.

Note: `cargo fmt --all --check` reports an existing formatting difference in
`tooling/rust/crates/agov/tests/coord_graph.rs`, unrelated to this MR. Touched
Rust files were formatted directly with `rustfmt --edition 2021`.
