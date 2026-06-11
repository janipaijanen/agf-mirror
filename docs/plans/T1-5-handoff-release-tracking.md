---
id: T1-5-handoff-release-tracking
status: pending-gate1
topic: handoff-release-tracking
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: read-only channel scan; no security boundaries, auth, or secrets changes
  - role: contribution-auditor
    status: not-applicable
    reason: self-contained read-only module; same risk profile as T1-2 and T1-4; governor-reviewer Gate 1 and Gate 2 review is sufficient
---

# Plan: T1-5 — Handoff release tracking

**Date:** 2026-06-11T09:09:31Z
**Status:** PENDING GATE 1 APPROVAL
**Depends on:** T1-2 (ChannelReader, merged ✓), T2-7 (CoordConfig, merged ✓)

---

## Intent

`mix agov.coord.status` currently shows all handoff messages as pending because
it has no resolution tracking — it simply lists every handoff by recency. In the
live channel (28 messages), all eight historical handoffs are shown as pending
even though each has been responded to. The output is noise, not signal.

The correct output for a clean channel is `Pending handoffs: none`.

The resolution rule is straightforward and already evident in the channel data:
a handoff is resolved when any subsequent message's `references-to` list contains
the handoff's filename. Every properly-completed handoff in the project follows
this convention — gate approvals, reviews, and completions all reference their
parent handoff by filename.

---

## Scope

Two deliverables:

1. **`Agov.HandoffTracker`** — pure module. Takes `[{filename, data}]` tuples
   from ChannelReader. Builds a references index, classifies each handoff as
   pending (no later message references it) or resolved (at least one later
   message references it). Returns `{:ok, %{pending: [...], resolved: [...]}}`.

2. **`mix agov.coord.status`** — replace `print_handoffs/1` with a call to
   `HandoffTracker.scan/1`. Show only pending handoffs. Pass the full
   `{filename, data}` tuple list (HandoffTracker needs filenames).

No new schema changes. No new message kinds. Read-only.

---

## Affected files

| File | Action |
|---|---|
| `tooling/elixir/lib/agov/handoff_tracker.ex` | Create — pure resolution module |
| `tooling/elixir/lib/mix/tasks/agov.coord.status.ex` | Modify — use HandoffTracker |
| `tooling/elixir/test/agov/handoff_tracker_test.exs` | Create — unit tests |

---

## Resolution algorithm

A handoff at filename `F` with seq `N` is **resolved** if:

> There exists a message `M` in the channel where:
> - `M["seq"] > N`, AND
> - `F` appears in `M["references-to"]`

A handoff is **pending** if no such `M` exists.

This rule is sufficient and complete for the project's coordination convention.
It does not require knowledge of the `to` field or who responded — any message
that explicitly references the handoff by filename counts as a response. This
handles the early-era messages (seq 8 where `to: human`) correctly: seq 9
references `00008-planner-architect.md`, so seq 8 is resolved.

### Implementation

```elixir
defmodule Agov.HandoffTracker do
  def scan(file_messages) do
    # Step 1: build references index
    # references_index: %{filename => [seq, ...]} — seqs of messages that reference it
    references_index = build_references_index(file_messages)

    # Step 2: classify each handoff
    handoffs = Enum.filter(file_messages, fn {_file, data} -> data["kind"] == "handoff" end)

    {pending, resolved} =
      Enum.split_with(handoffs, fn {filename, data} ->
        seq = data["seq"]
        referencing_seqs = Map.get(references_index, filename, [])
        not Enum.any?(referencing_seqs, fn ref_seq -> ref_seq > seq end)
      end)

    {:ok, %{pending: format(pending), resolved: format(resolved)}}
  end

  defp build_references_index(file_messages) do
    Enum.reduce(file_messages, %{}, fn {_file, data}, acc ->
      refs = data["references-to"] || []
      seq  = data["seq"]

      Enum.reduce(refs, acc, fn ref_filename, inner ->
        Map.update(inner, ref_filename, [seq], &[seq | &1])
      end)
    end)
  end

  defp format(file_messages) do
    Enum.map(file_messages, fn {_filename, data} ->
      %{
        seq:    data["seq"],
        from:   data["from"],
        to:     data["to"],
        topic:  data["topic"],
        status: get_in(data, ["handoff", "status"]) || "ready"
      }
    end)
    |> Enum.sort_by(& &1.seq, :desc)
  end
end
```

---

## Mix task output change

**Before (current):** shows all handoffs by recency, labeled "recent" — noise.

**After:** shows only unresolved handoffs.

```
Pending handoffs: none
```

or, if genuinely pending:

```
Pending handoffs:
  → governor-reviewer  seq: 29  T1-5-handoff-release  [ready]
```

The `print_handoffs/1` private function is replaced by a call that takes the
full `{filename, data}` tuple list (already available in the task since
ChannelReader returns them) and passes it to `HandoffTracker.scan/1`.

---

## Implementation plan

### Step 1 — Tests first (TDD RED)

Tests for `HandoffTracker.scan/1`:

| Test | Expected |
|---|---|
| Handoff with a later message referencing it → resolved | `resolved` list contains entry; `pending` is empty |
| Handoff with no later references → pending | `pending` list contains entry; `resolved` is empty |
| Two handoffs: one referenced, one not | One in each list |
| Message references handoff from BEFORE it (lower seq) — edge case | Reference at lower seq does not resolve the handoff |
| Non-handoff messages are not classified | Only handoffs appear in output |
| Empty message list → no pending, no resolved | `{:ok, %{pending: [], resolved: []}}` |
| Resolved entry contains seq, from, to, topic, status | All fields present |

### Step 2 — Implement HandoffTracker (GREEN)

Pure module. No I/O. See design above.

### Step 3 — Update Mix task

Replace `print_handoffs(messages)` call and the private `print_handoffs/1`
function in `agov.coord.status` with HandoffTracker-based output.

The task already has `messages` as `[{filename, data}]` tuples in the non-empty
branch. Pass them directly to `HandoffTracker.scan/1`.

### Step 4 — Verify

```bash
mix agov.coord.status
# Expected: Pending handoffs: none
# (all 8 historical handoffs are referenced by later messages)
```

---

## Test strategy

| Tier | What | Where |
|---|---|---|
| Unit | HandoffTracker classification — 7 cases | `test/agov/handoff_tracker_test.exs` |
| Manual | `mix agov.coord.status` against live channel | Verify "Pending handoffs: none" |

---

## Risks

| Risk | Mitigation |
|---|---|
| A handoff that truly has no response (process gap) disappears | No — it stays in `pending`; only referenced ones move to `resolved` |
| A message references a handoff spuriously (not an actual response) | Acceptable: the protocol convention is that `references-to` is intentional; spurious references are a process issue, not a tool issue |
| The `{filename, data}` tuples are already stripped to just `data` before the Mix task calls `print_handoffs` | Fix: pass the full tuple list from ChannelReader directly (not the stripped version) |
