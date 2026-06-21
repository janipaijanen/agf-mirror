---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260621T151036Z-coder-tester-rnt1
timestamp: 2026-06-21T15:10:36Z
immutable: true
model: codex-gpt-5
references-to:
  - 20260621T145946Z-planner-architect-h4cj.md
  - 20260621T150011Z-planner-architect-t8sw.md
topic: learning-record-id-policy-gate2
requires-ack: true
artifacts_owned:
  - AGENTS.md
  - docs/plans/learning-record-nonsequence-ids.md
  - agf/learnings/foundry.yaml
  - agf/schemas/learning.schema.json
  - agf/skills/learning/SKILL.md
  - agf/skills/README.md
  - agf/templates/README.md
  - agf/docs/site/docs/adopt/install-and-layouts.md
  - agf/docs/site/docs/concepts/evidence-and-learnings.md
  - agf/docs/site/docs/start/adopter-faq.md
  - tooling/elixir/lib/agov/validate_learnings.ex
  - tooling/elixir/test/agov/validate_learnings_test.exs
  - tooling/rust/crates/agov-validators/src/learnings.rs
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the learning-record ID policy implementation.
    - Decide Gate 2 for branch chore/learning-id-policy.
  must_read:
    - docs/coordination/20260621T145946Z-planner-architect-h4cj.md
    - docs/plans/learning-record-nonsequence-ids.md
    - agf/schemas/learning.schema.json
    - tooling/elixir/lib/agov/validate_learnings.ex
    - tooling/rust/crates/agov-validators/src/learnings.rs
  do_not_touch: []
---

# Gate 2 handoff: learning record ID policy

Implemented the Gate 1 decision in
`20260621T145946Z-planner-architect-h4cj` on branch
`chore/learning-id-policy`.

## Gate 1 Conditions

- Canonical format is `YYYYMMDDThhmmssZ` with optional `-<4-char>` suffix.
- No `learn-` prefix and no slug are documented or accepted for new IDs.
- Historical `L###` IDs remain valid during transition.
- Implementation is on separate branch `chore/learning-id-policy`, not
  `feat/rust-coord-stale`.

The two rejected `learn-...` entries were not present on `origin/main`; this
branch does not add them.

## Changes

- Added learning ID guidance to `AGENTS.md`.
- Added the approved plan file, with Gate 1 approval metadata.
- Updated `agf/learnings/foundry.yaml` comments to mark `L018-L021` as
  transitional and timestamp IDs as the new convention.
- Added `id` shape to `agf/schemas/learning.schema.json`.
- Added Elixir and Rust learning validators for:
  - duplicate IDs;
  - historical `L###` IDs;
  - canonical timestamp IDs;
  - rejection of non-canonical IDs such as `learn-...`.
- Updated the learning skill, skills README, template README, and adopter docs.

## Verification

Passed:

```bash
mix test test/agov/validate_learnings_test.exs
cargo test -p agov-validators learnings
agov plan validate docs/plans/learning-record-nonsequence-ids.md
agov validate learnings
uv run --project agents-govern --group docs mkdocs build -f agf/docs/build/mkdocs.yml --strict
git diff --check
agov coord validate docs/coordination/20260621T151036Z-coder-tester-rnt1.md
```

Notes:

- This fresh worktree prints a local `mise.toml` trust warning on shell startup.
  Commands completed successfully using explicit installed tool paths where
  needed.
- `mix local.hex --force` and `mix deps.get` were run locally to make the
  Elixir focused test executable; generated dependency/build directories are
  ignored and not part of the commit.
