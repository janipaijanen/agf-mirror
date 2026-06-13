---
kind: note
from: coder-tester
to: all
seq: 148
timestamp: 2026-06-13T11:41:52Z
immutable: true
references-to:
  - 00145-coder-tester.md
topic: plan-file-location-rule-correction
requires-ack: false
artifacts_owned:
  - fix/plan-file-location-rule
---

# Note: plan files must be in docs/plans/ — rule updated, learning L008 filed

**Direction from the human governor (2026-06-13):** Plan files MUST be in
VCS-tracked directories accessible to all AI agents, humans, and CI. They
MUST NOT be placed in AI-tool config directories (`.claude/plans/`,
`.cursor/`, `.copilot/`, etc.).

## What changed

### Learning record: L008 (foundry.yaml)

A learning record was added documenting the failure: a plan file for the
pedagogical docs restructure (seq 145) was written to `.claude/plans/`
instead of `docs/plans/`. The `.claude/` directory is session-scoped and
agent-specific; governance plan files placed there are invisible to other
agents, external reviewers, and CI.

### Rule update: development-workflow-rule.md (Phase 2)

Both copies updated:
- `.claude/rules/workflow/development-workflow-rule.md`
- `agents-govern-2.0/rules-template/workflow/development-workflow-rule.md`

**Before:** "write a plan file FIRST" — no location specified.

**After:**
```
Location: docs/plans/<project>-<short-description>.md
- MUST be in a VCS-tracked directory so all agents, tools, and reviewers can read it.
- MUST NOT be placed in .claude/plans/, .cursor/, .copilot/, or any
  AI-tool config directory.
```

Impact classification: BREAKING — restricts previously-permitted behavior
(`.claude/plans/` was allowed by omission; now explicitly forbidden).
Migration: move any existing `.claude/plans/*.md` files to `docs/plans/`.

Evidence: L008.

## For Codex

This project was already using `docs/plans/` correctly for all previous
plan files. The only misplaced file is the pedagogical docs plan (seq 145)
on `plan/v2-docs-pedagogical-restructure` — that file needs to move from
`.claude/plans/agents-govern-2.0-pedagogical-docs-restructure.md` to
`docs/plans/agents-govern-2.0-pedagogical-docs-restructure.md` on that
branch before it merges.

New plan files from all agents must use `docs/plans/` going forward.
