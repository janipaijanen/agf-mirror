---
id: agf-docs-governance-safety-fix
status: approved
topic: docs-governance-safety-fix
gate1_approved_by: "governor-reviewer"
gate1_approved_at: "2026-06-21T14:51:16Z"
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: documentation-only; no secrets, auth, or security boundary changes
  - role: contribution-auditor
    status: not-applicable
    reason: documentation-only; no code path, role definition, or governance structure changes
---

# Plan: Docs governance-safety fix (Slice 1 of 4)
#
# Timestamp: 2026-06-21T14:51:16Z
# Status: gate1-approved
# Source: Codex panel assessment 20260621T143538Z-coder-tester-drq5;
#         governor approval in-session 2026-06-21

## Intent

Fix two governance-safety issues in the public documentation that could train
adopters to create premature or missing governance records:

1. The first-governed-change setup prompt asks the assistant to create a Gate 1
   decision record, a Gate 2 handoff, and a completion message before those
   events exist. These records belong in `docs/coordination/` — the live append-
   only channel — and filing them before their triggering events trains adopters
   to see premature records as normal. Remove those three items from the setup
   prompt; the subsequent steps already guide the user to create each record at
   the right moment.

2. The CI validation example in contracts-and-validators.md uses the glob
   `docs/coordination/[0-9][0-9][0-9][0-9][0-9]-*.md`, which matches only the
   legacy five-digit sequence format and silently skips readable
   (`YYYYMMDDThhmmssZ-<role>-<suffix>.md`) and ULID coordination files. Adopters
   copying this example could run validation that silently ignores their current-
   format coordination files.

## Affected files

- `agf/docs/site/docs/start/first-governed-change.md`
- `agf/docs/site/docs/advanced/contracts-and-validators.md`

## Changes

### first-governed-change.md

In the assistant prompt (section 2), remove these three items:
- `one Gate 1 decision-record coordination message`
- `one Gate 2 handoff coordination message`
- `one completion coordination message`

Keep in the prompt:
- `docs/plans/sample-login-error-message.md` (the plan)
- `one Gate 1 handoff coordination message`
- `agf/learnings/sample-project.yaml`

Steps 4, 6, and 7 already correctly describe creating/filing each record at
the right event. No changes to those steps needed.

### contracts-and-validators.md

In the CI example script (the `git diff ... | xargs` line for coordination),
replace:
```
'docs/coordination/[0-9][0-9][0-9][0-9][0-9]-*.md'
```
with:
```
'docs/coordination/*.md'
```

This picks up all coordination filename formats (readable, ULID, and legacy
sequence) and matches the tooling-and-tokens.md CI example, which already
uses `docs/coordination/*.md`.

## Migration

No data migration. Documentation-only changes.

## Test strategy

- `uv run --project agents-govern --group docs mkdocs build -f agf/docs/build/mkdocs.yml --strict` must pass (verifies no broken internal links)
- Human review: confirm the first-governed-change prompt no longer asks for
  future records; confirm the CI glob is `*.md`

## Risks

Low. Both changes remove or simplify content; no new instructions are
introduced that could be misread. The glob change is strictly more inclusive:
it validates more files, never fewer.

## Rollback

Revert the branch before merge.
