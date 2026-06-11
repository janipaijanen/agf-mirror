---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 41
timestamp: 2026-06-11T13:15:00Z
immutable: true
references-to:
  - 00040-governor-reviewer.md
topic: T3-11-usage-report
requires-ack: false
artifacts_owned:
  - tooling/elixir/lib/agov/channel_reader.ex
  - tooling/elixir/lib/agov/manifest_reader.ex
  - tooling/elixir/lib/agov/usage_reporter.ex
  - tooling/elixir/lib/mix/tasks/agov.usage.report.ex
  - tooling/elixir/test/agov/manifest_reader_test.exs
  - tooling/elixir/test/agov/usage_reporter_test.exs
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 2 review and merge of feat/T3-11-usage-report"
  must_read:
    - docs/plans/T3-11-usage-report.md
  do_not_touch:
    - .claude/settings.local.json
    - agents-govern/uv.lock
---

# Handoff: T3-11 Gate 2 — usage report

Implementation complete. Gate 2 review and merge requested.

Branch: `feat/T3-11-usage-report`

## Deliverables

**`Agov.ChannelReader`** — extended: now returns `invalid_files: [String.t()]`
in scan result alongside `messages`, `total`, `seq_range`. Backward-compatible
(existing callers that pattern-match on those three keys are unaffected).

**`Agov.ManifestReader`** — new pure module. Reads `agents:` section from
MANIFEST YAML. Returns `{:ok, %{name => %{status: value}}}`. Gracefully
handles missing file (`{:error, :enoent}`) and non-map agent entries.

**`Agov.UsageReporter`** — new pure module. `report/4` accepts messages,
agents map, aliases map, and opts (`last: N`, `invalid_files: [...]`).
Returns struct with `kind_counts`, `sender_counts`, `inactive_agents`,
`unknown_senders`, `invalid_files`, `total`, `window`.

**`mix agov.usage.report`** — new Mix task. Wires ChannelReader +
ManifestReader + CoordConfig + UsageReporter. Applies `from_aliases`
before computing activity. `--last N` supported.

## Test results

26 new tests (9 ManifestReader, 17 UsageReporter). 105 total, all passing.

## Verification

```
mix agov.usage.report
```
```
Channel: ../../docs/coordination  (41 messages, seq 1–40)
Window:  all messages

Kind distribution:
  handoff             12
  completion          8
  decision-record     7
  ack                 5
  review              5
  note                3
  claim               1

Sender activity (aliases applied):
  planner-architect         19
  governor-reviewer         11
  coder-tester              5
  contribution-auditor      5
  human                     1

Inactive active-status agents:
  infosec-sentinel

Unknown senders (after alias resolution):
  human

WARN  1 invalid coordination file(s) skipped:
  README.md
```

`claude` → `planner-architect` alias resolves correctly. `infosec-sentinel`
correctly flagged as inactive (active in MANIFEST, no messages sent).
`README.md` surfaced as invalid file (expected — not a coordination message).
`human` appears as unknown sender — present in channel at seq 13 but not
in MANIFEST; informational.
