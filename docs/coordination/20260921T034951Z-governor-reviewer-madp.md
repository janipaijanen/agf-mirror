---
kind: decision-record
from: governor-reviewer
to: all
id: 20260921T034951Z-governor-reviewer-madp
timestamp: 2026-09-21T03:49:51Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260920T194655Z-governor-reviewer-kgd8.md
requires-ack: false
topic: high-risk-operations-domain-customization-gate2-approved
artifacts_owned:
  - agf/rules/always/high-risk-operations-rule.md
  - .claude/rules/always/high-risk-operations-rule.md
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

<!--
Filed into the coordination channel by the claude-sonnet-5 session at the
governor's explicit request ("proceed as proposed", 2026-09-21, live
session, following the session's proposal naming this exact merge as its
first step), in the same transcription pattern as q862/8274/rhgk/3es7/
kgd8: the YAML front matter and this header were added by the assistant
for schema validity; the decision itself is the governor's.
-->

# Gate 2 approved and merged: high-risk-operations-rule.md AGF-NG domain customization

The governor-reviewer approved Gate 2 for MR !3083 in the active session.
Approval applied to exact source head
`b2289ff62b9eeded61bdbcb7e780aae473a37535`, whose canonical MR pipeline
(id `2865716731`) passed.

The MR was merged to `main` at merge commit
`5216a2ef5989d8a7123ba95ccf5e98ded3ba6738`. The change replaces
`agf/rules/always/high-risk-operations-rule.md`'s generic SaaS-application
STOP-list with six areas grounded in AGF-NG's own verified incident and
enforcement history, applied identically to its `.claude/rules/` loader
projection, per Gate 1 decision `kgd8`.

This completes both fast-follow items named in the governance-mode
classification decision (`3es7`): the high-risk-operations-rule.md
customization (this MR) and the earlier finding that CI path-scoping was
already mature and needed no further work. The governance-mode work item
is now fully closed.
