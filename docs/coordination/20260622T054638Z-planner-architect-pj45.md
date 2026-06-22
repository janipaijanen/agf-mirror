---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260622T054638Z-planner-architect-pj45
timestamp: 2026-06-22T05:46:38Z
model: gpt-5-codex
immutable: true
references-to: []
topic: issue-6-upgrade-friction
requires-ack: false
artifacts_owned:
  - docs/plans/issue-6-upgrade-friction.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the Gate 1 plan for issue #6 upgrade-friction handling.
    - Decide whether to approve the advisory drift-detection first slice.
  must_read:
    - docs/plans/issue-6-upgrade-friction.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 1 handoff: issue #6 upgrade friction

Issue #6 reports that an adopter upgrade from AGF 2.1.0 to 2.1.4 left stale
copied `.claude/` assistant files and gave false confidence because
`agov validate` passed while direct plan and coordination validators failed.

The attached plan proposes a narrow first slice:

- advisory drift warnings for copied `.claude/rules/` and `.claude/skills/`
  trees compared with canonical `agf/rules/` and `agf/skills/`;
- an explicit `agov validate` coverage note saying that plan and coordination
  files require separate validators;
- docs updates for the 2.1.x loader migration path.

The plan intentionally defers a full `agov adoption drift` or upgrade command.
That keeps the first MR small and avoids creating blocking semantics before AGF
has a precise ownership model for assistant vendor directories.
