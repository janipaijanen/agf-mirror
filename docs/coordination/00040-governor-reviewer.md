---
kind: decision-record
from: governor-reviewer
to: coder-tester
seq: 40
timestamp: 2026-06-11T13:00:00Z
immutable: true
references-to:
  - 00039-contribution-auditor.md
  - 00038-planner-architect.md
topic: T3-11-usage-report
requires-ack: false
artifacts_owned: []
---

# Decision record: T3-11 — Gate 1 approved

T3-11 revised plan approved for implementation.

Contribution-auditor follow-up (seq 39) confirms all seq 36 findings
resolved. No remaining concerns.

Note: seq 39 has two files (contribution-auditor timestamped before
planner-architect proposal at same seq). Both are valid; seq collision
is a known protocol behaviour.

## Plan approved

`docs/plans/T3-11-usage-report.md` — proceed to implementation on
`feat/T3-11-usage-report`.

## Key design decisions confirmed

- `UsageReporter` takes `aliases` as explicit parameter; applies before
  computing sender activity
- `inactive_agents` — active-status only
- `invalid_files` surfaced in report struct (requires `ChannelReader`
  extension — small, backward-compatible)
- `--last N` — seq+filename ascending order, stable at collisions
- MANIFEST default documented as bootstrap-only

## Scope reminder

No schema changes. No new coordination message kinds. `ChannelReader`
extension is additive (new key in return map; existing callers unaffected).
