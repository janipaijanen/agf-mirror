---
id: v247-publication-recovery
status: approved
topic: v247-publication-recovery
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >
      The release executor reproduced the v2.4.6 tag failure twice on distinct
      GitLab SaaS runners and classified the proposed recovery boundary before
      drafting. A fresh exact-head contribution-auditor pass is requested on
      this plan before Gate 1, with particular attention to immutable-tag
      handling, partial-publication checks, SemVer classification, and whether
      optional Windows availability is described without implying support.
  - role: infosec-sentinel
    status: consulted
    reason: >
      This plan changes release asset selection but introduces no credentials,
      new registry, or trust boundary. The mechanical `agov validate infosec`
      check is required at plan and implementation heads. Existing provenance,
      checksum, OCI digest, and post-publication verification remain mandatory
      for every asset that v2.4.7 publishes.
---

# Plan: v2.4.7 publication recovery with optional Windows tooling

## Incident and authority

Issue [#95](https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/95)
records the publication incident. The governor-reviewer approved v2.4.6 Gate 2
at source head `d7ebcaf23ac324b83a2fbe11d11d06caeff4d033`. MR !840 merged as
`3d22406e8a4c342ee297ea38d3088d5495b659ba`, and the annotated immutable
`v2.4.6` tag correctly peels to that merge commit.

Tag pipeline `2719383818` failed twice in `rust:tooling:release` on two distinct
fresh GitLab SaaS runners. Both failures were `No space left on device` while
the combined job accumulated host, Linux x86_64, Linux aarch64, and Windows
x86_64 build outputs. The second run crossed a different crate boundary before
reproducing the same failure. All downstream test and publication jobs were
skipped. Read-only verification confirmed:

- no GitLab Release exists for v2.4.6;
- no v2.4.6 generic package exists;
- no v2.4.6 OCI publication job ran; and
- no partial public release asset escaped.

The governor then directed that Windows may be dropped from the release build
or retained only as optional because there are no current Windows consumers or
customers. Gate 1 was approved at exact plan head
`a7e2f3da6923c26d9c652f8187fa1a00d4dfe542` and recorded in
`20260730T174626Z-governor-reviewer-q4ad`; implementation may now proceed
within this plan's exact boundary.

## Decision and rationale

For v2.4.7, Windows tooling is **optional, unpublished preview evidence**, not a
mandatory release asset:

- framework tar/zip, Linux x86_64/aarch64 tooling, and OCI remain mandatory;
- the tag release job no longer builds Windows;
- the existing manually triggered Windows preview/package job remains present,
  `allow_failure: true`, and outside the release dependency graph; and
- release discovery and documentation treat each GitLab Release page's actual
  assets as the authoritative platform inventory.

This is preferable to another blind retry: the same finite runner-disk failure
reproduced twice, and retries consume paid minutes without changing the peak
build shape. It is preferable to moving/deleting v2.4.6, which the immutable-tag
rule forbids. It is preferable to manually publishing a partial v2.4.6 release,
which would bypass the tag's own mandatory-asset gate and provenance chain.

The change is patch-compatible. Windows is already labelled an optional tooling
archive in `agf/docs/site/reference/release-artifacts.md`; the supported
contract is release-page discovery, checksums, provenance, and exact pinning for
assets actually published. No CLI, schema, exit code, or framework behavior is
removed. The docs will stop making the contradictory claim that every tag
publishes Windows while retaining honest instructions for a Windows asset if a
future release provides one.

## Exact implementation scope

Implementation changes exactly 27 files.

### Release pipeline and asset contract (2)

1. `.gitlab-ci.yml`
   - change the Windows packaging block in `.rust-tooling-package` to run only
     when `AGOV_PACKAGE_WINDOWS_PREVIEW=1`, not automatically for tags;
   - update comments to state that tag releases publish Linux tooling and the
     Windows package path is manual preview evidence;
   - keep both existing Windows preview jobs manual and `allow_failure: true`;
   - do not weaken framework, Linux, OCI, checksum, provenance, release-create,
     or post-publication verification jobs.
2. `tooling/rust/scripts/release-assets.sh`
   - remove the three Windows archive/checksum/provenance filenames from the
     mandatory `expected` set;
   - keep `.zip` discovery so a deliberately supplied optional Windows asset
     can still be linked;
   - extend `selftest` to prove Windows is not required, an optional Windows zip
     is discovered when present, and a missing mandatory Linux/provenance asset
     still fails closed.

### Adopter/release documentation (2)

3. `agf/docs/site/reference/release-artifacts.md`
   - distinguish mandatory v2.4.7 Linux tooling from optional Windows preview
     output;
   - remove the claim that Windows is published on every tag pipeline;
   - preserve the platform-slug and provenance rules for any optional Windows
     archive that is intentionally produced later.
4. `agf/docs/site/docs/adopt/tooling-and-tokens.md`
   - make the Windows checksum example explicitly conditional on the chosen
     release containing that asset;
   - state that Windows is currently an optional preview/build-from-source path,
     while the release page remains authoritative.

### Repository release history (1)

5. `CHANGELOG.md`
   - add a dated v2.4.7 repository-level entry recording the failed/unpublished
     v2.4.6 tag, the runner-disk cause, and the optional-Windows recovery;
   - do not rewrite the historical v2.4.6 tag or imply it published assets.

### Standard release-truth surfaces (22)

6. `agf/VERSION` to `2.4.7`.
7. `agf/CHANGELOG.md`: finalize a dated v2.4.7 patch entry documenting the
   publication recovery and optional Windows release policy; retain the
   historical v2.4.6 change list but add a conspicuous publication-status note
   that v2.4.6 was tagged, never published as a GitLab Release, and is
   superseded for adoption by v2.4.7. Do not describe v2.4.6 as withdrawn or
   alter its immutable tag.
8. `agf/templates/AGENTS.md`: release comment to 2.4.7.
9. `tooling/rust/Cargo.toml`: workspace version and the three internal path
   dependency requirements to 2.4.7.
10. `tooling/rust/Cargo.lock`: only the five workspace package versions to
    2.4.7; no external dependency/checksum changes.
11. `agf/docs/site/roadmap.md`: v2.4.7 becomes the latest published baseline;
    record v2.4.6 as an immutable but unpublished recovery predecessor without
    changing unrelated roadmap statuses.
12–23. All 12 `agf/agents/*.yaml` definition `framework_version` fields to
    2.4.7; agent definition `id` and `version` remain unchanged.
24. `agf/agents/MANIFEST.yaml`: live framework-version header to 2.4.7 only.
25. `agf/ADOPTING.md`: copyable `framework_version` to 2.4.7; agent IDs remain
    the already-correct definition versions.
26. `agf/templates/learning-record.yaml`: copyable framework version to 2.4.7.
27. `agf/docs/governance/manifest.md`: live/current release claims and upgrade
    example to 2.4.7; historical v2.0/v1.x statements remain historical.

No other file is in scope. In particular: do not change Rust source, schemas,
governance rules, dependency versions/checksums, the v2.4.6 tag, future
release-truth refactor issue #94, or the separately planned Elixir cleanup.

## Gate 2 evidence before the v2.4.7 tag

At the exact implementation head:

1. GitLab CI lint accepts the expanded configuration for MR, default-branch,
   tag, and manual Windows-preview situations.
2. The release-asset selftest proves the new mandatory Linux/framework/OCI set,
   optional Windows discovery, and the negative missing-mandatory case.
3. A dry-run/tag-rule inspection proves `rust:tooling:release` does not enter
   the Windows block and both Windows preview jobs remain manual/allow-failure.
4. `agov` and `agov-mcp` self-report 2.4.7, the exact head, and clean source.
5. Cargo metadata shows five 2.4.7 workspace packages and internal requirements
   `^2.4.7`; the lockfile changes only those five versions.
6. The exact 27-file diff passes `git diff --check`, full Rust tests, clippy,
   cargo-deny, CVE scan, `agov validate`, agent/adopter/plan/infosec validation,
   release-path validation, and the documentation build.
7. Framework tar and zip build cleanly, contain no Elixir artifacts, and run
   every shipped Python test successfully.
8. A complete semver sweep classifies all remaining older values and finds no
   stale live package-release scalar.
9. The full MR pipeline is green at the exact Gate 2 head. Gate 2 is required
   before merge; no v2.4.7 tag exists before that merge.

The manual Windows cross-build is not mandatory Gate 2 evidence: the recovery
explicitly removes Windows from the published v2.4.7 contract, and re-running an
unneeded platform build would spend runner minutes without reducing release
risk. CI lint and rule inspection must still prove that the optional manual
surface remains well-formed and non-blocking.

## Post-tag verification

After Gate 2 and the release MR merge:

1. Create annotated `v2.4.7` on the resulting `main` merge commit and push it
   once. Never move/delete v2.4.6 or v2.4.7.
2. Confirm the tag pipeline runs one Linux tooling release build without a
   Windows cross-compile and passes the full test matrix.
3. Confirm `release:create` verifies and publishes the exact mandatory set:
   framework tar/zip, Linux x86_64/aarch64 tooling, their checksum/provenance
   sidecars, and OCI provenance. No Windows asset is required or claimed.
4. Confirm the GitLab Release exists and `release:verify` downloads the public
   assets, validates bytes/provenance/checksums, checks eligible packaged-binary
   self-reporting, and verifies OCI digest/labels.
5. Record completion and close issue #95 only after public verification passes.

## Rollback and stop conditions

Before tagging, revert the entire implementation commit if any mandatory
platform, provenance, or publication check regresses. After tagging, never
rewrite the tag: stop publication claims, preserve evidence, and recover only
through a later patch.

Stop and return to Gate 1 if implementation requires a file outside the 27-file
list, if any partial v2.4.6 publication is discovered, if Windows is found to be
a contracted mandatory consumer surface, or if CI cannot prove the optional
Windows block is absent from tag execution.
