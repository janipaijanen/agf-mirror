# plan: agent improvement-proposal norm (Gate 1)

Branch: `plan/agent-improvement-proposal-norm`

## Status

- **Gate 1 APPROVED** — `docs/coordination/20260720T160045Z-governor-reviewer-5h8j.md`.
- Ready to merge. Implementation (one small MR, no MR-B) follows on a fresh
  branch after this merges.

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

Codex's independent advisory review agreed with all three of the plan's
proposed answers; the governor's decision adopts them as-is (SHOULD,
`agf/rules/always/`, both AGF-scope and project-local observations in
scope for proposing).

## Files changed

| File | Change |
|---|---|
| `docs/plans/agent-improvement-proposal-norm.md` | Gate 1 plan, now `status: approved` |
| `docs/coordination/20260720T153719Z-planner-architect-bqd8.md` | Gate 1 handoff to governor-reviewer |
| `docs/coordination/20260720T154019Z-coder-tester-sknh.md` | Codex review supporting Gate 1 approval, with a docs-link implementation note |
| `docs/coordination/20260720T160045Z-governor-reviewer-5h8j.md` | Gate 1 decision-record: approved |

## Gate 1 answers

1. **Strength: SHOULD**, not MUST.
2. **Landing directory: `agf/rules/always/`.**
3. **Scope: both AGF-scope and project-local observations** are in scope for
   proposing; project-local ideas still need governor review to become AGF
   framework rules.

## Implementation condition

Per Codex's review, the `day-to-day.md` reference to the new rule file
MUST NOT be a raw markdown link across the MkDocs `docs_dir` boundary —
render as inline code text, or prove the link resolves under
`mkdocs build --strict` first.

## Review checklist

- [x] `agov plan validate docs/plans/agent-improvement-proposal-norm.md` passes
- [x] `agov coord validate` passes on all coordination files
- [x] Contribution-auditor self-review included in the plan (no blocking findings)
- [x] Codex review included (no blocking findings)
- [x] No `Co-authored-by` trailers in commit
- [x] Does not touch the MR-2 branch or any already-merged MR's files
