---
id: elixir-ci-cd-adopter-packaging
status: approved
gate1_approved_by: governor-reviewer
gate1_approved_at: "2026-06-16"
topic: elixir-ci-cd-adopter-packaging
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: CI/CD will run with repository credentials and publish externally consumed artifacts, so token exposure, forbidden paths, protected refs, and artifact contents must be explicit
  - role: contribution-auditor
    status: consulted
    reason: packaging changes affect downstream adopter trust, release evidence, review workflow, and claims about what AGF ships
  - role: fact-checker
    status: consulted
    reason: docs and MR descriptions must distinguish implemented CI behavior from planned binary bundles, package registry publishing, signing, and installer scripts
---

# Plan: Elixir CI/CD for adopter packaging

## Intent

Start the real GitLab CI/CD path for AGF-NG by wiring the Elixir tooling into
merge-request validation and manifest-driven framework packaging.

The first deliverable is a reproducible source framework package for adopters
and downstreamers: CI validates the Elixir project, validates the AGF release
manifest, builds the framework archives, and exposes them as review/release
artifacts. This is the practical foundation before binary tooling bundles or
package-registry publishing.

## Context

Current state on `main`:

- No root `.gitlab-ci.yml` is present.
- `tooling/elixir/` is the active `agov` implementation.
- `mix agov.dist.validate` and `mix agov.dist.build` already exist.
- `agf/dist/release-manifest.yaml` is the source of truth for adopter-facing
  framework archive contents.
- `docs/plans/release-pages-artifacts-policy.md` is already
  `gate1-approved` and defines tag-only source release archive policy, checksum
  and provenance expectations, and Pages behavior.
- Seq 275 recommends separating framework content bundles from future tooling
  bundles, and deferring binary tooling artifacts until adoption evidence
  justifies the platform matrix.

Existing implementation source:

- Branch `feat/release-pages-artifacts` already contains a root
  `.gitlab-ci.yml`, release guard implementation, checksum/provenance support,
  tests, and release artifact documentation updates.
- This CI/CD slice should reuse that branch as the starting implementation
  rather than recreate the pipeline from scratch.
- Gate 2 should reconcile that branch against current `main`, preserve the MR
  workflow decision from seq 318, and verify whether any coordination sequence
  entries on the implementation branch need renumbering or bridging before
  merge.

## Scope

Implement a root GitLab CI configuration and the smallest supporting Elixir
tooling/docs changes needed for adopter framework packaging:

1. Rebase or otherwise reconcile the existing `.gitlab-ci.yml` from
   `feat/release-pages-artifacts`, preserving its pinned Elixir `1.20.1` /
   OTP `29.0.2` image unless Gate 2 review finds a better pinned image.
2. Add merge-request and branch validation jobs:
   - fetch dependencies;
   - compile with warnings as errors;
   - run `mix test`;
   - run `mix agov.dist.validate`.
3. Add a packaging job that runs `mix agov.dist.build` and uploads generated
   framework archives as GitLab job artifacts for MR/default-branch review.
4. Add tag-only release artifact behavior aligned with
   `release-pages-artifacts-policy`:
   - release archive production runs only for tags;
   - tag/version mismatch blocks archive production before artifact upload;
   - generated archives are accompanied by SHA-256 checksums and provenance
     metadata.
5. Keep release archive contents manifest-driven by
   `agf/dist/release-manifest.yaml`; CI must not maintain a second file
   allowlist.
6. Update docs to describe the implemented CI/CD path, including how adopters
   and downstreamers consume source framework archives.
7. Add focused tests for any new release guard, checksum, and provenance
   generation behavior.

## Out of Scope

- Per-platform binary tooling bundles.
- Burrito, Mix release, escript, or archive publishing beyond the existing
  repo-local Elixir CLI support.
- Package registry publishing.
- Installer scripts.
- Release signing or Sigstore integration.
- SBOM generation.
- Third-party notices.
- Versioned historical Pages.

These are follow-on packaging and supply-chain slices. This plan creates the
CI/CD spine they can later attach to.

## CI Shape

Expected jobs:

| Job | Trigger | Purpose |
|---|---|---|
| `elixir:test` | merge requests, branches, tags | compile and run legacy Elixir tests |
| `agf:dist:validate` | merge requests, branches, tags | validate release manifest and archive policy |
| `agf:dist:preview` | merge requests and default branch | build source archives as non-release review artifacts |
| `agf:dist:release` | tags only | build release archives, checksums, and provenance |

Implementation may combine jobs if that keeps the first pipeline simpler, but
the release-vs-preview distinction must remain explicit in job names, rules, or
artifact naming.

## Artifact Policy

Preview artifacts are evidence for reviewers. They must not be described as
immutable releases.

Release artifacts are produced only from tags and must include:

- `agf-<version>.tar.gz`
- `agf-<version>.zip`
- checksum manifest or per-file `.sha256` files
- provenance JSON containing at least:
  - AGF version;
  - source commit;
  - tag;
  - build timestamp;
  - GitLab project/pipeline/job identity when available;
  - release manifest path;
  - archive filenames and checksums.

## Safety Requirements

- CI must not echo secrets or read `.env`.
- Archive generation must continue forbidding `.env`, `.env.*`, `.git/`,
  `.worktrees/`, `tmp/`, `dist/`, `public/`, `agents-govern/`, dependency
  directories, and generated site output.
- Release jobs must not run on arbitrary branches.
- A failed version/tag guard must prevent archive upload.
- Pipeline docs must say what is implemented now and what is deferred.

## Acceptance Criteria

- Root `.gitlab-ci.yml` exists and runs Elixir validation for merge requests.
- `mix test` and `mix compile --warnings-as-errors` run in CI.
- `mix agov.dist.validate` runs in CI.
- MR/default-branch preview packaging produces GitLab artifacts without
  claiming release status.
- Tag pipelines produce source archives, checksums, and provenance metadata.
- Tag/version mismatch is tested and blocks release artifact production.
- Docs explain how adopters/downstreamers obtain and verify framework source
  archives.
- No binary tooling bundle, installer, registry, signing, or SBOM claims are
  introduced as implemented behavior.

## Validation

Local validation before Gate 2 handoff:

```bash
git diff --check
cd tooling/elixir
mix deps.get
mix compile --warnings-as-errors
mix test
mix agov.dist.validate
mix agov.dist.build
```

Gate-self-test:

- run a fixture or explicit test where the git tag does not match
  `agf/VERSION`;
- confirm the release path fails before archive upload/provenance publication.

## Implementation Notes

- Prefer a pinned container image or pinned setup steps over floating `latest`
  images.
- Keep generated archives under ignored `dist/`.
- If GitLab's default branch is `main`, implement CI rules against
  `$CI_DEFAULT_BRANCH` rather than hardcoding `master`.
- The implementation branch should open a GitLab MR for Gate 2 per seq 318.
