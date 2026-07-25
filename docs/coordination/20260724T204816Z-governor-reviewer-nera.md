---
kind: handoff
from: governor-reviewer
to: planner-architect
id: 20260724T204816Z-governor-reviewer-nera
timestamp: 2026-07-24T20:48:16Z
model: human-governor
immutable: true
references-to:
  - 20260721T213536Z-governor-reviewer-727j
  - 20260724T174750Z-governor-reviewer-j55n
topic: release-engineer-role-draft-routing
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Publish the governor-requested release-engineer role draft through a separate governed proposal and Gate 1 path
    - Reconcile its jurisdiction with build-engineer, deployment-engineer, planner-architect, coder-tester, and governor-reviewer
    - Keep the role proposal outside the approved v2.4.2 implementation and do not activate it before explicit governor approval
  must_read:
    - agf/agents/MANIFEST.yaml
    - agf/agents/build-engineer.yaml
    - agf/agents/deployment-engineer.yaml
    - agf/rules/workflow/release-via-merge-request-rule.md
  do_not_touch:
    - docs/plans/v2.4.2-public-experience-and-evidence-presentation.md
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Route the governor-requested release-engineer role draft

The governor confirms that Claude drafted a new `release-engineer` role at the
governor's request. Treat this as intentional role-design work, not as an
unauthorized label or an accidental extension of v2.4.3.

The draft is not yet visible in the shared GitLab/coordination state. Publish
it through a separate proposal and Gate 1 plan so a cold-starting reviewer can
evaluate the exact role definition, manifest transition, and migration path.

The proposal must explicitly resolve the earlier `727j` premise that no
`release-engineer` existed and that `deployment-engineer` already owned release
verbs. Define without overlap:

- release-engineer's preparation, orchestration, verification, rollback, and
  notification responsibilities;
- build-engineer's CI, reproducibility, and artifact-production jurisdiction;
- deployment-engineer's remaining deployment jurisdiction, or its proposed
  retirement/merge if the new role supersedes it;
- planner/coder handoffs and the independent evidence roles;
- governor-reviewer's exclusive authority over Gate 1, Gate 2, version, tag,
  publication, rollback, and binding release decisions;
- activation criteria, minimal versus separated adopter profiles, migration,
  rollback, and duplicate-authority tests.

Do not activate or add the role to `agf/agents/MANIFEST.yaml` from this handoff.
Role creation/status is a non-trivial adopter-facing change requiring its own
critical review and exact-SHA governor Gate 1 and Gate 2 decisions.

Continue the already-approved v2.4.2 transition under `j55n` independently;
the role proposal neither blocks nor expands that release's nine frozen items.
