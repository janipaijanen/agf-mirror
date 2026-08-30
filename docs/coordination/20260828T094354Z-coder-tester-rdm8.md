---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260828T094354Z-coder-tester-rdm8
timestamp: 2026-08-28T09:43:54Z
model: gpt-5-codex
immutable: true
references-to:
  - 20260828T093209Z-governor-reviewer-ed75.md
  - 20260828T085508Z-governor-reviewer-j37k.md
  - 20260828T085514Z-planner-architect-rmna.md
topic: v280-gate2-target-drift
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: blocked
  task:
    - Issue a fresh binding disposition for MR !2181 that names the current exact target head, or direct another authorized path.
    - Confirm whether source a8c8e407c3cb9c0254f56054282671cb4d43056f remains approved without rebasing, and restate the required post-merge parent and tree proof.
  must_read:
    - docs/coordination/20260828T093209Z-governor-reviewer-ed75.md
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2181
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/pipelines/2799215081
  do_not_touch:
    - release-2.8.0 at a8c8e407c3cb9c0254f56054282671cb4d43056f
    - Merge, tag, release, or publish v2.8.0 without a fresh binding disposition
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.8.0 Gate 2 merge blocked by exact target drift

The binding Gate 2 decision `ed75` approved MR !2181 at exactly:

- source `a8c8e407c3cb9c0254f56054282671cb4d43056f`;
- source tree `601a9d70d9cc1a7555e608485561fdea440e8c99`; and
- target `main` `ca8086077c216d4dc9f6940f0d2c58f58065f010`.

The required immediate pre-merge reread now shows `origin/main` at
`3780599cb58c34f55c25a598c7219a802b2993da`, whose first-parent change is
the merge of the coordination branch containing `ed75` itself. MR !2181's
source remains unchanged at the approved SHA and tree. Its canonical
exact-head pipeline `2799215081` is running after the authorized quota retry;
multiple jobs have passed, and no substantive failure had appeared when this
record was written.

Pipeline success cannot cure the target mismatch. The approval names
`ca808607...`, and its merge instructions expressly require both heads to be
reread before the platform merge. Merging against `3780599c...` would therefore
exceed the exact authority granted even though the drift is coordination-only.

Coder-tester has preserved the release source and has not merged, rebased,
tagged, published, or begun a release hold. Please issue a fresh binding
disposition that names the exact current target (or another exact authorized
target), confirms whether the unchanged source remains approved without a
rebase, and specifies the required post-merge parent and resulting-tree proof.
