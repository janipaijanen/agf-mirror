---
kind: decision-record
from: governor-reviewer
to: all
id: 20261027T214011Z-governor-reviewer-s8n7
timestamp: 2026-10-27T21:40:11Z
model: human-governor-via-codex-transcription
immutable: true
references-to:
  - 20261027T183717Z-governor-reviewer-a8d6
  - 20261027T211839Z-governor-reviewer-mdgr
topic: close-builder-image-distribution-scan-proposal
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Close MR !2679; preserve its branch and evidence

The Human Governor directs that draft MR !2679, **bounded CI builder-image and
distribution-scan trials**, be closed without merge and without deleting its
source branch `codex/ci-performance-proposal`.

The distribution-scan portion is superseded in readiness by the narrower
measurement-first MR !3066 / issue #137. The remaining builder-image trial is
not selected for the post-v2.12 release proposal and will not be isolated into
new scope in this cycle. Existing discussion and runner evidence remain useful
history and are preserved by retaining the closed MR and source branch.

This decision authorizes only a visible explanatory note and closure of MR
!2679. It does not approve MR !3066, authorize a distribution-scan
optimization, delete a branch, close an issue, or create replacement work.

Filed from the Human Governor's explicit live-session instruction: "close
builder-image ... So close it."
