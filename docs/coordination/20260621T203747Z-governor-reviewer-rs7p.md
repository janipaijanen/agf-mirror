---
kind: decision-record
from: governor-reviewer
to: all
id: 20260621T203747Z-governor-reviewer-rs7p
timestamp: 2026-06-21T20:37:47Z
immutable: true
references-to:
  - 20260621T202628Z-coder-tester-bjdh.md
topic: coordination-id-wording-gate2-approved
requires-ack: false
artifacts_owned:
  - docs/coordination/20260621T203747Z-governor-reviewer-rs7p.md
decision:
  outcome: approved
  branch: docs/coordination-id-wording
  merged_at: 036ef78
---

# Gate 2 retroactive approval — docs/coordination-id-wording

Retroactive Gate 2 review of `docs/coordination-id-wording`, merged at `036ef78`.
Fast-track basis: docs-only changes to two public pages, no nav, schema, CI,
or security boundary change. Reviewing against the `bjdh` handoff
`20260621T202628Z-coder-tester-bjdh.md`.

## Review

**Scope check (fast-track validity)**

Two files changed: `agf/docs/site/docs/operate/day-to-day.md` and
`agf/docs/site/future/evolution.md`. No nav entry, no schema, no tooling,
no security boundary. Fast-track basis confirmed.

**day-to-day.md: coordination ID policy update**

The "Sequence numbers are sacred" bullet is replaced with the current rule:
new entries use `agov coord brief --id-format readable`; historical
sequence-numbered and ULID files remain immutable; manual IDs are
recovery-only with explanation. This accurately reflects the readable-ID
policy adopted in v0.47.0+ and removes guidance that would actively
mis-train adopters and agents. Correct.

**evolution.md: validator description updated**

"Sequence continuity" changed to "ID uniqueness" and "sequence anomalies"
changed to "coordination-ID anomalies". Accurate: the validator hypothesis
applies to all ID types now, not only sequence numbers. Correct.

## Decision

**Approved.** Both changes are accurate, timely, and improve the honest
communication of the current coordination-ID policy. Acks Gate 2 handoff
`20260621T202628Z-coder-tester-bjdh.md`.
