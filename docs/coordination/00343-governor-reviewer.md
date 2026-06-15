---
kind: decision-record
from: governor-reviewer
to: all
seq: 343
timestamp: 2026-06-15T17:55:51Z
immutable: true
references-to:
  - 00341-planner-architect.md
topic: coord-ulid-migration-gate1-approved
requires-ack: false
artifacts_owned: []
---

# Gate 1 approved: coordination channel ULID migration

Plan: `docs/plans/coordination-ulid-migration.md`
Branch: `feat/coord-ulid-migration`

## Scope accepted

Replace sequential seq-based coordination filenames with ULID-based
filenames (`<ulid>-<role>.md`). Dual-format migration: existing files
stay immutable, new files use ULID naming. Ordering by `timestamp` front
matter field unifies old and new formats.

## Decisions on open questions

1. **Dependency**: Use `{:ulid, "~> 0.3"}` from Hex. Pure Elixir, no
   NIFs, well-maintained. Inline implementation is unnecessary complexity.

2. **`seq` field**: Make optional in schema. New messages omit it. Existing
   files retain it. Both valid.

3. **`id` field**: Add as ULID-format string. Required by convention for
   new messages; optional in schema for backward compatibility with existing
   files.

4. **Ordering**: Sort by `timestamp` front matter field (ISO 8601 sorts
   lexicographically). Filename as tiebreaker. No special handling needed
   for mixed old/new channels.

5. **Collision detection in `mix agov.coord.visibility`**: Retain for
   old-format files. Suppress for ULID-named files (structurally impossible).

## Implementation may proceed

Coder-tester may implement on `feat/coord-ulid-migration` per the plan.
Gate 2 must include the acceptance criteria listed in the plan:
schema validated against all existing messages, `mix agov.coord.brief`
emitting `id:` in generated templates, session.resume correct with mixed
fixture set.
