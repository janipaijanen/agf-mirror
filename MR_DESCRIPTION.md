# Gate 1: tooling-first coordination ID policy — plan for review

## Summary

This MR submits the Gate 1 plan for formalising the tooling-first coordination
ID policy. It adds learning records L019 and L020 and the plan file that scopes
the implementation work for coder-tester.

**Policy to be enshrined:**

> AI agents MUST use AGF tooling to generate coordination IDs unless tooling is
> unavailable. Humans SHOULD use the tooling. Manually authored IDs remain valid
> when they pass schema and uniqueness validation.

## What is in this MR

- `docs/plans/agf-tooling-first-ids.md` — Gate 1 plan
- `agf/learnings/foundry.yaml` — L019 (filename convention bug) and L020
  (coordination discipline on context-resume)
- `docs/coordination/20260620T221104Z-planner-architect-fg3x.md` — Gate 1
  handoff to governor-reviewer
- `docs/coordination/20260620T220515Z-planner-architect-6j0e.md` — ack of
  Codex's proposal (previous commit)

## Background

MR !32 (`plan/readable-coordination-ids`) merged readable coordination IDs to
main. Codex filed a proposal (`01KVK62WGAZAMKHDQD3DAXPM9R`) to formalise a
tooling-first policy for ID generation. Planner-architect acked and committed to
drafting this Gate 1 plan.

This plan also covers the generator bug (L019) where `agov coord brief
--id-format readable` appended a redundant `-<role>` suffix to the output
filename. Two historical files with wrong suffixes are immutable on main.

## Scope (for Gate 1 review — no implementation in this MR)

The plan decomposes the implementation into 6 tasks for coder-tester:

1. New `agf/rules/workflow/coordination-id-rule.md`
2. AGENTS.md update (policy + filename convention table)
3. Generator fix — `agov coord brief --id-format readable` → `<id>.md`
4. Validator advisory — WARN on AI-authored manual IDs
5. Gate self-test (per `gate-self-test-rule.md`)
6. Learning records L019/L020 — already done on this branch

## Gate 1 checklist

- [x] Plan file in `docs/plans/` with timestamp from OS
- [x] Affected files list complete
- [x] Migration needed: No
- [x] Test strategy stated
- [x] Risks assessed: Low
- [x] Rollback path stated
- [x] Downstream impact: EDITORIAL
- [x] Decomposition into implementable tasks
- [x] Gate 1 handoff coordination message filed
