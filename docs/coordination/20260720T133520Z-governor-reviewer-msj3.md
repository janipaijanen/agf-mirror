---
artifacts_owned: []
from: governor-reviewer
id: 20260720T133520Z-governor-reviewer-msj3
immutable: true
kind: note
model: codex-gpt-5
references-to:
- 20260720T130837Z-planner-architect-phm5.md
- 20260720T132930Z-governor-reviewer-13p2.md
requires-ack: false
timestamp: 2026-07-20T13:35:20Z
to: all
topic: docs-audience-boundary-work-split
---

# Conditional work split for documentation audience-boundary plan

Codex-scribed record of governor-reviewer direction to split the planned work
across available AI services, including Claude, while preserving AGF role and
gate boundaries.

This split is conditional on advisory review and final Gate 1 approval of
MR !227. It does not authorize implementation before the approval record lands.

## Before final Gate 1

- `contribution-auditor`: Claude may take this advisory review in a separate
  role/session, if available. Focus on adoption-claim overstatement, scope
  hygiene, and folding prior docs plans into the current plan.
- `fact-checker`: Codex or another separate AI session may take this advisory
  review. If Claude is used, do not reuse the same planner session that authored
  the plan. Focus on page counts, audience labels, prior-plan status, and figure
  claims.
- `planner-architect`: Claude remains the plan steward and incorporates
  advisory findings or routes blockers back to governor-reviewer.

## After final Gate 1, if approved

- Claude as `coder-tester`: MR-1 taxonomy callout plus docs audience map; MR-3
  `first-governed-change.md` per-step markers; MR-5 new
  `session-continuity.md` page.
- Codex as `coder-tester`: MR-2 grep-driven audience relabeling, excluding
  MR-3's page; MR-4 `install-and-layouts.md` split; MR-6 prior-plan
  reconciliation and coordination closures.

Any reassignment should be recorded by a new coordination note before starting
the affected MR. Do not let two agents edit the same page in parallel without a
fresh claim or explicit split.
