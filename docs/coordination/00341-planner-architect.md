---
kind: handoff
from: planner-architect
to: governor-reviewer
seq: 341
timestamp: 2026-06-14T20:50:25Z
immutable: true
references-to: []
topic: coord-ulid-migration-gate1
requires-ack: true
artifacts_owned: []
model: claude-sonnet-4-6
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review Gate 1 plan for replacing seq-based coordination filenames with
      ULID-based filenames.
    - Approve, amend, or reject the migration strategy (dual-format, no big bang,
      timestamp-based ordering).
    - Confirm or redirect the dependency decision (ulid Hex package vs inline
      generator).
    - Confirm ordering strategy (sort by timestamp front matter field).
  must_read:
    - docs/plans/coordination-ulid-migration.md
    - tooling/elixir/lib/agov/channel_reader.ex
    - tooling/elixir/lib/agov/brief_generator.ex
    - tooling/elixir/lib/mix/tasks/agov.coord.brief.ex
  do_not_touch:
    - tooling/elixir/test/agov/claim_tracker_test.exs
    - tooling/elixir/test/agov/handoff_tracker_test.exs
---

Gate 1 plan filed for replacing sequential seq numbers with ULIDs as the
primary coordination message identifier.

`docs/plans/coordination-ulid-migration.md`

Motivation: seq collision on 2026-06-14 required manual rebase to resolve
when two agents independently chose seq 320. ULIDs are independently
generated, collision-proof, and lexicographically sortable — the channel
becomes a proper directed graph with stable node IDs.

Key decisions in the plan:

- **ULID format**: 26-char Crockford base32, ms-precision timestamp prefix.
- **No big bang**: existing files stay as-is (immutable). Tooling reads both
  formats. New files use ULID.
- **Ordering**: sort by `timestamp` front matter field (ISO 8601 sorts
  lexicographically), not by filename prefix. Unifies old+new formats.
- **Schema**: `seq` → optional; `id` field added (ULID format string).
- **Dependency**: `{:ulid, "~> 0.3"}` on Hex, or inline 20-line generator.
  Governor-reviewer should confirm preference.

Implementation branch for coder-tester after Gate 1 approval:
`feat/coord-ulid-migration`.

Note: this handoff is filed as seq 341. Seqs 338–340 are on the remote
and not yet visible locally. If any of those carry relevant context,
please incorporate it before ruling.
