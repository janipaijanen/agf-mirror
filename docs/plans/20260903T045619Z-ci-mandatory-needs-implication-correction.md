---
id: 20260903T045619Z-ci-mandatory-needs-implication-correction
status: pending-gate1
topic: ci-mandatory-needs-implication-correction
candidate_release: v2.9.0
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      This correction exists to restore a security control. A1's pre-tag
      container CVE scan currently cannot run on the change class it was
      built for, because pipeline creation fails before any rule is
      evaluated. The defect is therefore a silent absence of vulnerability
      coverage, not a cosmetic CI issue. The correction adds no credential,
      network, or data-flow surface; it changes job eligibility so an
      existing scan can execute. If the restored scan then reports a CVE,
      the release stops rather than proceeding.
  - role: contribution-auditor
    status: waived
    reason: >-
      No contribution-auditor disposition exists for this plan and the
      governed enum has no pending state. Recorded as waived rather than
      consulted so the metadata does not assert an audit that has not
      happened. This is gate mechanics, so an independent review is
      requested regardless; the governor may hold Gate 1 for an auditor
      pass.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# CI mandatory-needs implication correction

Discharges the blocker `tk2n` orders ahead of everything else: A1's container
CVE scan cannot fire on a Containerfile-only change, so the v2.9.0 release
path is stopped.

**This is a gate change.** It is ineligible for the fast-track waiver
regardless of diff size, per `simplicity-rule.md`, and it ships with the
negative test `always/gate-self-test-rule.md` requires for a modification —
demonstrated firing on known-bad input after the change, not before.

## The defect class

The invariant that `.gitlab-ci.yml` currently violates:

> In every pipeline source and changed-path class where consumer `C` is
> included, every non-optional producer `P` in `C.needs` must also be
> included.

When it is violated, GitLab rejects the pipeline **at graph creation**, before
any `rules:` block is evaluated and before any job runs. The observable
symptom is a pipeline that does not exist — which is indistinguishable, from
the outside, from a pipeline that ran and passed. That is why the gap survived:
nothing failed.

Twelve explicit mandatory `needs:` edges across six consumers were audited at
`origin/main` `86e92ad9` (`s5t7`, corrected from eleven to twelve by `mmms`).
Four consumers are coherent — `release:create`, `release:verify`,
`release:assets-verify-pre-oci`, and `agov:oci:release` are all tag-only on
both sides. Two fail.

### NRI-1 — `agov:oci:mr-build-scan` outruns `rust:tooling:preview`

The scan is eligible in merge-request pipelines, the first-push
non-default-branch fallback, and later branch contexts with matching changes.
Its producer `rust:tooling:preview` is eligible only for
`merge_request_event` or the default branch. Every non-default-branch push
context therefore includes the consumer and excludes the producer.

Already reproduced: `rhqb` records eight API-created non-default-branch
pipelines rejected for exactly this.

### NRI-2 — `release:assets-verify-preview` outruns `agf:dist:preview`

The verifier's path set is `tooling/rust/**/*` plus `.gitlab-ci.yml`.
`agf:dist:preview`'s is `agf/**/*`, `.gitlab-ci.yml`, two review documents,
and the single file `tooling/rust/scripts/check-release-self-sufficiency.sh`.
A change to `tooling/rust/Containerfile.agov` alone includes the verifier and
excludes the AGF producer.

Already reproduced: MR !2313's three attempted pipelines, all rejected at
graph creation on that exact single path (`k6m3`).

## A finding the graph analysis surfaced, and which outlives it

`agf:dist:preview` builds its artifact with `cargo run --quiet -p agov --
dist validate` and `dist build` (`.gitlab-ci.yml:423-424`). **Its output is
produced by the Rust binary, yet its path anchor excludes `tooling/rust/**/*`
except one script.**

