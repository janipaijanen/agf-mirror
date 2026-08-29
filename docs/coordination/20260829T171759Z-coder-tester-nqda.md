---
kind: claim
from: coder-tester
to: all
id: 20260829T171759Z-coder-tester-nqda
timestamp: 2026-08-29T17:17:59Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260829T170649Z-governor-reviewer-h3nb.md
topic: v290-release-integrity-slice-a-implementation
requires-ack: false
artifacts_owned:
  - .gitlab-ci.yml
  - tooling/rust/scripts/agov-oci-image.sh
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Claim v2.9.0 release-integrity Slice A implementation

Coder-tester claims the Gate-1-approved v2.9.0 Slice A work order at
`origin/main` `6bcc92f42a1c86bca76d65cad018533cc5a98493`.

The implementation is limited to A1 and A2, with A3 as their shared negative
proof obligation, exactly as approved in `h3nb` and
`docs/plans/20260828T194721Z-v290-release-integrity-and-tracker-truth.md`.

Expected implementation paths are limited to `.gitlab-ci.yml` and
`tooling/rust/scripts/agov-oci-image.sh`:

- add an MR-stage OCI build and Trivy scan consuming the existing
  `rust:tooling:preview` archive, with the complete approved path anchor and no
  second Rust build;
- separate unpublished build-and-scan behavior from tag-gated push and
  provenance behavior;
- expose one canonical OCI-reference derivation for both the checked and
  published name;
- provide reviewer-visible reproducible negative proofs for a vulnerable base
  and a mismatched OCI reference without leaving the canonical pipeline red.

No release namespace, release identity, deferred slice, roadmap item, or
recommendation-only `2dga` question is included. If either A1 or A2 requires a
third repository path or otherwise grows, implementation stops and returns to
Gate 1 as required by `h3nb`.
