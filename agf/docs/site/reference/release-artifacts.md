# Release artifacts

AGF source release archives are built from
`agf/dist/release-manifest.yaml`. The manifest is the release policy; GitLab CI
calls the AGF tooling instead of maintaining a separate artifact include/exclude
list.

There are two release surfaces:

| Surface | Published artifact | Build tooling (current) |
|---|---|---|
| Framework source distribution | `agf-X.Y.Z.tar.gz`, `agf-X.Y.Z.zip` | Manifest-driven packaging implemented by the Rust `agov` CLI |
| `agov` adopter tooling | `agov-X.Y.Z-<target>.tar.gz`, `agov-X.Y.Z-<target>.zip` | Rust binary archives built per target platform |

The framework source archives and adopter tooling archives are now both built
through the Rust tooling surface. Adopters downloading release tooling should use the Rust
`agov` archives on the [project GitLab Releases page](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases).

```bash
agov dist validate
agov dist build
```

`agov dist validate` expands the manifest and fails if selected files escape
the repository, match forbidden paths, or would collide in the archive.
`agov dist build` writes the configured `.tar.gz` and `.zip` archives under
`dist/`, along with SHA-256 checksum files and a machine-readable provenance
record. In merge-request and default-branch pipelines, these are preview
artifacts for review evidence. They are not immutable releases.

The framework source archive is the canonical versioned framework snapshot.
The Rust tooling archives are the convenient execution surface for running
`agov` without building it locally.

The current archive policy is allowlist-first:

- include the adopter-facing `agf/` framework, docs, schemas, rules, skills,
  and release manifest;
- exclude generated site output and generated release-info pages;
- forbid local or bootstrap paths such as `.worktrees/`, `tmp/`, `dist/`,
  `agents-govern/`, Elixir build directories, and `.env` files.

Downstream projects may fork the manifest or pass `--manifest` to use their own
release policy.

## Artifacts produced by a release build

For version `X.Y.Z`, the release artifact vocabulary is below. Framework,
Linux tooling, and OCI rows are mandatory for current tag releases; Windows
rows describe optional manual-preview output and appear on a Release page only
if that release deliberately publishes them.

The immutable v2.4.7 tag is a partial-publication exception: its OCI image was
published before a downstream release-creation check failed on the absent
optional Windows ZIP, so it has no generic package or GitLab Release. Use
v2.4.8 or later for a complete release surface.

| File | Content |
|---|---|
| `dist/agf-X.Y.Z.tar.gz` | Source archive |
| `dist/agf-X.Y.Z.zip` | Source archive |
| `dist/agf-X.Y.Z.tar.gz.sha256` | SHA-256 checksum (GNU coreutils format, `<hex>  <filename>`) |
| `dist/agf-X.Y.Z.zip.sha256` | SHA-256 checksum (GNU coreutils format, `<hex>  <filename>`) |
| `dist/agf-X.Y.Z.tar.gz-provenance.json` | Machine-readable build provenance for this archive |
| `dist/agf-X.Y.Z.zip-provenance.json` | Machine-readable build provenance for this archive |
| `dist/agov-X.Y.Z-linux-x86_64.tar.gz` | Mandatory Rust `agov` tooling archive for Linux x86_64 |
| `dist/agov-X.Y.Z-linux-x86_64.tar.gz.sha256` | Tooling archive SHA-256 checksum |
| `dist/agov-X.Y.Z-linux-x86_64.tar.gz-provenance.json` | Tooling archive provenance |
| `dist/agov-X.Y.Z-windows-x86_64.zip` | Optional manual-preview Rust `agov` tooling archive for Windows x64; not a mandatory tag-release asset |
| `dist/agov-X.Y.Z-windows-x86_64.zip.sha256` | Optional Windows preview checksum |
| `dist/agov-X.Y.Z-windows-x86_64.zip-provenance.json` | Optional Windows preview provenance |
| `dist/agov-oci-X.Y.Z-provenance.json` | Provenance for the published `agov` container image |

Tooling archive filenames use a short public platform slug
(`linux-x86_64`, `windows-x86_64`). The provenance record's
`target_platform` field keeps the full Rust target triple
(`x86_64-unknown-linux-musl`, `x86_64-pc-windows-gnu`) — the filename
identifies the platform for a person downloading; the record identifies
the exact build target for a verifier.

