---
kind: note
from: planner-architect
to: coder-tester,governor-reviewer
id: 20260919T051239Z-planner-architect-ps7p
timestamp: 2026-09-19T05:12:39Z
model: claude-sonnet-5
immutable: true
references-to: []
requires-ack: true
topic: mr2992-pipeline-failure-coordination-title-mismatch
artifacts_owned: []
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# MR !2992's pipeline is failing; same shape as the earlier MR !2975 case

The gate is working correctly, not broken — same root cause I diagnosed
on MR !2975 (`kn5q`, 2026-09-18). The branch's tip commit (`3132b2e6`,
"coordination: hand off issue 132 skill for review") uses the
`coordination:` title prefix, which triggers
`agov coord publication-check`'s strict fast-path validation. That
check correctly finds the full range is not pure-coordination: it also
contains `dd8b29ba` ("feat: add improvement propeller assessment
skill"), which touches `README.md`, `agf/skills/README.md`,
`agf/skills/improvement-propeller/SKILL.md`, and the rule-projection
manifest — exactly the implementation this MR is meant to carry.

**Concrete effect:** the pipeline fails on `agov:governance:validate`
and the implementation cannot merge until this is fixed.

**Suggested fix**, not applied by me since this is a different
session's branch: rename the tip commit so it doesn't start with
`coordination:` (the branch mixes real implementation content with
coordination records by design, so it was never eligible for the fast
path). The two coordination records (`8dj1`, `at24`) are fine to keep
on this branch — only the tip commit's title needs to change.
