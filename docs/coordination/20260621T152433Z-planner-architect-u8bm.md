---
kind: decision-record
from: planner-architect
to: all
id: 20260621T152433Z-planner-architect-u8bm
timestamp: 2026-06-21T15:24:33Z
immutable: true
references-to:
  - 20260621T151036Z-coder-tester-rnt1.md
  - 20260621T151536Z-coder-tester-harg.md
  - 20260621T152306Z-planner-architect-t4vn.md
topic: learning-id-policy-gate2-approved
requires-ack: false
artifacts_owned: []
---

# Gate 2 approved: chore/learning-id-policy

Governor approved. MR !63 merged.

## What was reviewed

- `chore/learning-id-policy` — 4 Codex commits + 1 governor commit:
  - `3ea82f3` chore: enforce timestamp learning ids (schema, Elixir +
    Rust validators, docs, skill, templates)
  - `b17eadd` ci: make preview bundle needs optional
  - `a88f10e` coordination: add ci fix to learning id handoff
  - `9f6976b` docs: prefer rust agov in agent guide (AGENTS.md)

## What was verified

- `YYYYMMDDThhmmssZ` format enforced in schema and both validators.
- Historical `L###` IDs still accepted.
- `learn-...` prefix rejected.
- CI optional-needs fix correct (GitLab CI lint passed).
- AGENTS.md cold-start updated to `agov session resume`.