Linux arm64 (`aarch64-unknown-linux-musl`, published as `linux-aarch64`
on releases v2.4.7 through v2.5.0-rc.2, and on `v2.5.0` itself as the
exact metadata-only promotion of that same RC2 candidate under
temporary provisioned runner capacity) is not currently built for
releases derived from current `main`. The cross-compilation step
requires more disk than the project's current shared CI runner tier
provides; it will resume once CI runner capacity for this target is
sponsored. Adopters needing arm64 Linux tooling in the meantime can
build `agov` and `agov-mcp` from source for
`aarch64-unknown-linux-musl` (or `aarch64-unknown-linux-gnu`).

### The unified release-provenance contract

Every provenance record above — framework archive, tooling archive, and
container image — is one instance of a single JSON Schema
(`schema_version: "2.4"`): `agf/schemas/release-provenance.schema.json`.
Earlier releases used three different, undeclared shapes (an aggregate
framework record covering both archive formats at once, a tooling record
with no embedded filename or checksum, and container-image metadata
carried only in OCI labels with no
JSON record at all). The schema replaces all three with one closed,
discriminated union keyed by `artifact_kind`, and the schema itself is
embedded into the `agov` binary at compile time, so a downloaded release
binary can validate provenance with no source checkout.

**Cardinality: exactly one record per published artifact.** A version that
publishes two framework archive formats publishes two provenance records,
not one aggregate record — this is why the framework archive filenames
above changed from `agf-X.Y.Z-provenance.json` to
`agf-X.Y.Z.tar.gz-provenance.json` / `agf-X.Y.Z.zip-provenance.json`.

Every record shares a common core: `schema_version` (`"2.4"`), `product`,
`artifact_kind`, `version`, `source_commit`, `tag`, `build_timestamp`,
`implementation`, `packaging`, and an optional `pipeline_identity`
(`pipeline_id`, `job_id`).

- **`artifact_kind: "file"`** (framework and tooling archives) adds
  `artifact_filename`, `artifact_bytes`, and `sha256`, plus
  `target_platform` for tooling binaries.
- **`artifact_kind: "oci-image"`** (the published container image) adds
  `image_reference`, `manifest_digest`, and a `labels` object
  (`version`, `revision`) that must agree with the record's own `version`
  and `source_commit` — a validator rule, not just a schema shape.

Every object in the schema sets `"additionalProperties": false`, including
inside each `artifact_kind` variant, so an unrecognized key or a field from
the wrong variant fails validation rather than silently passing through.

**Pre-v2.4 records are never upgraded implicitly.** A record with no
`schema_version` field is classified `assurance_level: legacy` and is never
reported as satisfying the v2.4 contract, even if every field it happens to
have would individually pass.

```bash
# Validate one or more provenance records against the schema (or classify
# them as legacy). --artifact-dir also cross-checks artifact_bytes/sha256/
# checksum-sidecar agreement, rejects dangling references, and enforces
# cardinality (every artifact exactly one record, no stale/extra records).
# --oci-image inspects a real image via `podman inspect` for label/digest
# evidence. --format json prints a stable machine-readable array instead
# of text.
agov dist validate-provenance dist/*-provenance.json --artifact-dir dist

# Anonymously discover and download every asset linked from a real
# published GitLab Release, then run the composed public-lifecycle check
# against the downloaded bytes -- not a CI workspace copy. This is more
# than validate-provenance: it also pulls the published OCI image by its
# immutable digest from the real registry and cross-checks real evidence
# (no --oci-image flag needed -- the record's own image_reference and
# manifest_digest are used), and extracts + executes the packaged agov
# binary from every downloaded tooling archive whose target_platform this
# host can run, cross-checking its own `version --format json` self-report.
# A cross-target archive (e.g. aarch64 or Windows, on an x86_64 Linux
# verifier) is not silently skipped -- an explicit SKIP line names the gap.
agov dist post-publication-check \
  --releases-url "https://gitlab.com/api/v4/projects/<id>/releases/v2.4.0" \
  --download-dir /tmp/release-check

# The self-report extraction/execution/comparison primitive that
# post-publication-check runs automatically for each eligible downloaded
# archive; also available standalone for a single already-local archive.
agov dist self-report-check \
  --archive dist/agov-2.4.0-linux-x86_64.tar.gz \
  --binary-name agov \
  --record dist/agov-2.4.0-linux-x86_64.tar.gz-provenance.json

# Build a file-kind record from an archive already on disk.
agov dist provenance-record file \
  --product agf --version 2.4.0 --source-commit "$CI_COMMIT_SHA" \
  --tag v2.4.0 --build-timestamp "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --implementation rust --packaging manifest-source-archive \
  --artifact dist/agf-2.4.0.tar.gz \
  --output dist/agf-2.4.0.tar.gz-provenance.json

# Build an oci-image record from a pushed image's digest.
agov dist provenance-record oci \
  --product agov --version 2.4.0 --source-commit "$CI_COMMIT_SHA" \
  --tag v2.4.0 --build-timestamp "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --implementation rust --packaging oci-image \
  --image-reference registry.example.com/agov:2.4.0 \
  --manifest-digest sha256:<digest> \
  --label-version 2.4.0 --label-revision "$CI_COMMIT_SHA" \
  --output dist/agov-oci-2.4.0-provenance.json
```

