---
id: agov-oci-image
status: pending-gate1
topic: agov-oci-image
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: waived
    reason: not consulted in this single-agent Gate 1 planning session. Container image publishing is directly a security-hardening-rule.md supply-chain concern (image tag pinning, base image pinning, vulnerability scanning) — flag for a required infosec-sentinel pass at Gate 2, not skip.
  - role: contribution-auditor
    status: waived
    reason: single-agent Gate 1 planning session; no contribution-auditor available to consult. Flag for Gate 2 review.
---

# Plan: Official pinned agov OCI image for CI (GitLab #18)

**Date:** 2026-07-20T07:47:31Z
**Status:** PENDING GATE 1 APPROVAL
**Depends on:** nothing in this cluster. Independent of #33/#32/#36/#37 —
pure CI-distribution/packaging concern layered on top of the existing
`agov` binary release artifacts. Can be planned, reviewed, and implemented
in parallel with the other three plans in this cluster.

---

## Intent

Publish a small, official, version-tagged OCI (container) image bundling
the `agov` binary, so downstream adopters running `agov` in GitLab CI (or
any container-based CI) can `FROM` or pull a pinned image instead of
downloading, extracting, and installing a release archive by hand in every
adopter pipeline.

This is a **build-engineer-relevant** change: it adds a new CI packaging
stage and a new artifact type. Per this repo's own
`.claude/rules/workflow/ci-trigger-scoping-rule.md` companion note, this
plan explicitly recommends build-engineer be consulted at implementation
time even though the current Gate 1 plan-validator rejects `build-engineer`
as an `advisory_consultation` entry (known validator bug, issue #30,
unrelated to this cluster but affecting how this plan's frontmatter is
written — `build-engineer` is intentionally omitted from the array above
and `contribution-auditor` used instead, per this cluster's task
instructions).

## Problem (grounded in the actual `.gitlab-ci.yml` and adopter-integration docs)

Reading `.gitlab-ci.yml`'s `rust:tooling:*` jobs top to bottom: this
project already builds and releases `agov` as **archives**, not container
images:

- `rust:tooling:preview` / `rust:tooling:release` (via the
  `.rust-tooling-package` template) cross-compile `agov` for
  `x86_64-unknown-linux-musl`, `aarch64-unknown-linux-musl`, and (on tag
  pipelines) `x86_64-pc-windows-gnu`, producing `.tar.gz`/`.zip` archives
  with sha256 checksums and a provenance JSON per target.
- `release:create` uploads those archives to the GitLab generic package
  registry and creates a GitLab release with asset links.
- There is **no** container image build, no `Dockerfile`, and no
  `$CI_REGISTRY_IMAGE` push anywhere in the pipeline today. The only
  container usage in CI is `docker:dind` as a *build service* (for `cross`
  cross-compilation and the InfoSec Trivy scan image) — not for publishing
  `agov` itself as an image.

The learning behind issue #18 is exactly this gap: a downstream adopter
wiring `agov` into their own GitLab CI must, in every adopter repository,
either (a) install `cargo` and build from source, or (b) hand-roll a
download-extract-checksum-verify-platform-select script against the
generic-package-registry archives this project already produces. Both are
boilerplate an official image removes.

## Scope

**In scope:**
- A `Dockerfile` building a minimal image from the existing
  `x86_64-unknown-linux-musl` (and, if multi-arch is feasible within CI
  budget, `aarch64-unknown-linux-musl`) release binary already produced by
  `.rust-tooling-package` — **not** a from-source multi-stage build inside
  the image; reuse the artifact the existing job already produces so the
  image is bit-for-bit the same binary an archive-based adopter would get,
  with the same provenance.
- A new CI stage, gated on tag pipelines only (parallel to
  `rust:tooling:release`), that builds and pushes the image to this
  project's own container registry (`$CI_REGISTRY_IMAGE`) with an
  **exact-version tag** (e.g. `agov:2.2.5`), matching `agf/VERSION` exactly
  as `rust:tooling:package`'s existing tag-vs-VERSION guard already does
  for archives.
- Immutable, digest-pinnable images: once pushed, a version tag's digest is
  never overwritten — this is the container-image analogue of
  `immutable-tags-rule.md`, applied to image tags instead of git tags, and
  should be stated as such in the image's own documentation.
- Optional convenience tags (e.g. a `latest`-equivalent floating tag) are
  explicitly **non-blocking-CI-recommended only** — documented as "useful
  for local exploration," never used in adopter CI examples, per
  `security-hardening-rule.md`'s "no `:latest` or floating tags in compose
  files or CI" and per this issue's own text ("the image simplifies
  installation, it doesn't license floating versions").
