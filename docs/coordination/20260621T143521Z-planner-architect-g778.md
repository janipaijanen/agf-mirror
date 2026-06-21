---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260621T143521Z-planner-architect-g778
timestamp: 2026-06-21T14:35:21Z
immutable: true
references-to:
  - 20260621T140549Z-planner-architect-e3rs.md
  - 20260621T140113Z-coder-tester-p9cw.md
topic: rust-dist-tooling-bundle-packaging
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Implement agov dist validate (manifest validation, no archive written)
    - Implement agov dist build (validate + tar.gz + zip + SHA256 + provenance JSON)
    - Implement agov dist check-tag (CI_COMMIT_TAG vs agf/VERSION guard)
    - Implement agov tooling build (cross-compile Rust binaries per platform, archive, checksum, provenance)
    - Implement agov bundle build (assemble bundle from dist/ archives + SHA256SUMS + provenance)
    - Add Rust CI jobs rust:agf:dist:validate, rust:agf:dist:preview in .gitlab-ci.yml
  must_read:
    - docs/plans/rust-agov-complete.md
    - agf/dist/release-manifest.yaml
    - tooling/elixir/lib/agov/dist_manifest.ex
    - tooling/elixir/lib/agov/release_package.ex
    - tooling/elixir/lib/agov/dist_release_guard.ex
    - tooling/rust/crates/agov/src/main.rs
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Handoff: Rust packaging primitives — dist / tooling / bundle

Governor Gate 2 for `feat/rust-packaging-ci` (CI path scoping + Windows
preview) was approved and the branch merged at `81ce8c0`. That slice completed
path scoping and added `agov.exe` as a preview artifact.

This handoff covers the next slice: implement the Rust packaging commands that
replace Elixir packaging in CI.

## Approved plan

`docs/plans/rust-agov-complete.md` section 3.2 (Rust-Owned Packaging) and
the rollout step 4 (Rust packaging primitives). Gate 1 is already approved;
no new plan file is needed for this slice.

## What to implement

### 1. `agov dist validate`

Rust port of `mix agov.dist.validate`. Reads `agf/dist/release-manifest.yaml`
(or `--manifest <path>`) from repo root (or `--root <path>`).

Validation steps (match Elixir `Agov.DistManifest.validate/2`):
- Parse YAML; reject non-mapping
- Assert `schema_version: 1`
- Assert `package` mapping with required keys: `name`, `version_file`,
  `output_dir`, `basename`, `formats`
- Read version from `package.version_file` (relative to root)
- Resolve `include` entries (path or glob, optional flag)
- Apply `exclude` patterns (glob match)
- Apply `forbidden` patterns; error if any included file matches forbidden
- Assert `formats` is a list containing only `tar.gz` and/or `zip`
- Print summary on success (version, file count, output paths); print errors
  and exit 1 on failure

On success, print:
```
OK  release manifest valid
Manifest: agf/dist/release-manifest.yaml
Package:  agf <version>
Files:    <N>
Outputs:
  - dist/agf-<version>.tar.gz
  - dist/agf-<version>.zip
```

### 2. `agov dist build`

Runs validate, then builds archives. Outputs to `dist/` (or `package.output_dir`):
- `agf-<version>.tar.gz` — gzipped tar of included files
- `agf-<version>.zip` — zip of included files
- `agf-<version>.tar.gz.sha256` — hex checksum + filename
- `agf-<version>.zip.sha256` — hex checksum + filename
- `agf-<version>-provenance.json` — JSON: version, source_commit
  (`CI_COMMIT_SHA` or "unknown"), tag (`CI_COMMIT_TAG` or "untagged"),
  build_timestamp (ISO-8601 UTC), pipeline_identity (`CI_PIPELINE_ID`,
  `CI_JOB_ID`), manifest_path, archive_filenames, checksums

Print the same summary as `dist validate` plus checksum hex values.

### 3. `agov dist check-tag`

Reads `agf/VERSION` (or `--version-file <path>`). Reads `CI_COMMIT_TAG` env.
- If `CI_COMMIT_TAG` unset/empty: print `OK  CI_COMMIT_TAG not set (local build, guard skipped)`, exit 0
- If tag (stripped of leading `v`) matches trimmed VERSION: print `OK  CI_COMMIT_TAG <tag> matches agf/VERSION`, exit 0
- If mismatch: print `ERR CI_COMMIT_TAG <tag> does not match agf/VERSION <version>: ...`, exit 1

### 4. `agov tooling build`

Rust-native: builds and packages Rust binaries for each target platform.
This is structurally different from the Elixir version (which ran `mix release`).

For Rust, `agov tooling build` should:
1. Use `cross build --release --target <triple>` (via `cross`, which must be
   available on PATH) for Linux targets; for Windows, also via `cross`.
2. Package each platform's binary into a tar.gz (Linux) or zip (Windows)
   inside `dist/`. Basename: `agov-<version>-<platform>` where platform is
   a human-readable slug (e.g. `linux-x86_64`, `linux-aarch64`,
   `windows-x86_64`).
