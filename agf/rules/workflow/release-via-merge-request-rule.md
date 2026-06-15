---
provenance: verified-agf-v2
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Releases: Via Merge Request Only
#
# Framework principle: Immutability Constraint, Quality Gates
# See: framework.md — Principle 3 (The Immutability Constraint)
# Companion rule: immutable-tags-rule.md

**Scope:** All version releases and git tag creation.

## Rule

A version tag MUST only be created on the main branch **after** a merge request has been reviewed and merged. Direct commits to main followed by tagging are forbidden for release versions.

## Release workflow

1. Ensure all feature commits are merged to main first.
2. Create a `release-X.Y.Z` branch **from main** — at this point, main
   and the release branch are identical.
3. On the release branch (not main), commit the release-only changes:
   - `VERSION` bump
   - `CHANGELOG.md` entry for the new version
   - Any other release metadata
4. Open a merge request from `release-X.Y.Z` → main. The MR diff
   contains **only** the version bump and changelog — making review
   trivial and the MR non-empty.
5. Review the MR: verify version consistency, CI passes, distribution
   archives build correctly.
6. Merge the MR.
7. Tag the merge commit on main with the version (e.g. `v0.15.0`).
8. Push the tag — CI creates the release with archives.

**Critical:** Never commit the version bump to main before creating the
release branch. If you do, the MR will have no diff and cannot be
reviewed.

## Why

Tags are immutable (see immutable-tags-rule.md). A broken tag cannot be fixed — only superseded by a new version. The merge request is the quality gate that prevents tagging broken commits. Without it:

- CI config errors produce broken releases (empty version strings, missing assets).
- Version references across files may be inconsistent.
- Distribution archives may contain wrong content.

The cost of a merge request is minutes. The cost of a broken immutable tag is a wasted version number and a confusing release history.

## Forbidden operations

- Tagging main directly after pushing commits without an MR review.
- Tagging a non-main branch for release.
- Creating tags from CI pipelines without a preceding MR merge.

## What the MR MUST verify

- [ ] `VERSION` file matches the intended release version.
- [ ] `CHANGELOG.md` contains an entry for this version (not backfilled later).
- [ ] All version references are consistent (agent definitions, documentation).
- [ ] `mix agov.dist.validate` and `mix agov.dist.build` pass from `tooling/elixir/` (if applicable).
- [ ] CI pipeline configuration is correct for the release.
- [ ] SPDX headers and license are correct across all files.

## Exceptions

Lightweight tags for non-release purposes (e.g. `deploy-staging-20260405`) are exempt from the MR requirement but still subject to the immutability rule.