- Multi-arch, where feasible within the existing `cross`/`docker:dind`
  CI budget already used by `.rust-tooling-package` — `linux/amd64` at
  minimum (from the existing `x86_64-unknown-linux-musl` build), `linux/arm64`
  as a stretch goal reusing the existing `aarch64-unknown-linux-musl`
  build if the multi-arch manifest push does not meaningfully extend
  pipeline duration on tag pipelines (tag pipelines are not
  quick-feedback-threshold-bound per `ci-trigger-scoping-rule.md` — they
  are "always run, advisory" per that rule's Trigger contexts section —
  so a few extra minutes on a release pipeline is acceptable in a way it
  would not be on every MR).
- OCI labels: `org.opencontainers.image.source` (this project's repo URL),
  `org.opencontainers.image.version` (matching `agf/VERSION`),
  `org.opencontainers.image.revision` (`$CI_COMMIT_SHA`),
  `org.opencontainers.image.licenses` (matching the project's dual license
  string already used in Rust crate manifests).
- Downstream documentation examples still pin an **exact version or
  digest** — the image is a packaging convenience, never a license to
  float. State this explicitly in the adopter-facing docs per the issue
  text.

**Out of scope:**
- Changing anything about the existing archive-based release path
  (`rust:tooling:release`, `release:create`). The OCI image is additive.
- Building the image on every merge request or every branch push — this
  would violate `ci-trigger-scoping-rule.md`'s spirit for a stage that
  involves a registry push; image builds happen on tag pipelines only,
  matching `rust:tooling:release`'s existing `rules: - if: $CI_COMMIT_TAG`.
  A preview/non-pushed image build on MRs touching the `Dockerfile` itself
  is a reasonable follow-up (build-without-push, to catch Dockerfile
  breakage early) but is not required for the initial deliverable.
- Any change to how `agov-mcp` (see `agov-mcp-server.md`, same cluster) is
  packaged — if that plan proceeds, its binary can be added to this same
  image or a sibling image in a follow-up; not scoped here to keep this
  plan's yes/no question single ("does the pinned `agov` image ship
  correctly") per branch-and-worktree-rule's sizing test.

## The removability constraint, applied to this plan

Per README.md's "Git history is a ledger, not a scratchpad" section and
the standing design principle restated in this cluster's task brief: *AGF
must be removable from a repository at any time without loss of project
history or working capability... `agov`, `agov-mcp`, and any future
component are conveniences over those files, never their owners.*

This plan's OCI image is a **packaging convenience layered on top of** an
already-independently-valid distribution mechanism (the archives
`rust:tooling:release` already produces, and a full source checkout that
builds `agov` with plain `cargo build`). Concretely:

- The image contains nothing that is not already true of a source
  checkout: the same binary, built the same way, from the same tagged
  commit.
- An adopter who never pulls the image and instead does a full source
  checkout (`git clone` + `cargo build -p agov --release`) gets the exact
  same tool with the exact same capability — the image is strictly an
  installation shortcut, not a dependency any workflow requires.
- No adopter-facing documentation in this plan may describe the image as
  required; every example must show the archive-based and source-checkout
  alternatives alongside it, or link to where they are already documented
  (`agf/docs/site/docs/adopt/install-and-layouts.md`).

## Affected files

| File | Change |
|---|---|
| `tooling/rust/Dockerfile` (new) | Minimal image: copy the pre-built musl binary from the release archive stage (or rebuild via a pinned base image — see Implementation notes), set entrypoint, non-root user per `security-hardening-rule.md`'s container hygiene section |
| `.gitlab-ci.yml` | New job, e.g. `agov:oci:release`, extending or sitting alongside `.rust-tooling-package` / `rust:tooling:release`; gated `rules: - if: $CI_COMMIT_TAG`; pushes to `$CI_REGISTRY_IMAGE/agov:$VERSION` (and, if multi-arch ships, a manifest list) |
| `agf/docs/site/docs/adopt/install-and-layouts.md` | Add an OCI-image installation option alongside the existing archive/source-build options, explicit exact-version-pin guidance |
| `README.md` | One line in the tooling section: pinned image reference example (`docker pull registry.gitlab.com/.../agov:2.2.5`), pointing to the docs page for detail, per marketing-tone-rule message-integrity (state once, link elsewhere) |
| `CHANGELOG.md` (or `agf/CHANGELOG.md`, whichever this project's release-via-merge-request-rule workflow uses for this version bump) | Entry for the new distribution channel, added during the release MR that first ships it |

## Migration needed

No schema change, no code change to `agov` itself. Purely a new CI stage
and a new documented installation path. No existing adopter is affected
until they opt into the image.

## Implementation notes

- **Do not multi-stage-build from source inside the Dockerfile** using a
  `rust:1` base and `cargo build` at image-build time. That would produce
  a binary built by a *different* toolchain invocation than the one
  `rust:tooling:release` already builds, verifies, and checksums — two
  build paths for the "same" binary is exactly the kind of drift this
  project's provenance JSON (`agov-${VERSION}-${target}-provenance.json`)
  exists to prevent. Instead, the OCI job should **depend on** (GitLab CI
  `needs:`) the existing `rust:tooling:release` job and `COPY` its
  produced `x86_64-unknown-linux-musl` binary into a minimal runtime base
  (e.g. `scratch` or `alpine`, pinned to a specific tag/digest per
  `security-hardening-rule.md`).
- Base image tag must be pinned to a specific version or digest (not
  `alpine:latest`), matching this project's own
  `always/security-hardening-rule.md` container-image-hygiene section
  verbatim.
- Run as non-root inside the image (same rule, same section) — `agov` has
  no privileged operation and no reason to run as root.
- Image vulnerability scanning: this project already runs Trivy in
  `dependency:cve-scan` against filesystem dependency manifests. Extend
  (or add a sibling job) to run Trivy's **image** scanner
  (`trivy image`) against the built OCI image before push, gated the same
  way (tag pipelines), addressing HIGH/CRITICAL findings before release
  per `security-hardening-rule.md`'s "Image scanning" bullet.
- Registry choice: default to this project's own GitLab Container
  Registry (`$CI_REGISTRY_IMAGE`), which requires no new external account
  or secret — `$CI_JOB_TOKEN`-based auth already works for the existing
  generic-package-registry uploads in `release:create`. A public registry
  mirror (Docker Hub, GHCR) is a possible follow-up but adds a new secret
  and a new trust surface not required for the initial deliverable.

## Test strategy

Per `visible-artifact-verification-rule.md`'s spirit (this produces an
artifact a human/CI will pull and run, even though it is not a rendered
visual) and `gate-self-test-rule.md` (this plan introduces a new gate —
the tag-version-matches-image-tag check):

| Tier | What | Where |
|---|---|---|
| Negative test (required by gate-self-test-rule.md, since the image-tag-matches-VERSION check is a new gate) | Feed the OCI job's tag-check logic a deliberately mismatched `CI_COMMIT_TAG` vs. `agf/VERSION` (mirroring `.rust-tooling-package`'s existing `TAG_VERSION != VERSION` guard, lines ~443-449 of the current `.gitlab-ci.yml`) and confirm the job fails before any image is pushed | New job's script, documented in the introducing MR's description with a transcript, per `gate-self-test-rule.md` |
| Manual/CI | `docker run --rm <registry>/agov:<version> --version` prints the expected version string and matches the archive build's `agov --version` output for the same tag | Verified once per release in the introducing MR, then implicitly re-verified every release by the tag-check gate above |
| Manual | `docker run --rm <registry>/agov:<version> validate` (or another read-only subcommand) executes successfully against a mounted repo checkout, proving the entrypoint and non-root permissions work end-to-end | Documented in `install-and-layouts.md` as the adopter-facing smoke test |
| CI | Trivy image scan reports zero HIGH/CRITICAL findings with available fixes before the image is pushed | New or extended CI job per Implementation notes |

