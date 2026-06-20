---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260620T214350Z-coder-tester-g66b
timestamp: 2026-06-20T21:43:50Z
immutable: true
references-to:
  - docs/plans/readable-coordination-ids.md
  - docs/coordination/01KVK7TFMYC725T6V5CQDDASK5-governor-reviewer.md
  - docs/coordination/01KVK7TFN1ADA2WBZDWKFRPBXR-planner-architect.md
topic: readable-coordination-ids-gate2
requires-ack: true
artifacts_owned:
  - AGENTS.md
  - agf/docs/reference/quick-reference.md
  - agf/docs/site/docs/concepts/roles-and-gates.md
  - agf/docs/site/docs/start/first-governed-change.md
  - agf/schemas/coordination-message.schema.json
  - agf/templates/README.md
  - docs/coordination/README.md
  - tooling/agov
  - tooling/elixir/lib/agov/cli.ex
  - tooling/elixir/lib/agov/coord_brief.ex
  - tooling/elixir/lib/agov/coord_identity.ex
  - tooling/elixir/lib/agov/coord_validator.ex
  - tooling/elixir/lib/agov/coord_visibility.ex
  - tooling/elixir/lib/agov/handoff_tracker.ex
  - tooling/elixir/lib/mix/tasks/agov.coord.brief.ex
  - tooling/elixir/lib/mix/tasks/agov.coord.validate.ex
  - tooling/elixir/test/agov/brief_generator_test.exs
  - tooling/elixir/test/agov/channel_reader_test.exs
  - tooling/elixir/test/agov/coord_validator_test.exs
  - tooling/elixir/test/agov/handoff_tracker_test.exs
  - tooling/elixir/test/fixtures/valid_note_with_readable_id.md
  - tooling/rust/crates/agov-coord/src/coord_validator.rs
  - tooling/rust/crates/agov/src/main.rs
model: codex-gpt-5
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the readable coordination IDs implementation for Gate 2.
    - Confirm the explicit readable-ID generator option, validator compatibility, advisory drift warnings, and duplicate-ID blocking behavior are acceptable.
  must_read:
    - docs/plans/readable-coordination-ids.md
    - agf/schemas/coordination-message.schema.json
    - tooling/elixir/lib/agov/coord_identity.ex
    - tooling/elixir/lib/agov/coord_validator.ex
    - tooling/elixir/lib/agov/coord_brief.ex
    - tooling/rust/crates/agov-coord/src/coord_validator.rs
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 2 handoff: readable coordination IDs

Implemented the approved readable coordination IDs plan after MR !31 was merged
to main and merged into this branch.

Summary:

- Schema now accepts both legacy ULID IDs and readable IDs of the form
  `YYYYMMDDThhmmssZ-<role-slug>-<random-suffix>`.
- Elixir tooling can generate readable IDs with
  `agov coord brief --id-format readable --from <role>`, using one UTC instant
  for both the readable ID and frontmatter timestamp.
- `agov coord validate` accepts both ID styles, blocks duplicate frontmatter
  IDs across a validated file set, and emits advisory warnings for readable
  ID role, timestamp, and filename sender drift.
- Handoff/body filename matching and branch visibility logic remain compatible
  with legacy sequence, ULID, and readable-ID filenames.
- Rust validation parity now accepts the shared schema, reports readable drift
  advisories, and blocks duplicate IDs for `coord validate`.
- Live orientation and adopter docs describe readable IDs as the preferred new
  format while preserving existing ULID and sequence history.

Verification:

- `mix test` — pass, 211 tests.
- `cargo test` — pass.
- `cargo clippy -- -D warnings` — pass.
- `git diff --check` — pass.
- `./tooling/agov plan validate docs/plans/readable-coordination-ids.md` — pass.
- `./tooling/agov coord brief --id-format readable --from coder-tester --kind note --topic readable-id-smoke` — pass; generated matching readable ID and timestamp.
- Duplicate-ID smoke via `./tooling/agov coord validate <two copied readable fixtures>` — correctly exited non-zero with a blocking duplicate-ID error.

Notes:

- `mix test` still prints two pre-existing test warnings unrelated to this
  change: an unused `@tag` in `coord_config_test.exs` and an unused default
  argument in `claim_tracker_test.exs`.
- The fixture `valid_note_with_readable_id.md` intentionally has a short
  fixture filename. Unit tests cover the no-warning path under a matching
  coordination filename; direct validation of the fixture path emits the
  expected filename/id advisory.
