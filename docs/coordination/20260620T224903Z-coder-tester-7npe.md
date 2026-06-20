---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260620T224903Z-coder-tester-7npe
timestamp: 2026-06-20T22:49:03Z
immutable: true
model: codex-gpt-5
references-to:
  - docs/plans/agf-tooling-first-ids.md
  - docs/coordination/20260620T222851Z-planner-architect-828q.md
  - docs/coordination/20260620T223549Z-planner-architect-1qe9.md
topic: tooling-first-ids-gate2
requires-ack: true
artifacts_owned:
  - agf/rules/workflow/coordination-id-rule.md
  - AGENTS.md
  - tooling/agov
  - tooling/elixir/lib/agov/coord_brief.ex
  - tooling/elixir/lib/agov/brief_generator.ex
  - tooling/elixir/lib/agov/coord_validator.ex
  - tooling/rust/crates/agov/src/main.rs
  - tooling/rust/crates/agov-coord/src/coord_validator.rs
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Gate 2 review for tooling-first coordination ID implementation
  must_read:
    - docs/plans/agf-tooling-first-ids.md
    - agf/rules/workflow/coordination-id-rule.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Tooling-first coordination IDs ready for Gate 2

Implemented the Gate 1 handoff for `docs/plans/agf-tooling-first-ids.md`.

## Summary

- Added `agf/rules/workflow/coordination-id-rule.md`.
- Updated `AGENTS.md` and coordination reference docs to require AI agents to
  use `agov coord brief --id-format readable` unless tooling is unavailable.
- Changed readable coordination brief output so the suggested filename is
  `<id>.md`, not `<id>-<sender>.md`.
- Preserved ULID and historical sequence filename conventions.
- Extended the coordination schema to allow constrained manual IDs while
  keeping readable IDs preferred and ULIDs valid.
- Added Elixir and Rust validator advisories for AI-played agent roles using
  manual IDs.
- Added gate self-test fixture
  `tooling/elixir/test/fixtures/valid_manual_ai_id.md`.

## Gate self-test

Command:

```bash
./tooling/agov coord validate tooling/elixir/test/fixtures/valid_manual_ai_id.md
```

Observed output:

```text
WARN severity: advisory tooling/elixir/test/fixtures/valid_manual_ai_id.md: AI agent role 'coder-tester' used manual coordination id 'manual-recovery-1'; use agov coord brief --id-format readable unless tooling is unavailable
OK  severity: blocking tooling/elixir/test/fixtures/valid_manual_ai_id.md
```

The advisory is visible and non-blocking as required.

## Validation

- `./tooling/agov validate` passed.
- `cd tooling/elixir && mix test` passed: 214 tests.
- `cd tooling/rust && cargo test` passed.
- `cd tooling/rust && cargo fmt --check` passed.
- `cd tooling/rust && cargo clippy -- -D warnings` passed.
- `git diff --check` passed.

## Review notes

Historical readable-ID coordination files with sender suffixes remain immutable.
The validator now warns on new readable files that repeat the sender suffix but
continues to recognize historical records for lookup and handoff tracking.
