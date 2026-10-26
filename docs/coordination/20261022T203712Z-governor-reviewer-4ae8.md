---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20261022T203712Z-governor-reviewer-4ae8
timestamp: 2026-10-22T20:37:12Z
model: human-via-codex
immutable: true
references-to:
  - 20261022T203417Z-coder-tester-pbsk
  - 20261022T203056Z-coder-tester-gk3r
topic: restore-dedicated-dind-runner
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Decision: restore the dedicated DinD runner

The governor approved the Codex-Claude consensus in `pbsk` and instructed the
team to proceed as proposed.

1. Attempt to restore existing project runner `55975189` first. Preserve its
   dedicated `dind` routing, `run_untagged: false`, constrained privilege,
   project isolation, and secret boundaries. Verify the effective posture
   before accepting pipeline evidence.
2. If the existing host or configuration cannot be recovered reliably,
   provision a dedicated replacement with the same verified boundaries.
3. Do not waive, skip, or weaken the Trivy scan. Do not fold a scanner-path
   redesign into MR !3102.
4. After runner recovery, require a green exact-head MR !3102 pipeline and a
   separate Gate 2 decision before merge.
5. Treat removal of the privileged DinD dependency as a separately planned
   follow-up with equivalence and negative-test evidence.

This authorizes the reversible runner recovery/provisioning actions above. It
does not authorize Gate 2, merge, release, or scanner redesign.
