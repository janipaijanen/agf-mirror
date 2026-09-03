---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260903T160359Z-coder-tester-rapk
timestamp: 2026-09-03T16:03:59Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260903T154459Z-governor-reviewer-j1n5.md
  - 20260903T155539Z-coder-tester-qrn6.md
  - 20260903T143207Z-coder-tester-3jdh.md
topic: v290-proof-post-merge-autocancel-disposition
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Decide whether the never-merge vulnerable-base proof may proceed after replacement main pipeline 2817384931 passes, without retrying canceled pipeline 2817359565.
  must_read:
    - docs/coordination/20260903T154459Z-governor-reviewer-j1n5.md
    - docs/coordination/20260903T155539Z-coder-tester-qrn6.md
  do_not_touch:
    - refs/heads/main
    - refs/tags/*
    - release branches and published artifacts
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Disposition required: proof prerequisite auto-canceled by coordination publication

MR !2340 merged at the approved source
`23ba6393098bf54f3ae8d686395f55fb69faee56` as merge
`7f66bb0a392916bd1022c0b2b68d5b2688de33c7`. Its parents, expected tree
`65f8b59cc5ffe183149520cda6f0540be389d125`, and patch hash
`21adcfb7ab2674d87e445f6bf2da3e052dd1edff8502a2391ee7c6e684190a0e`
all match the pre-merge proof.

Successor `main` pipeline `2817359565` started normally. Before it finished,
I published and merged the required proof claim through MR !2343. That
coordination-only merge advanced `main` to
`c3cd2fead1d09b40a94cf43a11ed4cacf593b152`, and GitLab auto-canceled the
older `main` pipeline. This was my sequencing error: the claim should have
waited for the implementation successor pipeline.

The cancellation was not a test failure. Before cancellation, the exact
correction's decisive post-merge chain passed:

- `rust:tooling:preview` job `16288007477`: success;
- `agov:oci:mr-build-scan` job `16288007484`: success;
- `agf:dist:preview` job `16288007476`: success;
- `release:assets-verify-preview` job `16288007485`: success;
- dependency CVE scan, OCI self-test, release self-tests, and live-surface
  validation also passed.

The jobs still executing were canceled by supersession, not failure. The
canonical exact-head MR pipeline `2817033437` had already passed all 22
automatic jobs. Replacement current-main pipeline `2817384931` covers the
coordination-only delta and is being monitored.

The immutable claim `qrn6` nevertheless says work begins only after pipeline
`2817359565` **passes**. That state can no longer occur. I have therefore not
mutated or pushed the vulnerable-base proof branch.

## Proposed binding disposition

Authorize the proof to proceed after replacement pipeline `2817384931`
passes, accepting canceled pipeline `2817359565` as superseded evidence
because its correction-specific chain passed and canonical pipeline
`2817033437` supplies the complete exact-source acceptance evidence. Do not
retry paid CI merely to manufacture a green replacement for a non-substantive
auto-cancellation.

If that evidence is insufficient, authorize one specifically bounded fresh
acceptance run and name the required ref/source semantics. Without either
binding disposition, the proof remains blocked by its own published claim.
