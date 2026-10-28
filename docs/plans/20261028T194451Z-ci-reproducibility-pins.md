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
      Supply-chain pinning change on the release-publication job. Consultation
      requested with this plan; no clearance is claimed yet.
  - role: contribution-auditor
    status: pending
    reason: >-
      Independent review requested with this plan; no clearance is claimed yet.
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

### Pin 1: `release:create`

Replace `:latest` with `:<version>@sha256:<digest>`, the same form `.rust-base`
uses. Before choosing the version, the implementer MUST confirm the pinned
image still provides every command the job's script uses (`release-cli create`,
`apk`, `curl`, `unzip`, `tar`, `gzip`). Keep a comment naming the date the
digest was resolved and the registry query used.

### Pin 2: `rust:tooling:oci-pull-test`

Replace `--default-toolchain stable` with `--default-toolchain 1.97.1`. The
version MUST equal the `.rust-base` version. Keep one source of truth: a CI
variable `RUST_TOOLCHAIN_VERSION: "1.97.1"` used by this job. Whether
`.rust-base` also reads that variable is left out of scope; it keeps its
digest-pinned image.

### Verification that pins hold (drift check)

A small POSIX script, `.gitlab/scripts/check-ci-pins.sh`, reads
`.gitlab-ci.yml` and exits non-zero when it finds any of:

- an `image:` value ending in `:latest` or with no tag at all;
- `--default-toolchain stable|beta|nightly`;
- `release:create`'s `image:` lacking `@sha256:`.

Output lines carry `severity: advisory` or `severity: blocking` per
`always/tool-authority-boundary-rule.md`.

Decision D1 is resolved by governor review
`20261028T202841Z-governor-reviewer-thps`: the check starts **advisory**
(job `allow_failure: true`, visible warning), because no named rule grants it
blocking authority. Promotion to blocking needs a separately governed rule
change; this plan does not imply one.

### Deliberate update procedure

A pin changes only through a merge request that:

1. names the old and new version and digest;
2. states the resolution date and the registry query;
3. shows the job passing on the exact head; and
4. for `release:create`, shows `release:assets-selftest` and one dry read of the
   job's tool list against the new image.

No automated bump tool is added. Cadence: review the two pins when a release
plan is drafted. No separate schedule is created.

## Negative test (`gate-self-test-rule.md`)

`tests/ci-pins-selftest.sh` runs the checker on fixtures:

| Fixture | Expected |
|---|---|
| `image: registry.gitlab.com/gitlab-org/release-cli:latest` | fails, names the line |
| `image: alpine` (no tag) | fails |
| `--default-toolchain stable` | fails |
| `release:create` image with a tag but no digest | fails |
| the corrected `.gitlab-ci.yml` | passes |

The MR description MUST include the transcript. The self-test also runs in CI.

## Out of scope

- `apk add --no-cache ...` inside `release:create` installs unpinned Alpine
  packages. This is a related reproducibility gap. It is noted here for the
  governor and is not part of this plan.
- MkDocs dependencies (ordered item 5), `mise.toml`, and the response-size
  plan (ordered item 3).
- Any blocking promotion of the check.

## Risks and rollback

- A pinned `release-cli` older than needed could lack a flag the job uses.
  Mitigation: the tool-list confirmation above. Rollback: revert the one-line
  image change by MR; no tag is involved because the job runs only on tag
  pipelines, so the first real exercise is the next release. The plan
  therefore requires a manual dry run of the job's commands in the pinned image
  before merge.
- The advisory check can be ignored. Accepted for the first slice.

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
