---
kind: handoff
from: coder-tester
to: governor-reviewer
model: gpt-5-codex
id: 20260622T004812Z-coder-tester-rs1q
timestamp: 2026-06-22T00:48:12Z
immutable: true
references-to:
  - 20260622T002642Z-coder-tester-hhgb.md
topic: ci-rust-release-docs-gate2
requires-ack: false
artifacts_owned:
  - .gitlab-ci.yml
  - docs/coordination/20260622T004812Z-coder-tester-rs1q.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Review CI update that makes release metadata changes rebuild Pages and moves release-tag confidence toward Rust tooling."
  must_read:
    - .gitlab-ci.yml
    - docs/coordination/20260622T004812Z-coder-tester-rs1q.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 2 handoff: CI Rust validation and release docs rebuild

Branch `ci/rust-release-docs` is ready for governor-reviewer review.

Intent:

- Fix the release-site gap observed after `v2.1.2`: version/changelog-only
  release MRs should trigger the docs build and default-branch Pages deploy.
- Add a Rust `agov validate` CI job for AGF framework content.
- Stop running `elixir:test` on release tags and on `.gitlab-ci.yml`-only
  changes. Elixir tests now remain scoped to actual `tooling/elixir/**`
  changes.
- Run Rust workspace tests and clippy on release tags.

Scope:

- CI configuration only.
- No Rust source changes.
- No Elixir source changes.
- Mix-backed `agf:dist:*` jobs remain in place because framework dist
  packaging has not yet been ported to Rust.

Validation performed:

```bash
glab ci lint
cd tooling/rust && cargo run --quiet -p agov -- validate
cd tooling/rust && cargo test --workspace
cd tooling/rust && rustup component add clippy >/dev/null && cargo clippy --workspace -- -D warnings
```

Results:

- `glab ci lint`: valid CI/CD YAML.
- Rust `agov validate`: OK for learnings, agents, adoption, infosec,
  contributions; framework config skipped because `agf/framework.yaml` is not
  present.
- `cargo test --workspace`: 40 tests passed.
- `cargo clippy --workspace -- -D warnings`: passed.

Known local noise:

- Linked-worktree commands print the existing `mise.toml` trust warning.
- Ruby is not installed locally, so a Ruby YAML parser check was skipped in
  favor of `glab ci lint`.
