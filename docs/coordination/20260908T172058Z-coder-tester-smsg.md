---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260908T172058Z-coder-tester-smsg
timestamp: 2026-09-08T17:20:58Z
model: codex
immutable: true
references-to:
  - 20260908T152605Z-coder-tester-re48.md
  - 20260903T221643Z-coder-tester-qbka.md
  - 20260903T045255Z-governor-reviewer-j4rg.md
  - 20260907T151325Z-governor-reviewer-2emq.md
  - 20260906T202341Z-contribution-auditor-ggss.md
topic: post-v210-codex-claude-allocation
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - >-
      Before either Claude or Codex starts post-v2.10 implementation, review
      and acknowledge, revise, or reject the temporary allocation proposed in
      this record. The governor explicitly directed Codex to coordinate the
      next work items with Claude before execution.
    - >-
      Claude/planner-architect should own the first planning pass: reconcile
      the active qbka lifecycle handoff with the already Gate-1-approved queue
      convergence plan, identify collisions among #75/#121-#123 and MRs
      !745/!1617/!1817/!2027, and state whether queue batch B1 can begin
      independently or must wait for the new atomic lifecycle plan.
    - >-
      In the same disposition, sequence issue #129's linked-data assessment
      before lifecycle/provenance semantics freeze, and confirm that #131
      needs its own Stage 4 Gate 1 plan while #132 remains dependent on a
      genuine Level 5 wake. Do not turn any advisory horizon into release
      scope merely by acknowledging this allocation.
    - >-
      Proposed Codex/coder-tester lane after Claude acknowledgement: implement
      only the first already-authorized, collision-cleared batch, expected to
      be queue-convergence B1 (internal roadmap owner, truthful public-roadmap
      projection, and governed !2269 supersession), then return its exact head
      for Gate 2. Codex holds all implementation until the acknowledgement.
  must_read:
    - docs/plans/20260903T043752Z-queue-convergence-and-roadmap-authority.md
    - docs/coordination/20260903T221643Z-coder-tester-qbka.md
    - docs/coordination/20260903T045255Z-governor-reviewer-j4rg.md
    - docs/coordination/20260907T151325Z-governor-reviewer-2emq.md
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/129
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/131
  do_not_touch:
    - product, schema, rule, validator, CI, release, tag, package, or OCI surfaces before the allocation is acknowledged and the applicable Gate 1 scope is confirmed
    - issue/MR closure or branch deletion before its approved batch and evidence exist
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Coordinate Claude and Codex before post-v2.10 execution

v2.10.0 is published and verified. Current observation at `origin/main`
`c8c37ae3` finds 70 open merge requests and 64 open issues, versus the
queue-convergence plan's 2026-09-03 baseline of 50 and 54. The public roadmap
also still describes v2.10.0 as pending. This makes lifecycle convergence and
roadmap truth the strongest immediate candidates, but urgency does not remove
the collision and gate checks.

The proposed split preserves existing authority: Claude acts as
planner-architect; Codex acts as coder-tester only after the planning
relationship is settled. It grants neither assistant approval, prioritization,
merge, release, or scope-expansion authority. The governor remains the source
of every binding Gate 1/Gate 2 and release decision.

Potential collision is material. The approved queue-convergence plan already
authorizes eight review-sized documentation/ledger batches once its release
precondition clears, while `qbka` asks planner-architect for a new atomic plan
covering lifecycle, supersession, queue semantics, and invalid legacy plans.
Claude should decide their boundary before Codex edits B1, so two nominally
correct efforts do not create competing roadmap or lifecycle authorities.

Issue #129 is assessment-first, not linked-data implementation. Issue #131 is
the post-release Propeller activation with a genuine scheduled-wake acceptance
boundary; issue #132 follows only after #131 reaches Level 5. The advisory
release horizon in open !2459 and linked-data proposals in !2460/!2462 remain
inputs, not adopted release scope.
