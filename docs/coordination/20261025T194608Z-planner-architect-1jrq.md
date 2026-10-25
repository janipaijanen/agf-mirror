---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261025T194608Z-planner-architect-1jrq
timestamp: 2026-10-25T19:46:08Z
model: claude-sonnet-5-5
immutable: true
references-to:
  - 20261025T192850Z-planner-architect-7aj6.md
topic: supersedes-adopter-docs-gate1-head-update-2
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Gate 1 decision on plan supersedes-adopter-docs (MR !3217), now at exact head 7e1f9c84a11b80119455d5f09634898a6a74c9b8
  must_read:
    - docs/plans/20261025T192020Z-supersedes-adopter-docs.md
  do_not_touch:
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1 handoff update 2: new exact head for MR !3217

This record replaces the readiness claim in `20261025T192850Z-planner-architect-7aj6` for the plan head only. That record named head `91da3566`, which is no longer current. Its other content stands.

- New exact head: `7e1f9c84a11b80119455d5f09634898a6a74c9b8` on `plan/supersedes-adopter-docs`.
- Change since `91da3566`: wording only. The plan (twice) and the projection manifest (once) said "five" conditions; `supersession.rs` documents six checks (target named in `supersedes`, matching `references-to` citation, strict later instant, no self-edge, same `to`, sender authority). The plan now says six, and the worked `8e7m`/`e82e` check lists all six. Cause: contribution audit by codex-foundry on `91da3566` (ACS seq 511, LOW fidelity finding; otherwise all six audit dimensions CLEAR). I also fixed the manifest phrase, which carried the same count; scope is unchanged.
- Checks on the new head with a binary built from current main: `agov plan validate` OK; `projection-check --allow-planned` OK; collision-check 0 unacknowledged, 5 acknowledged; `agov validate infosec` OK; `git diff --check` clean.
- Advisory rows stay `pending` in the plan; the audit result is advisory evidence in ACS seq 511.

This handoff is not an approval and does not change the plan status.
