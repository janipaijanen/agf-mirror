---
kind: handoff
from: build-engineer
to: coder-tester
id: 20261029T175542Z-build-engineer-1q4k
timestamp: 2026-10-29T17:55:42Z
model: claude
immutable: true
references-to:
  - 20261029T174147Z-build-engineer-hgns.md
  - 20261029T162807Z-governor-reviewer-kmp6.md
topic: ci-pins-implementation-handoff-and-claim-release
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Continue the approved CI reproducibility pins implementation from branch chore/ci-reproducibility-pins at 516142b2ff00faca3ec10784c3585119f99c8030.
    - Produce the two outstanding evidence items and open the Gate 2 MR.
  must_read:
    - docs/plans/20261028T194451Z-ci-reproducibility-pins.md
  do_not_touch:
    - evidence/ci-pins-tool-check (throwaway; never merge)
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# CI pins: handoff and release of the implementation claim

Per ACS seq 903, build-engineer **relinquishes** the claim in
`20261029T174147Z-build-engineer-hgns` (MR !3356) to coder-tester (Codex).
MR !3356 never merged, so it creates no standing claim. Recommendation: close
it unmerged; this record supersedes it.

## Branches and SHAs

- `chore/ci-reproducibility-pins` at `516142b2ff00faca3ec10784c3585119f99c8030`,
  from `main` `ee2a768a`. Pushed with `-o ci.skip`; no MR, no pipeline.
- `evidence/ci-pins-tool-check` at `d275f112`. Throwaway. It replaces almost
  all of `.gitlab-ci.yml`. Delete it unmerged.

## Verified facts

- `release-cli` tag `v0.24.0` is the highest plain semver tag. Digest
  `sha256:3f52d526f48e8d10625c722fa6d6423ee82aadeae1b1aa91f07b62551d96dacf`
  (registry manifest `GET /v2/gitlab-org/release-cli/manifests/v0.24.0`,
  `Docker-Content-Digest`; equals the body SHA-256). `latest` had the same
  digest. Image: linux/amd64, single OCI manifest. Resolved 2026-10-29.
- rustup `1.29.2`, `x86_64-unknown-linux-gnu`: published SHA-256
  `5be191e8c70a5007074254cb74d78dbfdd25cd5a61c27de7b8e5410122ec24f4`; the
  downloaded binary matched it.
- `glab ci lint` reports the changed `.gitlab-ci.yml` valid.

## Files changed on the branch (the plan's three paths)

- `.gitlab-ci.yml`: `RUST_TOOLCHAIN_VERSION: "1.97.1"`; `release:create` image
  pinned to the tag and digest; `rust:tooling:oci-pull-test` installs a
  checksum-verified `rustup-init` instead of the `sh.rustup.rs` pipe; new job
  `ci:pins:advisory` (`allow_failure: true`, alpine:3.22).
- `.gitlab/scripts/check-ci-pins.sh`: advisory checker; findings exit 0, an
  unreadable input exits 2.
- `tests/ci-pins-selftest.sh`: 13 cases, scalar and mapping `image:` forms.

## Commands already run

- `sh tests/ci-pins-selftest.sh` before the CI edit: 12 pass, 1 fail. The
  failing case ran the checker on the uncorrected repository file and found 4
  advisory findings (latest tag, missing digest, floating toolchain, rustup
  pipe). This is the negative proof. After the edit: all 13 pass, 0 findings.
- `agov` was not needed. No Rust file changed.

## Failed evidence runs (do not retry unchanged)

- Pipeline `3005203370`: YAML error (unquoted `echo` with `: `). Fixed.
- Pipeline `3005205359`: job `17584788854` (nested Podman) failed before any
  assertion: crun could not mount proc on the Docker executor. Job
  `17584788853` (Trivy 0.66.0) downloaded its DB, then hit a DNS timeout on
  `registry.gitlab.com`. Neither the tool check nor the scan is satisfied.

## Unresolved plan criteria

1. Step 1 tool check: the pinned image alone provides `release-cli` and `apk`
   (`release-cli --version`).
2. Step 2 isolated install: `apk add --no-cache curl unzip tar gzip`, then
   `command -v curl unzip tar gzip`, in a throwaway container with no token and
   no project mount.
3. Trivy HIGH and CRITICAL result for the pinned digest, with a fix or a
   governor risk decision for each finding.
4. Gate 2 MR with the `## Negative test` transcript (above) and exact-head
   pipeline. The residual-risk record already exists
   (`20261028T210122Z-planner-architect-2k4p`).

## Security caveat

Every GitLab job container inherits `CI_JOB_TOKEN`. Running the checks as the
job image does not by itself satisfy the plan's "no token, no project mount"
boundary. The direct-job idea (`GIT_STRATEGY: none` plus an explicit assertion
that no credential variable is set) needs confirmation against the plan and
InfoSec before it is relied on. Do not weaken that boundary to get a green run.
