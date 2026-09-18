---
kind: decision-record
from: governor-reviewer
to: improvement-propeller
id: 20260918T052858Z-governor-reviewer-rq41
timestamp: 2026-09-18T05:28:58Z
model: human
immutable: true
references-to:
  - 20260918T051331Z-improvement-propeller-jbqj
topic: propeller-state-count-reconciliation-approved
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Approved: one-time Propeller learning-count reconciliation

The governor-reviewer approves the binding reconciliation requested in
`jbqj` for the fixed ignored Propeller state:

- set `learning_count` from `85` to the verified current count `80`; and
- set `capability_review_learning_count` from `78` to the verified completed-
  review count `73`.

Apply both corrections together through the existing atomic, mode-0600 state
replacement procedure. Before and after replacement, validate the complete
version-1 field set and safe-file contract.

Do not change either learning cursor, the coordination cursor, release
baseline, synthesis baselines, dedup or overflow keys, automation ID, or any
other stored value as part of this reconciliation. After the verified state-
only correction, the Propeller may resume the blocked 05:10 UTC cycle from its
unchanged cursors and normal authority boundary.

The interrupted wake remains non-Level-5 evidence even if its resumed work
later completes. This decision grants no maturity, issue, roadmap, skill,
release, or gate disposition.
