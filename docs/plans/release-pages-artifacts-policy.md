---
id: release-pages-artifacts-policy
status: pending-gate1
topic: release-pages-artifacts
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: tagged release archives and public Pages are externally consumed artifacts; provenance, checksums, forbidden-path policy, and CI secret exposure must be explicit
  - role: contribution-auditor
    status: consulted
    reason: release automation changes project workflow obligations, tag discipline, and downstream adopter trust claims
  - role: fact-checker
    status: consulted
    reason: implementation must distinguish currently shipped local tooling from planned CI behavior and avoid overclaiming release guarantees
---

# Plan: public Pages and tagged release artifacts policy

## Intent

Make AGF's public documentation and release archives follow a clear release
model:

- public Pages represent current `master` documentation;
- source release archives are built only from immutable version tags;
- release artifacts carry enough checksum and provenance evidence for adopters
  to verify what they downloaded.

The immediate trigger is the post-Gate-2 finding that AGF can build archives
locally with `mix agov.dist.build`, but the repository does not currently wire
Pages or release artifact jobs in a root CI file.

## Current state

- No root `.gitlab-ci.yml` is present on current `master`.
- `agf/docs/build/README.md` describes intended Pages jobs, with `pages`
  publishing on the default branch only.
- `agf/docs/site/reference/release-artifacts.md` documents an example
  `agf-release-artifacts` job, but no real pipeline rule enforces tag-only
  release archive production.
- `mix agov.dist.build` names archives from `agf/VERSION`; it does not enforce
  that the current git ref is a tag matching that version.
- The MkDocs release-info hook records the build commit and branch for public
  docs. The generated release-info page is intentionally excluded from source
  release archives.

## Policy

1. Public Pages are built and published from `master` only.
2. Merge requests and branches may build MkDocs as review artifacts, but must
   not publish Pages.
3. Source release archives are built only in tag pipelines.
4. A release tag must match `agf/VERSION` before archives are produced.
5. Release artifacts must include or be accompanied by:
   - `agf-<version>.tar.gz`;
   - `agf-<version>.zip`;
   - SHA-256 checksum files or a checksum manifest;
   - machine-readable provenance containing at least version, source commit,
     tag, build timestamp, runner/pipeline identity when available, manifest
     path, archive filenames, and checksums.
6. Release archive contents remain manifest-driven by
   `agf/dist/release-manifest.yaml`; CI must call AGF tooling instead of
   maintaining a second include/exclude list.

## Implementation scope

- Add or restore root CI wiring for:
  - MkDocs strict build on branches and merge requests as a review artifact;
  - Pages publishing from `master` only;
  - tag-only release archive validation and build.
- Add a tag/version guard to the release-artifact path.
- Extend release artifact tooling to emit SHA-256 checksums.
- Add a provenance artifact beside the archives.
- Update public docs so they state the actual CI behavior, not only the local
  commands and aspirational job shape.
- Add tests for tag/version validation, checksum output, and provenance shape.

## Out of scope

- Per-platform binary/tooling bundles.
- SBOM generation and third-party notices for binary bundles.
- Signed releases or Sigstore integration.
- Versioned historical Pages such as `/releases/2.0.0/`.
- Publishing to a package registry.

## Acceptance criteria

- CI publishes Pages only from `master`.
- Tag pipelines fail before archive production when the tag does not match
  `agf/VERSION`.
- Tag pipelines produce `.tar.gz`, `.zip`, checksum, and provenance artifacts.
- Release archive contents still come from `agf/dist/release-manifest.yaml`.
- Docs explain the difference between current public Pages and immutable tagged
  release downloads.
- Tests cover the new release guard and generated artifact metadata.

## Validation

- `mix test`
- `mix compile --warnings-as-errors`
- `mix agov.dist.validate`
- `mix agov.dist.build`
- MkDocs strict build
- A deliberate local or fixture-based negative test showing tag/version
  mismatch blocks release artifact production
