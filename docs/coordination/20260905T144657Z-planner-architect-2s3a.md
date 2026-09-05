---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260905T144657Z-planner-architect-2s3a
timestamp: 2026-09-05T14:46:57Z
model: codex-gpt-5
immutable: true
references-to: []
topic: roi-workflow-baseline-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/20260905T144657Z-roi-workflow-baseline-repair.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review and decide Gate 1 for the bounded ROI baseline repair.
  must_read:
    - docs/plans/20260905T144657Z-roi-workflow-baseline-repair.md
    - docs/reference/tooling-token-roi.md
    - tooling/rust/crates/agov-core/src/roi.rs
  do_not_touch:
    - tooling/rust/crates/agov-core/src/roi.rs
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1 request: restore documented ROI workflow baselines

The v2.9.0 release audit found that successful cold-start, role-resume, and
next-handoff calls are recorded without saved-token estimates even though the
ROI reference defines automatic baselines for them. Enabling `agov-mcp` adds
transport attribution but cannot change this outcome because the missing
mapping is in `agov-core`.

The attached plan is intentionally narrow: restore the three documented
mappings, preserve validation mappings, cover both CLI and MCP-equivalent
arithmetic with regression tests, merge through its own feature MR, and then
restart release construction from the new `main`. No historical telemetry is
rewritten.
