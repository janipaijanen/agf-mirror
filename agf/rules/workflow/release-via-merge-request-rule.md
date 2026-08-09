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
8. Push the tag immediately — CI creates the release with archives.

**Steps 7–8 require no additional confirmation.** The merged MR is the
authorization. An agent or contributor MUST tag and push in a single action
immediately after merge — not stop to ask permission.

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
- [ ] For adopter-relevant changes, the release changelog entry contains
      `Adopter docs: updated`, `Adopter docs: not applicable`, or
      `Adopter docs: waived <coordination-id>`.
- [ ] All version references are consistent (agent definitions, documentation).
- [ ] `agov validate adopter-docs` passes.
- [ ] `agov dist validate` and `agov dist build` pass (if applicable).
- [ ] CI pipeline configuration is correct for the release.
- [ ] SPDX headers and license are correct across all files.

## Maintenance-line exception (optional, narrowly eligible)

The standard workflow above is the default and is not weakened by this
section. It exists for exactly one case the standard workflow cannot
represent safely: promoting a previously published, immutable
release-candidate commit to a stable version **after** the default
branch has accumulated unrelated later work, without pulling that
later work into the promoted release.

### Eligibility (all conditions required, evidenced before any branch is created)

- [ ] A binding governor record names the exact candidate tag object,
      peeled commit, tree, intended stable version, and why the
      standard current-main path is excluded.
- [ ] The candidate commit already has its own Gate 1, Gate 2,
      tag-pipeline, and post-publication evidence. A branch-only or
      unreviewed commit is ineligible.
- [ ] The new release changes only version, changelog, and other
      enumerated release-truth metadata. Any functional, schema,
      migration, test, dependency, CI, or artifact-layout change makes
      the candidate ineligible — use the standard workflow (a new RC
      or current-main release) instead.
- [ ] A separate Gate 1-approved release-publication plan freezes the
      exact base, branch names, metadata allowlist, validation matrix,
      immutable-namespace checks, and forward-sync procedure.
- [ ] The Git tag, GitLab Release, generic-package, and OCI namespaces
      for the new version are independently confirmed absent through
      their authoritative APIs.

If any condition becomes false, stop. Do not fall back to a temporary
rewind of the default branch, a custom tree-overwriting merge, or
artifact relabeling.

### Protected target and source

1. Create a maintenance target named `release/X.Y` at the exact
   eligible candidate commit. Protect it — reject force-push and
   deletion, restrict both push and merge to GitLab access level 40
   (Maintainer) — **before** any release content is created on it.
   Re-read the exact protection rule from the platform API as
   evidence; do not assume a wildcard pattern matched.
2. Create `release-X.Y.Z` at exactly the same candidate commit, with
   equivalent level-40 protection. Confirm both refs resolve to the
   frozen SHA before the first release-metadata edit.
3. Open a non-squash, non-rebase merge request from `release-X.Y.Z` to
   `release/X.Y`. Any change to source, target, merge base, or
   candidate head invalidates prior review.
4. The MR diff is restricted to the release plan's exact metadata
   allowlist.
5. A protected-tag rule matching the exact intended stable tag (or a
   governed `vX.Y.Z*` wildcard convention) must exist, with tag
   creation restricted to access level 40, before the tag is created.
6. Exact-head Gate 2 remains binding governor authority. Merge only
   the named SHA with the reviewed pipeline. **Retention of both
   maintenance refs is not satisfied by decision text alone** — the
   merge invocation's exact command (including its branch-retention
   option) and an immediate post-merge API read proving both refs
   still exist at the intended SHAs are required Gate 2 evidence.

### Tag and publication

After the maintenance-target merge, prove its tree differs from the
frozen candidate only by the approved metadata fields. Recheck all
four immutable namespaces and both ref identities. Then create one
annotated tag on the exact maintenance-target merge commit and push it
once — never move, delete, or recreate it (see immutable-tags-rule.md).
The tag pipeline binds every archive, checksum, provenance record,
package, and OCI digest to that tag commit; anonymous post-publication
checks operate on the downloaded bytes.

### Metadata-forward synchronization

Only after successful publication: freeze the latest default-branch
base, merge the tagged maintenance commit into a fresh sync branch
with `--no-ff` (squash and rebase are forbidden — they erase the
ancestry that proves the tagged commit is reachable), resolving so
every nonmetadata path stays byte-for-byte identical to the frozen
base. Open a separate MR to the default branch; prove the net diff
from the frozen base contains only the forward-metadata allowlist. Any
target advance before merge invalidates that proof and requires a
refreshed merge, pipeline, and review. If this sync fails, the
published release remains valid — correct the sync forward without
touching the tag or maintenance line.

## Exceptions

Lightweight tags for non-release purposes (e.g. `deploy-staging-20260405`) are exempt from the MR requirement but still subject to the immutability rule.
