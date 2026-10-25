---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261025T192850Z-planner-architect-7aj6
timestamp: 2026-10-25T19:28:51Z
model: claude-sonnet-5-5
immutable: true
references-to:
  - 20261025T192135Z-planner-architect-2gpj.md
topic: supersedes-adopter-docs-gate1-head-update
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Gate 1 decision on plan supersedes-adopter-docs (MR !3217), now at exact head 91da3566d33d244fbd4626d1885ab5f2ee06908a
  must_read:
    - docs/plans/20261025T192020Z-supersedes-adopter-docs.md
  do_not_touch:
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1 handoff update: new exact head for MR !3217

This record replaces the readiness claim in `20261025T192135Z-planner-architect-2gpj` for the plan head only. That record named head `d2563a4a`, which is no longer current. Its other content stands.

- New exact head: `91da3566d33d244fbd4626d1885ab5f2ee06908a` on `plan/supersedes-adopter-docs`.
- Cause of the change: CI `agov:governance:plan-collision-gate` (pipeline 2987029092) blocked `d2563a4a` on four unacknowledged plan overlaps. The plan's `depends_on` now lists them and its collision section records why each is deliberate: `20260921T040743Z-assistant-rule-drift-resolution`, `20261021T220000Z-retire-windows-support`, `20261023T155559Z-agov-strict-args-ref-list`, `20261024T070435Z-timer-reality-polling-alignment`. Scope is unchanged.
- Correction: my earlier handoff reported `collision-check 0` for the old head. That came from a stale local binary (2026-10-22). The check on this head used a binary built from current `main` (19:28Z): 0 unacknowledged, 5 acknowledged.
- Other checks on the new head: `agov plan validate` OK; `projection-check --allow-planned` OK; `agov validate infosec` OK; `git diff --check` clean.
- Advisory rows stay `pending` in the plan.

This handoff is not an approval and does not change the plan status.
