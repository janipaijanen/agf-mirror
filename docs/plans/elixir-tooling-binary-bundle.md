---
id: elixir-tooling-binary-bundle
status: pending-gate1
topic: elixir-tooling-binary-and-release-bundle
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: CI builds and publishes a binary artifact derived from the tooling source; ERTS inclusion, artifact provenance, and forbidden-path exclusions must be explicit
  - role: contribution-auditor
    status: consulted
    reason: adds adopter-facing binary artifact and bundle; claims about what ships must match what is actually built
---

# Plan: Elixir tooling binary and release bundle

**Date:** 2026-06-16T07:10:07Z
**Depends on:** `elixir-ci-cd-adopter-packaging` Gate 1 approved and CI on main.
**Proposed branch:** `feat/tooling-binary-bundle`
**Expected implementer:** coder-tester

---

## Intent

Add the two remaining release artifacts to the CI pipeline:

1. **Tooling binary** — a linux-x86_64 Mix release (ERTS-bundled) of the
   `agov` CLI. Adopters untar and run without installing Elixir or OTP.
2. **Release bundle** — a single archive combining the framework source
   archive and the tooling binary, for adopters who want one download.

The framework source archive (`agf:dist:release`) already ships. This plan
adds two jobs on top of it.

---

## Packaging approach

**Mix release with bundled ERTS** (not Burrito, not escript).

- The existing CI image (`hexpm/elixir:1.20.1-erlang-29.0.2-debian-bookworm`)
  already targets linux-x86_64 with the correct OTP version. No cross-compile
  toolchain is needed.
- Mix release bundles the ERTS runtime into the archive; the adopter unpacks
  and runs `bin/agov` without Elixir or OTP on the host.
- Burrito is deferred until there is adoption evidence for platforms beyond
  linux-x86_64.

---

## Artifact specification

| Artifact | Filename pattern | Produced by |
|---|---|---|
| Framework source | `agf-<version>.tar.gz` / `.zip` | existing `agf:dist:release` |
| Tooling binary | `agov-<version>-linux-x86_64-otp-29.tar.gz` | new `agov:tooling:release` |
| Checksum + provenance | `agov-<version>-linux-x86_64-otp-29.sha256` / `-provenance.json` | new `agov:tooling:release` |
| Bundle | `agf-<version>-bundle.tar.gz` | new `agf:bundle:release` |
| Bundle checksum | `agf-<version>-bundle.sha256` | new `agf:bundle:release` |

The OTP major version (29) appears in the tooling binary filename so adopters
know the runtime version without reading provenance. If the CI base image OTP
version changes, the filename changes accordingly — derive it from
`elixir -e 'IO.puts System.otp_release()'` at build time rather than
hardcoding.

### Bundle contents

`agf-<version>-bundle.tar.gz` contains:

```
agf-<version>-bundle/
  agf-<version>.tar.gz          # framework source archive
  agf-<version>.zip             # framework source archive (zip)
  agov-<version>-linux-x86_64-otp-29.tar.gz  # tooling binary
  SHA256SUMS                    # checksums for all three archives
  bundle-provenance.json        # combined provenance
```

The bundle does not re-expand the inner archives; it wraps them as-is.

### Provenance fields (tooling binary)

Same minimum set as framework provenance, plus:

- `otp_version` — from `System.otp_release()`
- `elixir_version` — from `System.version()`
- `target_platform` — `linux-x86_64`
- `packaging` — `mix-release`

---

## Scope

### New Mix release configuration

Add `rel/` directory and `mix.exs` release configuration in
`tooling/elixir/`. The release target is named `agov`. It must:

- include the `agov` application and all dependencies;
- strip debug symbols;
- set `include_erts: true`;
- exclude the `agf/` framework source tree (CI mounts it at build time
  but it must not be embedded in the binary artifact).

### New Mix tasks

Add two Mix tasks under `tooling/elixir/lib/mix/tasks/`:

- `mix agov.tooling.build` — runs `mix release agov`, assembles the
  tarball, computes SHA-256, and writes provenance JSON.
- `mix agov.bundle.build` — assembles the bundle archive from the
  framework archives (already in `dist/`) and the tooling binary archive,
  writes `SHA256SUMS` and `bundle-provenance.json`.

Both tasks must fail cleanly if their inputs are missing.

### New CI jobs

Add to root `.gitlab-ci.yml`:

```yaml
agov:tooling:release:
  extends: .elixir-base
  stage: deploy
  script:
    - mix agov.dist.check_tag          # reuse existing tag/VERSION guard
    - mix agov.tooling.build
  artifacts:
    name: "agov-$CI_COMMIT_TAG"
    paths:
      - dist/agov-*.tar.gz
      - dist/agov-*.sha256
      - dist/agov-*-provenance.json
    expire_in: never
  rules:
    - if: $CI_COMMIT_TAG
  needs:
    - agf:dist:release                 # framework archives built first

agf:bundle:release:
  extends: .elixir-base
  stage: deploy
  script:
    - mix agov.dist.check_tag
    - mix agov.bundle.build
  artifacts:
    name: "agf-bundle-$CI_COMMIT_TAG"
    paths:
      - dist/agf-*-bundle.tar.gz
      - dist/agf-*-bundle.sha256
    expire_in: never
  rules:
    - if: $CI_COMMIT_TAG
  needs:
    - agf:dist:release
    - agov:tooling:release
```

### Preview jobs (MR/default branch)

Add corresponding preview jobs (`agov:tooling:preview`, `agf:bundle:preview`)
that run the same build tasks on merge requests and the default branch,
producing artifacts that expire in 1 week, without the tag/VERSION guard.

### Tests

Add tests for:

- `mix agov.tooling.build` produces expected archive structure, provenance
  fields, and SHA-256 file.
- `mix agov.bundle.build` produces expected directory layout inside the
  bundle archive and `SHA256SUMS` listing all three inner archives.
- `mix agov.bundle.build` fails cleanly when the tooling binary archive
  is missing from `dist/`.

The gate-self-test requirement: run `agov.dist.check_tag` with a
deliberate version mismatch to confirm both new jobs are blocked before
any archive is written. Demonstrate this in the Gate 2 handoff.

---

## Non-Goals

- macOS, Windows, or ARM binaries.
- Burrito / escript / Nerves packaging.
- Package registry publishing (Hex, Docker Hub, GitHub Packages).
- Installer scripts or PATH manipulation.
- Release signing or Sigstore.
- SBOM generation.

---

## Safety Requirements

- The tooling binary archive must not contain `agf/` framework content.
- The tooling binary archive must not contain `.env`, `.git/`, `tmp/`,
  or `agents-govern/`.
- CI must not echo secrets.
- `agov:tooling:release` and `agf:bundle:release` must run only on tags.
- A failed `mix agov.dist.check_tag` must prevent archive upload.

---

## Acceptance Criteria

- `mix agov.tooling.build` and `mix agov.bundle.build` run locally and
  in CI.
- Tag pipeline produces all five artifacts listed in the artifact table.
- Bundle archive unpacks to the expected directory structure.
- Provenance JSON for the tooling binary includes `otp_version`,
  `elixir_version`, `target_platform`, `packaging`.
- Gate-self-test demonstrates tag/VERSION mismatch blocks the new jobs.
- Tests pass: 197 + new tests, 0 failures.
- No binary content, installer, registry, or signing claims added to docs
  beyond what is implemented.

---

## Validation

```bash
cd tooling/elixir
mix deps.get
mix compile --warnings-as-errors
mix test
mix agov.dist.validate
mix agov.tooling.build
mix agov.bundle.build
ls -lh ../../dist/
```
