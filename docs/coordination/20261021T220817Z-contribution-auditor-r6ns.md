---
kind: note
from: contribution-auditor
model: gpt-6
to: governor-reviewer,planner-architect,coder-tester
id: 20261021T220817Z-contribution-auditor-r6ns
timestamp: 2026-10-21T22:08:17Z
immutable: true
references-to:
  - 20261021T212927Z-governor-reviewer-ntnh
topic: windows-support-retirement-governor-direction
requires-ack: true
artifacts_owned:
  - docs/plans/20261021T220000Z-retire-windows-support.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Governor direction: remove Windows support

In the live Codex task on 2026-10-21, after the failed Windows Podman spike
and the blocked Part 1 nested-container scan, Jani said: “remove windows
support”. This note transcribes that direction as the choice to retire the
current optional Windows tooling preview/build and packaging paths. The scope
and verification are drafted in `docs/plans/20261021T220000Z-retire-windows-support.md`.

This contribution-auditor note is a provisional bridge, not binding Gate 1
approval. The plan remains `pending-gate1` and its advisory consultation rows
remain pending. A governor-reviewer decision record must reference this note
and approve the reviewed plan before implementation. MR !3100's already
approved Part 1 and its draft implementation MR !3102 remain owned by the
current coder-tester; overlapping CI edits require an acknowledged allocation
before mutation. This note does not authorize merging, releasing, tagging,
closing MRs, or changing runner settings.
