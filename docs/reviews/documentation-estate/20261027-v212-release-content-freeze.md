<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.12.0 release-content freeze sweep — 2026-10-27

**Role:** documentation-estate-steward  
**Pre-implementation `main`:** `e03b3261ccb8a4205c45801ea1d034971f437a2d`  
**Previous release tag object:** `d732e372f35bacd1f37981bb89031041d35121e4`  
**Previous release peeled commit:** `c1a13f8cdf626a9c10df5ddc82cc8cf6ebefb48a`  
**Status:** complete for the v2.12 release-content-freeze cadence; exact-head
fact review and Gate 2 bind the correction MR's final SHA externally.

## Frozen content reconciliation

| Item | Reviewed head | Merge commit | Changelog disposition |
|---|---|---|---|
| Retained MR !3098 skill-command correction | `6fccd783116bef86a908b4e53eadf9fd32d67b14` | `dc5523f6e9b5372e374aa3368b6e98ae3800553c` | Existing `[Unreleased]` Fixed entry retained; correction record `pbgd` preserves the non-retroactive Gate 2 chronology. |
| Tooling coexistence guidance, MR !3311 | `d347c4b75d0d64231e323ed7b8e88aa7ad9731b7` | `b703704c394624db387840be693c304dde9d264d` | Added entry: discoverability, authority/data-flow checklist, and bounded evidence calibration. |
| Homepage modernization, MR !3314 | `44c14e7ce630b72911146dfb9f293c8f7ce7d00f` | `dce4396f7b78e520df828876f7ae04ff9377949c` | Changed entry: four canonical routes and reviewed responsive/light-dark/focus presentation. |

The root `CHANGELOG.md` needs no content row: it delegates adopter-facing
changes to `agf/CHANGELOG.md`, and these items add no repository-operation
change.

## Findings and dispositions

| ID | Finding | Disposition |
|---|---|---|
| V212-F1 | The two frozen documentation items lacked `[Unreleased]` entries. | Fixed by the Added and Changed rows above. |
| V212-F2 | MCP publication-hold prose still called v2.11 rollout pending. | Fixed: shipped in v2.11.0; compatible artifacts remain required. |
| V212-F3 | Urgency and challenge-response prose still said Level 4 / unreleased, and publication holds requested confirmation of v2.11 shipment. | Fixed: shipped capability is Level 5 Deployable; mixed-version warning retained. |
| V212-F4 | The normative publication rule called v2.11.0 an intended future first release. | Fixed as factual release status only; no obligation or authority changed. |
| V212-F5 | Gate 2 record `n7mh` for already-merged MR !3256 was stranded on MR !3257. | Fixed: coordination-only MR !3257 merged as `4bc8eec4`. |
| V212-F6 | Bridge record `3mts`, cited by five binding governor records, was stranded on MR !3284. | Fixed: coordination-only MR !3284 merged as `7d218988`. |

## Released-byte evidence for v2.11 shipment claims

Assets were discovered anonymously through the official GitLab Releases HTTPS
API, downloaded to a fresh temporary directory, and verified before extraction
or execution.

- `agov-2.11.0-linux-x86_64.tar.gz`: SHA-256
  `01a8f2cf63cda43f0eefd711b44ba67ffb263cfad7984cad802c3955349ebfce`.
- `agf-2.11.0.tar.gz`: SHA-256
  `9477aab6dd24ac7b15e9791fae0a809de447e61fbb57acc46af20c8c509aa939`.
- Both provenance records bind the named artifact, checksum, tag `v2.11.0`,
  and source commit `c1a13f8cdf626a9c10df5ddc82cc8cf6ebefb48a`.
- The binary reports `agov 2.11.0` at that clean source commit and exposes
  `coord hold-status`.
- Disposable fixtures passed urgent ordering through `handoff next`, `coord
  pending`, and `session resume`, and passed open, expired, replied, and
  escalated challenge states through `coord pending`.
- The framework archive contains the coordination message schema and the
  challenge-response and publication rules. Temporary downloads and fixtures
  were removed after the checks.

No credential, shell trace, local path, or environment content is retained in
this evidence.

## Candidate inventory

No candidate entered the frozen two-item scope. MR !3066 and issues #105,
#111, #113, #119, #128, #133, #136, #137 and #138 remain open and unpromoted.
MR !3097 remains evidence only; no schema migration implementation exists.
Issue #129 remains withdrawn and #134 remains blocked. MR !2679 and the eight
park-radar merge requests remain closed.

MR !3272 is an unapproved record-only-publication plan. It may remain open as
deferred work and is not v2.12 release content.

## Release-metadata derivation boundary

The separate publication child plan must re-derive the exact allowlist from
the post-merge `main` tree. Expected current-version cells include `AGENTS.md`,
`agf/VERSION`, `agf/ADOPTING.md`, `agf/agents/MANIFEST.yaml`, all shipped agent
YAML files, the current version cells in governance docs and templates, the
live adoption cell in `agf/learnings/foundry.yaml`, and Rust Cargo metadata.
Release-truth edits include both changelogs and current status prose in the
governance manifest and two roadmaps. Historical v2.11 references remain
byte-identical.

This report authorizes no version bump, release branch, protected-tag change,
tag, artifact, publication, deployment, or announcement.
