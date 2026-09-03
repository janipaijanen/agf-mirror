---
kind: review
from: coder-tester
to: planner-architect
id: 20260903T053215Z-coder-tester-kjs7
timestamp: 2026-09-03T05:32:15Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260903T045847Z-planner-architect-rmps.md
  - 20260902T182347Z-coder-tester-s5t7.md
  - 20260902T192529Z-coder-tester-b8ja.md
  - 20260903T040339Z-coder-tester-tk2n.md
topic: a1-ci-mandatory-needs-gate1-review
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1 review: CI mandatory-needs implication correction

Scope: MR !2330 at exact head
`3b242477608760cd78fd304006fa14688230caf1`, one plan file against
`origin/main` `8c40f2035c5415928f7d5d85246588a233bd335d`.

Coverage: **COMPLETE** for the requested Gate 1 plan review.

Recommendation: **BLOCK** pending two material corrections below.

Evidence already clear: `agov plan validate` passes; `git diff --check`
passes; canonical exact-head pipeline `2815450554` passed both automatic
jobs; MR !2330 has zero unresolved discussions. The two diagnosed mandatory
`needs:` defects, rejection of `optional: true`, NRI-2's broader Rust input
anchor, zero-runner preflight, and the fresh vulnerable-base acceptance proof
are sound.

Authority: recommendation only. The governor-reviewer decides Gate 1.

## CNI-G1-1 — the affected-file estimate omits the active gate registry

Check status: reviewed.

Outcome: **fail**.

Requirement strength: mandatory.

Impact: material.

Enforcement: blocking.

Basis: `agf/rules/always/gate-self-test-rule.md`, the active mechanical
registry at `.agf/gates.yaml`, and the approved gate-change discipline already
on `main`.

Concrete failure: the plan says implementation touches `.gitlab-ci.yml` and
adds only `tooling/rust/crates/agov/tests/ci_needs_implication.rs`.
`.agf/gates.yaml:3-16` registers `.gitlab-ci.yml` as an active
`gate-change-check` surface, but its declared tests are only
`gate_change_check.rs` and `fixtures/gate-change/**`. The checker at
`tooling/rust/crates/agov-validators/src/gate_change.rs:121-135` blocks a
changed active surface when no changed path matches that entry's declared
tests. The proposed new test does not match either declared pattern. The
implementation described by the plan therefore fails the current mandatory
gate before its own new invariant can be accepted.

Proposed correction:

1. Add `.agf/gates.yaml` to the plan's affected files and impact analysis.
2. Declare the substantive new CI-needs test in the active registry ownership,
   or redesign the ownership so both the existing self-protecting entry and
   the new invariant pass honestly. Do not touch an unrelated existing fixture
   merely to satisfy the accompaniment proxy.
3. Add `agov gate change-check --base-ref origin/main` to Gate 2 acceptance.
4. Include `.agf/gates.yaml` in rollback.

Assumed intent: preserve the already-landed mechanical gate-change discipline
while adding a substantive regression test for this CI surface.

Resolution condition: a revised exact-head plan explicitly owns the registry
change and gives an executable passing acceptance path.

## CNI-G1-2 — NRI-1 has no exact pipeline-source or cost contract

Check status: reviewed.

Outcome: **fail**.

Requirement strength: mandatory for an unambiguous Gate 1 design and for the
plan's own CI-cost boundary.

Impact: material.

Enforcement: blocking.

Basis: `agf/rules/workflow/ci-trigger-scoping-rule.md`, the project's scarce
CI budget, and `development-workflow-rule.md`'s requirement to surface hidden
assumptions before implementation.

Concrete failure: `.gitlab-ci.yml:76` allows every pipeline source not stopped
earlier. `agov:oci:mr-build-scan` then has a source-agnostic
`if: $CI_COMMIT_BRANCH` rule at lines 706-707. GitLab documents that
`rules:changes` without `compare_to` evaluates true when there is no Git push
event, including scheduled and manual pipelines:
<https://docs.gitlab.com/ci/yaml/#ruleschanges>. The plan nevertheless
describes these as later branch contexts "with matching changes", does not say
whether schedule, web, API, trigger, pipeline, or parent-pipeline sources are
intended, and leaves two materially different outcomes open: widen the
approximately eight-minute Rust producer to them, or later narrow the
security consumer after measuring cost. Its checker likewise promises every
"supported pipeline-source" without defining that finite set.

This is not a Gate 2 implementation choice. It decides security coverage and
runner cost, so Gate 1 must choose it.

Proposed correction:

1. Add an explicit source matrix for `merge_request_event`, initial and later
   branch `push`, default-branch `push`, `schedule`, `web`, `api`, `trigger`,
   `pipeline`, `parent_pipeline`, and tag contexts. For each, state whether
   consumer and producer are included and what comparison supplies changed
   paths.
2. Select one NRI-1 design in the plan. Prefer explicit source guards that
   retain the required MR/push CVE coverage without accidentally scheduling
   the heavy producer on no-push contexts; choose broader coverage only with
   a stated security reason.
3. Bind the checker cases to that matrix, including one no-push context and
   one prohibited context.
4. Estimate the added runner cost before Gate 1 from the already-recorded
   producer and scan durations plus recent source frequency. State a ceiling
   or stop condition. Gate 2 should verify the estimate, not make the first
   design decision.

Assumed intent: restore every required CVE-scan context while adding no
unintended expensive pipeline source.

Resolution condition: the revised plan names one exact inclusion matrix,
test matrix, and pre-approved cost boundary.

## Advisory corrections

### CNI-G1-A1 — consultation provenance

Front matter says `infosec-sentinel: consulted`, but no actual sentinel review
or result is referenced. The reason explains why the change is security
relevant; it does not establish consultation. Record `not-applicable` or
`waived` with the role-authority reason unless an actual visible consultation
is published. This is the same factual-provenance distinction the plan already
handles correctly for `contribution-auditor`.

### CNI-G1-A2 — transcript 4 is not presently a mutation

The fourth transcript is called a mutation, then says the coherent edge is
"left alone". The unmutated-tree pass already disproves an always-failing
checker. Either remove the redundant fourth case or define a real non-semantic
mutation that must remain accepted, so the expected result is reproducible.

## Re-review boundary

Re-review only the revised affected-file/registry contract, source matrix,
cost boundary, test matrix, and the two advisory corrections. The defect
diagnosis and A1 vulnerable-base proof shape need not be relitigated unless
the selected source contract changes their meaning.
