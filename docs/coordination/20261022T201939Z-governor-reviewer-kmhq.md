---
kind: decision-record
from: governor-reviewer
to: planner-architect,coder-tester
id: 20261022T201939Z-governor-reviewer-kmhq
timestamp: 2026-10-22T20:19:39Z
model: human-via-codex
immutable: true
references-to:
  - 20261022T200907Z-improvement-propeller-383m
  - 20261022T201635Z-improvement-propeller-dr67
  - 20261022T185300Z-planner-architect-t4pe
  - 20261022T185703Z-coder-tester-4jsj
  - 20261022T194618Z-improvement-propeller-hb8s
topic: ratify-narrowed-mr3102-allocation-and-scope
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Decision: approve the narrowed MR !3102 path

The governor explicitly authorized Codex to file this record on the
governor's behalf after stating **"approved. proceed."** The condition in
`dr67` is satisfied: Claude confirmed `383m` exactly as written.

The governor ratifies `383m` and makes these decisions binding:

1. **Temporary allocation.** Coder-tester (Codex) operates and pushes MRs
   !3102 and !3105. Planner-architect acting as build-engineer (Claude)
   consults read-only. Neither agent pushes to the other's owned branch.
2. **Narrowed MR !3102 scope.** Move only `release:verify` and
   `rust:tooling:oci-pull-test` to the `podman,linux,amd64` runner pool.
   Keep `agov:oci:mr-build-scan` and `agov:oci:release` on DinD because their
   current scan path starts the digest-pinned Trivy container with nested
   `podman run`.
3. **No scanner refactor.** A different Trivy execution path is outside this
   decision and requires a separate plan and equivalence evidence.
4. **Plan amendment.** Planner-architect may update plan !3100 to reflect the
   corrected capability boundary and narrowed implementation. The amended
   affected paths must include the declared test companions required when
   `.gitlab-ci.yml` changes:
   `tooling/rust/crates/agov/tests/gate_change_check.rs` and
   `tooling/rust/crates/agov/tests/plan_collision.rs`, in addition to the
   routing test and other already declared implementation paths. This
   decision supplies Gate 1 approval for that exact amendment; it does not
   authorize broader scope.
5. **Acceptance.** Remove the temporary vulnerable scan fixture from the
   acceptance candidate. Obtain a green exact-head pipeline with meaningful
   tests for the narrowed routing and the declared gate companions, then
   request a separate Gate 2 decision.

MR !3105 remains ordered after !3102 under decision `382s`. This record does
not authorize Gate 2, a merge, a scanner-execution redesign, or any
irreversible action.
