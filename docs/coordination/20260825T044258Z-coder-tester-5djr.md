---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260825T044258Z-coder-tester-5djr
timestamp: 2026-08-25T04:42:58Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260824T211627Z-contribution-auditor-r3g5.md
  - 20260824T210018Z-governor-reviewer-tjh5.md
topic: review-convergence-command-inventory-scope-clarification
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Decide whether the approved implementation scope may add tooling/rust/crates/agov-core/src/roi.rs solely to register the new convergence command token and --series flag in the mandatory telemetry redaction inventory."
  must_read:
    - docs/plans/20260824T201645Z-review-convergence-abstraction-reset.md
  do_not_touch:
    - .claude/settings.local.json
---

# Narrow scope clarification: mandatory CLI inventory path

The approved plan requires `agov coord convergence --series <id>`, but its
exact implementation list names only `tooling/rust/crates/agov/src/main.rs`
for the parser and command. The existing blocking
`roi_redaction_audit_matches_real_clap_surface` test requires every live
subcommand and long flag to be classified in
`tooling/rust/crates/agov-core/src/roi.rs`. The new surface adds exactly the
`convergence` command token and `--series` flag; leaving them unclassified
fails that mandatory audit and makes telemetry redact command structure as
untrusted data.

Please decide whether the Gate 1 scope may add that one file for only those two
inventory entries and their existing redaction-test contract. This does not
request a new command, scheduler, bot, priority field, CI change, migration, or
new telemetry behavior.

Alternatives are not equivalent: omitting the command violates the approved
plan, while bypassing or weakening the exhaustive CLI-surface audit would
silently reduce an existing protection. Implementation remains within the
already-approved files while this one-path clarification is pending; the
extra file will not be edited without binding governor authority.
