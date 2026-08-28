---
kind: decision-record
from: governor-reviewer
to: all
id: 20260828T070826Z-governor-reviewer-m8ra
timestamp: 2026-08-28T07:08:26Z
model: human-directed-claude-sonnet-5
immutable: true
references-to:
  - 20260828T061002Z-coder-tester-eqq3.md
  - 20260828T051652Z-governor-reviewer-qt1a.md
  - 20260827T192914Z-governor-reviewer-hsj8.md
topic: v270-rc2-authorized-with-third-scope-correction
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Binding: v2.7.0-rc.1 disposed, v2.7.0-rc.2 authorized, third scope correction granted

This record transcribes the governor-reviewer's in-session decision to
proceed to `-rc.2`, answering the blocked handoff `eqq3`.

## 1. Disposition of v2.7.0-rc.1

`v2.7.0-rc.1` is **abandoned in place, never reused**. Tag object
`eb9a59372755104097ca59c9b9cf456a398e83f0`, peeled
`25e6d752b4a7f6964fecec2d5daebd0a3d8c7660`, remains immutable and is
**never** moved, deleted, recreated, or force-updated
(`immutable-tags-rule.md`). Its publication is permanently incomplete:
Git tag present; GitLab Release, generic package, and OCI image absent —
independently re-verified. It is superseded by `v2.7.0-rc.2`. One RC
number is the correct price and is accepted.

The blocking `agov:oci:release` failure in tag pipeline `2798735953`
(job `16162519437`) is confirmed a genuine security finding, not
flakiness: `CVE-2026-14456`, HIGH, `libcrypto3`/`libssl3` at `3.5.7-r0`,
fixed `3.5.8-r0`. The gate behaved correctly and is **not** weakened,
waived, or retried.

## 2. Gate 2 for the remediation on `main` (MR !2174)

MR !2174 on branch `security/alpine-openssl-cve-2026-14456` receives
Gate 2 approval **conditional on its exact-head pipeline passing**. If
that pipeline fails, this approval lapses and the work returns to the
governor. It carries two commits:

- `85540db4` — adds `apk upgrade --no-cache` to `Containerfile.agov`.
  Re-pinning the base digest **cannot** remediate this finding: verified
  2026-08-28 that Alpine `3.22.5`, `3.23.5` and `edge` all ship
  `3.5.7-r0`, while the `v3.22/main` package repository already serves
  `3.5.8-r0`. Verified locally by building the pinned base with the
  change and reading the resulting apk database: `libcrypto3` and
  `libssl3` both `3.5.8-r0`. The digest pin is retained.
- `045e514c` — adds `tooling/rust/Containerfile.agov` to the
  `agov-oci-image:selftest` path anchor. A Containerfile-only change
  previously matched no job's `changes:` rule, producing an empty failed
  pipeline (`2798869592`, zero jobs) that `only_allow_merge_if_pipeline_
  succeeds` made unmergeable.

This fix is required on `main` on its own merits: `main` carried the
identical vulnerable pin, so every release was blocked, not only this one.

## 3. Third necessary-scope correction — granted

`tooling/rust/Containerfile.agov` and `.gitlab-ci.yml` are both in the
approved inventory's Section B. Replaying the MR !2174 remediation into
the reconstruction is granted as a **third** necessary correction under
the same rule and on the same basis as `6gg4` and `2h2s`: an objective
mandatory gate proves the approved RC outcome incomplete, and the exact
existing fix is the smallest complete correction. It changes neither the
four-issue product scope (#86, #87, #88, #90) nor any release interface,
dependency, security boundary, or gate authority.

Bound to the **exact merge-commit SHAs MR !2174 produces on `main`** —
not to the pre-merge branch commits, and not re-authored. Path union and
Section B membership must be regenerated and re-presented; the previous
45-path Section C figure no longer applies.

## 4. v2.7.0-rc.2 construction authorized

Per the approved plan's own `-rc.2` provision, construction proceeds on a
fresh `release-v2.7.0-rc.2` branch **from the same protected target
`reconstruct/v2.7`**, which is unchanged at
`25e6d752b4a7f6964fecec2d5daebd0a3d8c7660`. `reconstruct/v2.7` remains
the target and is never constructed on. The approved Gate 1 plan
(`hsj8`) governs unchanged; this is its `-rc.2` branch, not a new plan.

Release metadata moves `2.7.0-rc.1` → `2.7.0-rc.2` across the same
governed surface list, including both changelogs' section heading. All
`qt1a` publication requirements carry forward unchanged.

## 5. Mandatory addition: pre-tag OCI build and scan

**The one new requirement, and it is not optional.** Before the
`v2.7.0-rc.2` tag is created, build the OCI image from the candidate and
run the Trivy scan **locally against the exact candidate head**, and
record the result as Gate 2 evidence.

Reason: `agov:oci:release` — the only job that builds and scans the
image — is `if: $CI_COMMIT_TAG` only. There is no MR-stage container CVE
coverage, so this class of finding can otherwise surface only after the
tag is immutable. That is precisely what burned `-rc.1`, and repeating
the sequence would burn `-rc.2` identically. `045e514c` does **not**
close this: the selftest validates target resolution and neither builds
nor scans.

This extends the standing local pre-flight instruction from `qt1a`,
which already requires the complete governance job before any push, to
the OCI build and scan before the tag. The structural CI fix is routed
to the v2.9.0 CI cluster and is deliberately **not** attempted inside
this release.

## Not authorized

No change to the `-rc.1` tag or its artifacts. No weakening, waiving, or
retrying of the Trivy gate. No stable `v2.7.0` promotion — that still
requires its own binding candidate record and Gate 1 plan after an
accepted candidate completes its 48-hour hold. No product-scope change
beyond #86, #87, #88, #90.

Any stop condition in
`docs/plans/20260827T160105Z-v270-release-publication.md`, any namespace
non-absence, or any scan finding returns to the governor rather than
proceeding under assumed authority.
