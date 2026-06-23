---
kind: handoff
from: governor-reviewer
to: coder-tester
id: 20260623T210039Z-governor-reviewer-ah3b
timestamp: 2026-06-23T21:00:39Z
immutable: true
references-to:
  - 01KVK7J9N2AW8DJZQAG2-planner-architect.md
  - 20260623T203039Z-planner-architect-mrjh.md
  - 20260623T204317Z-governor-reviewer-nmpn.md
topic: readable-coordination-ids-proceed
requires-ack: false
artifacts_owned:
  - docs/coordination/20260623T210039Z-governor-reviewer-ah3b.md
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Implement readable coordination IDs per docs/plans/readable-coordination-ids.md
    - Both original preconditions are now satisfied — proceed immediately
    - Implement in rollout order per plan §6 (schema → validator → generator → guidance)
    - File Gate 2 handoff to governor-reviewer when implementation MR is ready for review
  must_read:
    - docs/plans/readable-coordination-ids.md
    - docs/plans/rc1-sequence.md
    - docs/coordination/01KVK7J9N2AW8DJZQAG2-planner-architect.md
    - tooling/rust/crates/agov-coord/src/readable_id.rs
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Proceed: readable coordination IDs (Step 2 of RC-1 sequence)

Both preconditions from `01KVK7J9N2AW8DJZQAG2` are now satisfied:

1. **MR !31 (Rust phase 1)** — merged long ago.
2. **MR !153 (readable ID utility extraction)** — merged at `08c9e9e`. Gate 2
   approved via `nmpn`. `agov-coord::readable_id` module now provides the
   canonical library seam: `readable_id()`, `role_slug()`, `readable_suffix()`,
   `parse_readable_id()`, `is_readable_id()`, and the timestamp formatters.

## Scope (from plan §6)

1. **Schema**: update `agf/schemas/coordination-message.schema.json` to accept
   both ULID and readable ID formats. The readable format is
   `YYYYMMDDThhmmssZ-<role-slug>-<4-6char-suffix>`.
2. **Validators**: uniqueness check (blocking), filename/frontmatter drift
   warning (advisory), backward compatibility for ULIDs.
3. **Generator**: readable ID support in `agov coord brief` (`--id-format readable`
   is already implemented per the skeleton used to generate this message ID;
   verify it is wired to `agov-coord::readable_id` and extend as needed).
4. **Guidance**: update AI contributor docs to prefer tooling-generated readable IDs.

## Key library seam

`tooling/rust/crates/agov-coord/src/readable_id.rs` is the canonical module.
Do not duplicate its logic in `main.rs`. Import from `agov_coord::readable_id`.

## Out of scope

- `agov watch start/status/stop`
- Readable IDs for plan files or other artifacts
- Historical ULID record renames (never authorized)

## Gate

File Gate 2 handoff to governor-reviewer on the implementing branch. This
is Step 2 of the RC-1 sequence (`docs/plans/rc1-sequence.md`). Step 3 (MR 2:
version provenance + contract docs) follows after Gate 2 approval of this item.
