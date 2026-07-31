---
id: v2410-release-publication
status: pending-gate1
topic: v2410-release-publication
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: "Renewed implementation review 20260731T171313Z-contribution-auditor-pqrs verified issue #92, and release-plan review 20260731T175742Z-contribution-auditor-p7rj returned CLEAR with three advisories. This revision incorporates the plan-self-update, structural closes_issues API check, and tag-checkout wording clarifications; contract-stability documentation remains a separate follow-up."
  - role: infosec-sentinel
    status: not-applicable
    reason: "The release consumes the reviewed issue #92 implementation and changes version, changelog, and release-truth metadata only. It adds no credential handling, dependency, network trust boundary, learning payload, authorization semantic, or security-control implementation; existing provenance and artifact checks remain mandatory."
---

# Plan: publish v2.4.10 repository-local agov reliability

## Decision requested

Authorize the patch release that makes the Gate 2-approved issue #92 fix
deployable. MR !869 was approved at exact head
`a8384b1aac0260ce39418c21ceb3b7ffb7bdb7c1`, with passed pipeline
`2722350441` and renewed independent CLEAR review
`20260731T171313Z-contribution-auditor-pqrs`. It merged to `main` as
`e6f960128528795504bce5c7f753c8de185612f8`; the merge commit's second
parent is the exact approved head and the source branch was retained.

The requested version is **2.4.10 (patch)**. The implementation restores the
documented repository-local operator contract without changing adopter-facing
command semantics. This plan authorizes only release metadata, validation, the
release MR, one immutable annotated tag after the release MR merges,
publication verification, and issue #92 closure after verification.

At planning time there is no `v2.4.10` tag, GitLab Release, or generic package.
Issue #92 remains open and assigned to the v2.4.10 milestone.

## Release-only change set

Create the release commit on `release-v2.4.10`, based on the then-current
`main`, with only these release-truth changes:

1. After Gate 1 approval, update this plan's frontmatter with the exact
   approval identity and UTC timestamp before release implementation is
   presented for Gate 2.
2. Set `agf/VERSION` to `2.4.10`.
3. Add dated v2.4.10 entries to `CHANGELOG.md` and `agf/CHANGELOG.md` that:
   - identify the release as the issue #92 repository-local `agov`
     reliability correction;
   - record transparent forwarding to the authoritative Rust parser;
   - record deterministic source fingerprinting for behavior-affecting
     tooling inputs;
   - record compatible-binary selection, one-shot verified release rebuild,
     and fail-closed recovery;
   - record actionable cross-worktree mismatch diagnostics;
   - note that standalone published binaries remain unaffected by the
     repository-wrapper policy;
   - report the final release-branch test count and state `Adopter docs:
     updated`; and
   - reference issue #92 without auto-closing it before publication.
4. Update the five Rust workspace package versions, three internal path
   requirements, and the corresponding five `Cargo.lock` package entries from
   2.4.9 to 2.4.10. No dependency or checksum may change.
5. Update current framework-version surfaces only: adopter-facing agent
   definitions, `agf/agents/MANIFEST.yaml`, `agf/ADOPTING.md`,
   `agf/templates/AGENTS.md`, `agf/templates/learning-record.yaml`, and current
   version statements/examples in `agf/docs/governance/manifest.md`.
6. Update `agf/docs/site/roadmap.md` to identify v2.4.10 as the latest release
   and mark issue #92 deployable only after publication verification. Preserve
   historical release evidence and later batch commitments.
7. Leave the README major/minor citation unchanged because 2.4 remains the
   current release line. Leave historical coordination, plans, changelog
   entries, and valid earlier-or-later compatibility statements unchanged.

No product implementation, command semantic, schema, validator semantic,
dependency, CI behavior, issue #97 wording, mdBook/Ayu work, or branch deletion
is in scope.

## Proposed changelog substance

The two changelog entries may follow their existing file-specific formatting,
but must express this same release truth:

> v2.4.10 makes the repository-local `tooling/agov` launcher source-compatible
> with its checkout. It forwards the full Rust command tree, verifies a
> deterministic fingerprint of behavior-affecting tooling inputs, accepts only
> compatible local binaries, performs at most one verified release rebuild,
> fails closed when recovery is unavailable, and diagnoses cross-worktree
> invocation with the correct wrapper path. Published standalone binaries keep
> their existing behavior. Adopter docs: updated. Issue #92 closes only after
> public artifact verification.

