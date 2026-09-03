---
kind: decision-record
from: governor-reviewer
to: all
id: 20260903T154459Z-governor-reviewer-j1n5
timestamp: 2026-09-03T15:44:59Z
model: human-directed-codex-gpt-5
immutable: true
references-to:
  - 20260903T143207Z-coder-tester-3jdh.md
  - 20260903T134709Z-governor-reviewer-qepm.md
  - 20260903T135544Z-coder-tester-8s5h.md
topic: gate2-approved-ci-mandatory-needs-implication
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 2 approved: CI mandatory-needs implication correction

Binding human decision. The governor-reviewer approves Gate 2 for MR !2340
at exact source head
`23ba6393098bf54f3ae8d686395f55fb69faee56`.

The decision adopts the exact-head evidence in handoff `3jdh`: canonical MR
pipeline `2817033437` passed all 22 automatic jobs, both corrected mandatory
producer/consumer chains executed successfully, the two Windows preview jobs
remain allowed manual jobs, local invariant and existing gate-change suites
passed, and the MR has zero unresolved discussions.

Authorized next sequence:

1. re-read the exact current target and recompute the clean merge result;
2. merge only source head
   `23ba6393098bf54f3ae8d686395f55fb69faee56` when the proof remains exact;
3. verify the resulting merge parents, tree and patch identity plus the
   successor main pipeline; and
4. perform the approved separate vulnerable-base acceptance proof on a fresh
   MR that must never merge, proving that the corrected graph reaches and
   fails specifically at the Trivy gate.

This decision does not authorize moving or creating a release tag, publishing
artifacts, mutating an immutable release identity, weakening the CVE scan,
merging the vulnerable proof, expanding beyond the approved correction, or
retrying a paid job merely to seek green status.
