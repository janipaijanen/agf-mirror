---
id: 20261029T185703Z-dist-profile-gate-test-amendment
status: pending-gate1
topic: dist-profile-shared-command-gate-tests
affected_paths:
  - tooling/rust/crates/agov/tests/gate_change_check.rs
  - tooling/rust/crates/agov/tests/plan_collision.rs
depends_on:
  - 20260920T083259Z-distribution-scan-profiling-slice-b
  - 20260914T130217Z-row6-s4-issue123-collision-check-blocking-promotion
  - 20260915T075705Z-issue130-pending-advisory-consultation-status
  - 20261021T204757Z-remove-dind-ci
  - 20261021T220000Z-retire-windows-support
  - 20261029T183000Z-ci-pins-gate-test-amendment
advisory_consultation:
  - role: contribution-auditor
    status: pending
    reason: Independent review requested before Gate 1 approval.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Test-only accompaniment. No filesystem containment policy, credentials,
      network calls, scanner behavior, runtime instrumentation or gate changes.
      Parent profiling boundaries and CI-pin security blockers remain intact.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Distribution profiling: required shared-command gate tests

## Evidence and boundary

The approved parent profiling plan changes `tooling/rust/crates/agov/src/main.rs`
to expose feature-gated `agov dist diagnose`. The live `.agf/gates.yaml` assigns
that file to `gate-change-check` and `plan-collision-block`. Both gates require
their own declared test accompaniment. The parent omits those test paths; its
distribution-specific tests cannot substitute for them. The same omission
caused the confirmed CI-pin pipeline failure recorded in gkq5.

This amendment adds only the two listed integration-test files to the parent
implementation's allowed scope. It changes no gate, registry, command behavior,
feature, instrumentation, fixture policy, dependency, release scope or security
decision. It cannot authorize implementation while pending Gate 1.

## Test design

Reuse existing repository fixtures and command probes; inspect current coverage
before editing and extend a relevant case where possible.

1. In `gate_change_check.rs`, model an additive feature-gated diagnostic dispatch
   change on the registered `main.rs` surface. A candidate carrying only its
   own diagnostic tests must fail for missing declared gate companions. Adding
   the required companions must pass. A feature being absent from default builds
   must not bypass shared-surface evidence requirements.
2. In `plan_collision.rs`, extend the existing negative overlap/acknowledgement
   coverage to two live plans sharing the literal `main.rs` path. With no direct
   dependency the changed-plan blocking mode must fail; acknowledging the other
   plan must resolve that overlap. Preserve the live-registry assertion that
   `main.rs` remains a `plan-collision-block` surface.

Both changes must be substantive tests, not comment-only accompaniment. Retain
all existing negative controls, including CI-pin controls if present in the
implementation base. Do not alter `.agf/gates.yaml` or gate implementation code.

## Collision and ownership reconciliation

The four historical shared-test plans are acknowledged directly: Row 6 S4
established blocking collision and wiring controls; issue #130 added the pending
consultation fixture; OCI runner migration and Windows retirement supplied
shared-surface accompaniment. Their landed controls must remain intact.

The CI-pin test amendment is also acknowledged directly. Its reviewed test delta
is published in !3358 at `8687df2e0fe3c1434d7a678d415546711e1a14c5`, while its
image findings keep that MR unmerged. Codex owns that implementation lane after
Claude's published handoff. This plan neither releases nor reassigns it.
Profiling changes may use an isolated branch from current main; they must avoid
editing CI-pin additions and must reconcile both test deltas before either final
acceptance against a changed target. No CI-pin image changes may be carried into
the profiling MR merely to obtain its test additions. If reconciliation reveals
an actual semantic conflict or another live claim, stop that mutation and resolve
ownership/scope first.

## Acceptance

- Validate this plan and run changed-plan collision checking against current
  main; reconcile every unacknowledged overlap before Gate 1.
- Independent contribution review must be clear and governor approval published
  before editing the two test files.
- Run both integration-test targets with the parent implementation, preserving
  actual failure and success assertions for the negative controls.
- Run `agov gate change-check --base-ref origin/main` on the committed candidate.
- Preserve every parent acceptance requirement: default-build correctness and
  absence of the diagnostic command, feature-build counters, deterministic
  tracked/untracked/hidden/in-root-symlink fixture and reproducible baseline.
- Require independent implementation review and a successful pipeline at the
  submitted head before Gate 2. This amendment adds no optimization, caching,
  performance claim, outside-resolving symlink policy or CI diagnostic job.

Rollback removes only this test delta and leaves the parent implementation
unmergeable until its registered gate evidence is satisfied. No adopter-facing
file, schema or default runtime behavior changes in this amendment.
