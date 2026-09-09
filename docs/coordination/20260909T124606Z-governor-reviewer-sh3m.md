---
kind: decision-record
from: governor-reviewer
to: all
id: 20260909T124606Z-governor-reviewer-sh3m
timestamp: 2026-09-09T12:46:06Z
model: human
immutable: true
references-to:
  - 20260909T085759Z-planner-architect-pkbt.md
  - 20260909T105121Z-coder-tester-8pps.md
  - 20260909T105646Z-coder-tester-rjar.md
topic: v211-gates-2579-2581-2597-approved
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Binding approvals: !2579, !2581, and !2597

The governor-reviewer states: **"Gate requests approved. Proceed. 2597, 2581
2579."** This record supplies the binding decisions at the exact reviewed heads:

1. **MR !2579 — Gate 1 approved** at
   `f90f79cbb16716790948304de6c305bad2f33f7a`. This confirms planner bridge
   `pkbt` and approves the expanded v2.11 scope, including every Propeller
   coordination-protocol row made release-blocking by `er78`. The plan's
   metadata may be updated to cite this record and the plan may merge after its
   corrected exact-head pipeline succeeds.
2. **MR !2597 — Gate 2 approved** at
   `ea9924ef32bb05b5c9889b174046ed79bcb87f51`, with canonical pipeline
   `2832639088` green 8/8 and Claude exact-head contribution, InfoSec, and build
   reviews `p75h`, `t5b2`, and `q5pk` all CLEAR 0/0. Merge is authorized.
   Runtime activation must then follow the approved post-merge state-file,
   heartbeat, and view-back sequence; Level 5 still requires a genuine
   unattended wake.
3. **MR !2581 — Gate 2 approved** at
   `8bcaef318b3f921d50bcaaad928b2dcbc3fbb617`, with canonical pipeline
   `2832666665` green, 47/47 archived records valid, and 47/47 SHA-256 digests
   matched. Merge is authorized. The 24 audited source coordination-carrier MRs
   may close only after this exact head is merged to `main`, with closure notes
   linking the canonical archive; excluded MRs !2409, !2410, and !2452 remain
   open.

This decision does not authorize a v2.11 version bump, tag, or release
publication. Those remain later binding boundaries under the approved release
plan.
