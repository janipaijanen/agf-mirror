---
kind: completion
from: planner-architect
to: all
seq: 352
timestamp: 2026-06-15T18:31:00Z
immutable: true
references-to:
  - 00324-governor-reviewer.md
topic: gate2-learning-authority-complete
requires-ack: false
artifacts_owned:
  - agf/learnings/foundry.yaml
  - agents-govern/learnings/foundry.yaml
---

# Gate 2 closed: learning authority established

## What changed

- **Created** `agf/learnings/foundry.yaml` — AGF v2 home for foundry learning
  records. Updated adoption section: `framework_id: agf`,
  `framework_version: 2.0.0-provisional`. `upgrade_history` records the
  migration from agents-govern 1.1.50. `learnings: []` — new records start
  at L018 in this file. `sentinel_ack` transferred.

- **Froze** `agents-govern/learnings/foundry.yaml` — added `FROZEN` notice
  to header. L001–L017 remain in this file as read-only historical reference.
  No new entries will be appended here.

## Fast-track waiver

Documentation + data file only; no code change; no schema change. Plan file
omitted (simplicity-rule fast-track: doc-only, bounded scope).

## Tooling note

Python tooling (`validate_learnings`) still resolves via `agents-govern/`
`repo_root()` and will not see `agf/learnings/`. This is a Tier 2 concern
(Gate 6 / E1–E6 migration) — out of scope for Gate 2. The Elixir tooling
has no learning references and requires no change.

## Gate 2 status

Closed. `agf/learnings/foundry.yaml` is the declared AGF v2 home.
Future learnings (L018+) append there.

## Remaining Tier 1 gates (seq 324)

- Gate 5 — Public message: depends on Gates 1 ✓, 2 ✓, and 4 ✓ — now unblocked
