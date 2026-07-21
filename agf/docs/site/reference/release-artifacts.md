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
through the Rust tooling surface. GitLab CI does not publish an Elixir/ERTS
`agov` package. Adopters downloading release tooling should use the Rust
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

For version `X.Y.Z`, a full release build produces:

| File | Content |
|---|---|
| `dist/agf-X.Y.Z.tar.gz` | Source archive |
| `dist/agf-X.Y.Z.zip` | Source archive |
| `dist/agf-X.Y.Z.tar.gz.sha256` | SHA-256 checksum (BSD format) |
| `dist/agf-X.Y.Z.zip.sha256` | SHA-256 checksum (BSD format) |
| `dist/agf-X.Y.Z.tar.gz-provenance.json` | Machine-readable build provenance for this archive |
| `dist/agf-X.Y.Z.zip-provenance.json` | Machine-readable build provenance for this archive |
| `dist/agov-X.Y.Z-x86_64-unknown-linux-musl.tar.gz` | Optional Rust `agov` tooling archive for Linux x86_64 |
| `dist/agov-X.Y.Z-x86_64-unknown-linux-musl.tar.gz.sha256` | Tooling archive SHA-256 checksum |
| `dist/agov-X.Y.Z-x86_64-unknown-linux-musl.tar.gz-provenance.json` | Tooling archive provenance |
| `dist/agov-X.Y.Z-aarch64-unknown-linux-musl.tar.gz` | Optional Rust `agov` tooling archive for Linux arm64 |
| `dist/agov-X.Y.Z-aarch64-unknown-linux-musl.tar.gz.sha256` | Tooling archive SHA-256 checksum |
| `dist/agov-X.Y.Z-aarch64-unknown-linux-musl.tar.gz-provenance.json` | Tooling archive provenance |
| `dist/agov-X.Y.Z-x86_64-pc-windows-gnu.zip` | Optional Rust `agov` tooling archive for Windows x64 |
| `dist/agov-X.Y.Z-x86_64-pc-windows-gnu.zip.sha256` | Tooling archive SHA-256 checksum |
| `dist/agov-X.Y.Z-x86_64-pc-windows-gnu.zip-provenance.json` | Tooling archive provenance |
| `dist/agov-oci-X.Y.Z-provenance.json` | Provenance for the published `agov` container image |

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
# them as legacy). --artifact-dir also cross-checks artifact_bytes/sha256
# against the real files, and rejects dangling references.
agov dist validate-provenance dist/*-provenance.json --artifact-dir dist

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

# Optional Rust tooling archives, including Linux tarballs and Windows zip on tag pipelines
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
files, and one provenance JSON record per archive. Linux Rust tooling targets
are published as `.tar.gz`; Windows x64 is published as `.zip`. The
`agov:oci:release` job publishes the container image and, after the push
succeeds, an `agov-oci-X.Y.Z-provenance.json` record carrying the pushed
image's digest — `release:create` links it alongside the archive records. No
Elixir/ERTS `agov` package is published to the
[project GitLab Releases page](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases).

Installer scripts, release signing, Software Bill of Materials (SBOM)
generation, and third-party notices are deferred follow-on work.
