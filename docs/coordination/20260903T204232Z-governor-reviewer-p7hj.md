---
kind: decision-record
from: governor-reviewer
to: all
id: 20260903T204232Z-governor-reviewer-p7hj
timestamp: 2026-09-03T20:42:32Z
model: human-directed-codex-gpt-5
immutable: true
references-to:
  - 20260903T160359Z-coder-tester-rapk.md
  - 20260903T160953Z-planner-architect-t615.md
  - 20260903T155539Z-coder-tester-qrn6.md
  - 20260903T154459Z-governor-reviewer-j1n5.md
topic: v290-proof-autocancel-accepted
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Approved: accept supersession and proceed with the vulnerable-base proof

Binding human decision. The governor-reviewer accepts successor `main`
pipeline `2817359565` as superseded without retry. Its cancellation followed
the coordination-only merge that published the proof claim; it was not a test
failure.

The evidence is sufficient because canonical exact-head MR pipeline
`2817033437` passed all 22 automatic jobs, and the correction-specific
post-merge chains completed successfully before supersession:

- `rust:tooling:preview` `16288007477` to `agov:oci:mr-build-scan`
  `16288007484`; and
- `agf:dist:preview` `16288007476` to
  `release:assets-verify-preview` `16288007485`.

Replacement current-main pipelines `2817384931`, `2817408763`, and
`2817429934` also passed. Planner clarification `t615` correctly observes
that the approved plan requires the successful canonical exact-head MR
pipeline, not a green successor-main pipeline.

The `qrn6` precondition naming pipeline `2817359565` is superseded by this
decision. The coder-tester is authorized to proceed immediately with the
separate, fresh vulnerable-base acceptance proof. That proof must construct
a valid merge-request pipeline and fail specifically at the blocking Trivy
HIGH/CRITICAL gate. It must never merge. Capture and publish the exact branch,
commit, pipeline, job, CVE finding, and trace evidence, then close the proof
MR without merging it.

This decision does not authorize a paid retry merely to seek green status,
weakening the scan, changing CI configuration or image tooling, publishing a
release, creating or moving a tag, or mutating any protected release evidence.
