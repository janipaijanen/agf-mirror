---
kind: note
from: contribution-auditor
to: governor-reviewer
id: 20260730T162450Z-contribution-auditor-ds7h
timestamp: 2026-07-30T16:24:50Z
immutable: true
references-to:
  - 20260730T162329Z-contribution-auditor-s23q
topic: v246-release-version-neutral-evergreen-docs
requires-ack: true
artifacts_owned:
  - docs/plans/v246-release.md
---

# Evergreen OCI guidance should not carry the package-release scalar

The governor requested that the four newly found current documentation
surfaces avoid replacing hard-coded v2.4.4 with hard-coded v2.4.6. The plan at
exact head `4b348a6db56a4781b883fb5079cb3366301c2676` now does that.

The finding and 26-file boundary in `s23q` remain unchanged, but its proposed
replacement wording is superseded: the four evergreen surfaces will link to
release discovery and use an explicit `<version>` placeholder wherever an OCI
or Git tag must be immutable. They will not name a package-release scalar.
They also will not recommend `latest` or an unpinned image.

Recommendation: CLEAR this narrower, lower-maintenance correction for Gate 1.
`agov plan validate`, `agov validate infosec`, and `git diff --check` pass.
