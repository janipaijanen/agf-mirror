---
kind: handoff
from: governor-reviewer
to: coder-tester
id: 20260722T173647Z-governor-reviewer-a7mp
timestamp: 2026-07-22T17:36:47Z
model: human-governor
immutable: true
references-to:
  - 20260722T172042Z-contribution-auditor-hqdj
  - 20260722T145245Z-contribution-auditor-rsam
  - 20260722T164331Z-governor-reviewer-smmj
topic: v2-4-0-zip-etxtbsy-corrective-remediation
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Acknowledge this handoff and `hqdj`, then claim the surviving ZIP `ETXTBSY` corrective remediation.
    - Branch from current `main` at or after `c0cf4532` and implement a narrow structural correction for the ZIP extraction-then-execution race in a separate MR.
    - Prove both TAR and ZIP round trips with repeated focused execution, the full Rust workspace suite, and exact-head CI evidence.
    - After the correction merges and post-merge `main` is green, rebase and complete the seven-figure Class 1 slice in MR !405.
  must_read:
    - docs/coordination/20260722T172042Z-contribution-auditor-hqdj.md
    - docs/coordination/20260722T145245Z-contribution-auditor-rsam.md
    - tooling/rust/crates/agov/src/provenance.rs
  do_not_touch:
    - .gitlab-ci.yml
    - agf/VERSION
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Governor handoff: close the ZIP race before resuming the figure slice

Proceed with the surviving ZIP `ETXTBSY` remediation as corrective work within
the already approved Workstream B executable self-report scope. A new Gate 1
plan is not required for restoring the acceptance condition that MR !398 was
intended to satisfy, but the corrective implementation still requires exact-
head review and a governor Gate 2 decision before merge.

Keep the correction structurally narrow and separate from MR !405. The prior
`sync_all()` plus explicit destination-writer `drop()` treatment remains
present and nevertheless failed in push pipeline `2697899502`, job
`15482226971`; do not represent that treatment alone as sufficient evidence or
mask the race with an unbounded retry. Establish and test the actual lifecycle
or execution boundary that survives the CI filesystem.

Minimize paid CI while the deduplication change is still unapproved: finish and
stress-test the correction locally before the first ordinary branch push, then
open the MR for review. Do not make speculative remote pushes or retry the
known-failing superseded pipeline.

MR !405 may continue locally but must not be rerouted for Gate 2 until this
correction is merged, post-merge `main` is green, and all seven remaining Class
1 figures are included. No `.gitlab-ci.yml` edit, version bump, release branch,
tag, or publication is authorized by this handoff.
