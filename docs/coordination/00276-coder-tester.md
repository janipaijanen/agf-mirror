---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 276
timestamp: 2026-06-13T22:16:00Z
immutable: true
references-to:
  - 00241-planner-architect.md
topic: gate2-provenance-enum-schema-consolidation
requires-ack: false
artifacts_owned:
  - agf/schemas/rule.schema.json
  - agf/schemas/agent.schema.json
model: claude
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review commit `4415892` on branch `feat/tier1-provenance-labels`.
    - Decide Gate 2 for provenance enum schema consolidation.
    - Confirm whether the schema-only fast-track note is accepted.
  must_read:
    - docs/coordination/00276-coder-tester.md
    - agf/schemas/rule.schema.json
    - agf/schemas/agent.schema.json
  do_not_touch:
    - agents-govern/
    - agents-govern/agents/MANIFEST.yaml
    - agents-govern/learnings/foundry.yaml
---

Gate 2 handoff: provenance enum consolidated into rule and agent schemas.

Branch: `feat/tier1-provenance-labels`
Commit: `4415892`

## What changed

Added `provenance` property to `agf/schemas/rule.schema.json` and
`agf/schemas/agent.schema.json` with the four canonical enum values
from PA seq 241 (Q1):

- `inherited-v1x-unverified` — carried from v1.x, no AGF v2 review
- `adapted-from-v1x` — touched during v2 development, not reviewed
- `verified-agf-v2` — explicitly reviewed against AGF v2 operating model
- `foundry-native` — created natively for AGF v2

## Why

`rule.schema.json` had `additionalProperties: false` without a `provenance`
property, making the 41 rules that carry `provenance:` in their front
matter technically schema-invalid. `agent.schema.json` lacked the field
entirely. Both schemas now formally accept the provenance field with
constrained enum values.

## Validation

All 41 existing rule provenance values and all 12 agent provenance values
pass the new enum constraint. The 40 pre-existing rule failures (missing
required `id`/`scope`/`applies_to`/`evidence` in inherited minimal-
frontmatter rules) are unchanged — these are a separate gap, not
introduced by this change.

Elixir suite: 129 tests, all passed.

## Fast-track note

Schema-only, no behaviour change, no migration needed. Eligible per
simplicity-rule fast-track waiver.
