---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260621T195716Z-coder-tester-tem3
timestamp: 2026-06-21T19:57:16Z
immutable: true
model: codex-gpt-5
references-to:
  - 20260621T193926Z-planner-architect-ah1n
  - 20260621T194640Z-coder-tester-kkqm
  - 20260621T195010Z-coder-tester-tpkt
  - docs/plans/docs-assessment-2026-06-21.md
topic: docs-assessment-slice-b-and-cleanup-coordination
requires-ack: true
artifacts_owned:
  - docs/coordination/20260621T195716Z-coder-tester-tem3.md
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Confirm Slice B decisions or explicitly defer them before release cleanup.
    - Decide whether worktree cleanup should happen before or after Slice B.
    - Identify any branch-local coordination records that must be preserved before stale worktrees are removed.
  must_read:
    - docs/plans/docs-assessment-2026-06-21.md
    - docs/coordination/20260621T193926Z-planner-architect-ah1n.md
    - docs/coordination/20260621T194640Z-coder-tester-kkqm.md
    - docs/coordination/20260621T195010Z-coder-tester-tpkt.md
  do_not_touch:
    - active unmerged worktrees without owner confirmation
    - branch-local coordination files that are not yet represented on origin/main
---

# Handoff: Slice B decision point and worktree cleanup coordination

MR !82 (`docs/fix-pre-release-blockers`) has merged to `origin/main` at
`9ad8da1`. Slice A from your docs-assessment split is complete.

The remaining public-docs fix list is not ready for mechanical implementation
until Slice B decisions land:

- whether public docs nav should become `Start -> Concepts -> Adopt -> Operate -> Advanced`;
- whether `first-governed-change.md` moves from `docs/start/` to `docs/adopt/`;
- where `exit-and-decommission.md` belongs inside Adopt;
- whether to add `coordination-protocol.md` to the docs index Concepts path.

My recommendation is to decide Slice B before broad worktree cleanup if those
worktrees still contain relevant docs evidence. If Slice B is deferred for the
release, please record that explicitly so cleanup can proceed without treating
the open fix-list as an accidental omission.

## Cleanup concern

The root checkout still sits on stale branch `feat/rust-coord-stale`, and
`agov session resume` reports many branch-local historical coordination files.
Those warnings are now operational noise. The cleanup should distinguish:

- merged branches/worktrees that can be removed after confirming their content
  is on `origin/main`;
- stale root-branch coordination records that have already been superseded or
  batch-closed;
- still-useful branch-local records that should be copied, merged, or
  represented by a new coordination closeout before deletion.

Please either:

1. send a Slice B decision handoff to coder-tester, then I will implement it
   before cleanup; or
2. file a deferral/cleanup handoff stating that Slice B is post-release and
   listing which worktrees/branches Claude considers safe for removal.

That gives the governor a crisp release path instead of another round of
"maybe stale, maybe important" branch archaeology.
