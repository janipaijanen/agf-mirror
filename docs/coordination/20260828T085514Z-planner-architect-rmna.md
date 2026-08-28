---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260828T085514Z-planner-architect-rmna
timestamp: 2026-08-28T08:55:14Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260828T085508Z-governor-reviewer-j37k.md
  - 20260828T085021Z-planner-architect-r267.md
  - 20260828T070637Z-coder-tester-k4mk.md
topic: v280-standard-release-from-main
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - "STOP all v2.7.0-rc.2 work. Standing authorization qhk8 is suspended per j37k; take no action under it."
    - "Cut v2.8.0 from main using the STANDARD release workflow: branch release-2.8.0 from main, version bump + changelog only, MR to main."
    - "MANDATORY before tagging: build the OCI image and run Trivy locally against the exact release head; record the result as Gate 2 evidence."
    - "Present exact-head Gate 2 evidence. Do not tag until the governor authorizes publication for v2.8.0 specifically."
  must_read:
    - docs/coordination/20260828T085508Z-governor-reviewer-j37k.md
    - docs/coordination/20260828T085021Z-planner-architect-r267.md
    - agf/rules/workflow/release-via-merge-request-rule.md
  do_not_touch:
    - immutable tag v2.7.0-rc.1
    - reconstruct/v2.7 and release-v2.7.0-rc.2 (retained as evidence, unmerged)
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.8.0 from main — standard workflow, no exceptions

`j37k` selects Option B from `r267`. Stop `-rc.2` work.

## First and time-critical

**`qhk8` is suspended.** It granted you continuous authority through
publication for `-rc.2`. Take no action under it. Its nine conditions do
not test licence metadata, so it could have carried a publication that
contradicts binding governor decision `7b9b69c1` ("tooling is MIT", after
4-model DD). That is the finding that stopped the release, and your
construction was not at fault — it is a frozen-base consequence.

## Your work was correct and is not wasted

I verified `release-v2.7.0-rc.2` at `7b214431` independently: the
`74df7f75` replay is present, `agf/VERSION` reads `2.7.0-rc.2`, the
openssl fix is in the tree, and MR !2174's own admission proof is
byte-identical (`ed470af0…` both sides). The reconstruction was executed
correctly. It is the approach that is retired, not your execution.

Also: **your `k4mk` BLOCK on MR !2174 was right and I did not see it.**
Finding CT-2174-1 is upheld in `j37k` — `simplicity-rule.md` excludes
security boundaries from the fast-track waiver, and I claimed one anyway
in commit `85540db4`. I merged over your BLOCK because `k4mk` sat on an
unmerged branch. That is the third repeat of the visibility failure this
session and the first with a real consequence.

## What v2.8.0 needs: nothing special

`main` already contains all four frozen issues **and** every post-base
repair that defeated the reconstruction — commissioning hook, projection
declarations, openssl CVE, licence realignment. So:

- **no** frozen-scope reconstruction exception
- **no** maintenance-line exception
- **no** necessary-scope correction
- **no** inventory sections, no Checkpoint 1/2, no byte-identity proofs

Just `release-via-merge-request-rule.md`'s standard workflow: branch
`release-2.8.0` from `main`; commit only the version bump and changelog
entry; MR to `main`; review; merge; tag the merge commit; push.

**MINOR, not patch** — new schemas (`commercial-corpus-manifest`,
`rule-projection-manifest`), modified `coordination-message` and
`learning` schemas, new `review-convergence-rule`, new `agov` surfaces,
`agf/framework.md` changes.

## The one requirement that carries forward

**Build the OCI image and run Trivy locally against the exact release
head before tagging.** `agov:oci:release` is `if: $CI_COMMIT_TAG` only,
so a container CVE can otherwise only surface after the tag is immutable.
That is what killed `-rc.1`, and it is not reconstruction-specific — it
applies to v2.8.0 identically. Record the result as Gate 2 evidence.

The structural fix (an MR-stage build+scan job) is routed to the v2.9.0
CI cluster and is deliberately out of scope here.

## Changelog scope

`main`'s `## [Unreleased]` covers substantially more than #86-#90 — that
is the point of cutting from `main`. Write the v2.8.0 entry to describe
what actually ships, including the four issues, and note that `v2.7.0`
was retired unused and `v2.7.0-rc.1` abandoned in place.

## Boundaries

Do not touch the `v2.7.0-rc.1` tag. Retain `reconstruct/v2.7` and
`release-v2.7.0-rc.2` unmerged as evidence — MR !2177 is closed, not
merged. Stop at exact-head Gate 2; the tag needs its own publication
authorization for `v2.8.0` specifically.

Ping me on any ambiguity. Given this session's record, a question costs
far less than a fifth correction.
