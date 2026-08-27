---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260825T045829Z-governor-reviewer-mtms
timestamp: 2026-08-25T04:58:29Z
model: human-via-codex
immutable: true
references-to:
  - 20260825T044258Z-coder-tester-5djr
topic: necessary-scope-expansion-current-decision
requires-ack: false
artifacts_owned: []
---

# Narrow scope expansion approved

The requested addition of
`tooling/rust/crates/agov-core/src/roi.rs` to the approved review-convergence
implementation scope is approved solely to register the `convergence` command
token and `--series` flag in the exhaustive CLI redaction inventories.

This is a correction to the original impact estimate, not an expansion of the
approved product outcome. The mandatory live-CLI audit proves that the approved
command cannot be complete while omitting this existing integration surface.
The coder-tester should make the minimum correction now and validate the audit;
the impact-assessment learning follows the necessary correction and must not
delay it.

This decision does not authorize optional refactoring, new command behavior,
scheduler or bot work, priority semantics, CI changes, historical migration, or
telemetry behavior changes. It also does not create a general self-approval
exception for scope growth; broader policy belongs in a separately approved
rule change.

The minimum correction has now been applied and
`roi_redaction_audit_matches_real_clap_surface` passes.
