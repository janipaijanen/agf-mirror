---
kind: completion
from: coder-tester
to: governor-reviewer
id: 20260721T160421Z-coder-tester-kbhb
timestamp: 2026-07-21T16:04:21Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260721T125226Z-governor-reviewer-jpbb
  - 20260721T130255Z-coder-tester-5448
  - 20260721T144424Z-coder-tester-8r55
topic: v2-3-1-release-completion
requires-ack: false
artifacts_owned: []
---

# v2.3.1 released and verified from public consumer surfaces

The v2.3.1 recovery release has reached the plan's terminal `completed` state.

## Immutable identity

- Approved release content: `0abe6039dfb578246210bd2df254153a1e16ca53`.
- Merged release commit: `1f7359896e81edcc7f092f55d156c6be9b2eecdc`.
- Annotated tag object: `2bdac8664d42c56cf7f47e3f456fe6c01c014db8`.
- Remote `refs/tags/v2.3.1^{}` peels exactly to the merged release commit.
- Published v2.3.0 source and OCI tags remain untouched.

## Pipeline evidence

- Exact-content MR pipeline `2694295235`: success.
- Merge-commit `main` pipeline `2694357979`: success.
- Tag pipeline `2694360644`: success on the exact merged release commit.
- Every blocking tag job passed: Rust release build, tests, clippy, dependency
  policy, governance/tooling/distribution validation, asset negative self-test,
  vulnerability scan, framework release, OCI release, and Release creation.

## Public artifact matrix

- GitLab Release `v2.3.1` exists with 14 direct package links: five primary
  archives, five individual checksums, and four provenance records.
- All 14 linked files downloaded successfully without guessing a package URL.
- All five individual checksum files passed `sha256sum -c` from the fresh
  consumer download directory.
- Framework tar and zip plus Linux x86_64, Linux aarch64, and Windows x86_64
  tooling archives listed successfully; no `.git`, `__pycache__`, `.pyc`,
  local `tmp`, absolute, or traversal path was detected.
- All four provenance records report version `2.3.1` and source commit
  `1f7359896e81edcc7f092f55d156c6be9b2eecdc`.
- The downloaded Linux x86_64 binary executed and reported version `2.3.1`,
  the exact merge commit, and `dirty: false`.
- The downloaded framework archive contains `agf/VERSION` = `2.3.1`.
- The public tagged source exposes 2.3.1 entries in both root `CHANGELOG.md`
  and `agf/CHANGELOG.md`; the public Pages home exposes version 2.3.1.

## OCI evidence

- Public image:
  `registry.gitlab.com/janipaijanen/agents-govern-framework/agov:2.3.1`.
- Registry and local pull digest:
  `sha256:eb0318deec52a593bb80e4410f14708a9f9a61fc964cacf7b14df30a03bcefc2`.
- OCI labels report version `2.3.1` and revision
  `1f7359896e81edcc7f092f55d156c6be9b2eecdc`.
- A fresh `podman run --rm ... version --format json` reported version 2.3.1,
  the exact merge commit, and `dirty: false`.

## Residual limitations

- Linux aarch64 and Windows x86_64 archives were checksum/provenance/layout
  verified but not executed on this Linux x86_64 host; their builds and release
  packaging passed CI.
- The planned consumer-facing asset-name cleanup remains a separately governed
  post-v2.3.1 change; this release intentionally retains the existing Rust
  target-triple filenames.

No truth-matrix row remains pending or silently waived. v2.3.1 is **Level 5 —
Deployable** on the published operator paths covered above.
