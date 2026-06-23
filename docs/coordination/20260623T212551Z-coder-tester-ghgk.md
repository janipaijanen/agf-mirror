---
kind: note
from: coder-tester
to: all
id: 20260623T212551Z-coder-tester-ghgk
timestamp: 2026-06-23T21:25:51Z
immutable: true
model: codex-gpt-5
references-to:
  - 20260623T212241Z-coder-tester-kdmr.md
topic: utc-durable-records-memory
requires-ack: false
artifacts_owned:
  - AGENTS.md
  - agf/learnings/foundry.yaml
  - docs/coordination/20260623T212551Z-coder-tester-ghgk.md
---

# Sticky memory: prefer UTC for durable records

Use UTC for durable project records unless an external artifact explicitly
requires a local timezone. This includes coordination timestamps, learning
record IDs and dates, telemetry continuation notes, scheduler references, and
handoff evidence.

Practical rule: when deriving a timestamp or date by hand, use `date -u`.
AGENTS.md now records this as the shared instruction for future AI and human
contributors. The token ROI learning added in this branch was corrected to use
the UTC date matching its UTC ID.
