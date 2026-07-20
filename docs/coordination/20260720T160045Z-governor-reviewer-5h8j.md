---
kind: decision-record
from: governor-reviewer
to: all
id: 20260720T160045Z-governor-reviewer-5h8j
timestamp: 2026-07-20T16:00:45Z
model: claude-sonnet-5
immutable: true
references-to:
  - docs/plans/agent-improvement-proposal-norm.md
  - 20260720T153719Z-planner-architect-bqd8.md
  - 20260720T154019Z-coder-tester-sknh.md
requires-ack: false
artifacts_owned: []
topic: agent-improvement-proposal-norm-gate1
---

# Gate 1 approved: agent improvement-proposal norm

Claude-scribed governor-reviewer decision for MR !235,
`docs/plans/agent-improvement-proposal-norm.md`.

Approval basis:

- The human governor directed the plan's originating norm in session and,
  when asked to choose the next action, explicitly selected "approve Gate 1
  on !235" over starting MR-5 or anything else.
- Codex's independent advisory review (`20260720T154019Z-coder-tester-sknh.md`)
  supports the same three answers the plan itself proposed, with no
  blocking findings.
- The plan's own contribution-auditor self-review found no blocking
  findings (scope-creep foreclosed by the draft rule's explicit
  idea-is-not-approval boundary clause).

Decision: Gate 1 approved.

Answers to the plan's three questions:

1. **Strength: SHOULD**, not MUST. The norm authorizes agents to speak up
   about friction; it does not turn a missed observation into a rule
   violation.
2. **Landing directory: `agf/rules/always/`.** The behavior is a general
   agent habit that applies regardless of workflow phase, not scoped to
   one phase the way `workflow/` rules are.
3. **Scope: both AGF-scope and project-local observations are in scope for
   proposing.** The boundary stays where the draft rule already put it:
   proposing is not deciding, and a project-local idea does not become an
   AGF framework rule without governor review and the normal evidence path.

Conditions:

- Implementation is one small MR (per the plan; no MR-B needed), creating
  `agf/rules/always/agent-improvement-proposal-rule.md` from the plan's
  exhibit text and the `day-to-day.md` paragraph.
- Per Codex's implementation note in `20260720T154019Z-coder-tester-sknh.md`:
  the rule-file reference in `day-to-day.md` MUST NOT be a raw markdown
  link across the MkDocs `docs_dir` boundary — render it as inline code
  text, or prove the link resolves under `mkdocs build --strict` before
  shipping a link.
- Implementer runs `agov plan validate` was already clean; Gate 2 evidence
  should reconfirm `mkdocs build --strict` and `git diff --check` after the
  new rule file lands.
