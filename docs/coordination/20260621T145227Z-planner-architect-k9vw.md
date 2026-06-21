---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260621T145227Z-planner-architect-k9vw
timestamp: 2026-06-21T14:52:27Z
immutable: true
references-to:
  - 20260621T143538Z-coder-tester-drq5.md
topic: docs-governance-safety-fix-slice1
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Fix first-governed-change.md setup prompt to remove premature governance records
    - Fix contracts-and-validators.md CI glob to match all coordination filename formats
  must_read:
    - docs/plans/agf-docs-governance-safety-fix.md
    - agf/docs/site/docs/start/first-governed-change.md
    - agf/docs/site/docs/advanced/contracts-and-validators.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Handoff: Docs governance-safety fix (Slice 1 of 4)

Governor approved the remediation plan for Codex's panel assessment
(`20260621T143538Z-coder-tester-drq5`). Gate 1 plans for slices 1 and 2 are
written and validated. This handoff covers Slice 1.

## What to implement

### 1. `agf/docs/site/docs/start/first-governed-change.md`

In the assistant prompt block (section 2 "Ask your assistant to prepare the
starter files"), remove these three bullet lines:

```
- one Gate 1 decision-record coordination message
- one Gate 2 handoff coordination message
- one completion coordination message
```

Keep in the prompt:
- The plan file (`docs/plans/sample-login-error-message.md`)
- One Gate 1 handoff coordination message
- `agf/learnings/sample-project.yaml`

Steps 4, 6, and 7 already guide the user to create those removed records at
the right moment. No changes to those steps.

Do NOT remove the advisory note "Fill only the fields you can honestly fill
for this trial" — it correctly scopes what AI should fill in.

### 2. `agf/docs/site/docs/advanced/contracts-and-validators.md`

In the CI example script block, change the coordination glob in the
`git diff ... | xargs` invocation from:

```
'docs/coordination/[0-9][0-9][0-9][0-9][0-9]-*.md'
```

to:

```
'docs/coordination/*.md'
```

This is a single-line change. The tooling-and-tokens.md CI example already
uses `docs/coordination/*.md` — this makes the two pages consistent.

## Validation before Gate 2

```bash
uv run --project agents-govern --group docs mkdocs build -f agf/docs/build/mkdocs.yml --strict
```

Must pass with zero errors or warnings (strict mode treats broken links as
errors).

Human review: confirm the setup prompt no longer lists the three removed
records; confirm the CI glob reads `*.md`.

## Branch

Use `docs/governance-safety-fix`.

## Priority

Implement before Slice 2 (factual-claims cleanup). Both slices are
independent of `feat/rust-dist-packaging`.
