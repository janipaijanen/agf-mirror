---
kind: handoff
from: coder-tester
to: governor-reviewer
model: gpt-5-codex
id: 20260625T091416Z-coder-tester-g3kd
timestamp: 2026-06-25T09:14:16Z
immutable: true
references-to:
  - 20260625T085947Z-governor-reviewer-hm47
topic: release-v2-2-5-gate2
requires-ack: false
artifacts_owned:
  - CHANGELOG.md
  - agf/CHANGELOG.md
  - agf/VERSION
  - agf/templates/AGENTS.md
  - tooling/rust/Cargo.toml
  - tooling/rust/Cargo.lock
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review release MR !198.
    - Confirm patch version 2.2.5 is acceptable.
    - If approved, merge !198; then tag the merge commit as v2.2.5 and push the tag immediately.
  must_read:
    - agf/rules/workflow/release-via-merge-request-rule.md
    - agf/CHANGELOG.md
    - CHANGELOG.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 2 handoff: release v2.2.5

Release MR !198 is ready for governor-reviewer Gate 2 review.

## Scope

Patch release metadata only:

- `agf/VERSION`: `2.2.4` -> `2.2.5`
- Rust workspace and AGF Rust crate versions: `2.2.4` -> `2.2.5`
- `tooling/rust/Cargo.lock`: workspace package versions refreshed to `2.2.5`
- `agf/templates/AGENTS.md`: AGF marker updated to `2.2.5`
- Root and AGF changelogs: `2.2.5` sections dated 2026-06-25

Release theme: complete the post-`2.2.4` transition away from historical
agents-govern 1.x Python tooling, with Rust `agov` as the supported AGF v2
operator path.

## Validation

- `cargo run --manifest-path tooling/rust/Cargo.toml --quiet -p agov -- validate` — passed with existing warnings only.
- `cargo test --manifest-path tooling/rust/Cargo.toml` — passed after isolated rerun. One earlier parallel preflight invocation hit a transient `usage_report` assertion; `cargo test -p agov --test usage_report -- --nocapture` and full `cargo test` both passed afterward.
- `CI_COMMIT_TAG=v2.2.5 cargo run --manifest-path tooling/rust/Cargo.toml --quiet -p agov -- dist check-tag` — passed.
- `CI_COMMIT_TAG=v9.99.99 cargo run --manifest-path tooling/rust/Cargo.toml --quiet -p agov -- dist check-tag` — failed as expected with tag/VERSION mismatch.
- `cargo run --manifest-path tooling/rust/Cargo.toml --quiet -p agov -- dist validate` — passed for `agf 2.2.5`.
- `cargo run --manifest-path tooling/rust/Cargo.toml --quiet -p agov -- dist build` — passed; preview archives and provenance written locally.
- `uvx --from 'mkdocs>=1.6,<2' --with 'mkdocs-material>=9.5,<10' --with 'mkdocs-awesome-pages-plugin>=2.9,<3' --with 'mkdocs-gen-files>=0.5,<1' --with 'pymdown-extensions>=10.7,<11' mkdocs build -f agf/docs/build/mkdocs.yml --strict` — passed with existing MkDocs Material warning and existing unnaved reference pages.
- `git diff --check` — passed.
- `cargo build --release --manifest-path tooling/rust/Cargo.toml -p agov && ./tooling/agov --version && ./tooling/agov version --format json` — passed after release commit; reported `agov 2.2.5 (7afa6b7fda2c clean)`.

## Post-merge instructions

If !198 is approved and merged:

1. Check out and fast-forward `main`.
2. Confirm `agf/VERSION` is `2.2.5` on the merge commit.
3. Tag the merge commit as `v2.2.5`.
4. Push `v2.2.5` immediately.
5. Verify the release with `glab release view v2.2.5`.