The implementation MR's locally reproduced count is 299 passed, 0 failed, and
3 ignored. The release changelogs must use the count from the final release
branch run rather than assuming this count remains unchanged.

## Validation before Gate 2

The release MR must provide exact-head evidence for:

- a focused version audit showing every current release-truth surface is
  2.4.10 and historical evidence was not rewritten;
- Cargo metadata showing exactly five workspace packages at 2.4.10 and the
  three internal requirements at `^2.4.10`;
- `cargo test --manifest-path tooling/rust/Cargo.toml`, with passed and ignored
  counts recorded in changelog and MR evidence;
- strict workspace Clippy with warnings denied;
- the wrapper freshness regression matrix, including staged, unstaged,
  untracked, executable-mode, rebuild, parser-forwarding, and cross-worktree
  cases;
- `agov validate`, including adopter-docs validation;
- `agov dist validate` and clean tar/zip builds;
- all shipped archive self-sufficiency tests for both archive formats;
- `tooling/rust/scripts/check-live-tooling-surfaces.sh check` and `selftest`;
- built `agov` and `agov-mcp` self-reporting 2.4.10, the release MR exact head,
  and clean source;
- tag/version guard evidence with `CI_COMMIT_TAG=v2.4.10`;
- a GitLab API check immediately before merge showing the release MR's
  `closes_issues` response is empty, rather than relying on visible description
  text to prevent premature issue #92 closure;
- release-path, provenance, artifact-exclusion, SPDX/license, strict MkDocs,
  links, and `git diff --check` checks; and
- a green exact-head GitLab MR pipeline.

The release MR must target `main`, contain only the governed release-only
change set, retain its source branch, and receive exact-head Gate 2 approval.

## Merge, tag, and publication sequence

1. Merge only the exact Gate 2-approved release MR head and retain its source
   branch.
2. Confirm `origin/main` resolves to the resulting merge commit and that no
   `v2.4.10` tag exists locally or remotely.
3. Create one annotated `v2.4.10` tag on that exact `main` merge commit and
   push it immediately. Never move, delete, or recreate the tag.
4. Require the tag pipeline to pass, including framework archives, Linux Rust
   tooling archives, provenance/checksum validation, OCI publication, and
   GitLab Release creation. Report optional Windows artifacts only when they
   are present in the actual Release inventory.
5. Run the released Rust `agov dist post-publication-check` against the
   anonymous GitLab Release API and a fresh download directory. Verify all
   downloaded bytes and provenance, executable self-reporting, source archive
   contents, and real OCI evidence.
6. Verify the GitLab Release, generic package `agf/v2.4.10`, and OCI tag
   `agov:2.4.10` correspond to the immutable tag pipeline.
7. From a fresh git clone of the repository checked out at the exact immutable
   `v2.4.10` tag—not from the framework release archive, which intentionally
   excludes `tooling/`—demonstrate that `tooling/agov` reaches `coord publication-check` and `dist
   self-report-check`, detects source incompatibility, and follows the
   documented one-shot rebuild/fail-closed contract without legacy fallback.
8. Only after all public checks pass, close issue #92 with links to the tag,
   pipeline, Release, and durable completion record. Close the v2.4.10
   milestone after its issue inventory is verified complete.
9. Publish and merge a validated completion coordination record before the
   next release batch treats v2.4.10 as its baseline.

## Acceptance criteria

1. The release MR is release-only and receives exact-head Gate 2 approval.
2. All current version surfaces and built binary self-reports agree on 2.4.10.
3. The immutable tag points to the exact release-MR merge commit on `main`.
4. The tag pipeline and anonymous downloaded-asset validation pass.
5. Published source and tooling artifacts demonstrate the issue #92 wrapper
   contract through supported Rust paths.
6. Issue #92 closes only after the release reaches Level 5 Deployable.
7. Issue #97 and the separate public-site migration do not enter v2.4.10.

## Stop conditions

Stop and return to the governor if release preparation requires a product or
CI change, a dependency/checksum change, a command-semantic change, weakening
a release or fingerprint check, changing an existing tag, including issue #97
or mdBook/Ayu work, deleting a source branch, closing issue #92 before public
verification, or publishing a platform claim unsupported by actual assets.
Also stop if the release MR has any structural `closes_issues` association
before publication verification, even when its visible description contains
no closing keyword.
