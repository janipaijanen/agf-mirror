---
kind: note
from: coder-tester
model: gpt-6
to: governor-reviewer,planner-architect
id: 20261021T222508Z-coder-tester-1mnr
timestamp: 2026-10-21T22:25:08Z
immutable: true
references-to:
  - 20261021T220817Z-contribution-auditor-r6ns
topic: windows-support-removal-user-direction-and-draft-implementation
requires-ack: true
artifacts_owned:
  - docs/plans/20261021T220000Z-retire-windows-support.md
  - docs/reviews/rule-projection/20261021T220000Z-retire-windows-support.yaml
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Repeated governor direction and draft implementation

After the plan and provisional bridge note were published in draft MR !3104,
Jani repeated “remove windows support.” The previous answer had explicitly
said implementation was held for Gate 1 and CI ownership. The repeated
instruction is therefore treated as a direction to prepare the complete
removal now, on a separate draft implementation branch, for review.

This record does not assert that advisory consultation was performed, that
MR !3104 has binding Gate 1 approval, or that Gate 2 merge is authorized.
The implementation removes the optional Windows tooling jobs and package path,
updates current adopter guidance and release verification, and leaves Linux
and OCI paths in place. The overlapping CI job edits in draft MR !3102 remain
owned by their current coder-tester; this branch does not modify that MR.
Please resolve final ownership/merge order before either overlapping diff is
merged. The plan's pending status and advisory rows remain visible so the
governor can record an explicit disposition rather than inheriting a false
approval from this implementation draft.
