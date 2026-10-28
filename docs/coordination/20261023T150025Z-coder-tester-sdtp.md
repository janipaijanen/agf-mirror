---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20261023T150025Z-coder-tester-sdtp
timestamp: 2026-10-23T15:00:25Z
model: gpt-5-codex
immutable: true
references-to:
  - 20261022T222724Z-coder-tester-krqb
  - 20261022T052553Z-governor-reviewer-qbsb
  - 20261022T041904Z-governor-reviewer-382s
  - 20261021T222508Z-coder-tester-1mnr
topic: windows-removal-gate2-exact-head-refresh
requires-ack: true
artifacts_owned:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3105
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review MR !3105 at exact source head 62285acc75b0de2d9e1352873e3f7505872c31eb and issue a binding Gate 2 decision.
    - If approved, explicitly authorize the merge target and remote before merging; this handoff does not authorize merge.
    - Decide the disposition of separate plan MR !3104, whose approved plan is included in !3105's main-target diff.
  must_read:
    - docs/plans/20261021T220000Z-retire-windows-support.md
    - docs/coordination/20261022T052553Z-governor-reviewer-qbsb.md
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3105
  do_not_touch:
    - refs/heads/main before binding Gate 2 approval and explicit merge authorization
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Refreshed exact-head Gate 2 handoff: retire Windows tooling support

MR !3105 is ready for Human Governor Gate 2 review at exact source head
`62285acc75b0de2d9e1352873e3f7505872c31eb`, targeting `main`. It retires the
optional Windows preview jobs, DinD/Cross packaging path, current Windows ZIP
contract, and current download guidance while preserving Linux archives, OCI,
framework source ZIPs, and historical records.

The canonical exact-head pipeline
[`2981615620`](https://gitlab.com/janipaijanen/agents-govern-framework/-/pipelines/2981615620)
succeeded: all 20 instantiated jobs passed, including the Rust test, Clippy,
deny, release-assets, release-self-sufficiency, OCI build/scan/pull, CI
selection, changed-files, governance, and collision jobs. No required job
failed or remained pending.

Fresh verification against current `origin/main`:

- `agov plan validate`: OK, blocking severity;
- `agov plan collision-check --plans-only --changed-only
  --fail-on-unacknowledged-collision`: zero unacknowledged collisions; seven
  acknowledged overlaps;
- `git diff --check origin/main...HEAD`: clean;
- every implementation path is declared by the approved plan;
- range-diff from previously handed-off head `b1bb3f32` shows the same seven
  implementation commits after rebase, with conflict resolution for Elixir
  references already removed on `main`, plus one plan-only `depends_on`
  correction for the two merged Elixir cleanup plans.

This supersedes `krqb` only as to the exact source head and pipeline. Its
scope and authority boundary are unchanged. Gate 2 and merge remain with the
Human Governor because the MR changes CI and release mechanics. If approved,
the decision must name this exact head, target `main`, and the GitLab remote.
No release, tag, deployment, runner deregistration, or historical artifact
mutation is requested.
