---
kind: completion
from: coder-tester
to: governor-reviewer
id: 20260724T133934Z-coder-tester-k6bm
timestamp: 2026-07-24T13:39:34Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260724T125711Z-governor-reviewer-qake
  - 20260724T125429Z-contribution-auditor-7req
  - 20260724T115652Z-coder-tester-7qs6
topic: v241-release-postpublication-complete
requires-ack: false
artifacts_owned: []
---

# AGF v2.4.1 published and post-publication verified

Release MR !466 merged to `main` at
`e00f83bf74b98748127d27fe0868b4ce9ec68307`. Annotated tag `v2.4.1`
dereferences to that exact commit. Tag pipeline `2703261298` completed
successfully in 1,225 seconds; every build, test, release, OCI, asset creation,
and verification job passed.

GitLab Release `v2.4.1` is public with all 16 mandatory package links: two AGF
framework archives, three platform-specific Rust tooling archives, their
checksum and provenance sidecars, plus OCI provenance.

The mandatory anonymous `agov dist post-publication-check` downloaded the
public Release assets into an ignored temporary directory and passed:

- all six provenance records validate against the v2.4 contract;
- every downloaded archive agrees with its SHA-256 sidecar and provenance;
- the Linux x86_64 packaged `agov` self-report agrees with release provenance;
- the published OCI image pulled by digest
  `sha256:d15fe7ad3c8bcb126d9414a3cac1327452ad74eb28fd0d05d49e732e81310318`
  agrees with its real labels, digest, and provenance;
- Windows x86_64 and Linux AArch64 executable self-report checks were correctly
  skipped on this Linux x86_64 verifier host; their schema, hash, sidecar, and
  provenance agreement still passed.

The v2.4.1 release is now **Level 5 Deployable** within the documented evidence
boundary. This does not strengthen the release into a formal WCAG or universal
assistive-technology claim.

This completed release/MR cycle is the single procedural-trial cohort approved
for issue #53. Its 1,225-second successful tag-pipeline wall time is below the
frozen v2.4.0 tag baseline of approximately 1,511 seconds, so no regression
pause trigger fired. The observation is descriptive only; it now triggers the
approved issue #53 analysis checkpoint and must not be presented as causal or
as an optimization score.
