---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260625T072349Z-coder-tester-shad
timestamp: 2026-06-25T07:23:49Z
immutable: true
model: gpt-5-codex
references-to:
  - 20260625T071009Z-coder-tester-5q8e
topic: issues-18-19-upstream-learning-validation
requires-ack: false
artifacts_owned:
  - agf/learnings/foundry.yaml
  - tooling/rust/crates/agov-validators/src/learnings.rs
  - tooling/rust/crates/agov/src/main.rs
  - tooling/rust/crates/agov/tests/validate.rs
  - agf/docs/site/docs/advanced/contracts-and-validators.md
  - agf/docs/site/docs/start/adopter-faq.md
  - agf/docs/site/reference/environment-variables.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review issue #18 and #19 handling.
    - Decide whether to merge this focused branch or request a separate plan/MR split.
  must_read:
    - agf/learnings/foundry.yaml
    - agf/docs/site/docs/advanced/contracts-and-validators.md
    - tooling/rust/crates/agov/src/main.rs
    - tooling/rust/crates/agov-validators/src/learnings.rs
    - tooling/rust/crates/agov/tests/validate.rs
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Handoff: issues #18 and #19 addressed

I assessed the open issues:

- `#18` — learning for upstream: CI OCI image
- `#19` — learning for upstream: prerequisites or self-sufficiency
- `#17` — existing external reviewer comments; already feeding recent docs work
  and MR !185.
- `#13` — release archive BOM; broader release artifact enhancement, not part
  of this focused branch.

Changes made:

- Added issue #18 and #19 learning records to `agf/learnings/foundry.yaml`.
  #18 remains `status: proposed`; #19 is `status: implemented`.
- Implemented `agov validate learnings <file-or-dir>...` so
  upstream-submittable learning packets outside `agf/learnings/` can be
  validated directly with Rust `agov`.
- Documented that the release archive is the current supported CI distribution
  and that an official OCI/container image is not shipped yet.
- Documented explicit-path learning validation in contracts, FAQ, and
  environment-variable reference docs.

Validation:

- `cargo test --manifest-path tooling/rust/Cargo.toml` — pass.
- `cargo test --manifest-path tooling/rust/Cargo.toml -p agov --test validate`
  — pass.
- `./tooling/agov validate` — pass with known warnings for historical bare
  timestamp learning IDs and existing assistant drift.
- `./tooling/agov validate infosec` — pass.
- `./tooling/agov validate learnings agf/learnings/foundry.yaml` — pass with
  known historical bare timestamp warnings.
- `git diff --check` — pass.

Local note:

The release `agov` binary was rebuilt after the CLI change so the repo-local
launcher supports the new explicit learning path form in this checkout.
