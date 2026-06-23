---
id: T1-4-coord-brief
status: approved
gate1_approved_by: "governor-reviewer"
gate1_approved_at: "2026-06-11T07:23:00Z"
topic: coord-brief-skeleton-generator
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: read-only channel scan; outputs a text skeleton to stdout; no secrets, auth, or security boundaries
  - role: contribution-auditor
    status: not-applicable
    reason: simple generative utility; same risk profile as T1-2 and T1-3; governor-reviewer Gate 1 and Gate 2 review is sufficient
---

# Plan: T1-4 — mix agov.coord.brief

**Date:** 2026-06-11T07:16:36Z
**Status:** PENDING GATE 1 APPROVAL
**Depends on:** T1-1 Phase B (CoordValidator, merged ✓), T1-2 (ChannelReader, merged ✓)

---

## Intent

Writing a coordination message currently requires an agent to manually look up
the current channel state (next seq, open handoffs, ownership) and then
construct the YAML front matter from memory or a template. Both steps are
error-prone: seq collisions happen, required fields are omitted, field names
are misremembered.

`mix agov.coord.brief` eliminates both steps. Given a message kind and a few
flags, it scans the channel, computes the next seq, and prints a ready-to-edit
coordination message skeleton to stdout. The agent pastes it into a new file
and fills in the blanks.

The secondary output (comment-style channel summary) gives the same orientation
information as `mix agov.coord.status` in a single command, eliminating the
need to run two tools before composing a message.

---

## Scope

Two deliverables:

1. **`Agov.BriefGenerator`** — pure module; takes channel stats and options;
   returns a skeleton string for the requested message kind. No I/O.
2. **`mix agov.coord.brief [--dir PATH] [--kind KIND] [--from ROLE]
   [--to ROLE] [--topic TOPIC]`** — scans channel via `ChannelReader`,
   derives next seq, calls `BriefGenerator`, prints to stdout.

No new schema changes. No new message kinds. Read-only channel access.

---

## Affected files

| File | Action |
|---|---|
| `tooling/elixir/lib/agov/brief_generator.ex` | Create — pure skeleton renderer |
| `tooling/elixir/lib/mix/tasks/agov.coord.brief.ex` | Create — Mix task |
| `tooling/elixir/test/agov/brief_generator_test.exs` | Create — unit tests |

---

## Output format

```
# Channel: 21 messages, seq 1–21
# Ownership: 3 active claims (run agov coord status --roles for details)
# Next seq: 22
---
kind: note
from: <from>
to: <to>
seq: 22
timestamp: <TIMESTAMP>
immutable: true
references-to: []
topic: <topic>
requires-ack: false
artifacts_owned: []
---

# <message body>
```

The `#`-prefixed lines are Markdown comments (rendered as headings in a Markdown
viewer, silently ignored by YAML parsers reading from `---`). They provide
orientation without polluting the YAML block.

Placeholder tokens (`<from>`, `<to>`, `<TIMESTAMP>`, `<topic>`) are left literal
in the output when not supplied via flags. The agent replaces them before saving.

---

## Kind-specific skeletons

### note (default)
Standard fields only. No extra sections.

### handoff
Adds `handoff:` block:
```yaml
handoff:
  next_agent: <to>
  status: ready
  task: []
  must_read: []
  do_not_touch:
    - .claude/settings.local.json
    - agents-govern/uv.lock
```

### claim
`artifacts_owned` promoted to front with reminder comment:
```yaml
artifacts_owned:
  # list paths you are claiming ownership of
  - <path>
```

### completion
`artifacts_owned` promoted with reminder comment:
```yaml
artifacts_owned:
  # list paths being released; [] releases all by this sender
  []
```

### ack
`references-to` promoted with reminder comment:
```yaml
references-to:
  # seq filename(s) this ACK acknowledges
  - <filename>
```

### review
Standard fields only; body placeholder cues the reviewer:
```
# <findings — severity: BLOCK | WARN | INFO>
```

---

## BriefGenerator design

