# Changelog

All notable repository-level changes to AGF-NG are recorded here.

The canonical adopter-facing AGF v2 changelog lives at
[`agf/CHANGELOG.md`](agf/CHANGELOG.md). The bootstrap reference changelog for
agents-govern 1.x lives at [`agents-govern/CHANGELOG.md`](agents-govern/CHANGELOG.md).

## [Unreleased]

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
