---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260727T154845Z-coder-tester-bsg4
timestamp: 2026-07-27T15:48:45Z
model: codex-gpt-5
immutable: true
references-to: []
topic: issue-board-workflow-plan
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Draft a small Gate 1 plan for a GitLab issue-board workflow using mutually exclusive scoped workflow labels.
    - Define column semantics, entry and exit criteria, transition ownership, and migration of existing open issues.
    - Preserve AGF Gate 1, Gate 2, capability-maturity, and release distinctions in the proposed names.
  must_read:
    - AGENTS.md
    - agf/docs/site/reference/maturity-vocabulary.md
    - agf/rules/workflow/merge-request-workflow-rule.md
    - agf/rules/always/tool-authority-boundary-rule.md
  do_not_touch:
    - GitLab labels or board lists before Gate 1 approval
    - agf/VERSION
    - any release branch or tag
---

# Plan an issue-board workflow for foundry

The governor requested coordination with Claude after accepting the proposal
to make work state visible on the existing empty GitLab **Development** board.
The current project labels describe work type (`bug`, `documentation`,
`improvement`, and similar) but do not describe lifecycle state.

The initial design direction is a mutually exclusive scoped-label workflow:

1. `workflow::reported`
2. `workflow::planning`
3. `workflow::ready` — Gate 1 approved
4. `workflow::wip`
5. `workflow::verification` — implementation exists; CI/reviews remain
6. `workflow::gate-2` — ready for the governor's Gate 2 decision
7. GitLab closed state — merged/released/otherwise resolved, with resolution
   semantics recorded rather than assuming every closure means deployment

Please test and refine this model in a Gate 1 plan. In particular, avoid a
generic `Tested` column that could imply Gate 2 approval, and avoid `Deployed`
as a universal terminal state because AGF distinguishes Level 4 Validated from
Level 5 Deployable/released work. The plan should also decide whether radar and
portfolio/discovery issues participate in the same workflow or use a separate
view, and whether automation is advisory or receives blocking authority from a
named rule.

No label, board, or issue migration changes are authorized until the resulting
plan receives governor-reviewer Gate 1 approval.
