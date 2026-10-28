---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261025T204657Z-planner-architect-adtq
timestamp: 2026-10-25T20:46:58Z
model: claude-sonnet-5-5
immutable: true
references-to:
  - 20261025T202503Z-planner-architect-q5pm.md
supersedes:
  - 20261025T202503Z-planner-architect-q5pm.md
topic: supersedes-adopter-docs-gate2-head-update
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Gate 2 review of MR !3222 at exact head 063ced333fdd940de7b037e4b012a91584cc4d33
  must_read:
    - docs/plans/20261025T192020Z-supersedes-adopter-docs.md
    - docs/reviews/rule-projection/20261025T192020Z-supersedes-adopter-docs.yaml
  do_not_touch:
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 2 handoff update: new exact head for MR !3222

This record supersedes `20261025T202503Z-planner-architect-q5pm` for the implementation head. That record named `aed74a3d`, which is no longer current. This record uses the `supersedes` field the MR documents, as the first live use.

- New exact head: `063ced333fdd940de7b037e4b012a91584cc4d33` on `docs/supersedes-adopter-docs`.
- Change since `aed74a3d`: wording only, in the rule subsection and the Pages bullet. The text now names the verified surfaces: `agov coord list` (marks it superseded), `agov coord pending` (records that require acknowledgement), and `agov handoff next` and `agov coord status` (handoffs). Cause: Gate 2 finding by codex-foundry (ACS seq 543), which I tested instead of accepting.
- Evidence that the finding's premise does not hold: `agov handoff next` and `agov coord status` are backed by `pending_handoffs` in `tooling/rust/crates/agov/src/main.rs`, which calls `valid_superseder`. The finding looked at `find_pending` in `handoff_tracker.rs`, a different function. A real run in a scratch directory with one handoff and one later note: without `supersedes`, `agov handoff next governor-reviewer` shows the handoff as ready and `agov coord status` lists it as pending; with `supersedes` (same note otherwise), `handoff next` prints "No handoff found" and `coord status` prints "Pending handoffs: none". The approved plan already said "pending and handoff views", so the approved contract is unchanged and no plan amendment is needed.
- Checks on the new head with a binary built from main (2026-10-25T19:28Z): strict `projection-check` OK, `adopter-docs` OK, `infosec` OK, plan collision-check 0 unacknowledged, `git diff --check` clean.
- Exact-head MR pipeline: see MR !3222.

This handoff is not an approval.
