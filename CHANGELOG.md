# Changelog

All notable repository-level changes to AGF-NG are recorded here.

The canonical adopter-facing AGF v2 changelog lives at
[`agf/CHANGELOG.md`](agf/CHANGELOG.md). The bootstrap reference changelog for
agents-govern 1.x lives at [`agents-govern/CHANGELOG.md`](agents-govern/CHANGELOG.md).

## [Unreleased]

## [2.3.1] - 2026-07-21

Patch release recovering the repository's public release truth after v2.3.0
was published with this changelog still ending at 2.2.5. Tag `v2.3.0` remains
immutable; its historical entry was added on `main`, and v2.3.1 publishes that
correction under a new release identity.

### Added

- Proposed learnings from the v2.3.0 release now record consumer-layout host
  preflight and complete release truth-surface enumeration as future target
  states. Their generalized recommendations are not implemented by this patch.

### Fixed

- GitLab Release creation now exposes direct links for all individual checksum
  and provenance sidecars, verifies the mandatory archive/sidecar set before
  upload, and carries a negative self-test for a missing provenance record.
- Added the immutable recovery trail that supersedes premature v2.3.0 terminal
  completion claims without moving the published v2.3.0 source or OCI tags.

## [2.3.0] - 2026-07-21

Repository-level changes accompanying the v2.3.0 framework release
(see [`agf/CHANGELOG.md`](agf/CHANGELOG.md) `[2.3.0]` for the
adopter-facing feature changelog). Found and recorded during this
release's own public-truth verification pass — this repository-level
changelog had not been updated since 2.2.5.

### Fixed

- `agf/dist/release-manifest.yaml`: excluded `__pycache__/` and `.pyc`
  paths from the release package `exclude`/`forbidden` lists, closing a
  leak of stale Python bytecode into the framework release archives
  (MR !303).
- `.gitlab-ci.yml`: the `rust:tooling:preview`/`rust:tooling:release`
  checksum-generation step recorded a build-tree path
  (`../../dist/<archive>`) in each `.sha256` file instead of the archive
  basename, breaking the standard `sha256sum -c` consumer-verification
  flow. Fixed to match the basename-only convention `agov dist build`
  already used for framework archives (MR !308).

### Changed

- Enabled the project's GitLab Container Registry (previously disabled),
  required for the `agov:oci:release` tag-pipeline job to publish the
  `agov` OCI image. This had not been exercised end-to-end against a real
  tag before v2.3.0.

## [2.2.5] - 2026-06-25

This patch release completes AGF-NG's transition away from historical
agents-govern 1.x Python tooling and records the repository-level metadata
needed for the current Rust-only operator path.

### Changed

- AGF v2 contract documentation now marks v1.x Python-only tooling surfaces as
  retired or deferred before historical Python code removal. Binding validation
  is explicitly deferred in current Rust `agov`; `agov-validate-acronyms`,
  `agov-bump`, `agov --capabilities`, and `agov --contract-changelog` are
  retired; `agov-health` remains deferred.

### Removed

- Historical agents-govern 1.x Python package sources and package metadata are
  removed from `agents-govern/`; Rust `tooling/agov` is the only supported
  AGF-NG operator tooling.

### Added

- Root `LICENCE` file summarising the repository licence map:
  CC BY-SA 4.0 for framework/governance content, MIT for tooling, and
  separate commercial or other terms where expressly granted.
- Root `CHANGELOG.md` to make repository-level changes visible before readers
  enter the adopter-facing `agf/` distribution or bootstrap reference tree.

## [2.2.0-rc.2] - 2026-06-24

### Fixed

- Rust `agov` release archives now embed CI commit provenance in the binary
  itself, so `agov --version` and `agov version --format json` report the tag
  pipeline commit instead of `unknown dirty`.

## [2.2.0-rc.1] - 2026-06-24

### Added

- Readable coordination IDs are now the default format for
  `agov coord brief`.
- Rust `agov` reports version provenance through `agov --version` and
  `agov version --format json`.
- `agov coord graph-index` now has layered contract documentation covering
  human operation, AI cold-start use, output fields, and exit behavior.

### Changed

- RC-1 readiness audit completed: the Rust operator path was verified,
  inherited v1.x material in high-risk public paths was labelled, and release
  documentation was aligned with current tooling behavior.
