---
id: 20261028T194451Z-ci-reproducibility-pins
status: pending-gate1
topic: ci-reproducibility-pins-release-create-and-oci-pull-test
depends_on:
  - 20260914T130217Z-row6-s4-issue123-collision-check-blocking-promotion
  - 20261021T204757Z-remove-dind-ci
  - 20261021T220000Z-retire-windows-support
  - 20261023T083302Z-remove-elixir-leftovers
affected_paths:
  - .gitlab-ci.yml
  - .gitlab/scripts/check-ci-pins.sh
  - tests/ci-pins-selftest.sh
advisory_consultation:
  - role: infosec-sentinel
    status: pending
    reason: >-
      Independent review `20261028T205413Z-governor-reviewer-tqds` returned BLOCK
      at head 861070e2. This revision answers its four findings. The row stays
      pending until an exact-head re-review; no clearance is claimed.
  - role: contribution-auditor
    status: pending
    reason: >-
      Independent review `20261028T205413Z-governor-reviewer-tqds` returned BLOCK
      at head 861070e2. This revision answers its four findings. The row stays
      pending until an exact-head re-review; no clearance is claimed.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: pin two floating CI inputs and make later drift visible

Governance mode: STRICT. The plan touches a release-publication job
(`always/high-risk-operations-rule.md`, "Release, tag, and publication
mechanics") and adds a check (`always/gate-self-test-rule.md`).
Ordered item 2 of governor handoff `20261028T174041Z-governor-reviewer-s7ar`
(findings 1 and 2 of ACS seq 849). This plan does not authorize
implementation.

## Intent

Two CI inputs float, against `always/security-hardening-rule.md` ("pin image
tags", "pin tool versions in CI"):

1. `release:create` (`.gitlab-ci.yml`, job `release:create`) uses
   `registry.gitlab.com/gitlab-org/release-cli:latest`. It runs only on a tag
   pipeline. A tag is immutable, so a bad upstream image costs a version number.
2. `rust:tooling:oci-pull-test` (`.gitlab-ci.yml`, `before_script`) installs
   Rust by piping `sh.rustup.rs` into `sh` with `--default-toolchain stable`.
   Other jobs use `rust:1.97.1@sha256:1bcff4be...` (`.rust-base`). The two
   toolchains can differ and `stable` moves without a repository change.

## Evidence (read 2026-10-28, `origin/main` `cc2976a3`)

- The highest semver tag in the `gitlab-org/release-cli` registry repository
  was `v0.24.0`. I did not confirm that `latest` equals `v0.24.0`; the
  implementer MUST resolve and record the digest of the tag actually chosen.
- The declared Rust version for CI is `1.97.1` (`.rust-base`, with its
  digest-resolution comment citing `docs/plans/ci-rust-cache-policy-slice-b.md`).
  `mise.toml` has `rust = "stable"`; that is local developer tooling and is out
  of scope here.

## Collision reconciliation (2026-10-28)

Pipeline `3000929469` failed `agov:governance:plan-collision-gate` on four
unacknowledged `.gitlab-ci.yml` overlaps (governor review
`20261028T202841Z-governor-reviewer-thps`). All four plans are `approved` and
landed. Each was read against the current CI file. This plan edits only the
`image:` line of `release:create` and the `--default-toolchain` argument in
`rust:tooling:oci-pull-test`.

| Landed plan | Effect on the two jobs | Effect on this plan |
|---|---|---|
| `20260914T130217Z-row6-s4-issue123-collision-check-blocking-promotion` | Added the collision-check job that failed this MR. | None on the two jobs. Needed here: the check correctly requires this `depends_on`. |
| `20261021T204757Z-remove-dind-ci` | Changed `oci-pull-test` runner tags to `podman`, `linux`, `amd64`. The job image and the rustup line are unchanged. | None. Pin 2 edits the rustup line only. |
| `20261021T220000Z-retire-windows-support` | Removed Windows preview CI. Neither job changed. | None. |
| `20261023T083302Z-remove-elixir-leftovers` | Removed Elixir artifact checks from CI scripts. | None. The `release:create` tool list is unchanged (`release-cli`, `apk`, `curl`, `unzip`, `tar`, `gzip`). |

All four are added to `depends_on`. Line numbers moved because of these plans,
so this plan now cites jobs by name, not by line.

## Design

Revision 2. It answers the BLOCK findings in
`20261028T205413Z-governor-reviewer-tqds` (CA-1 to CA-4, IS-1 to IS-4).

### Pin 1: `release:create` image (CA-4, IS-2, IS-3)

- **Tag.** Pin `registry.gitlab.com/gitlab-org/release-cli` to the exact tag
  `v0.24.0` plus its `@sha256:` digest. `v0.24.0` was the highest plain
  `vMAJOR.MINOR.PATCH` tag on 2026-10-28. Selection rule if the implementer
  finds `v0.24.0` unusable: the highest plain-semver tag that passes the
  tool check below; no pre-release tag; no `latest`.
- **Digest.** Resolve it at implementation from the registry manifest of that
  tag and record the date and query in a comment next to the `image:` line.
- **Tool check (credential-free, non-mutating).** Run in the pinned image with
  no CI token and no upload: `command -v release-cli curl unzip tar gzip apk`
  and `release-cli --version`. This replaces the earlier "dry run of the job's
  commands", which would upload packages and create a Release. No command that
  needs `CI_JOB_TOKEN` runs.
- **Vulnerability scan.** The repository already scans with Trivy
  (`aquasec/trivy:0.66.0`, job `dependency:cve-scan`). At implementation run
  `trivy image` on the pinned digest and put the HIGH and CRITICAL result in
  the MR. Each HIGH or CRITICAL finding is fixed (choose another tag by the
  rule above) or accepted by a governor risk record before merge.

### Pin 2: `rust:tooling:oci-pull-test` toolchain (CA-2, IS-1)

- **One declared version.** Add a CI variable
  `RUST_TOOLCHAIN_VERSION: "1.97.1"`. The job passes
  `--default-toolchain "$RUST_TOOLCHAIN_VERSION"`.
- **Equality check.** The drift checker (below) extracts the version from the
  `.rust-base` image tag and compares it to `RUST_TOOLCHAIN_VERSION`. A
  mismatch is an advisory finding. `.rust-base` keeps its digest-pinned image.
- **Verifiable installer.** Remove `curl https://sh.rustup.rs | sh`. Download
  the versioned `rustup-init` binary for a named rustup release from
  `static.rust-lang.org`, and verify it against its published SHA-256 with
  `sha256sum -c` before running it. The rustup version and the checksum are
  literals in `.gitlab-ci.yml`, resolved and recorded at implementation. A
  failed checksum stops the job.

### Drift checker (CA-1, CA-3)

`.gitlab/scripts/check-ci-pins.sh` is **advisory and always exits 0** when it
runs to completion. It prints one line per finding, each tagged
`severity: advisory`. It exits non-zero only for its own malfunction (missing
file, unreadable input). The CI job needs no `allow_failure` to stay advisory,
because the tool itself grants itself no blocking authority.
Per `always/tool-authority-boundary-rule.md`, blocking behavior needs a
separately governed authority grant; this plan does not create one.

It checks both GitLab image forms:

- scalar: `image: name:tag@sha256:...`;
- mapping: `image:` with a nested `name:` (and optional `entrypoint:`).

Findings: a `:latest` tag; no tag; for `release:create` only, no `@sha256:`;
`--default-toolchain stable|beta|nightly`; a `sh.rustup.rs` pipe;
`RUST_TOOLCHAIN_VERSION` differing from the `.rust-base` tag.

### Deliberate update procedure

A pin changes only through a merge request that:

1. names the old and new version and digest;
2. states the resolution date and the registry query;
3. shows the credential-free tool check and the Trivy result for the new
   digest (release image) or the checksum verification (rustup); and
4. shows the job passing on the exact head.

No automated bump tool is added. Review the pins when a release plan is
drafted.

## Negative test (`gate-self-test-rule.md`)

`tests/ci-pins-selftest.sh` feeds fixtures to the checker and asserts both
the advisory output and the exit code (0 for findings, non-zero only for a
missing input file).

| Fixture | Expected output | Exit |
|---|---|---|
| scalar `image: ...release-cli:latest` | advisory finding naming the line | 0 |
| scalar `image: alpine` (no tag) | advisory finding | 0 |
| mapping `image:` with `name: alpine` (no tag) | advisory finding | 0 |
| mapping `image:` with `name: alpine:3.22` (tag) | no finding | 0 |
| `--default-toolchain stable` | advisory finding | 0 |
| `curl ... sh.rustup.rs \| sh` | advisory finding | 0 |
| `release:create` image with tag, no digest | advisory finding | 0 |
| `RUST_TOOLCHAIN_VERSION` != `.rust-base` tag | advisory finding | 0 |
| corrected `.gitlab-ci.yml` | no findings | 0 |
| missing input file | error line | non-zero |

The MR description MUST include the transcript. The self-test also runs in CI.

## Out of scope and residual risk

- `apk add --no-cache ...` inside `release:create` installs unpinned Alpine
  packages. **This plan does not make `release:create` fully reproducible and
  does not claim to.** A durable residual-risk record MUST exist before the
  next release (governor review `tqds`); it is filed with the revised plan.
- MkDocs dependencies (ordered item 5), `mise.toml`, and the response-size
  plan (ordered item 3).
- Any blocking promotion of the check.

## Risks and rollback

- A pinned `release-cli` could lack a command the job uses. Mitigation: the
  credential-free tool check, run before merge.
- **Rollback never restores `:latest`.** Roll back to a verified last-known-good
  pinned digest. No such digest exists before this change, so if the new pin
  fails and no other verified digest is available, **suspend release
  publication** until one is verified.
- The advisory check can be ignored. Accepted for the first slice.
- The rustup checksum literal can go stale. The update procedure above covers
  it; a stale value fails the job, which is the safe direction.

## Impact statement

```
Impact:
  Foundry:          CI-only; changes two jobs and adds one advisory check.
  Direct adopters:  none; no shipped artifact changes.
  Downstream:       none.
  Migration:        none.
  Evidence:         ACS seq 849 findings 1 and 2; governor intake s7ar.
```

Adopter docs parity: not applicable (foundry CI configuration only).

## Gate 1 request

Approve the two pins, the advisory drift check (D1), and the update
procedure. Implementation, if approved, is one MR.
