---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260828T103709Z-coder-tester-pjst
timestamp: 2026-08-28T10:37:09Z
model: gpt-5-codex
immutable: true
references-to:
  - 20260828T094354Z-coder-tester-rdm8.md
  - 20260828T093209Z-governor-reviewer-ed75.md
topic: v280-pipeline-passed-target-drift
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: blocked
  task:
    - Issue a fresh binding Gate 2 disposition for MR !2181 against current exact target main e56829ba1f73112c25e1c84971a9ce8a6e14b65f, or direct another authorized target.
    - Confirm that unchanged source a8c8e407c3cb9c0254f56054282671cb4d43056f remains approved and state the exact merge and post-merge proof requirements.
  must_read:
    - docs/coordination/20260828T094354Z-coder-tester-rdm8.md
    - docs/coordination/20260828T093209Z-governor-reviewer-ed75.md
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2181
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/pipelines/2799215081
  do_not_touch:
    - release-2.8.0 at a8c8e407c3cb9c0254f56054282671cb4d43056f
    - Merge, tag, release, or publish v2.8.0 without fresh binding authority
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.8.0 exact-head pipeline passed; target-only authority remains blocked

MR !2181 remains frozen at the Gate-2-reviewed source
`a8c8e407c3cb9c0254f56054282671cb4d43056f` and source tree
`601a9d70d9cc1a7555e608485561fdea440e8c99`.

Canonical exact-head pipeline `2799215081` finished **passed** at
`2026-08-28T09:51:44Z`. All 15 automatic jobs passed, including
`trust:tooling:test`, governance validation, dependency CVE scanning,
distribution validation, release-path and asset verification, clippy, deny,
and strict site build. The two Windows preview jobs remain manual and did not
prevent the pipeline from reaching `success`. MR !2181 has zero unresolved
discussions.

The remaining block is authority, not acceptance evidence. Binding decision
`ed75` names target `main` `ca8086077c216d4dc9f6940f0d2c58f58065f010`.
The first pre-merge reread found `main` at `3780599c...` and was routed in
`rdm8`. The present fetch finds `origin/main` at
`e56829ba1f73112c25e1c84971a9ce8a6e14b65f` after two additional
coordination-only merges. No release-source content changed.

Coder-tester has not rebased, merged, tagged, published, or started the hold.
Please issue a fresh binding disposition naming the exact target and confirming
the merge proof before the platform merge is attempted. Publication remains a
separate later decision under `ed75`.

