# Plan: T1-3 — Async Handoff State Tooling (`agov handoff next`)

**Date:** 2026-06-11T00:12:56Z
**Status:** PENDING GATE 1 APPROVAL
**Depends on:** T1-1 Phase B (tooling/elixir/ Mix project, merged ✓)

---

## Intent

Implement `mix agov.handoff.next` — a Mix task that reads the coordination
channel, finds the most recent `kind: handoff` message addressed to a given
agent role, and prints a compact "what is ready for me" summary.

The goal: a cold-start agent can orient in one command instead of reading the
full coordination tail. The human Governor stops scheduling; human approves.

The `handoff` object schema is already in `spec/schemas/coordination-message.schema.json`.
This task is pure tooling — no schema changes required.

---

## Affected files

| File | Action |
|---|---|
| `tooling/elixir/lib/agov/handoff_reader.ex` | Create — parse and query handoff messages |
| `tooling/elixir/lib/mix/tasks/agov.handoff.next.ex` | Create — Mix task |
| `tooling/elixir/test/agov/handoff_reader_test.exs` | Create — unit tests |
| `tooling/elixir/test/fixtures/handoff_ready.md` | Create — valid ready handoff fixture |
| `tooling/elixir/test/fixtures/handoff_blocked.md` | Create — blocked handoff fixture |
| `tooling/elixir/mix.exs` | No change — deps sufficient |

---

## Migration needed

No.

---

## Implementation plan

### Step 1 — Write tests FIRST (TDD RED)

Fixtures:
- `handoff_ready.md` — `kind: handoff`, `handoff.status: ready`,
  `handoff.next_agent: coder-tester`, with `must_read` and `do_not_touch` lists
- `handoff_blocked.md` — same but `status: blocked`, with a reason implied in body

Tests for `HandoffReader`:
- `find_next/2` given a list of messages and an agent role, returns the most
  recent handoff addressed `to:` that role (by `seq`)
- Returns `{:none}` when no handoff exists for the role
- Correctly parses the `handoff` object from validated front matter
- `format_summary/1` produces the expected multi-line string

### Step 2 — Implement `HandoffReader` (GREEN)

`lib/agov/handoff_reader.ex`:

1. `find_next(coord_dir, agent_role)` — scan `coord_dir/*.md`, validate each
   with `CoordValidator.validate_file/1`, collect `kind: handoff` messages
   where `to == agent_role`, return the one with highest `seq`
2. `format_summary(message_data)` — format output:

```
Handoff for: <next_agent>
Status:      ready | blocked
Task:
  - <item>
  - <item>
Must read:
  - <file>
Do not touch:
  - <file>
Source: docs/coordination/<seq>-<sender>.md
```

### Step 3 — Implement Mix task

`lib/mix/tasks/agov.handoff.next.ex`:

Invocation: `mix agov.handoff.next <agent-role> [--dir <coord-dir>]`

- `<agent-role>` — role identifier (e.g. `coder-tester`, `planner-architect`)
- `--dir` — coordination directory (default: `../../docs/coordination`)
- Calls `HandoffReader.find_next/2`, prints `format_summary/1` output
- Exit 0 if handoff found, 2 if none found, 1 on error

---

## Test strategy

| Tier | What | Where |
|---|---|---|
| Unit | `find_next/2` with fixture dir, `format_summary/1` output shape | `test/agov/handoff_reader_test.exs` |
| Manual | `mix agov.handoff.next planner-architect` against real `docs/coordination/` | Run locally, inspect output |

TDD order: fixtures → tests → RED confirmed → HandoffReader → GREEN → task.

---

## Risks

| Risk | Mitigation |
|---|---|
| Multiple handoffs for same agent (normal in long sessions) | Return highest `seq` only — documented |
| `blocked` handoffs returned to agent | Caller must act on `status` field; task prints it prominently |
| Large coord dir (hundreds of files) | Linear scan; acceptable at current scale |

---

## Out of scope

- Handoff state database / index (defer until scan becomes slow in practice)
- `agov coord status` (T1-2, deferred — no concurrency issue yet)
- Graph traversal of `references-to`
