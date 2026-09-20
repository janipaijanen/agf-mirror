---
kind: decision-record
from: governor-reviewer
to: all
id: 20260920T132728Z-governor-reviewer-hq1h
timestamp: 2026-09-20T13:27:28Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260920T115342Z-governor-reviewer-3es7.md
requires-ack: false
topic: governance-mode-classification-gate2-approved
artifacts_owned:
  - agf/rules/workflow/simplicity-rule.md
  - .claude/rules/workflow/simplicity-rule.md
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

<!--
Filed into the coordination channel by the claude-sonnet-5 session at the
governor's explicit request ("gate 2 approved", 2026-09-20, live session),
in the same transcription pattern as q862/8274/rhgk/3es7: the YAML front
matter and this header were added by the assistant for schema validity;
the decision itself is the governor's.
-->

# Gate 2 approved and merged: LIGHT/NORMAL/STRICT governance-mode classification

The governor-reviewer approved Gate 2 for MR !3077 in the active session.
Approval applied to exact source head
`d713d0729b12c380069941ca86ce7aa7f4284630`, whose canonical MR pipeline
(id `2865258742`) passed.

The MR was merged to `main` at merge commit
`7efe886762148be947e30d819232e3abd3938ce6`. The change replaces
`agf/rules/workflow/simplicity-rule.md`'s "Calibrating workflow overhead to
task size" and "Fast-track waiver" sections (and the matching text in its
`.claude/rules/` loader projection) with the named "Governance mode:
LIGHT / NORMAL / STRICT" section approved by Gate 1 decision `3es7`.

This completes the LIGHT/NORMAL/STRICT governance-mode classification work
item: plan approved (MR !3074), Gate 1 decision recorded (MR !3076),
implementation merged (MR !3077). Two fast-follow items remain named but
unscheduled, per `3es7`: (1) customize
`agf/rules/always/high-risk-operations-rule.md` for AGF-NG's actual domain
and reconsider the contractual/compliance STRICT trigger there; (2) wire
LIGHT/NORMAL/STRICT mode into CI path-scoping per `ci-trigger-scoping-rule.md`.
