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
    status: consulted
    reason: reviewed at seq 36 (contribution-auditor, codex-gpt-5); all findings incorporated
---

# Plan: T3-11 — `mix agov.usage.report`

**Date:** 2026-06-11T11:35:00Z
**Revised:** 2026-06-11 — addresses contribution-auditor review (seq 36)
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

- Message counts by agent (sender activity, aliases resolved)
- Agents in MANIFEST with `status: active` who have never posted
  (candidates for suspension review)
- Kind distribution (how the channel is being used)
- Warning for invalid/skipped coordination files
- Optional: last N messages as the analysis window

This is a **read-only reporting tool** — it never modifies the channel,
MANIFEST, or plan files.

---

## Scope

Two deliverables:

1. **`Agov.UsageReporter` module** — pure analysis. Takes a list of parsed
   coordination messages, a MANIFEST agent map, and a from-aliases map.
   Returns a structured report. No filesystem access.

2. **`mix agov.usage.report [--dir <coord-dir>] [--manifest <path>] [--last N]`**
   Mix task that owns I/O: reads channel via `ChannelReader`, reads MANIFEST
   via a new thin reader, loads aliases via `CoordConfig`, calls `UsageReporter`,
   prints result.

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

**Default path:** `../../agents-govern/agents/MANIFEST.yaml` (relative to
`tooling/elixir/`). This is the **bootstrap default** for the foundry project.
It is not intended as the permanent role-authority source. Once a
project-local role-authority mechanism lands (deferred — see seq 13 note
below), this default should be revisited.

> **Deferred dependency:** seq 13 raised the role-authority question: which
> source is authoritative for agent role definitions once `agents-govern/` is
> no longer the bootstrap reference? T3-11 intentionally reports against the
> bootstrap MANIFEST until that decision lands. `--manifest <path>` allows
> callers to override the default if a project-local authority exists sooner.

### `Agov.UsageReporter`

Pure module. Input: `messages` (list of parsed message maps), `agents`
(map from `ManifestReader`), `aliases` (from-alias map from `CoordConfig`).
Optional `opts: [last: N]`.

```elixir
defmodule Agov.UsageReporter do
  def report(messages, agents, aliases \\ %{}, opts \\ [])
  # returns %{
  #   total: integer,
  #   window: :all | {:last, N},
  #   kind_counts: %{String.t() => integer},
  #   sender_counts: %{String.t() => integer},  # canonical (aliases applied)
  #   inactive_agents: [String.t()],            # status: active, never sent in window
  #   unknown_senders: [String.t()],            # in channel but not in MANIFEST after alias resolution
  #   invalid_files: [String.t()]               # filenames skipped by ChannelReader
  # }
end
```

**Alias resolution:** before computing `sender_counts` and `inactive_agents`,
apply `aliases` (from `CoordConfig.from_aliases`) to each message's `from:`
field. A `from: claude` with alias `claude => planner-architect` counts as
`planner-architect` in `sender_counts`. `unknown_senders` lists names that
remain unresolved after alias application — these are genuinely unknown.

**`inactive_agents`** — MANIFEST agents with `status: active` whose
canonical name does not appear as `from:` in any message in the analysis
window. `status: suspended` and `status: skipped` agents are excluded from
this list. Only active-status agents are flagged as candidates for review.

**`unknown_senders`** — canonical agent names (after alias resolution)
appearing as `from:` in the channel but absent from the MANIFEST. Expected
to be empty in a well-maintained project. Presence signals a stale sender
name or a missing MANIFEST entry.

**`--last N` ordering:** sort all messages by `seq` ascending, then by
filename ascending (deterministic tiebreak for seq collisions). Take the
last N messages in that ordering. The window boundary never falls inside a
collision group because filename ordering is stable.

**`invalid_files`** — filenames that `ChannelReader` skipped due to
validation failure. These are passed through from the ChannelReader result
and included in the report. The Mix task prints a warning line for each.

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

Sender activity (aliases applied):
  governor-reviewer    14  (active)
  planner-architect    12  (active)
  coder-tester          5  (active)
  contribution-auditor  2  (active)
  infosec-sentinel      1  (active)

Inactive active-status agents (no messages sent):
  (none)

Unknown senders (after alias resolution):
  (none)

Invalid coordination files: none
```

Default `--dir` is `../../docs/coordination`. Default `--manifest` is
`../../agents-govern/agents/MANIFEST.yaml`. Default `--config` resolves
via `CoordConfig` (reads `agov-config.yaml` in the coord dir). All paths
resolve relative to `tooling/elixir/` (consistent with other commands).

`--last N` narrows the analysis window to the last N messages by the
ordering defined above.

Exit codes:
- `0` — report generated successfully
- `1` — input/read/parse error (coord dir not found, manifest not found,
  unreadable files)

---

## Test strategy

| Tier | What | Where |
|---|---|---|
| Unit | `ManifestReader.load/1` — valid file, missing file, agents section | `test/agov/manifest_reader_test.exs` |
| Unit | `UsageReporter.report/4` — counts, alias resolution, inactive detection, unknown senders, windowing, invalid_files passthrough | `test/agov/usage_reporter_test.exs` |
| Manual | `mix agov.usage.report` against live channel + MANIFEST | Verify counts match expected |

TDD order: `ManifestReader` tests → implementation → `UsageReporter` tests
→ implementation → Mix task → manual verification.

---

## Test cases for `UsageReporter`

| Scenario | Expected |
|---|---|
| Agent in MANIFEST (active) with messages | Appears in sender_counts, not in inactive_agents |
| Agent in MANIFEST (active) with no messages | Appears in inactive_agents |
| Agent in MANIFEST (suspended) with no messages | NOT in inactive_agents — only active-status agents flagged |
| Agent in MANIFEST (skipped) with no messages | NOT in inactive_agents |
| Sender in channel resolves via alias to known agent | Counted under canonical name; not in unknown_senders |
| Sender in channel with no alias, not in MANIFEST | Appears in unknown_senders |
| `last: N` narrows to last N by seq+filename order | window = {:last, N}; counts reflect only those messages |
| Seq collision at window boundary | All messages with that seq included (filename ordering is stable) |
| Empty message list | All counts zero; all active MANIFEST agents in inactive_agents |
| Empty MANIFEST agents map | No inactive_agents; all senders (after alias) in unknown_senders |
| Non-empty `invalid_files` passed in | Forwarded to report struct; Mix task prints warning |

---

## Risks

| Risk | Mitigation |
|---|---|
| MANIFEST path varies between projects | `--manifest` flag; default documented as bootstrap default, not permanent authority |
| Role-authority decision (seq 13) changes the right default | Deferred note in design; `--manifest` flag provides override path |
| `from:` in channel uses pre-alias names | Resolved via `CoordConfig.from_aliases`; unresolved names surface in `unknown_senders` |
| MANIFEST grows new status values | `inactive_agents` only flags `status: active`; new statuses are transparent |
| `ChannelReader` skips invalid files silently | `invalid_files` field surfaces skips; Mix task warns |
| Seq collisions at `--last N` boundary | Ordering is seq+filename; window boundary is deterministic |

---

## Implementation plan

### Step 1 — ManifestReader tests + implementation

Parse `agents:` section from YAML. Map to `%{name => %{status: value}}`.
Handle missing file gracefully.

### Step 2 — UsageReporter tests + implementation

Pure function. All scenarios from test-cases table. Fixed message lists
in tests — no filesystem access. Alias map injected as parameter.

### Step 3 — Mix task

Wire ManifestReader + ChannelReader + CoordConfig + UsageReporter.
Format and print. Verify manually against live channel.
