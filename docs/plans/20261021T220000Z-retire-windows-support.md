---
id: 20261021T220000Z-retire-windows-support
status: pending-gate1
topic: retire-windows-tooling-support-and-preview-ci
advisory_consultation:
  - role: infosec-sentinel
    status: pending
  - role: contribution-auditor
    status: pending
affected_paths:
  - .gitlab-ci.yml
  - CHANGELOG.md
  - agf/CHANGELOG.md
  - docs/reviews/rule-projection/20261021T220000Z-retire-windows-support.yaml
  - tooling/rust/crates/agov/tests/ci_needs_implication.rs
  - tooling/rust/crates/agov/src/ci_cache_topology.rs
  - tooling/rust/crates/agov/src/main.rs
  - tooling/rust/crates/agov-mcp/src/main.rs
  - tooling/rust/scripts/check-release-self-sufficiency.sh
  - tooling/rust/scripts/release-assets.sh
  - tooling/rust/scripts/agov-oci-image.sh
  - agf/docs/site/docs/adopt/tooling-and-tokens.md
  - agf/docs/site/docs/adopt/mcp-server.md
  - agf/docs/site/docs/start/commission.md
  - agf/docs/site/reference/release-artifacts.md
depends_on:
  - 20261021T204757Z-remove-dind-ci
  - 20260914T130217Z-row6-s4-issue123-collision-check-blocking-promotion
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Retire Windows tooling support

## Decision and scope

The governor said “remove windows support” on 2026-10-21 after the Windows
Podman feasibility spike failed and the current runner blocked the nested
container scan. This plan makes that direction reviewable. Retire the optional
Windows x64 `agov`/`agov-mcp` build and packaging paths, and stop presenting
Windows as an available or prospective tooling platform in current adopter
and release documentation. Historical release, changelog, plan, learning, and
coordination records remain unchanged. Linux release archives and the OCI
image remain supported. Incidental portable Rust code need not be deleted
merely because it can compile on Windows; this change withdraws the support
contract and its maintained build path.

The prior approved plan, `20261021T204757Z-remove-dind-ci`, explicitly stopped
at a failed Windows spike and reserved dropping Windows packaging for a later
governor decision. This is that disposition, with its own implementation scope.
Part 1 of the prior plan is being attempted in draft MR !3102 and has an
independent runner blocker; this plan neither changes nor claims that MR.
Coordinate `.gitlab-ci.yml` and its CI test with its owner before editing.

## Implementation

1. Delete the two manual Windows preview jobs, their private DinD/Cross
   template, the optional Windows packaging block in `.rust-tooling-package`,
   the `AGOV_PACKAGE_WINDOWS_PREVIEW` switch, ZIP artifact globs and optional
   ZIP validation from `.gitlab-ci.yml`. Keep the framework source ZIP and
   historical release artifact verification contract intact.
2. Adjust CI assertions for the removed jobs. Remove Windows-specific
   package fixtures only where their purpose is to support the retired
   current build; preserve checks that protect the existing Linux archive
   and general archive parsing. Remove obsolete comments tied to Windows
   preview jobs. Keep explicit rejection of Windows targets on the Linux OCI
   path if it guards a live interface, with accurate wording.
3. Update current adopter and release-reference pages to state the supported
   Linux archive and OCI path, without a Windows download, PowerShell checksum
   example, or preview promise. Existing immutable release history stays
   factual. Do not change `mise.lock`, which is generated and currently has
   unrelated local edits in the primary checkout.
4. Leave the failed Windows feasibility evidence in MR !3103 as the audit
   trail. Close or supersede the Windows migration part of MR !3100 only through
   its owner and a published governor decision; do not silently alter that
   active branch.

## Acceptance evidence and gate

- Search active CI, scripts, and current adopter docs for Windows build jobs,
  `x86_64-pc-windows-gnu`, `AGOV_PACKAGE_WINDOWS_PREVIEW`, and Windows archive
  promises; remaining matches have a documented generic or historical reason.
- Validate the full CI configuration and run the CI selection/needs tests,
  release asset self-tests, packaging checks, and current docs build affected
  by the deletion. Confirm Linux archive and OCI jobs remain selected and
  their `needs` are satisfiable.
- Publish the implementation on its own branch and obtain an exact-head green
  MR pipeline. Gate 2 remains a governor-reviewer merge decision; this plan
  does not authorize merge, release, tag, or runner deregistration.

This is non-trivial CI and release-surface work, so the normal Gate 1 plan and
advisory consultation apply. Implementation starts only after a published
binding Gate 1 decision records approval for this plan and ownership of the
shared CI paths is resolved.

## Downstream impact and migration

Impact:
  Foundry: BREAKING — the two manual Windows preview jobs and their DinD/Cross
    setup disappear; Linux packaging and OCI release jobs remain.
  Direct adopters: BREAKING for anyone using the optional Windows preview ZIP
    or building it from this project's CI. Use the published Linux x86_64
    archive on a Linux host or the OCI image. Previously downloaded Windows
    preview files remain historical artifacts, without ongoing support.
  Downstream: No framework source archive format, governance file, or agent
    behavior changes; the optional Windows tooling artifact is no longer
    produced or listed by current release tooling.
  Migration: Move Windows tooling automation to a Linux x86_64 runner or
    consume the Linux OCI image. Remove references to the retired Windows
    preview jobs and `AGOV_PACKAGE_WINDOWS_PREVIEW` variable.
  Evidence: MR !3103 records the failed Windows Podman feasibility spike;
    MR !3102 records the current runner's nested-container failure.

Adopter docs parity: updated — current installation and release-reference
pages remove the Windows preview/download path and state the Linux path.
