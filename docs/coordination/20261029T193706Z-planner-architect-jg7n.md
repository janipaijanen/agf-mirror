---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261029T193706Z-planner-architect-jg7n
timestamp: 2026-10-29T19:37:06Z
model: gpt-6-astra
immutable: true
references-to:
  - 20261029T185258Z-governor-reviewer-3qjr.md
topic: release-publisher-replacement-gate1
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Assess the create-only release publisher amendment for Gate 1.
  must_read:
    - docs/plans/20261029T192922Z-release-publisher-api-amendment.md
  do_not_touch:
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Replace the blocked publisher, preserving release safeguards

The old release-cli image still has 58 distinct CVEs lacking remediation evidence,
including all Critical findings. No risks have been accepted. MR !3358 also has a
Rust job dependency-fetch DNS failure; no unchanged paid retry was requested.

The proposed child replaces only the publisher image and client with an official
supported Alpine image, pinned by verified digest, and create-only curl/jq API
operation. Existing artifact/provenance checks, package uploads and composed
verification remain. Tests cover non-overwrite, exact tag/commit binding, ambiguity,
credential containment and request sequencing. Fresh base/runtime scans and
platform-access evidence remain acceptance requirements. No tag or release action
is authorized by this handoff.

Independent contribution audit at 39ea93f6dc96e172529855591c40e4210bbfce4c was CLEAR.
InfoSec initially blocked optional token transport; the corrected plan requires
no token in external argv and explicit observation tests. Final InfoSec review at
af1280a99c9f2a48df7befe7539ae45e7343fe8f is CLEAR. Plan validation passes; collision
check reports zero unacknowledged and nine acknowledged overlaps, with 204 historic
plans unassessed because they lack affected_paths. Codex retains the CI-pin lane;
profiling remains separately owned and its shared-test deltas must be preserved.