So a change to `agov`'s own `dist build` implementation changes the shipped
AGF archive while leaving the job that validates that archive unrun. This is
not a consequence of the `needs:` mismatch — it is an independent,
pre-existing coverage gap in the same file, of exactly the shape A1 itself
exists to fix: an incomplete anchor silently produces no coverage, as
`045e514c` already demonstrated once.

It matters for the correction because it settles a choice that would otherwise
be a judgment call. Adding `tooling/rust/**/*` to `agf:dist:preview` is not a
graph-parity hack that happens to fix NRI-2 — it is the semantically correct
anchor for that job on its own terms, and fixing NRI-2 is a side effect of
correcting it.

## Correction: producer parity, not `optional: true`

`needs: optional: true` is rejected for both defects, and the reason is
semantic rather than stylistic. Both consumers declare `artifacts: true` and
consume the producer's artifact —`agov-oci-image.sh build-unpublished`
requires the packaged Rust archive; `release-assets.sh verify-pre-oci`
requires the complete AGF **and** Rust artifact set. An optional missing
producer would let the graph build and then fail on absent inputs, converting
a loud creation error into a quieter runtime one. A lint returning
`valid: true` would prove the graph coheres, not that the consumer functions.

| Defect | Correction | Basis |
|---|---|---|
| NRI-1 | Extend `rust:tooling:preview`'s rules to cover every branch context in which `agov:oci:mr-build-scan` is intended to run | Producer inclusion becomes a superset of intended consumer inclusion |
| NRI-2 | Add `tooling/rust/**/*` to `agf:dist:preview`'s `changes:` in both its MR and default-branch rules | Corrects that job's own anchor; NRI-2 parity follows |

If NRI-1's cost proves disproportionate, the alternative is to **narrow the
consumer's contexts deliberately**, with an explicit written statement of what
scan coverage is given up. Silently dropping coverage is not an option — that
is the defect, not the fix.

## Zero-cost preflight

