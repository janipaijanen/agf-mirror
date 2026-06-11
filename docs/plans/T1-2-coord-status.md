---
id: T1-2-coord-status
status: approved
topic: coord-status-command
gate1_approved_by: "governor-reviewer"
gate1_approved_at: "2026-06-11T07:01:58Z"
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: read-only tooling — no security boundary, auth, or secrets changes
  - role: contribution-auditor
    status: not-applicable
    reason: incremental read-only Mix task; same pattern as T1-3 and T2-6; governor-reviewer Gate 1 and Gate 2 review is sufficient
---

# Plan: T1-2 — mix agov.coord.status

**Date:** 2026-06-11T06:43:41Z
**Status:** PENDING GATE 1 APPROVAL
**Depends on:** T1-1 Phase B (CoordValidator, merged ✓), T1-3 (HandoffReader, merged ✓), T2-6 (ClaimTracker, merged ✓)

---

## Intent

A cold-start agent reading the coordination channel today must manually scan
all files to answer basic orientation questions: How many messages exist? Who
owns what? Are there pending handoffs for my role? What happened recently?

`mix agov.coord.status` answers all four in a single command, composing the
three existing modules (CoordValidator, HandoffReader, ClaimTracker) into one
human-readable channel overview.

---

## Scope

Two deliverables:

1. **`Agov.ChannelReader`** — reads and parses all coordination messages from
   a directory into a sorted list; provides channel statistics. Thin wrapper
   over `CoordValidator`; no new domain logic.
2. **`mix agov.coord.status [--dir <dir>] [--recent N]`** — prints channel
   overview: stats, active ownership, stale claims, pending handoffs, recent
   messages.

No new schema changes. No new message kinds. Read-only.

---

## Affected files

| File | Action |
|---|---|
| `tooling/elixir/lib/agov/channel_reader.ex` | Create — directory scanner + stats |
| `tooling/elixir/lib/mix/tasks/agov.coord.status.ex` | Create — Mix task |
| `tooling/elixir/test/agov/channel_reader_test.exs` | Create — unit tests |

---

## ChannelReader design

```elixir
defmodule Agov.ChannelReader do
  def scan(coord_dir)
  # Returns {:ok, %{messages: [map], total: N, seq_range: {min, max}}}
  # or {:error, :dir_not_found}
  # Skips files that fail CoordValidator (README, non-message files)
  # Messages sorted ascending by seq
end
```

`messages` is a list of `{filename, data}` tuples where `data` is the parsed
YAML map from `CoordValidator.validate_file/1`. Files that fail validation are
silently skipped — the same pattern used by the Mix task in T2-6.

`seq_range` is `{min_seq, max_seq}` across all valid messages, or `nil` if
the channel is empty.

---

## Mix task output format

```
Channel: ../../docs/coordination/  (18 messages, seq 1–18)

Ownership:    none
Stale claims: none

Pending handoffs:
  (none)

Recent messages (last 5):
  seq 18  planner-architect → contribution-auditor  ack     T2-6-gate1-review
  seq 17  contribution-auditor → planner-architect  review  T2-6-gate1-review
  seq 16  planner-architect → contribution-auditor  handoff T2-6-gate1-review
  seq 15  contribution-auditor → all                review  AGENTS-orientation-review
  seq 14  coder-tester → planner-architect          handoff T2-5-mr-flow-handoff
```

When active ownership exists:
```
Ownership:
  docs/plans/T2-7.md  claimed-by: planner-architect  seq: 42
```

When stale claims exist:
```
Stale claims:
  docs/old-plan.md  claimed-by: planner-architect  seq: 7  expired: 2026-03-01T00:00:00Z
```

When pending handoffs exist:
```
Pending handoffs:
  → coder-tester   seq: 16  T2-6-gate1-review  [ready]
  → governor-reviewer  seq: 14  T2-5-mr-flow-handoff  [ready]
```

Options:
- `--dir <coord-dir>` — default `../../docs/coordination`
- `--recent N` — number of recent messages to show (default 5, 0 = suppress)

Exit codes: `0` always (status is informational; errors print to stderr).

---

## Implementation plan

### Step 1 — Write tests FIRST (TDD RED)

Tests for `ChannelReader.scan/1`:
- Against `test/fixtures/` directory: returns known message count and seq range
- Empty directory (create a temp dir): returns `{:ok, %{messages: [], total: 0, seq_range: nil}}`
- Non-existent directory: returns `{:error, :dir_not_found}`
- Directory with mixed files (fixtures + README-like non-message): skips invalid files

### Step 2 — Implement ChannelReader (GREEN)

Thin wrapper: list `.md` files, sort by name, call `CoordValidator.validate_file/1`
on each, collect successes as `{filename, data}` tuples, compute stats.

### Step 3 — Implement Mix task

Calls `ChannelReader.scan/1`, `ClaimTracker.scan/2`, and inspects handoff
messages directly from the parsed message list (does not call `HandoffReader`
— `HandoffReader.find_next/2` returns only the highest-seq handoff for one
role; the status view needs all pending handoffs for all roles).

Handoff detection: filter messages where `kind == "handoff"` from the parsed
list; a handoff is "pending" if no later `completion` or `ack` from the target
agent references it. For the first slice, show all handoffs in the recent
message window without tracking release — a simple, honest approximation noted
in the output as "recent handoffs" if the window is constrained.

---

## Test strategy

| Tier | What | Where |
|---|---|---|
| Unit | `ChannelReader.scan/1` — count, seq range, skip invalid, empty dir, missing dir | `test/agov/channel_reader_test.exs` |
| Manual | `mix agov.coord.status` against `docs/coordination/` | Inspect output for correctness |

---

## Risks

| Risk | Mitigation |
|---|---|
| "Pending handoff" detection is approximate in first slice | Output labels it clearly; exact release tracking is a follow-on |
| Large channels slow down status | All existing channels are small; no optimisation needed yet |
| `--recent 0` edge case | Suppress recent-messages section entirely |
