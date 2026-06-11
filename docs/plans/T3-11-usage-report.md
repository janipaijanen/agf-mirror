---
id: T3-11-usage-report
status: pending-gate1
topic: usage-telemetry
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: read-only channel analysis — no security boundary, auth, or secrets changes
  - role: contribution-auditor
    status: not-applicable
    reason: internal tooling only; no upstream contribution or governance schema change
---

# Plan: T3-11 — `mix agov.usage.report`

**Date:** 2026-06-11T11:35:00Z
**Status:** PENDING GATE 1 APPROVAL
**Priority:** Low (Tier 3 — Framework Maintainability)

---

## Intent

The coordination channel and MANIFEST contain evidence of agent activity,
but there is no command to surface it. Over time, role proliferation adds
cold-start reading overhead and governance drift. The question "which roles
have been active recently?" currently requires manual channel inspection.

`mix agov.usage.report` answers it in one command. It parses the
coordination channel and cross-references the MANIFEST to produce:

- Message counts by agent (sender activity)
- Agents in MANIFEST who have never posted (candidates for review/suspension)
- Kind distribution (how the channel is being used)
- Optional: last N messages as the analysis window

This is a **read-only reporting tool** — it never modifies the channel,
MANIFEST, or plan files.

---

## Scope

Two deliverables:

1. **`Agov.UsageReporter` module** — pure analysis. Takes a list of parsed
   coordination messages and a MANIFEST agent map. Returns a structured report.
   No filesystem access.

2. **`mix agov.usage.report [--dir <coord-dir>] [--manifest <path>] [--last N]`**
   Mix task that owns I/O: reads channel via `ChannelReader`, reads MANIFEST
   via a new thin reader, calls `UsageReporter`, prints result.

**Out of scope for this task:**
- Plan-file advisory consultation scanning (requires YAML parsing of plan
  front matter — separate concern, deferred)
- Lifecycle annotations on MANIFEST (additive schema change — useful but
  independent; deferred to follow-on T3-11b if warranted)
- Skill/command invocation tracking (not logged in coordination channel)

---

## Affected files

| File | Action |
|---|---|
| `tooling/elixir/lib/agov/usage_reporter.ex` | Create — pure analysis module |
| `tooling/elixir/lib/agov/manifest_reader.ex` | Create — thin YAML reader for MANIFEST |
| `tooling/elixir/lib/mix/tasks/agov.usage.report.ex` | Create — Mix task |
| `tooling/elixir/test/agov/usage_reporter_test.exs` | Create — unit tests |
| `tooling/elixir/test/agov/manifest_reader_test.exs` | Create — unit tests |

---

## Design

### `Agov.ManifestReader`

Reads `agents-govern/agents/MANIFEST.yaml` (or a given path) and returns
a map of `agent_name => %{status: "active" | "suspended" | "skipped"}`.

```elixir
defmodule Agov.ManifestReader do
  def load(manifest_path) :: {:ok, %{String.t() => %{status: String.t()}}} | {:error, term()}
end
```

Reads only the `agents:` section. Ignores all other top-level keys.
Returns `{:error, :enoent}` if the file doesn't exist. Does not validate
agent schema — callers use what they get.

### `Agov.UsageReporter`

Pure module. Input: `messages` (list of parsed message maps) and
`agents` (map from `ManifestReader`). Optional `opts: [last: N]`.

```elixir
defmodule Agov.UsageReporter do
  def report(messages, agents, opts \\ [])
  # returns %{
  #   total: integer,
  #   window: :all | {:last, N},
  #   kind_counts: %{String.t() => integer},
  #   sender_counts: %{String.t() => integer},
  #   inactive_agents: [String.t()],    # in MANIFEST as active/suspended but never sent
  #   unknown_senders: [String.t()]     # appear in channel but not in MANIFEST
  # }
end
```

`inactive_agents` — MANIFEST agents with `status: active` whose name does
not appear as `from:` in any message in the window. These are candidates
for suspension review.

`unknown_senders` — agent names appearing as `from:` in the channel but
not present in the MANIFEST. Expected to be empty in a well-maintained
project; useful to detect stale agent names or unrenamed senders.

When `last: N` is given, only the last N messages (by seq, descending)
are analysed. Total message count and window are reported separately.

### `mix agov.usage.report`

```
$ mix agov.usage.report
Channel: ../../docs/coordination  (34 messages, seq 1–34)
Window: all messages

Kind distribution:
  completion        8
  handoff           7
  decision-record   5
  review            4
  ack               4
  claim             3
  note              2
  fast-track-waiver 1

Sender activity:
  governor-reviewer   14  (active)
  planner-architect   12  (active)
  coder-tester         5  (active)
  contribution-auditor 2  (active)
  infosec-sentinel     1  (active)

Inactive active-status agents (no messages sent):
  (none)

Unknown senders (in channel, not in MANIFEST):
  claude  (appears in seq 1–4; pre-alias era)
```

Default `--dir` is `../../docs/coordination`. Default `--manifest` is
`../../agents-govern/agents/MANIFEST.yaml`. Both resolve relative to
`tooling/elixir/` (consistent with other commands).

`--last N` narrows the analysis window to the last N messages by seq.

Exit codes: 0 always (reporting tool; no error states beyond dir/manifest
not found, which exits 1).

---

## Test strategy

| Tier | What | Where |
|---|---|---|
| Unit | `ManifestReader.load/1` — valid file, missing file, agents section | `test/agov/manifest_reader_test.exs` |
| Unit | `UsageReporter.report/3` — counts, inactive detection, unknown senders, windowing | `test/agov/usage_reporter_test.exs` |
| Manual | `mix agov.usage.report` against live channel + MANIFEST | Verify counts match expected |

TDD order: `ManifestReader` tests → implementation → `UsageReporter` tests
→ implementation → Mix task → manual verification.

---

## Test cases for `UsageReporter`

| Scenario | Expected |
|---|---|
| Agent in MANIFEST (active) with messages | Appears in sender_counts, not in inactive_agents |
| Agent in MANIFEST (active) with no messages | Appears in inactive_agents |
| Agent in MANIFEST (suspended/skipped) with no messages | NOT in inactive_agents — only active-status agents flagged |
| Sender in channel not in MANIFEST | Appears in unknown_senders |
| `last: N` narrows to N most recent messages | window = {:last, N}; counts reflect only those messages |
| Empty message list | All counts zero; all active MANIFEST agents in inactive_agents |
| Empty MANIFEST agents map | No inactive_agents; all senders in unknown_senders |

---

## Risks

| Risk | Mitigation |
|---|---|
| MANIFEST path varies between projects | `--manifest` flag; sensible default relative to `tooling/elixir/` |
| `from:` in channel uses pre-alias names | Shown as `unknown_senders` — informational, not an error |
| MANIFEST grows new status values | `inactive_agents` only flags `status: active`; new statuses are transparent |

---

## Implementation plan

### Step 1 — ManifestReader tests + implementation

Parse `agents:` section from YAML. Map to `%{name => %{status: value}}`.
Handle missing file gracefully.

### Step 2 — UsageReporter tests + implementation

Pure function. All scenarios from test-cases table. Fixed message lists
in tests — no filesystem access.

### Step 3 — Mix task

Wire ManifestReader + ChannelReader + UsageReporter. Format and print.
Verify manually against live channel.
