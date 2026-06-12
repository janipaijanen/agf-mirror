---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 89
timestamp: 2026-06-12T21:42:00Z
immutable: true
references-to:
  - 00087-coder-tester.md
  - 00088-coder-tester.md
topic: agents-govern-2.0-gate1
requires-ack: true
artifacts_owned: []
model: claude-sonnet-4-6
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 1 review of agents-govern-2.0 directory structure plan"
    - "Approve or reject docs/plans/agents-govern-2.0-directory-structure.md"
  must_read:
    - docs/plans/agents-govern-2.0-directory-structure.md
    - agents-govern/docs/adr/20260506-agents-govern-2-scope.md
  do_not_touch:
    - agents-govern/
    - .claude/settings.local.json
    - agents-govern/uv.lock
---

# Gate 1 Handoff: agents-govern-2.0 directory structure

Human governor decision: build `agents-govern-2.0/` (the v2.0 side-by-side
directory authorized by ADR 20260506-agents-govern-2-scope.md).

## What the plan covers

`agents-govern-2.0/` at repo root, side-by-side with `agents-govern/` (v1.x
canonical, untouched throughout).

Key structural changes vs. v1.x:
- Root reduced from 18 to ~9 files (platform-convention justified)
- 6 governance-meta files moved from root to `docs/governance/` + renamed
  for clarity (LEARNINGS.md → learning-system.md; LESSONS-LEARNED.md →
  retrospective.md)
- 3 contract docs moved from `docs/` root to `docs/contracts/`
- 2 new files: `docs/adopt/decommission.md` (high priority gap);
  `docs/reference/quick-reference.md` (medium priority gap)
- v1.x untouched; v2.0 ships with Provisional banner

## ADR alignment

- Scope: first-arrival clarity ✓, archive/source split ✓, broken-link
  repair ✓
- Out-of-scope: persona content ✓, new agents ✓, YAML-LD work ✓
- Downstream: ADDITIVE, Provisional only

## Two open items also awaiting governor-reviewer

1. **Seq 88 Gate 2** — `feat/dist-foundry-replace-on-upgrade` (foundry.yaml
   replace-on-upgrade in dist). Small, independent of v2.0 plan.
2. **This Gate 1** — v2.0 directory structure.

Both can be reviewed independently.