## Tag/version guard

The tag/version guard is implemented as `agov dist check-tag`. It reads
`CI_COMMIT_TAG` from the environment and compares it to `agf/VERSION`. The
comparison strips an optional `v` prefix from the tag.

```bash
# exits 0: tag matches VERSION
CI_COMMIT_TAG=v2.0.0 agov dist check-tag

# exits 1: mismatch — no archive is written
CI_COMMIT_TAG=v9.99.99 agov dist check-tag
```

If `CI_COMMIT_TAG` is not set the guard exits 0 (local build, guard skipped).

## GitLab CI wiring

The root `.gitlab-ci.yml` wires validation, preview packaging, and tag-only
release packaging:

```yaml
# MkDocs strict build — review artifact on MRs and branches
site-html-build:
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
    - if: $CI_COMMIT_BRANCH && $CI_COMMIT_BRANCH != $CI_DEFAULT_BRANCH

# GitLab Pages — published from the default branch
pages:
  rules:
    - if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH

# Manifest validation for source archives
agf:dist:validate:
  script:
    - cargo run --quiet -p agov -- dist validate

# Preview archives — MRs and default branch only, not releases
agf:dist:preview:
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
    - if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH

# Release archives — tag pipelines only, tag/VERSION guard runs first
agf:dist:release:
  rules:
    - if: $CI_COMMIT_TAG

# Mandatory Linux Rust tooling archives on tag pipelines; Windows packaging is an optional manual preview
rust:tooling:release:
  rules:
    - if: $CI_COMMIT_TAG
```

The `agf:dist:validate` job validates the framework source-archive manifest
without writing archives. The `agf:dist:preview` job runs
`agov dist build` for merge-request and default-branch review artifacts; those
artifacts expire and must not be described as releases.

The `agf:dist:release` job runs the tag/version guard before validating and
building source archives. That sequence is `agov dist check-tag`,
`agov dist validate`, and `agov dist build`. If the tag does not match
`agf/VERSION`, the job exits 1 before any archive is written.

Artifacts produced by `agf:dist:release` and `rust:tooling:release` are CI
handoff artifacts for the `release:create` job. They are retained briefly in CI
and then expire; the durable release copies are the package registry uploads
and GitLab Release links created by `release:create`.

Framework release assets include the `.tar.gz`, `.zip`, all `.sha256` checksum
files, and one provenance JSON record per archive. Mandatory Linux Rust tooling
targets are published as `.tar.gz`. Windows x64 packaging remains available as
optional manual preview evidence and is not built or promised by every tag;
when deliberately produced, it uses `.zip` plus checksum and provenance
sidecars. Each GitLab Release page's actual asset links are the authoritative
platform inventory. The
`agov:oci:release` job publishes the container image and, after the push
succeeds, an `agov-oci-X.Y.Z-provenance.json` record carrying the pushed
image's digest — `release:create` links it alongside the archive records.
After `release:create` publishes the release, `release:verify` runs the
composed public-lifecycle check (`agov dist post-publication-check`)
against the just-published release: real OCI-registry pull-by-digest
evidence for the container image, and real packaged-binary self-report
execution for every tooling archive this runner can execute, alongside the
schema/hash/sidecar/cardinality checks `release:create` already ran
build-time against the local `dist/` copy. `release:create`'s check is a
pre-upload gate; `release:verify` is the public-lifecycle counterpart —
it re-fetches from the GitLab Release API and downloads fresh bytes,
never reusing the pipeline's own `dist/` copy.

Installer scripts, release signing, Software Bill of Materials (SBOM)
generation, and third-party notices are deferred follow-on work.
