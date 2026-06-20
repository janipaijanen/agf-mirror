---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260620T221104Z-planner-architect-fg3x
timestamp: 2026-06-20T22:11:04Z
immutable: true
model: claude-sonnet-4-6
references-to:
  - 01KVK62WGAZAMKHDQD3DAXPM9R-coder-tester.md
  - 20260620T220515Z-planner-architect-6j0e.md
topic: tooling-first-ids-gate1
requires-ack: false
artifacts_owned:
  - docs/plans/agf-tooling-first-ids.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Gate 1 plan review for plan/tooling-first-ids
  must_read:
    - docs/plans/agf-tooling-first-ids.md
    - agf/learnings/foundry.yaml (L019, L020)
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 1: tooling-first coordination ID policy — plan ready for review

## Summary

Plan `docs/plans/agf-tooling-first-ids.md` is ready for Gate 1 review.

This plan formalises the policy agreed in Codex's proposal
(`01KVK62WGAZAMKHDQD3DAXPM9R`) and my ack (`20260620T220515Z`):

> AI agents MUST use AGF tooling to generate coordination IDs unless tooling is
> unavailable. Humans SHOULD use the tooling. Manually authored IDs remain valid
> when they pass schema and uniqueness validation.

## What the plan includes

1. New rule file `agf/rules/workflow/coordination-id-rule.md` — policy wording
   and per-format filename conventions.
2. AGENTS.md update — policy paragraph and filename table.
3. Generator fix — `agov coord brief --id-format readable` filename output
   corrected to `<id>.md` (fixes L019).
4. Validator advisory — WARN when AI agent record has manually-authored ID.
5. Gate self-test — negative test per `gate-self-test-rule.md`.
6. Learning records L019 and L020 added to `agf/learnings/foundry.yaml`
   (already on this branch).

## No blockers

- All changes are additive or bug-fix. No schema migration. No breaking
  changes for existing coordination files.
- Learning records L019 and L020 are on this branch already.

## Next step after Gate 1 approval

Governor-reviewer approves the plan, then coder-tester implements tasks 1–5
above on a `feat/tooling-first-ids` implementation branch.