## Risks

| Risk | Mitigation |
|---|---|
| Image and archive binaries silently diverge (built differently, checksums don't match) | `needs:` dependency on the existing `rust:tooling:release` job artifact; `COPY` the already-built, already-checksummed binary rather than rebuilding |
| Adopters treat the image as licensing floating-version use | Explicit documentation requirement (Scope section) — every example pins exact version or digest |
| New registry push adds a new secret/trust surface | Use `$CI_REGISTRY_IMAGE` + `$CI_JOB_TOKEN`, already trusted by this pipeline for the existing generic-package-registry uploads; no new external secret |
| Multi-arch manifest push meaningfully lengthens the release pipeline | Tag pipelines are explicitly "always run, advisory" per `ci-trigger-scoping-rule.md`'s Trigger-contexts section — acceptable; `arm64` support can also ship in a fast-follow release if the first cut is `amd64`-only |
| Base image later develops a known CVE after release (images are immutable once tagged, per `immutable-tags-rule.md`'s analogue) | Same remedy as the git-tag case: bump to a new patch version, rebuild, push a new tag; the old tag stays in the registry as historical record, exactly as a broken git tag stays in history |

## Rollback

The new CI job and Dockerfile are additive. Reverting removes the job and
file; the existing archive-based release path (`rust:tooling:release`,
`release:create`) is completely untouched and continues to work
identically. Previously-published image tags are not deleted (per the
immutable-tags-rule analogue above) — they simply stop being updated.

## Assumption flagged for the identity/accountability cluster

None. This plan has no dependency on coordination-channel sender identity,
role, or accountability semantics — it is a pure CI-packaging/supply-chain
concern with no coordination-channel interaction.
