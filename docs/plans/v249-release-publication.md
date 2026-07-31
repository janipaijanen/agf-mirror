---
id: v249-release-publication
status: pending-gate1
topic: v249-release-publication
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: "The advisory review published with this plan checks release-only scope, version-surface completeness, immutable-tag safety, issue-closure timing, and separation from v2.4.10."
  - role: infosec-sentinel
    status: not-applicable
    reason: "The release consumes the already reviewed v2.4.9 implementation and changes version/changelog/release-truth metadata only; it adds no credential, secret, dependency, network trust boundary, learning payload, or security-control implementation. Existing release provenance and artifact checks remain mandatory."
---

# Plan: publish v2.4.9 live tooling-surface cleanup

## Decision requested

Authorize the patch release that makes the Gate 2-approved issue #93 correction
deployable. MR !865 was approved at exact head
`8481d8b463488ebeb40c0ba3aa795b445e12d134` and merged to `main` as
`f46a6519b54ad7e1d6d62a9f9dafc2a4f29d270f`. Its exact-head pipeline
`2721221648` passed. The source branch was retained as required.

The requested version is **2.4.9 (patch)**. This plan authorizes only release
metadata, validation, the release MR, one immutable annotated tag after the
release MR merges, publication verification, and issue #93 closure after that
verification. It does not authorize v2.4.10 implementation changes.

At planning time there is no `v2.4.9` tag, GitLab Release, or generic package.
The prepared local `release-v2.4.9` branch is still identical to
`origin/main`; no release metadata has been edited or committed.

## Release-only change set

Create the release commit on `release-v2.4.9`, based on the then-current
`main`, with only these release-truth changes:

1. Set `agf/VERSION` to `2.4.9`.
2. Add dated v2.4.9 entries to `CHANGELOG.md` and `agf/CHANGELOG.md` that:
   - identify the release as the issue #93 live tooling-surface cleanup;
   - record removal of the broken `bin/agf` Mix wrapper and dead Elixir parity
     harness;
   - record conversion of current contributor/adopter instructions to the Rust
     `agov` path and the fail-closed regression scan;
   - report the release validation count from the final release-branch run;
   - state `Adopter docs: updated`; and
   - reference issue #93 without auto-closing it before publication.
3. Update the five Rust workspace package versions, three internal path
   requirements, and the corresponding five `Cargo.lock` package entries from
   2.4.8 to 2.4.9. No dependency or checksum may change.
4. Update current framework-version surfaces only: all adopter-facing agent
   definitions, `agf/agents/MANIFEST.yaml`, `agf/ADOPTING.md`,
   `agf/templates/AGENTS.md`, `agf/templates/learning-record.yaml`, and current
   version statements/examples in `agf/docs/governance/manifest.md`.
5. Update `agf/docs/site/roadmap.md` to identify v2.4.9 as the latest release
   and mark issue #93 deployable only after publication verification. Preserve
   historical v2.4.8 recovery evidence and later release-batch commitments.
6. Leave the README major/minor citation unchanged because 2.4 remains the
   current release line. Leave historical coordination, plans, changelog
   entries, and valid "v2.4.8 or later" compatibility statements unchanged.

No product implementation, command surface, schema, validator semantic,
dependency, CI behavior, issue #92 work, v2.4.10 content, or branch deletion is
in scope.

## Validation before Gate 2

The release MR must provide exact-head evidence for:

- a focused version audit showing every current release-truth surface is 2.4.9
  and historical evidence was not rewritten;
- Cargo metadata showing exactly five workspace packages at 2.4.9 and the
  three internal requirements at `^2.4.9`;
- `cargo test --manifest-path tooling/rust/Cargo.toml`, with passed and ignored
  counts recorded in the changelog/MR evidence;
- strict workspace Clippy with warnings denied;
- `agov validate`, including adopter-docs validation;
- `agov dist validate` and clean tar/zip builds;
- all shipped archive self-sufficiency tests for both archive formats;
- `tooling/rust/scripts/check-live-tooling-surfaces.sh check` and `selftest`;
- built `agov` and `agov-mcp` self-reporting 2.4.9, the release MR exact head,
  and clean source;
- tag/version guard evidence with `CI_COMMIT_TAG=v2.4.9`;
- release-path, provenance, artifact-exclusion, SPDX/license, MkDocs, and
  `git diff --check` checks; and
- a green exact-head GitLab MR pipeline.

The release MR must target `main`, contain only the governed release-only
change set, retain its source branch, and receive exact-head Gate 2 approval.

## Merge, tag, and publication sequence

1. Merge only the exact Gate 2-approved release MR head and retain the source
   branch.
2. Confirm `origin/main` resolves to the resulting merge commit and that no
   `v2.4.9` tag exists locally or remotely.
3. Create one annotated `v2.4.9` tag on that exact `main` merge commit and push
   it immediately. Never move, delete, or recreate the tag.
4. Require the tag pipeline to pass, including framework archives, Linux Rust
   tooling archives, provenance/checksum validation, OCI publication, and
   GitLab Release creation. Optional Windows artifacts are reported according
   to the actual Release asset inventory and are not overclaimed.
5. Run `agov dist post-publication-check` against the anonymous GitLab Release
   API and a fresh download directory. Verify downloaded bytes and provenance,
   executable self-reporting, source archive contents, and the live tooling
   regression from the published framework archives.
6. Verify the GitLab Release, generic package `agf/v2.4.9`, and OCI tag
   `agov:2.4.9` exist and correspond to the immutable tag pipeline.
7. Only after all public checks pass, close issue #93 with links to the tag,
   pipeline, Release, and durable completion record. Mark the v2.4.9 milestone
   complete if one is present.
8. Publish and merge a validated completion coordination record. Only then may
   v2.4.10 MR !869 be rebased onto the released `main` baseline.

## Acceptance criteria

1. The release MR is release-only and receives exact-head Gate 2 approval.
2. All current version surfaces and built binary self-reports agree on 2.4.9.
3. The immutable tag points to the exact release-MR merge commit on `main`.
4. The tag pipeline and anonymous downloaded-asset validation pass.
5. Published framework archives contain the issue #93 correction and pass the
   live-surface positive and negative regression checks.
6. Issue #93 closes only after the release reaches Level 5 Deployable.
7. No v2.4.10 implementation enters the v2.4.9 tag.

## Stop conditions

Stop and return to the governor if the release requires a product or CI change,
a dependency/checksum change, a new command or alias, weakening a release
check, changing an existing tag, including v2.4.10 work, closing issue #93
before public verification, or publishing a platform claim unsupported by the
actual assets.
