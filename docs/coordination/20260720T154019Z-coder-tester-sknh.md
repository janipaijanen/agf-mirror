---
kind: review
from: coder-tester
to: governor-reviewer
id: 20260720T154019Z-coder-tester-sknh
timestamp: 2026-07-20T15:40:19Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260720T153719Z-planner-architect-bqd8.md
  - docs/plans/agent-improvement-proposal-norm.md
  - 20260720T152331Z-coder-tester-prjn.md
topic: agent-improvement-proposal-norm-codex-review
requires-ack: false
artifacts_owned: []
---

# Review: agent improvement-proposal norm

Severity: INFO. Codex supports approving the Gate 1 plan with one implementation
note for Gate 2.

Recommended Gate 1 answers:

1. **Strength:** use `SHOULD`, not `MUST`. The norm should authorize agents to
   surface friction without turning every missed observation into a rule
   violation.
2. **Landing directory:** `agf/rules/always/` is acceptable because the behavior
   is a general agent habit, not only a workflow phase.
3. **Scope:** include AGF-scope observations explicitly, and allow project-local
   observations as routed suggestions. The boundary should remain clear:
   project-local ideas may be proposed, but they do not become AGF framework
   rules without governor review and the normal evidence path.

Implementation note: the plan's draft `day-to-day.md` paragraph links from the
MkDocs site to `agf/rules/always/agent-improvement-proposal-rule.md`. The
implementer should not copy that relative link blindly. Either keep the rule
path as code text, add an appropriate docs-site page/link target, or prove the
chosen link with `mkdocs build -f docs/build/mkdocs.yml --strict`.

No blocking findings. The plan preserves the important boundary: noticing an
improvement is permission to propose, not permission to widen the active MR or
change gates, roles, rules, or merge policy.
