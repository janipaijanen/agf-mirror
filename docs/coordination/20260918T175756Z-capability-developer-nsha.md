---
kind: handoff
from: capability-developer
to: planner-architect
id: 20260918T175756Z-capability-developer-nsha
timestamp: 2026-09-18T17:57:56Z
model: codex (gpt-6)
immutable: true
references-to:
  - 20260918T162825Z-governor-reviewer-8hs2.md
  - 20260918T170438Z-planner-architect-pqk6.md
topic: issue132-improvement-propeller-skill-gate1-planning
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - >-
      Prepare the smallest bounded Gate 1 plan for issue #132: export the
      Improvement Propeller method as a thin human-invoked advisory skill.
      Keep the existing rule as the single normative owner, preserve the
      authority disclaimer, and route finished assessments back through the
      normal governed contribution path.
  must_read:
    - GitLab issue #132
    - agf/rules/always/improvement-propeller-rule.md
    - agf/skills/poll-and-coordinate/SKILL.md
    - agf/skills/capability-review/SKILL.md
    - agf/skills/README.md
    - docs/plans/20260905T221252Z-improvement-propeller-role.md
    - docs/coordination/20260918T162825Z-governor-reviewer-8hs2.md
  do_not_touch:
    - implementation before a binding Gate 1 approval
    - the optional capability-developer triage skill unless separately scoped
    - the canonical rule text merely to duplicate it into the skill
    - release, Gate, role, or blocking authority
---

# Issue #132 is unblocked: prepare its bounded Gate 1 plan

The governor instructed the published next-work sequence to proceed. Issue
#131 is closed with binding Level 5 evidence in `8hs2`, so issue #132's stated
dependency is satisfied. The four queue-successor reviews in `pqk6` are CLEAR
and now wait only for governor Gate 2; their wait does not block this planning
lane.

Plan only the issue's required skill export:

- add `agf/skills/improvement-propeller/SKILL.md` as a thin, human-invoked,
  advisory wrapper;
- point to `improvement-propeller-rule.md` as the single normative owner;
- package evidence gathering, deduplication, bounded ranking, counterargument,
  smallest useful outcome, dependencies/gate, collision disposition, and
  suggested receiving role;
- preserve the role disclaimer: the skill observes and proposes, grants
  nothing, and makes no decision;
- end through the ordinary learning/issue/proposal contribution path, with no
  bypass;
- inventory distribution/index/docs projections and specify validation and
  negative fixtures without widening into the optional triage skill.

The plan must declare `contribution-auditor` and `infosec-sentinel`
consultation statuses and remain pending Gate 1 until a governor decision.
