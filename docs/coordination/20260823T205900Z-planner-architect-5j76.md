---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260823T205900Z-planner-architect-5j76
timestamp: 2026-08-23T20:59:00Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260823T205859Z-planner-architect-mkke
topic: research-maintenance-fast-path-gate1
requires-ack: false
artifacts_owned:
  - docs/plans/20260823T205706Z-research-maintenance-fast-path.md
handoff:
  next_agent: governor-reviewer
  status: blocked
  task:
    - After the independent contribution audit is published and any findings are resolved, approve, revise, or reject the exact plan head at Gate 1.
    - Confirm that the path-and-content-qualified exception removes only research-maintenance ceremony and retains human exact-head Gate 2 merge authority.
  must_read:
    - docs/plans/20260823T205706Z-research-maintenance-fast-path.md
    - docs/coordination/20260823T205859Z-planner-architect-mkke.md
    - agf/rules/workflow/simplicity-rule.md
    - agf/rules/workflow/development-workflow-rule.md
    - agf/rules/workflow/merge-request-workflow-rule.md
    - agf/rules/always/agent-improvement-proposal-rule.md
  do_not_touch:
    - docs/sources.md
    - docs/radar.md
    - agf/rules/
    - AGENTS.md
    - .claude/settings.local.json
---

# Gate 1 routing: proportional sources/radar research maintenance

The pending plan translates the governor's anti-ceremony direction into a
narrow exception for advisory, reversible changes limited to
`docs/sources.md` and/or `docs/radar.md`. It preserves normal protected-branch
MR review, source-fidelity evidence, targeted validation, exact-head CI, and
the governor-reviewer's binding Gate 2 merge decision.

This Gate 1 handoff is deliberately blocked on the independent contribution
audit requested in `20260823T205859Z-planner-architect-mkke`. The plan itself
marks that consultation as outstanding and must not be approved until its
findings are resolved. Once unblocked, please decide the exact plan head; do
not infer approval from the governor's request to draft the change.

No rule implementation, source/radar edit, merge, release, tag, or deployment
is authorized by this handoff.