```elixir
defmodule Agov.BriefGenerator do
  @moduledoc false

  @supported_kinds ~w(note handoff claim completion ack review)

  def generate(channel_info, opts \\ []) do
    kind      = Keyword.get(opts, :kind, "note")
    from      = Keyword.get(opts, :from, "<from>")
    to        = Keyword.get(opts, :to, "<to>")
    topic     = Keyword.get(opts, :topic, "<topic>")
    timestamp = Keyword.get(opts, :timestamp, "<TIMESTAMP>")

    if kind not in @supported_kinds do
      {:error, {:unknown_kind, kind}}
    else
      {:ok, render(kind, from, to, topic, timestamp, channel_info)}
    end
  end

  # render/6 builds the summary header + front matter + body placeholder
  # kind-specific extra fields are injected after artifacts_owned
end
```

`channel_info` is `%{next_seq: integer, total: integer, seq_range: nil | {min, max}, active_claims: integer}`.

The Mix task derives `next_seq` as `max_seq + 1` (or `1` if the channel is empty).
`active_claims` comes from `ClaimTracker.scan/2`.

---

## Mix task CLI

```
mix agov.coord.brief [--dir PATH] [--kind KIND] [--from ROLE] [--to ROLE] [--topic TOPIC]
```

| Option | Default | Description |
|---|---|---|
| `--dir` | `../../docs/coordination` | Coordination directory |
| `--kind` | `note` | Message kind skeleton |
| `--from` | `<from>` | Value for `from:` field |
| `--to` | `<to>` | Value for `to:` field |
| `--topic` | `<topic>` | Value for `topic:` field |

Exit codes: `0` always (prints error to stderr on unknown kind or missing dir; still exits 0 for brief output errors — the tool is generative, not a validator).

---

## Implementation plan

### Step 1 — Tests first (TDD RED)

Tests for `BriefGenerator.generate/2`:

| Test | Expected |
|---|---|
| Default kind `note` with `next_seq: 1` | Skeleton contains `seq: 1`, `kind: note`, placeholder tokens |
| `kind: handoff` | Skeleton contains `handoff:` block with `next_agent:` |
| `kind: claim` | Skeleton contains promoted `artifacts_owned:` with reminder comment |
| `kind: completion` | Skeleton contains `artifacts_owned: []` with reminder |
| `kind: ack` | Skeleton contains promoted `references-to:` with reminder |
| `kind: review` | Skeleton contains review body hint |
| Unknown kind | Returns `{:error, {:unknown_kind, "bogus"}}` |
| `next_seq: 42` | Skeleton contains `seq: 42` |
| `from: "planner-architect"` | Skeleton contains `from: planner-architect` |

Tests for Mix task (integration, via captured IO):
- `--kind bogus` prints error to stderr
- Default invocation against fixtures dir produces a valid front matter block

### Step 2 — Implement BriefGenerator (GREEN)

String-interpolation template per kind. Keep it simple: no external template
library; the output is small and the field list is stable.

### Step 3 — Implement Mix task

Calls `ChannelReader.scan/1` to get total/seq_range; calls `ClaimTracker.scan/2`
for active_claims count; derives `next_seq`; calls `BriefGenerator.generate/2`;
prints to stdout.

### Step 4 — Manual verification

```
cd tooling/elixir
mix agov.coord.brief
mix agov.coord.brief --kind handoff --from planner-architect --to governor-reviewer
mix agov.coord.brief --kind claim --topic T1-4-coord-brief
```

Confirm: seq matches current channel state; handoff block present; placeholder
tokens present where not supplied.

---

## Test strategy

| Tier | What | Where |
|---|---|---|
| Unit | All `BriefGenerator.generate/2` cases (7 kinds + unknown + seq + from) | `test/agov/brief_generator_test.exs` |
| Integration | Mix task against fixtures dir; `--kind bogus` error | `test/mix/tasks/agov_coord_brief_test.exs` |
| Manual | `mix agov.coord.brief` variants against live channel | Inspect for correctness |

---

## Risks

| Risk | Mitigation |
|---|---|
| Placeholder tokens look like valid YAML if left in a file | Tokens use angle brackets (`<from>`); YAML parsers accept them as strings; downstream validator will reject them if accidentally left in |
| `do_not_touch` list in handoff skeleton gets stale | Skeleton is a starting point; agent fills in — same situation as all existing templates |
| Channel empty (seq_range nil) | next_seq defaults to 1; output clearly states "0 messages" |
