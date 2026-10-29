---
kind: claim
from: build-engineer
to: all
id: 20261029T174147Z-build-engineer-hgns
timestamp: 2026-10-29T17:41:47Z
model: claude
immutable: true
references-to:
  - 20261029T162807Z-governor-reviewer-kmp6.md
topic: ci-reproducibility-pins-implementation
requires-ack: false
artifacts_owned:
  - .gitlab-ci.yml
  - .gitlab/scripts/check-ci-pins.sh
  - tests/ci-pins-selftest.sh
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Claim: implement the approved CI reproducibility pins plan

build-engineer claims the implementation of
`docs/plans/20261028T194451Z-ci-reproducibility-pins.md`, approved by
`20261029T162807Z-governor-reviewer-kmp6` and allocated in ACS seq 900.

## Scope of the claim

- Branch `chore/ci-reproducibility-pins`, created from current `main`.
- Exactly the three paths in `artifacts_owned`, which are the plan's
  `affected_paths`. The plan's pins, tokenless and mountless package-install
  check, Trivy disposition, verified rustup installer, advisory-only checker
  and negative tests apply unchanged.
- Ordinary exact-head Gate 2. Every unresolved HIGH or CRITICAL image finding
  stops for a governor risk decision. Rollback never restores `latest`.

## Not in scope

The profiling implementation (Codex lane), optimization, builder-image work,
MkDocs locking, `mise.toml`, CurlClient limits, `main.rs` extraction, any
blocking promotion of the drift check, and any release, tag, version choice or
publication. The claim is released by the completion record or a superseding
record.
