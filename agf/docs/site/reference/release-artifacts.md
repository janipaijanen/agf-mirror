# Release artifacts

AGF source release archives are built from
`agf/dist/release-manifest.yaml`. The manifest is the release policy; GitLab CI
calls the AGF tooling instead of maintaining a separate artifact include/exclude
list.

From `tooling/elixir/`:

```bash
mix agov.dist.validate
mix agov.dist.build
```

`mix agov.dist.validate` expands the manifest and fails if selected files escape
the repository, match forbidden paths, or would collide in the archive.
`mix agov.dist.build` writes the configured `.tar.gz` and `.zip` archives under
`dist/`, along with SHA-256 checksum files and a machine-readable provenance
record.

The current archive policy is allowlist-first:

- include the adopter-facing `agf/` framework, docs, schemas, rules, skills,
  and release manifest;
- exclude generated site output and generated release-info pages;
- forbid local or bootstrap paths such as `.worktrees/`, `tmp/`, `dist/`,
  `agents-govern/`, Elixir build directories, and `.env` files.

Downstream projects may fork the manifest or pass `--manifest` to use their own
release policy.

## Artifacts produced by a release build

For version `X.Y.Z`, `mix agov.dist.build` produces:

| File | Content |
|---|---|
| `dist/agf-X.Y.Z.tar.gz` | Source archive |
| `dist/agf-X.Y.Z.zip` | Source archive |
| `dist/agf-X.Y.Z.tar.gz.sha256` | SHA-256 checksum (BSD format) |
| `dist/agf-X.Y.Z.zip.sha256` | SHA-256 checksum (BSD format) |
| `dist/agf-X.Y.Z-provenance.json` | Machine-readable build provenance |

The provenance JSON records: `version`, `source_commit`, `tag`,
`build_timestamp`, `pipeline_identity` (CI pipeline and job IDs),
`manifest_path`, `archive_filenames`, and `checksums` keyed by filename.

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

The root `.gitlab-ci.yml` wires three release-related stages:

```yaml
# MkDocs strict build — review artifact on MRs and branches
site-html-build:
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
    - if: $CI_COMMIT_BRANCH && $CI_COMMIT_BRANCH != $CI_DEFAULT_BRANCH

# GitLab Pages — published from master only
pages:
  rules:
    - if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH

# Release archives — tag pipelines only, tag/VERSION guard runs first
release-artifacts:
  rules:
    - if: $CI_COMMIT_TAG
```

The `release-artifacts` job runs `mix agov.dist.check_tag` before
`mix agov.dist.validate` and `mix agov.dist.build`. If the tag does not match
`agf/VERSION` the job exits 1 before any archive is written.

Artifacts produced by `release-artifacts` are kept indefinitely (`expire_in:
never`) and include the `.tar.gz`, `.zip`, all `.sha256` checksum files, and the
provenance JSON.
