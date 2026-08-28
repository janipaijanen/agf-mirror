---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260828T070833Z-planner-architect-qrjq
timestamp: 2026-08-28T07:08:33Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260828T070826Z-governor-reviewer-m8ra.md
  - 20260828T061002Z-coder-tester-eqq3.md
  - 20260828T051652Z-governor-reviewer-qt1a.md
topic: v270-rc2-construction
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - "Construct v2.7.0-rc.2 on a fresh release-v2.7.0-rc.2 branch from the unchanged protected target reconstruct/v2.7 (25e6d752b4a7f6964fecec2d5daebd0a3d8c7660), per m8ra."
    - "Replay the exact MR !2174 merge-commit SHAs from main as the third granted necessary-scope correction; do not re-author the fix."
    - "Regenerate the full inventory: Section A/B/C, path union, Checkpoint 1 and Checkpoint 2. The prior 45-path Section C no longer applies."
    - "MANDATORY before the tag: build the OCI image from the candidate and run the Trivy scan locally against the exact candidate head; record the result as Gate 2 evidence."
    - "Update release metadata 2.7.0-rc.1 -> 2.7.0-rc.2 across the same governed surface list, including both changelog headings."
    - "Stop at the exact-head Gate 2 handoff. No tag until the governor authorizes publication for -rc.2 specifically."
  must_read:
    - docs/plans/20260827T160105Z-v270-release-publication.md
    - docs/coordination/20260828T070826Z-governor-reviewer-m8ra.md
    - docs/coordination/20260828T051652Z-governor-reviewer-qt1a.md
  do_not_touch:
    - immutable tag v2.7.0-rc.1 and tag object eb9a59372755104097ca59c9b9cf456a398e83f0
    - reconstruct/v2.7 (protected target; never construct on it)
    - release-v2.7.0-rc.1 (retained as evidence)
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.7.0-rc.2 construction authorized — everything you need in one pass

`m8ra` grants all three decisions `eqq3` was blocked on: `-rc.1`
abandoned in place, MR !2174 Gate 2 approved (conditional on its
exact-head pipeline passing), the third necessary-scope correction
granted, and `-rc.2` construction authorized. Intent is to get you moving
without another round-trip.

## What changed since `-rc.1`, and why the naive path repeats the failure

The `-rc.1` OCI failure is real and deterministic: `CVE-2026-14456`
HIGH in `libcrypto3`/`libssl3`, `3.5.7-r0` installed, `3.5.8-r0` fixed.

**A base-image digest bump does not fix it.** Verified 2026-08-28 by
extracting each image's apk database: Alpine `3.22.5`, `3.23.5` and
`edge` all ship `3.5.7-r0`. There is no fixed image to point at. The
`v3.22/main` **package repository**, however, already serves `3.5.8-r0`
— the image snapshot merely lags the repo. Hence `apk upgrade` at build
time, which I verified locally produces `3.5.8-r0` for both packages.

## Two things that will bite if you skip them

**1. The pre-tag OCI scan is mandatory, not advisory.**
`agov:oci:release` is `if: $CI_COMMIT_TAG` only — it is the sole job that
builds and scans the image. There is no MR-stage container CVE coverage
at all. So a container CVE can only surface *after* the tag is immutable.
That is exactly how `-rc.1` died, and running the same sequence will kill
`-rc.2` the same way. Build and scan locally against the exact candidate
head before the tag, and put the result in Gate 2 evidence.

Note `045e514c` does **not** close this. It adds the Containerfile to
`agov-oci-image:selftest`'s path anchor so Containerfile changes stop
producing an empty pipeline — but that selftest validates target
resolution and neither builds nor scans. I was explicit about that in the
file and the commit rather than letting it read as coverage.

**2. Every inventory figure is stale.** The correction adds
`tooling/rust/Containerfile.agov` and `.gitlab-ci.yml`, both currently in
Section B. Section A/B/C, the path union, and both checkpoints must be
regenerated from scratch. Do not carry forward 45 paths, `58a2757b…`, or
`8a865ac3…` — all superseded.

## Bind to merge commits, not branch commits

The correction replays the exact SHAs **MR !2174 produces on `main`**,
not `85540db4`/`045e514c` as they stand on the branch. Per the
frozen-scope rule, allowlist entries are Gate-2-approved exact SHAs or
merge commits with a proven byte-identical first-parent patch. Use the
merge result and prove the byte-identity as you did for `960124e1` and
`acaec947`.

## Unchanged

Target `reconstruct/v2.7` stays at `25e6d752b4a7f6964fecec2d5daebd0a3d8c7660`
and is never constructed on. The Gate 1 plan (`hsj8`) governs unchanged —
this is its own `-rc.2` provision, not a new plan. All `qt1a` publication
requirements carry forward. Product scope remains exactly #86, #87, #88,
#90.

Stop at the exact-head Gate 2 handoff. The `-rc.2` tag needs its own
publication authorization; `qt1a` was bound to `-rc.1` and does not
extend.

Ping me on any plan ambiguity during construction — a defect found while
executing is cheaper as a superseding record than as a silent workaround.
