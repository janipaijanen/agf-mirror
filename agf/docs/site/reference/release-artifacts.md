# Release artifacts

AGF source release archives are built from
`agf/dist/release-manifest.yaml`. The manifest is the release policy; GitLab CI
calls the AGF tooling instead of maintaining a separate artifact include/exclude
list.

From `tooling/elixir/`:

```bash
mix agov.dist.validate
mix agov.dist.build
mix agov.tooling.build
mix agov.bundle.build
```

`mix agov.dist.validate` expands the manifest and fails if selected files escape
the repository, match forbidden paths, or would collide in the archive.
`mix agov.dist.build` writes the configured `.tar.gz` and `.zip` archives under
`dist/`, along with SHA-256 checksum files and a machine-readable provenance
record. In merge-request and default-branch pipelines, these are preview
artifacts for review evidence. They are not immutable releases.

`mix agov.tooling.build` builds the optional linux-x86_64 packaged `agov`
tooling archive with bundled ERTS. `mix agov.bundle.build` wraps the framework
source archives and the tooling archive into one convenience bundle. The
framework source archive remains the primary adopter artifact; the tooling
archive and bundle are optional.

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
| `dist/agf-X.Y.Z-provenance.json` | Machine-readable build provenance |
| `dist/agov-X.Y.Z-linux-x86_64-otp-<major>.tar.gz` | Optional packaged `agov` tooling archive |
| `dist/agov-X.Y.Z-linux-x86_64-otp-<major>.sha256` | Tooling archive SHA-256 checksum |
| `dist/agov-X.Y.Z-linux-x86_64-otp-<major>-provenance.json` | Tooling archive provenance |
| `dist/agf-X.Y.Z-bundle.tar.gz` | Optional convenience bundle containing the framework archives and tooling archive |
| `dist/agf-X.Y.Z-bundle.sha256` | Bundle SHA-256 checksum |

The framework provenance JSON records: `version`, `source_commit`, `tag`,
`build_timestamp`, `pipeline_identity` (CI pipeline and job IDs),
`manifest_path`, `archive_filenames`, and `checksums` keyed by filename.
The tooling provenance also records `otp_version`, `elixir_version`,
`target_platform`, and `packaging`.

## Tag/version guard

`mix agov.dist.check_tag` reads `CI_COMMIT_TAG` from the environment and
compares it to `agf/VERSION`. The comparison strips an optional `v` prefix from
the tag.

```bash
# exits 0: tag matches VERSION
CI_COMMIT_TAG=v2.0.0 mix agov.dist.check_tag

# exits 1: mismatch — no archive is written
CI_COMMIT_TAG=v9.99.99 mix agov.dist.check_tag
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

# Manifest validation — MRs, branches, and tags
agf:dist:validate:
  script:
    - mix agov.dist.validate

# Preview archives — MRs and default branch only, not releases
agf:dist:preview:
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
    - if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH

# Release archives — tag pipelines only, tag/VERSION guard runs first
agf:dist:release:
  rules:
    - if: $CI_COMMIT_TAG

# Optional tooling and bundle archives
agov:tooling:release:
  rules:
    - if: $CI_COMMIT_TAG

agf:bundle:release:
  rules:
    - if: $CI_COMMIT_TAG
```

The `agf:dist:validate` job runs `mix agov.dist.validate` without writing
archives. The `agf:dist:preview` job runs `mix agov.dist.build` for
merge-request and default-branch review artifacts; those artifacts expire and
must not be described as releases.

The `agf:dist:release` job runs `mix agov.dist.check_tag` before
`mix agov.dist.validate` and `mix agov.dist.build`. If the tag does not match
`agf/VERSION` the job exits 1 before any archive is written.

Artifacts produced by `agf:dist:release` are kept indefinitely (`expire_in:
never`) and include the `.tar.gz`, `.zip`, all `.sha256` checksum files, and the
provenance JSON.

Artifacts produced by `agov:tooling:release` and `agf:bundle:release` are also
kept indefinitely. The bundle depends on the framework and tooling archives,
but framework-only adoption does not depend on the bundle.

Installer scripts, package registry publishing, release signing, Software Bill
of Materials (SBOM) generation, and third-party notices are deferred follow-on
work.
