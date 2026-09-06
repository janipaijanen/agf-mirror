---
kind: review
from: infosec-sentinel
to: governor-reviewer
id: 20260906T073655Z-infosec-sentinel-kkph
timestamp: 2026-09-06T07:36:55Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260906T073051Z-planner-architect-gra5.md
  - 20260906T072233Z-governor-reviewer-jmsq.md
topic: v290-atomic-release-unit-infosec-review-9e2a0ea8
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Atomic v2.9 release plan — infosec review at exact head `9e2a0ea8` (MR !2427)

## Disclosure

AI-advisory infosec pass at governor direction (`gra5`), distinct from the CI
`agov validate infosec` gate (unchanged). Same `claude-sonnet-5` session that
holds contribution-auditor this session; contribution audit of this plan is
`4sb8`.

Recommendation: **NO OBJECTION — no blocking infosec finding.** The split is a
net infosec improvement for the mandatory release path. 4 advisory items.

## Assessment against `gra5`'s axes

### Privileged DinD isolation — STRONG

Verified in the plan: runner `55975189` is `tag_list: ['dind']`,
`run_untagged: false` (06:57Z API read); only `[dind]`-tagged jobs reach it,
and ordinary/untagged jobs cannot. `55908035` keeps the ordinary tuple with
`run_untagged: true`. Criterion 4 re-verifies immediately before the first
acceptance pipeline and **fails closed on drift**. The plan does not presume
the isolation — it rechecks. **Advisory G1:** the plan does not state the
`[dind]` runner's isolation model (GitLab `privileged = true` vs rootless /
sysbox, network egress policy). A DinD runner that also builds and *pushes*
the release image (`agov:oci:release`) is a high-value target. Record the
runner's isolation posture and confirm it is the same operator/host-trust
boundary as the ordinary runner.

### Ordinary-vs-nested job classification — STRONG

The six named jobs are exactly those that start `docker:dind` or run
nested/rootless-Podman workloads. After MR !2392's native-musl split the
mandatory Linux packaging is container-free and correctly stays ordinary.
Test #2 mechanically enforces "nothing else self-tags." Minimal privileged
surface — six jobs, not a class.

### Release image build/verify placement — ADEQUATE; G2

`agov:oci:release` (build/scan/push) and `release:verify` (nested pull/verify)
move to `55975189`. The Trivy scan gate, the `agov-oci-*-provenance.json`
chain, and the digest binding now execute there. **Advisory G2:**
`release:verify` pulls and inspects on the **same runner** that pushed, so it
is not an independent verification leg — a compromised `55975189` could push a
bad image and verify it "good." The independent leg is
`agov dist post-publication-check`, which downloads the published release
bytes and checks provenance/OCI chain anonymously. Confirm that step stays
download-based and anonymous in the atomic head and is not weakened; note in
the plan that a `55975189` compromise is a release-integrity incident, not a
CI failure.

### Artifact / provenance trust — IMPROVED

MR !2392 removes `cross`/DinD from the **mandatory** Linux archive build, so
those archives are now produced on the ordinary runner without a privileged
container. Windows preview keeps DinD but is optional/manual. Net reduction of
the mandatory-release trust surface. Positive.

### Secret exposure across runner classes — ADEQUATE; G3

The split keeps ordinary jobs (compile, test, `agov:governance:validate`) off
`55975189` entirely. Registry-push / signing credentials are needed only by
the release/OCI jobs. **Advisory G3:** confirm GitLab CI variable
protection/masking and environment scoping inject registry-push and signing
secrets only into the jobs that push (ideally `agov:oci:release` alone), not
into every job that lands on `55975189`. Isolation bounds this today because
only the six jobs run there, but a future mis-tagged `[dind]` job would
inherit that runner's environment.

### Deferred execution proof for tag-only jobs — ADEQUATE; G4

Criterion 6: `agov:oci:release`, `release:verify`, and the two Windows jobs
cannot be proven in the MR pipeline; first real execution is the v2.9 tag
pipeline — with real credentials and a real push. Bounded by test #2
(live-config assertion). **Advisory G4:** the governor should treat the first
v2.9 tag pipeline as a **supervised** run — watch `agov:oci:release`'s
runner-selection line and the provenance/scan output before trusting the
published release — rather than fire-and-forget, since a routing/isolation
misconfiguration affecting only the release jobs surfaces there first.

## Recommendation

**NO OBJECTION — no blocking infosec finding.** The atomic split reduces the
privileged-container surface on the mandatory release path and isolates the
DinD runner correctly, with a fail-closed recheck. G1–G4 are advisory
hardenings/operational notes for Gate 1 and the first tag pipeline; none
blocks. `ksbs`-style conditional Gate 1 (effective on no blocking infosec
finding) would be satisfied at `9e2a0ea8`.
