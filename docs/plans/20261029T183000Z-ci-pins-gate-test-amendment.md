---
id: 20261029T183000Z-ci-pins-gate-test-amendment
status: approved
gate1_approved_by: 20261029T183449Z-governor-reviewer-rqka
gate1_approved_at: 2026-10-29T18:34:49Z
topic: ci-pins-shared-gate-negative-test-accompaniment
affected_paths:
  - tooling/rust/crates/agov/tests/gate_change_check.rs
  - tooling/rust/crates/agov/tests/ci_needs_implication.rs
  - tooling/rust/crates/agov/tests/plan_collision.rs
depends_on:
  - 20261028T194451Z-ci-reproducibility-pins
  - 20260914T130217Z-row6-s4-issue123-collision-check-blocking-promotion
  - 20260915T075705Z-issue130-pending-advisory-consultation-status
  - 20261021T204757Z-remove-dind-ci
  - 20261021T220000Z-retire-windows-support
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Independent contribution audit first blocked the missing direct
      acknowledgements of four overlapping plans. Focused re-review at
      b32566bd34d710288b65ce074d81dbf275b1a1c3 returned CLEAR, with no blocking
      or advisory findings, after all four dependencies and their landed
      test coverage were reconciled. Scope and security boundaries unchanged.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Test-only amendment. No credentials, scanner behavior, image selection,
      CI configuration or risk acceptance changes. The parent plan's
      image-isolation and vulnerability dispositions remain mandatory.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# CI pins: add required shared-gate test coverage

## Evidence and intent

MR !3358 at `516142b2ff00faca3ec10784c3585119f99c8030` changes the
registered `.gitlab-ci.yml` surface. Pipeline `3005299125`, job `17585559090`,
correctly rejects missing declared test changes for `gate-change-check`,
`ci-needs-implication` and `plan-collision-block`. Review
`20261029T182725Z-coder-tester-gkq5` records the failure. The parent plan omitted
these three required test paths. Its new shell checker tests do not replace
the registered gates' negative controls.

This child amendment authorizes only the three listed test paths, after Gate 1.
Its changes accompany the parent implementation in MR !3358; they are not a
separate gate-behavior change. The parent's original three implementation paths
and all acceptance requirements remain governed by its existing approval.

## Smallest complete correction

### Collision reconciliation

The four overlapping approved plans are directly acknowledged above; the
parent's dependencies are not treated as transitive acknowledgement. Their
landed coverage was inspected on main `ee2a768a`: Row 6 S4 supplies blocking
collision and gate-wiring controls across all three files; issue #130 adds
the pending-consultation collision fixture; the OCI runner migration supplies
Podman/DinD pool assertions; Windows retirement supplies current supported-job
and packaging assertions. Those controls remain intact. This amendment adds
CI-pin coverage after their landed changes and claims no overlapping live
implementation ownership.

### Test changes

Reuse existing fixture builders, live-CI parsers and command probes. Inspect
current coverage before adding a test; extend an existing relevant case when
that supplies the same proof without duplication.

1. In `gate_change_check.rs`, exercise an advisory CI-job addition on the
   shared surface with only its own new shell test: the gate must reject the
   missing declared companion. Adding the declared companion must pass. This
   demonstrates that advisory-only status cannot bypass shared gate evidence.
2. In `ci_needs_implication.rs`, assert the actual `ci:pins:advisory` job keeps
   `allow_failure: true`, runs both checker and self-test, and introduces no
   required edge to an absent producer. Retain/re-run the existing deliberately
   broken needs-edge negative control after the live-CI change. Add a negative
   mutation for the advisory flag so removing it is detected by the test.
3. In `plan_collision.rs`, extend the live-CI collision-gate coverage with a
   negative fixture of two independent live plans owning `.gitlab-ci.yml`:
   unacknowledged overlap must block, and explicit dependency acknowledgement
   must resolve that pair. Preserve the gate job and self-test wiring checks.

All three files must contain substantive test coverage or a justified extension
of existing coverage. Whitespace-only or comment-only accompaniment is not the
acceptance condition of this amendment. No new dependency or helper framework.

## Validation and acceptance

- Validate this plan and reconcile all live overlapping plans before approval.
- Run the three integration test targets against the corrected implementation,
  retaining command, source SHA and actual negative-control results.
- Run `tests/ci-pins-selftest.sh` and verify all 13 inherited cases still pass.
- Run `agov gate change-check --base-ref origin/main` on the complete candidate.
- Preserve the parent's advisory checker exit semantics and CI allow-failure
  boundary; do not edit `.agf/gates.yaml` or any enforcement code.
- Update the MR's Negative test section with actual results, then require its
  full pipeline to pass at the submitted SHA and independent Gate 2 review.

No image finding is resolved or accepted by passing these tests. The 72 HIGH
and 4 CRITICAL package findings recorded in !3358 remain a separate merge
blocker until fixed or explicitly dispositioned under the parent plan.

## Authority and release boundary

This is a bounded correction to the already-approved v2.13 CI-pin item, not a
new release feature. No version/tag/publication, registry change, security
waiver, gate weakening, or alternative image-building workflow is authorized.
Independent review must precede approval; implementation cannot begin while
this amendment is pending. Rollback before merge removes this test-only delta
and leaves !3358 blocked; rollback after merge uses an ordinary reviewed revert.

## Downstream impact

Foundry tests only. No adopter-facing file, schema, binary behavior or migration
changes. The observed pipeline failure is empirical evidence of this coverage
gap; it is not evidence that the gates themselves are defective.
