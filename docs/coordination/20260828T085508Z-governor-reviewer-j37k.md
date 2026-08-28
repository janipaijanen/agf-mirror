---
kind: decision-record
from: governor-reviewer
to: all
id: 20260828T085508Z-governor-reviewer-j37k
timestamp: 2026-08-28T08:55:08Z
model: human-directed-claude-sonnet-5
immutable: true
references-to:
  - 20260828T085021Z-planner-architect-r267.md
  - 20260828T080705Z-governor-reviewer-qhk8.md
  - 20260828T070826Z-governor-reviewer-m8ra.md
  - 20260827T192914Z-governor-reviewer-hsj8.md
topic: v270-reconstruction-abandoned-v280-from-main-authorized
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Binding: frozen-scope reconstruction abandoned; v2.8.0 cut from main

This record transcribes the governor-reviewer's in-session decision,
"bake the release right", given in response to the blocking finding and
the A/B recommendation in `r267`. **Option B is selected.**

## 1. Standing authorization `qhk8` is SUSPENDED — effective immediately

`qhk8` granted coder-tester continuous authority through construction,
merge, tag, publication and hold start for `v2.7.0-rc.2`. That authority
is **withdrawn in full**. Its nine conditions do not test licence
metadata, so it could have carried a publication that contradicts a
binding governor decision. No action may be taken under `qhk8`.

This is the first instruction in this record because it is the only one
that is time-critical.

## 2. The frozen-scope reconstruction is abandoned

Construction was correct throughout; this is not a coder-tester fault.
The approach itself is retired, for the reason `r267` gives: four
corrections, four-for-four post-base repairs the reconstruction excluded
by construction, each discovered later than the last and by a different
mechanism —

1. `6gg4` — commissioning-hook aarch64 drift (MR !1686);
2. `2h2s` — projection-manifest declarations, widened after pre-flight;
3. `m8ra` — openssl CVE-2026-14456 (MR !2174);
4. unfixed — tooling licence metadata (`7b9b69c1`).

Nothing establishes there is no fifth. The base is 963 commits stale and
the reverse-consumer enumeration `hgte` proposed has never been
performed. Continuing to patch is not the cheaper path.

Consequently:

- **MR !2177 is closed unmerged.** `release-v2.7.0-rc.2` and
  `reconstruct/v2.7` are retained as evidence, unmerged and untagged.
- **`v2.7.0` is retired unused.** It is never assigned to a stable
  release. `main` contains far more than the frozen four-issue scope, so
  publishing it as `v2.7.0` would misrepresent what `v2.7.0` was frozen
  to mean.
- **`v2.7.0-rc.1` remains immutable and abandoned in place**, exactly as
  `m8ra` set. Never moved, deleted, or reused.

## 3. v2.8.0 is authorized, cut from `main` by the STANDARD workflow

`main` already contains all four frozen issues (#86, #87, #88, #90) *and*
every post-base repair that defeated the reconstruction: the
commissioning-hook fix, the projection declarations, the openssl CVE fix,
and the licence realignment. It therefore requires **no exception of any
kind** — not the frozen-scope reconstruction exception, not the
maintenance-line exception, not a necessary-scope correction.

`agf/rules/workflow/release-via-merge-request-rule.md`'s standard
workflow applies unmodified: branch `release-2.8.0` from `main`, commit
only the version bump and changelog entry, open an MR to `main`, review,
merge, then tag the merge commit and push.

**MINOR is the correct increment**, not patch: the scope includes new
schemas (`commercial-corpus-manifest`, `rule-projection-manifest`),
modified `coordination-message` and `learning` schemas, a new
`review-convergence-rule`, new `agov` command surfaces, and
`agf/framework.md` changes.

## 4. Mandatory pre-tag OCI scan carries forward

The single requirement that survives from `qhk8` and `m8ra`: **build the
OCI image and run the Trivy scan locally against the exact release
candidate head before tagging**, and record the result as Gate 2
evidence. `agov:oci:release` is `if: $CI_COMMIT_TAG` only, so container
CVEs cannot otherwise surface before the tag is immutable. This is what
killed `v2.7.0-rc.1` and it is not specific to the reconstruction.

## 5. Corrections and findings to carry, not to lose

- `k4mk`'s finding **CT-2174-1 is upheld**: `simplicity-rule.md` excludes
  security boundaries from the fast-track waiver, and MR !2174's commit
  `85540db4` claimed one for a security fix. The claim was wrong. It is
  permanent history on `main`; `m8ra` supplied the real authorization
  after the fact. Record it in the reconstruction learning rather than
  correcting history.
- The **third repeat of the unmerged-coordination visibility failure**
  had a real consequence this time: `k4mk`'s BLOCK was not visible when
  MR !2174 was merged. Publish coordination branches promptly.
- The licence divergence is resolved by this decision rather than
  patched: `main` already carries `7b9b69c1`, so v2.8.0 ships the correct
  MIT tooling metadata with no correction needed.
- The **structural CI gap remains open**: no MR-stage job builds or scans
  the container. Routed to the v2.9.0 CI cluster per `nsne`'s roadmap
  direction; explicitly not attempted inside this release.

## Not authorized

No change to any existing tag. No deletion of `reconstruct/v2.7` or
`release-v2.7.0-rc.2`. No weakening of the Trivy gate. No scope beyond a
release-only version bump and changelog on `main`.
