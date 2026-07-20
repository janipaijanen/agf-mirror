# plan: agent improvement-proposal norm (Gate 1)

Branch: `plan/agent-improvement-proposal-norm`

## Status

- **Pending:** Gate 1 decision by governor-reviewer
  (`docs/coordination/20260720T153719Z-planner-architect-bqd8.md`).
- **Blocking Claude:** nothing. Next action (implementation, if approved)
  arrives as a pushed coordination decision-record.

## Summary

Responds to Codex's handoff (`docs/coordination/20260720T152331Z-coder-tester-prjn.md`),
which recorded the governor-reviewer's in-session directive that agents may
propose improvements when they notice friction, oddness, or token-saving
opportunities — asked to be turned into a small, non-blocking rule-ish norm.

Plan proposes one new SHOULD-strength (Provisional) rule,
`agf/rules/always/agent-improvement-proposal-rule.md` (full exhibit text in
the plan file), plus one short paragraph in
`agf/docs/site/docs/operate/day-to-day.md`. No new coordination message kind
— routes through the existing `note` / `review` / `lesson-proposal` / Gate 1
`plan` kinds, already declared in the schema.

## Files changed

| File | Change |
|---|---|
| `docs/plans/agent-improvement-proposal-norm.md` | New Gate 1 plan, with the draft rule and doc paragraph as exhibits |
| `docs/coordination/20260720T153719Z-planner-architect-bqd8.md` | Gate 1 handoff to governor-reviewer |
| `docs/coordination/20260720T154019Z-coder-tester-sknh.md` | Codex review supporting Gate 1 approval, with a docs-link implementation note |

## Open questions (see plan for detail)

1. SHOULD vs. MUST strength?
2. Landing directory: `agf/rules/always/` vs. `agf/rules/workflow/`?
3. Is project-local (non-AGF-scope) friction explicitly in scope, or left to a project's own local rules?

## Review checklist

- [x] `agov plan validate docs/plans/agent-improvement-proposal-norm.md` passes
- [x] `agov coord validate` passes on the handoff and Codex review
- [x] Contribution-auditor self-review included in the plan (no blocking findings)
- [x] Codex review included (no blocking findings)
- [x] No `Co-authored-by` trailers in commit
- [x] Does not touch the MR-2 branch or any already-merged MR's files
