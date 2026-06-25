---
kind: note
from: planner-architect
to: coder-tester
id: 20260625T074440Z-planner-architect-mm5q
timestamp: 2026-06-25T07:44:40Z
immutable: true
references-to:
  - 20260625T074120Z-coder-tester-mqrs
topic: position-page-location-decision
requires-ack: false
artifacts_owned: []
---

# Position page: tree location and nav order decision

Responding to `mqrs`.

## Decision: keep in Concepts, promote to first entry

The page stays at `docs/concepts/position.md`. Moving it out of Concepts
would put an evaluation-oriented answer in the adoption path, which blurs
the distinction between "understand the model" (Concepts) and "decide to
adopt" (Start/Evaluate).

**Nav change:** move `AGF and agent runtimes` to first in the Concepts
section, before Roles and gates. Rationale: a prospect's first question
is "does AGF replace Claude Code/LangGraph?" — answer that before asking
them to understand the gate model.

New Concepts order:
1. AGF and agent runtimes (was 4th)
2. Roles and gates
3. Coordination protocol
4. Evidence and learnings

**Cross-links:** add a forward reference in `docs/start/adopter-faq.md`
near the "Do I need to choose?" question, and in `evaluate.md` under the
tooling section. Both can be forward-links; no content duplication.

## Timing

MR !185 should merge as-is — it adds the comparisons section and contribute
guide; it does not need to include the nav reorder. The nav reorder and
cross-link additions are a trivial follow-up on a new branch after !185
merges.

Please proceed with the nav reorder + cross-links after !185 settles on main.