`POST /ci/lint` accepts `content` plus `ref`, so a candidate configuration is
evaluated in a chosen branch context **before it is written to disk,
committed, or pushed**, consuming no runner time (`ersd`, confirming and
extending `b8ja`'s GET form):

```sh
glab api --method POST projects/83321848/ci/lint \
  -f content="$(cat .gitlab-ci.yml)" \
  -f dry_run=true \
  -f ref=<branch>
```

For a defect whose entire symptom is pipeline creation failing, creation-time
linting observes it directly. Verified today: current content at
`ref=docs/site-adoption-rust-tooling` returns the NRI-1 error with `jobs: 0`;
the same content at `ref=main` returns `valid: true` with 24 jobs.

This is the preflight, not the acceptance evidence. It cannot discharge the
semantic question above, and it cannot reproduce merge-request context.

## The negative test

Per `gate-self-test-rule.md`, the gate must be shown firing on known-bad input
**after** the change. Reusing the pre-change reproductions is explicitly
forbidden by that rule.

A deterministic checker asserts `consumer inclusion => producer inclusion`
across all twelve mandatory edges and every supported pipeline-source and
path class. It lives as a Rust test at
`tooling/rust/crates/agov/tests/ci_needs_implication.rs`, following the
existing precedent of `roi_redaction_audit_matches_real_clap_surface`, which
asserts an invariant against the real artifact rather than a copy of it. No
new tool, script convention, or CI job type is introduced.

Required transcripts, all four visible in the MR description:

1. **NRI-1 mutation** — revert `rust:tooling:preview`'s branch coverage; the
   checker must fail naming that edge.
2. **NRI-2 mutation** — remove `tooling/rust/**/*` from `agf:dist:preview`;
   the checker must fail naming that edge.
3. **Unmutated tree** — the checker passes.
4. **A mutation the checker must *not* flag** — an unrelated coherent edge
   left alone, proving the checker is not simply always-failing.

Transcript 4 exists because a checker that fails on everything satisfies
transcripts 1-3 while being worthless.

## Acceptance for A1

Graph correction alone does not discharge A1. Its acceptance criterion 1
requires a fresh vulnerable-base merge request that:

1. **constructs a valid pipeline graph** — proving this correction worked; and
2. **fails specifically at the Trivy gate** — proving the scan ran and caught
   a real CVE.

A pipeline that fails at creation proves nothing about the scan. The prior
attempt reverted `Containerfile.agov` to `alpine:3.22.1` for this purpose and
never produced a pipeline at all.

## Cost

The incremental runner cost of NRI-1's broader `rust:tooling:preview`
eligibility and NRI-2's broader `agf:dist:preview` eligibility is measured and
disclosed before Gate 2, not estimated at Gate 1. CI quota is currently
constrained, which is a further reason the preflight loop above carries the
iteration and paid pipelines are spent only on acceptance.

`ci-trigger-scoping-rule.md` cuts toward inclusion here: a lockfile or build
input wrongly scoped out costs a missed regression, and its own guidance is
"when in doubt, include the path."

## Scope boundary

Touches `.gitlab-ci.yml` and adds one test file. Does not touch
`tooling/rust/scripts/agov-oci-image.sh`, `refs/tags/v2.8.0`, or
`release-2.9.0`, all of which `tk2n` places off limits.

`tk2n` also lists `.gitlab-ci.yml` as do-not-touch. That list scopes the
**queue-convergence plan** `tk2n` commissioned; the same handoff orders the
CI-rule correction completed first, which is not possible without editing that
file. This plan reads the restriction as plan-scoped rather than global. If
that reading is wrong, this plan stops at Gate 1.

## Test and validation strategy

1. `agov plan validate` on this plan — passing at submission.
2. CI Lint preflight for every candidate, in both MR and branch contexts, with
   no pipeline created.
3. The four negative-test transcripts above.
4. One canonical exact-head pipeline on the correction MR.
5. The vulnerable-base MR discharging A1 acceptance criterion 1.
6. Cost measurement recorded in the MR description.

## Risks

- **Broader producer rules raise CI cost while quota is constrained.**
  Mitigated by the zero-cost preflight absorbing iteration, and by measuring
  before Gate 2 rather than asserting.
- **The checker encodes today's twelve edges and silently rots.** Mitigated by
  asserting against the real parsed `.gitlab-ci.yml`, so a thirteenth edge is
  seen rather than missed. Transcript 4 guards the opposite failure.
- **Correcting `agf:dist:preview`'s anchor surfaces pre-existing failures.**
  Possible, and would be a real finding rather than a regression — the job has
  not been running on Rust changes that affect its output. Any such failure
  returns to Gate 1 rather than being absorbed here.

## Rollback

Revert the `.gitlab-ci.yml` rules changes and delete the test file. No schema,
tag, release artifact, or external state is mutated.

## Evidence

- `docs/coordination/20260902T090221Z-planner-architect-at7q.md` — A1 scan does not fire on a Containerfile change
- `docs/coordination/20260902T181632Z-coder-tester-k6m3.md` — pipeline-creation root cause, MR !2313 transcripts
- `docs/coordination/20260902T182111Z-planner-architect-m2nh.md` — mechanism confirmed, and my own overclaim corrected
- `docs/coordination/20260902T182347Z-coder-tester-s5t7.md` — full twelve-edge audit, NRI-1 and NRI-2
- `docs/coordination/20260902T191658Z-coder-tester-mmms.md` — edge-count erratum
- `docs/coordination/20260902T192529Z-coder-tester-b8ja.md` — no-runner preflight and acceptance shape
- `docs/coordination/20260902T193228Z-planner-architect-ersd.md` — POST /ci/lint content+ref, verified
- `docs/coordination/20260903T040339Z-coder-tester-tk2n.md` — ordering constraint
- `.gitlab-ci.yml:418-449, 665-700, 715-736, 1453-1478` — the four jobs, read at `013c4a04`
