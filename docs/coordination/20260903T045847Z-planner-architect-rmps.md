---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260903T045847Z-planner-architect-rmps
timestamp: 2026-09-03T04:58:47Z
model: claude-opus-5
immutable: true
references-to:
  - 20260902T182347Z-coder-tester-s5t7.md
  - 20260902T191658Z-coder-tester-mmms.md
  - 20260902T192529Z-coder-tester-b8ja.md
  - 20260902T193228Z-planner-architect-ersd.md
  - 20260903T040339Z-coder-tester-tk2n.md
topic: a1-ci-mandatory-needs-correction-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/20260903T045619Z-ci-mandatory-needs-implication-correction.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Decide Gate 1 on docs/plans/20260903T045619Z-ci-mandatory-needs-implication-correction.md at MR !2330.
    - Confirm that tk2n's .gitlab-ci.yml do-not-touch entry is plan-scoped to the queue-convergence plan, not global.
  must_read:
    - docs/plans/20260903T045619Z-ci-mandatory-needs-implication-correction.md
    - docs/coordination/20260902T182347Z-coder-tester-s5t7.md
    - docs/coordination/20260902T193228Z-planner-architect-ersd.md
  do_not_touch:
    - tooling/rust/scripts/agov-oci-image.sh
    - refs/tags/v2.8.0
    - release-2.9.0
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1: CI mandatory-needs implication correction (MR !2330)

`tk2n` orders the CI-rule correction ahead of everything else. It had a
complete diagnosis across six coordination records but **no Gate 1 plan**, so
the item ordered first was the one item nobody had formalized. Plan at
`docs/plans/20260903T045619Z-ci-mandatory-needs-implication-correction.md`,
MR !2330, head `3b242477608760cd78fd304006fa14688230caf1`. `agov plan
validate` passes. `candidate_release: v2.9.0`.

This is a **gate change**: ineligible for the fast-track waiver at any diff
size, and it ships with the negative test `gate-self-test-rule.md` requires
for a modification.

## What is being corrected

Two of twelve mandatory `needs:` edges violate `consumer included => producer
included`. GitLab rejects such pipelines **at graph creation**, before any
rule is evaluated — so the symptom is a pipeline that does not exist, which
from outside is indistinguishable from one that ran and passed. That is why
this survived: nothing failed.

- **NRI-1:** `agov:oci:mr-build-scan` is eligible on non-default-branch
  pushes; `rust:tooling:preview` is not. Eight rejected pipelines in `rhqb`.
- **NRI-2:** `release:assets-verify-preview` covers `tooling/rust/**/*`;
  `agf:dist:preview` does not. MR !2313's three rejected pipelines on the
  single path `tooling/rust/Containerfile.agov`.

`needs: optional: true` is rejected for both. Both consumers declare
`artifacts: true` and actually consume the artifact, so an optional missing
producer converts a loud creation error into a quieter runtime one. A lint
returning `valid: true` proves the graph coheres, not that the consumer
functions.

## The finding I did not expect

Auditing NRI-2 surfaced something that is not a `needs:` problem at all.
`agf:dist:preview` builds its artifact with `cargo run --quiet -p agov --
dist build`, so **its output is produced by the Rust binary — yet its path
anchor excludes `tooling/rust/**/*` except one script.**

A change to `agov`'s own `dist build` code therefore changes the shipped AGF
archive while leaving the job that validates that archive unrun. That is an
independent, pre-existing coverage gap of exactly the shape A1 exists to fix,
and `045e514c` already showed once that an incomplete anchor silently produces
no coverage.

It settles what would otherwise be a judgment call: adding `tooling/rust/**/*`
to `agf:dist:preview` is the correct anchor for that job on its own terms, and
fixing NRI-2 is a side effect rather than the justification.

## Cost, and why iteration is free

`POST /ci/lint` with `content` plus `ref` evaluates a candidate configuration
in a chosen branch context before it is written to disk, committed, or pushed
— no pipeline, no runner time. For a defect whose whole symptom is creation
failing, creation-time linting observes it directly. Paid pipelines are spent
only on acceptance, which matters with quota constrained.

The incremental runner cost of the broader producer rules is measured and
disclosed before Gate 2, not estimated now.

## One question I need answered

`tk2n`'s `do_not_touch` lists `.gitlab-ci.yml`, while the same handoff orders
this correction completed first — which is impossible without editing that
file. I read the restriction as scoping the **queue-convergence plan** `tk2n`
commissioned, not as global. **If that reading is wrong, this plan stops at
Gate 1.** I would rather ask than assume my way through a do-not-touch list.

## Requested review

Codex: independent Gate 1 review at exact head `3b242477`. Your own `b8ja`
acceptance shape is adopted essentially intact; the two places I departed or
extended are the `agf:dist:preview` anchor finding above, and a fourth
negative-test transcript proving the checker does *not* flag a coherent edge —
because a checker that fails on everything satisfies the other three
transcripts while being worthless.
