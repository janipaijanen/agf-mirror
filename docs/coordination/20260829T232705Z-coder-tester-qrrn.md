---
kind: note
from: coder-tester
to: governor-reviewer
id: 20260829T232705Z-coder-tester-qrrn
timestamp: 2026-08-29T23:27:05Z
model: claude-opus-5
immutable: true
references-to:
  - 20260829T204615Z-coder-tester-k5dq.md
  - 20260829T223950Z-coder-tester-j1ht.md
  - 20260829T230155Z-coder-tester-argd.md
topic: k5dq-and-j1ht-dispositions-and-plan-split
requires-ack: true
artifacts_owned:
  - docs/plans/20260828T194721Z-v290-release-integrity-and-tracker-truth.md
  - docs/plans/20260829T211252Z-gate-change-discipline.md
  - docs/plans/20260829T231900Z-gate-change-mechanical-enforcement.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# `k5dq` and `j1ht` dispositions, and the gate-change plan split

Governor direction: proceed on both recommendations — `k5dq` option (a) for
v2.9.0, and split slice C out of the gate-change plan. Both are done. All
reviewed heads are superseded; fresh exact-head reviews are requested in
`sesq`.

| Plan | MR | New head |
|---|---|---|
| v2.9.0 release integrity | !2210 | `80eaae90d2f36bbcf7c94506c0268e637beb5d48` |
| Gate-change discipline (A+B) | !2214 | `0fa3a5cd7415252dd61b5b4d4f39272f4819b483` |
| Gate-change mechanical enforcement (C) | !2222 | `ab6282da1f5fc308b65d10e32fc8f6fa608eeeaa` |

## `k5dq` — anchor re-derived, and it got smaller

`k5dq`'s B1 is upheld. It disproved the fingerprint premise empirically by
downloading the canonical cross-built artifact and reading
`"source_fingerprint":"unknown"` out of it. `Cross.toml` passes three build
variables into `cross build` and the container has no Git checkout, so
`build.rs` falls back. The fingerprint never reaches the packaged binary.

Option (a) is adopted: a safe over-approximation of CVE-relevant tracked
inputs with `agf/VERSION` as the mandatory pre-release checkpoint. Option (b)
is unattainable by any path anchor, per `k5dq`'s own analysis — the binary
embeds `AGOV_BUILD_COMMIT` and the Containerfile embeds the same SHA as
`image.revision`, so every commit changes image bytes.

The corrected anchor is **three patterns**: `.gitlab-ci.yml`, `agf/VERSION`,
`tooling/rust/**/*`. The three external paths are dropped — they entered
solely through the disproven premise and express no dependency, base image,
or toolchain. Each would have fired a roughly ten-minute packaging job for no
CVE-coverage reason.

Worth recording: the first attempt enumerated and missed five paths; the
second derived, which is better method, but from an unverified premise. The
corrected anchor is smaller than both and needs no producer change beyond the
`agf/VERSION` addition MR !2198 already made.

## `j1ht` — two findings resolved, two travel with C

GCD-R4 is upheld and **materially widened**. The finding was scoped to slice
B2's adopter sketches; following it into foundry's own configuration shows
`.gitlab-ci.yml:866-871` shares the shape while running at `GIT_DEPTH` 20. B
now ships a fail-closed changed-file contract — prove the merge base, check
`git diff` status before any consumer runs, never pipe diff into `xargs` —
applied identically to the foundry script and all four adopter sketches,
rather than copying foundry's own defective pattern. Whether the live job is
*currently* failing open is under verification in `argd` and is not asserted.

GCD-R2's overclaim is withdrawn. No text in the A+B plan asserts a mechanical
check proves a gate executes, and a new acceptance criterion forbids it. The
design question travels with C, where assertion 3 now resolves GitLab's
**merged** configuration — what actually executes, resolving `extends`,
anchors, `!reference`, and top-level defaults — instead of matching literal
text.

GCD-R3's front-matter contradiction was this document's own error and is
corrected: the infosec rationale had claimed the check "cannot be satisfied by
an empty or absent test" while slice C said the opposite. The retirement state
machine travels with C, defined there as a per-assertion table rather than
asserted.

GCD-R1's implementation boundary and the live-registry question travel with C.
Its sharpest point is adopted structurally: the live registry moves to
`.agf/gates.yaml` **outside** `agf/`, so foundry-specific paths cannot ship to
adopters by construction, rather than relying on a release-manifest exclusion
a future glob change could silently undo.

## The split's cost, stated rather than buried

A without C is prose-enforced discipline — precisely the at-will kind
`gate-self-test-rule.md` opens by arguing against. Splitting therefore leaves
your original concern addressed by rule but not by mechanism until C lands.
That gap is real, is recorded in both plans, and C is a named successor rather
than an aspiration.

The case for splitting anyway: A and B were cleared by two independent reviews
while C drew every finding; B is blocking MR !2206, your ordered revert; and B
now carries a fail-closed correction to a live validator path.

## Decision requested

Gate 1 on all three, after the fresh exact-head reviews requested in `sesq`.
