---
id: 20261029T192050Z-dist-profile-roi-amendment
status: pending-gate1
topic: dist-profile-telemetry-vocabulary-parity
affected_paths:
  - tooling/rust/crates/agov/Cargo.toml
  - tooling/rust/crates/agov-core/Cargo.toml
  - tooling/rust/crates/agov-core/src/roi.rs
depends_on:
  - 20260920T083259Z-distribution-scan-profiling-slice-b
  - 20261029T185703Z-dist-profile-gate-test-amendment
  - 20261023T083302Z-remove-elixir-leftovers
advisory_consultation:
  - role: contribution-auditor
    status: pending
    reason: Independent review requested before approval.
  - role: infosec-sentinel
    status: pending
    reason: Review the minimal feature-gated telemetry vocabulary change and retained redaction boundary.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Profiling: keep the telemetry vocabulary consistent with the feature build

## Observed blocker

Profiling implementation commit `10dfbaa4fd86e7bbb00d76c08733e121ae769d1b`
passes the full default `agov` suite (497 tests) and targeted feature tests.
The full feature suite fails
`tests::roi_redaction_audit_matches_real_clap_surface`: its actual command
inventory contains `diagnose`, absent from `agov_core::roi::COMMAND_PATH_TOKENS`.
Inspection also finds new flag `--json` absent from `KNOWN_FLAG_NAMES`; the audit
stops at the command mismatch first. The implementer stopped before editing
these out-of-scope core paths. The audit is working as intended.

The parent already owns `agov/Cargo.toml`. This child adds the two core paths
and explicitly declares the feature-forwarding change in the parent path.
No implementation is authorized until governor Gate 1 approval is published.

## Minimal correction

- Define an empty optional `dist-profile` feature in `agov-core/Cargo.toml`.
- Forward `agov`'s existing `dist-profile` feature to `agov-core/dist-profile`.
- Add only the literal command token `diagnose` and flag name `--json` to the
  appropriate core lists, guarded by that feature. Keep default feature sets
  empty and do not add dependencies.
- Add focused tests in `roi.rs` for default/feature vocabulary presence and
  diagnostic argument redaction. Manifest paths, arbitrary positional values,
  unknown flags and their values must stay redacted. `--json` is a boolean flag,
  not a new closed-value allowance. Preserve `CLOSED_VALUE_FLAGS`, workflows,
  telemetry schema, MCP allowlist and logging destinations unchanged.
- Preserve the full bidirectional clap/vocabulary audit. Do not skip it,
  remove its stale-entry assertions, or add unconditional feature-only names.

Supported profiling builds enable the feature through the `agov` package; the
parent's default and feature builds must both pass. Check workspace all-features
compatibility and document any Cargo feature-unification limitation rather than
claim independent dependency features expose a CLI command. The correction must
not introduce the diagnostic command into a default `agov` binary.

## Collision and scope boundary

The parent and shared-gate test amendment are directly acknowledged. The only
additional reported overlap is the approved Elixir-leftovers plan; its landed
commit `ceececc9` changed the ROI module's historical comment and renamed the
mixed-record fixture to `summarize_groups_mixed_rust_and_legacy_records`, preserving
fixture data. Those landed changes remain intact; this child only adds the
feature vocabulary and focused tests. It does not take over another live
implementation. It belongs in the same isolated profiling branch; no CI-pin
image or publisher change enters it.

No matcher behavior, filesystem traversal policy, instrumentation design,
performance optimization, release scope, security waiver or role authority
changes. Parent acceptance and the prior test amendment remain mandatory.

## Acceptance

1. Plan validation, zero unacknowledged changed-plan collisions, independent
   contribution and infosec consultation, then published governor approval.
2. Core ROI tests in default and feature configurations, including arbitrary
   path/value redaction and literal vocabulary presence/absence assertions.
3. The unchanged real-clap audit passes in both default and profiling builds.
4. Full parent default/feature acceptance, required shared-gate controls and
   baseline evidence remain required; record the initial failure and correction.
5. Independent exact-head review and successful MR pipeline before Gate 2.

Rollback removes the optional vocabulary entries and feature wiring together;
without an alternative approved correction, profiling remains blocked by its
valid audit. No default telemetry format or adopter migration is introduced.