3. Write per-archive SHA256 checksums (`.sha256` files).
4. Write a provenance JSON per platform with: version, source_commit,
   tag, build_timestamp, pipeline_identity, archive_filename, checksum,
   target_triple, packaging ("rust-cross").

Target triples (match `.gitlab-ci.yml` `rust:tooling:build:preview` script):
- `x86_64-unknown-linux-musl` → slug `linux-x86_64` → `.tar.gz`
- `aarch64-unknown-linux-musl` → slug `linux-aarch64` → `.tar.gz`
- `x86_64-pc-windows-gnu` → slug `windows-x86_64` → `.zip` (binary: `agov.exe`)

Accept `--target <triple>` to build a single platform (for CI jobs that build
one target per runner). Default (no `--target`): build all three.

Cargo.toml path for `cross`: `tooling/rust/Cargo.toml`. The crate being built
is `agov`.

Expected output per platform in `dist/`:
```
agov-<version>-linux-x86_64.tar.gz
agov-<version>-linux-x86_64.sha256
agov-<version>-linux-x86_64-provenance.json
agov-<version>-linux-aarch64.tar.gz
agov-<version>-linux-aarch64.sha256
agov-<version>-linux-aarch64-provenance.json
agov-<version>-windows-x86_64.zip
agov-<version>-windows-x86_64.sha256
agov-<version>-windows-x86_64-provenance.json
```

### 5. `agov bundle build`

Assembles a bundle from existing `dist/` archives. Inputs expected in `dist/`:
- `agf-<version>.tar.gz` (required)
- `agf-<version>.zip` (required)
- at least one `agov-<version>-*.tar.gz` (Linux tooling; required)
- optionally `agov-<version>-windows-x86_64.zip`

Output in `dist/`:
- `agf-<version>-bundle.tar.gz` — archive containing all inputs as flat files
- `SHA256SUMS` inside the bundle — one line per input file
- `bundle-provenance.json` inside the bundle — version, source_commit, tag,
  build_timestamp, pipeline_identity, archive_filenames, checksums by basename

Print summary: archive path, checksum, input count.

## Where to add the new commands

Add a new top-level crate `tooling/rust/crates/agov-dist/` (or add to
`agov-core` if the scope is small enough) for manifest reading, archive
building, and provenance writing logic. Wire `dist`, `tooling`, and `bundle`
as top-level subcommands in `tooling/rust/crates/agov/src/main.rs`.

The manifest reader is the most substantial new piece; extract it to a library
crate so it can be tested independently.

## CI jobs to add

After the commands work, add these jobs to `.gitlab-ci.yml`:

```yaml
rust:agf:dist:validate:
  stage: test
  script:
    - cargo run --manifest-path tooling/rust/Cargo.toml -p agov -- dist validate
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
      changes:
        - agf/**/*
        - tooling/rust/**/*
        - .gitlab-ci.yml
    - if: $CI_COMMIT_BRANCH
      changes:
        - agf/**/*
        - tooling/rust/**/*
        - .gitlab-ci.yml
    - if: $CI_COMMIT_TAG

rust:agf:dist:preview:
  stage: build
  script:
    - cargo run --manifest-path tooling/rust/Cargo.toml -p agov -- dist validate
    - cargo run --manifest-path tooling/rust/Cargo.toml -p agov -- dist build
  artifacts:
    paths:
      - dist/agf-*.tar.gz
      - dist/agf-*.zip
      - dist/agf-*.sha256
      - dist/agf-*-provenance.json
    expire_in: 1 week
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
      changes:
        - agf/**/*
        - tooling/rust/**/*
        - .gitlab-ci.yml
    - if: $CI_COMMIT_BRANCH && $CI_COMMIT_BRANCH != $CI_DEFAULT_BRANCH
      changes:
        - agf/**/*
        - tooling/rust/**/*
        - .gitlab-ci.yml
    - if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH
      changes:
        - agf/**/*
        - tooling/rust/**/*
        - .gitlab-ci.yml
```

Do not replace or remove the Elixir `agf:dist:validate` and `agf:dist:preview`
jobs in this slice. That comes after Rust parity is confirmed at Gate 2.

## Validation before Gate 2

Local (from repo root):
```
cd tooling/rust && cargo fmt --check
cd tooling/rust && cargo test --workspace
cd tooling/rust && cargo clippy --workspace -- -D warnings
cd tooling/rust && cargo deny check
./tooling/agov dist validate
./tooling/agov dist build
# confirm dist/agf-<version>.tar.gz and .zip exist with matching .sha256 files
./tooling/agov dist check-tag  # should print "not set" locally
```

CI: `rust:agf:dist:validate` and `rust:agf:dist:preview` green on the MR
pipeline. The Elixir jobs must still pass independently (they're not removed
in this slice).

## Branch

Use `feat/rust-dist-packaging`.

## Output parity check

The `agov dist validate` and `agov dist build` output must produce the same
set of files that `mix agov.dist.validate` and `mix agov.dist.build` produce
for the same manifest. Add an integration test that runs both and compares
the file list (not the archive contents, which may differ in timestamp
ordering).
