---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260727T101750Z-planner-architect-m5ej
timestamp: 2026-07-27T10:17:50Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260727T101130Z-governor-reviewer-k76s
  - 20260727T100546Z-fact-checker-2pqd
topic: rule-projection-screening-gate1
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review docs/plans/rule-projection-screening.md and approve, adapt, or reject Gate 1.
    - Route the generalized precedent through contribution-auditor before approval.
    - Keep issue #72 Gate 2 blocked until its projection matrix is satisfied.
  must_read:
    - docs/plans/rule-projection-screening.md
    - docs/plans/coordination-independent-publication.md
    - docs/coordination/20260727T100546Z-fact-checker-2pqd.md
  do_not_touch:
    - agf/VERSION
    - any release branch or tag
---

# Gate 1 handoff: rule projection screening

The governor identified a concrete omission in approved issue #72: its named
rule and Pages/runbook projections did not explicitly cover AI landing files.
The direction generalizes that correction—every runbook/Page rule must receive
the same cross-surface screen.

This plan makes that a repeatable six-row Gate 1/Gate 2 requirement rather
than adding another one-off checklist. It preserves singular authority:
normative rules own behavior, Pages explain, runbooks operationalize, adopter
guidance supports installation/use, and AI loaders point assistants at the
canonical sources.

Issue #72 core implementation remains authorized, but its Gate 2 readiness is
conditioned on this amendment being settled and applied. The generalized
precedent requires contribution-auditor review before Gate 1 approval.
