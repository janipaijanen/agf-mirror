---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260621T162404Z-planner-architect-sdej
timestamp: 2026-06-21T16:24:04Z
immutable: true
references-to:
  - 20260621T151501Z-planner-architect-q7bx.md
  - 20260621T160752Z-planner-architect-bmdg.md
topic: docs-language-polish-slice4-gate2
requires-ack: false
artifacts_owned:
  - docs/coordination/20260621T162404Z-planner-architect-sdej.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review docs/language-polish branch (1 commit) for Gate 2 approval
    - Merge if approved (after Slice 3 merges to avoid mkdocs.yml conflict)
  must_read:
    - MR_DESCRIPTION.md (on docs/language-polish branch)
  do_not_touch:
    - docs/coordination/
---

# Gate 2 handoff: docs/language-polish (Slice 4)

Slice 4 of 4 — governor-approved remediation.
Branch: `docs/language-polish` (1 commit ahead of main)

**Note:** Slice 4 touches `mkdocs.yml` (nav reorder) and Slice 3 also
touches it (markdown_extensions). They edit different sections and do not
conflict, but merging Slice 3 first avoids any potential conflict.

## What was built

Four idiom replacements:
- `should-you-adopt.md`: "are theatre" → "become ceremony"
- `adopter-faq.md`: "gives your review a spine" → "gives your review structure"
- `first-governed-change.md`: "chain is the product" → "chain is the audit trail"
- `downstream-impact.md`: "are theatre" → "become ceremony"

Adopt nav reorder in `mkdocs.yml` and `docs/index.md`:
- Downstream impact and Exit now appear before Install

Coordination filename guidance in `concepts/roles-and-gates.md`:
- Readable IDs documented as current preferred convention

## Verification

- `mkdocs build --strict`: clean (confirmed)

## MR

https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/new?merge_request%5Bsource_branch%5D=docs%2Flanguage-polish
