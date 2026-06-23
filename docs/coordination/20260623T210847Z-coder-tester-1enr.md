---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260623T210847Z-coder-tester-1enr
timestamp: 2026-06-23T21:08:47Z
immutable: true
model: codex-gpt-5
references-to:
  - 01KVK7J9N2AW8DJZQAG2-planner-architect.md
  - 20260623T204317Z-governor-reviewer-nmpn.md
  - 20260623T210039Z-governor-reviewer-ah3b.md
topic: readable-coordination-ids-gate2
requires-ack: false
artifacts_owned:
  - AGENTS.md
  - README.md
  - agf/docs/build/mkdocs.yml
  - agf/docs/site/index.md
  - agf/docs/site/evaluate.md
  - agf/docs/site/docs/adopt/tooling-and-tokens.md
  - agf/docs/site/reference/environment-variables.md
  - agf/docs/site/reference/maturity-vocabulary.md
  - agf/docs/site/reference/tooling-conformance.md
  - agf/learnings/foundry.yaml
  - docs/coordination/README.md
  - tooling/rust/crates/agov/src/main.rs
  - tooling/rust/crates/agov/tests/coord_brief.rs
  - tooling/rust/crates/agov/tests/token_roi.rs
  - tooling/rust/crates/agov/tests/fixtures/token_roi/README.md
  - tooling/rust/crates/agov/tests/fixtures/token_roi/codex-agov-token-roi-20260623.jsonl
  - docs/coordination/20260623T210847Z-coder-tester-1enr.md
  - docs/coordination/20260623T212241Z-coder-tester-kdmr.md
  - docs/coordination/20260623T212551Z-coder-tester-ghgk.md
  - docs/coordination/20260623T214651Z-coder-tester-em7r.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review Gate 2 for readable coordination IDs implementation.
    - Confirm `agov coord brief` now defaults to readable IDs while `--id-format ulid` remains available for compatibility.
    - If approved, merge before Step 3 of the RC-1 sequence (version provenance and command contract docs).
  must_read:
    - docs/plans/readable-coordination-ids.md
    - docs/plans/rc1-sequence.md
    - docs/coordination/20260623T210039Z-governor-reviewer-ah3b.md
    - tooling/rust/crates/agov-coord/src/readable_id.rs
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 2 handoff: readable coordination IDs

Branch `feat/readable-coordination-ids-v2` implements Step 2 of the RC-1
sequence. The schema, validator, and readable-ID library seam were already
present after MR !153; this change completes the generator/guidance slice by
making `agov coord brief` default to readable coordination IDs.

## Scope completed

- `agov coord brief` now defaults to `--id-format readable`.
- `--id-format ulid` remains supported for explicit legacy compatibility.
- Integration tests cover both the new readable default and the ULID option.
- AI/contributor guidance no longer requires spelling `--id-format readable`
  on the common path, while documenting that the explicit flag remains valid.
- Local build guidance now prefers release builds for routine agent/operator
  work.
- Token ROI guidance now records per-assistant live log paths and includes a
  committed Codex snapshot fixture for realistic parser/report testing.
- AGF 1.x maturity vocabulary is carried forward as sticky AI/human guidance:
  Level 1 Planned, Level 2 Surface-defined, Level 3 Instantiated, Level 4
  Validated, and Level 5 Deployable.

## Verification

```bash
cargo test --manifest-path tooling/rust/Cargo.toml -p agov --test coord_brief
cargo test --manifest-path tooling/rust/Cargo.toml -p agov --test token_roi
cargo test --manifest-path tooling/rust/Cargo.toml -p agov-coord
cargo test --manifest-path tooling/rust/Cargo.toml -p agov
./tooling/agov plan validate docs/plans/readable-coordination-ids.md
./tooling/agov coord validate docs/coordination/20260623T214651Z-coder-tester-em7r.md
```

The coordination ID for this handoff was generated with `./tooling/agov coord
brief --from coder-tester --to governor-reviewer --kind handoff --topic
readable-coordination-ids-gate2 --dir docs/coordination`, which also smoke
tests the readable default.
