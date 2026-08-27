---
kind: handoff
from: governor-reviewer
to: coder-tester
id: 20260808T194048Z-governor-reviewer-pspp
timestamp: 2026-08-08T19:40:48Z
model: human
immutable: true
references-to:
  - 20260808T193155Z-governor-reviewer-get2.md
  - 20260808T191033Z-contribution-auditor-rkb7.md
topic: independent-implementation-reviewer-role-implementation
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Claude must implement the approved independent-implementation-reviewer role plan from current main in a fresh feature worktree.
    - Keep the implementation within the plan's role, manifest, adopter-documentation, projection, and changelog scope.
    - Run every required validation and route contribution-auditor plus independent exact-head review before requesting Gate 2.
  must_read:
    - docs/plans/20260808T181423Z-independent-implementation-reviewer.md
    - docs/coordination/20260808T193155Z-governor-reviewer-get2.md
    - docs/coordination/20260808T191033Z-contribution-auditor-rkb7.md
    - agf/rules/always/tool-authority-boundary-rule.md
    - agf/rules/workflow/documentation-rule-projection-rule.md
  do_not_touch:
    - docs/piyaz-sources-radar
    - docs/beagle-crdt-source
    - broader Pages-rework implementation
    - agents-govern/
    - .claude/settings.local.json
---

# Claude handoff: implement the approved independent reviewer role

MR !1244 merged as `a4c091f3806058877c357d7886763927d1a789e4` after
the approved plan metadata update. The plan is now authoritative on `main`.

Claude is assigned as `coder-tester` for this implementation. This preserves a
clear separation from Codex's plan authorship and enables a later read-only
Codex exact-head review under the newly defined role if its activation and
provenance requirements are satisfied.

This handoff authorizes implementation only. It does not pre-clear the role's
implementation, activate new mechanical authority, approve Gate 2, or permit
Pages redesign work.

