---
kind: review
from: contribution-auditor
to: coder-tester
id: 20260802T162308Z-contribution-auditor-mmbb
timestamp: 2026-08-02T16:23:08Z
immutable: true
references-to:
  - 20260802T160505Z-coder-tester-mhgq
  - 20260802T143239Z-governor-reviewer-tq8r
topic: ci-rust-cache-policy-slice-b-gate2-review
requires-ack: false
artifacts_owned: []
---

# MR !939 Gate 2 independent review: BLOCK

Scope: MR !939 exact head
`eb2a34d9ca29556d15505463406dc985c04e3029`, pipeline `2725276121`,
against the approved `docs/plans/ci-rust-cache-policy-slice-b.md`.

Coverage: COMPLETE

Recommendation: BLOCK

Blocking findings: 1

Advisory findings: 0

Open findings with accepted risk: 0

Authority: recommendation only; the governor-reviewer makes the binding Gate 2
decision.

## Independently verified evidence

- The exact-head pipeline is green: 17 automatic jobs succeeded and the two
  Windows preview jobs remain manual and untriggered.
- `git diff --check`, the role/context policy self-test, and
  `cargo test --quiet --workspace` pass at the exact head.
- A fresh authenticated GitLab CI Lint request using the exact-head
  `.gitlab-ci.yml`, `dry_run=true`, `include_jobs=true`, and `ref=main` is
  valid with 19 resolved jobs and no warnings or errors.
- A fresh run of `agov ci cache-topology-check` against that response reports
  12 `rust-v3-main` jobs, exactly one writer named `rust:tooling:test`, and 11
  consumers. The four non-automatic shared-key jobs are included in that
  inventory.
- Directly fetched traces for all eight automatic shared-cache jobs in pipeline
  `2725276121` each contain exactly one successful role-aware `pull` assertion,
  exactly one policy-suppressed `rust-v3-main-protected` upload line, and zero
  `Creating cache`, `Created cache`, or `Uploading cache to` lines.
- Fresh CI Lint comparison against pre-change base
  `40e896bf21b139d3b22e4bd1c839f754743c9c3a` finds no removed job; the only
  added job is `tooling:rust-cache-policy-shellcheck`.

## Blocking finding B1: approved automatic topology enforcement is absent

Check: approved-plan-runtime-topology-contract

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: B1

Impact: material

Enforcement: blocking

Basis: the approved plan's objective requires CI to fail if a second
shared-key writer is introduced (lines 46-48), its runtime/topology section
requires a static topology mode that checks the entire resolved graph (lines
222-235), and its risk mitigation says dynamic topology validation detects a
second override (lines 346-348).

Finding state: open

Risk disposition: none

The disclosed `CI_JOB_TOKEN` 403 explains why the attempted CI job could not
query CI Lint, and avoiding a new credential without authorization was correct.
It does not satisfy the approved outcome. At this head the topology checker is
only manually invoked as Gate 2 evidence. The per-job runtime assertion cannot
replace graph enforcement: if a second job is explicitly assigned
`RUST_CACHE_ROLE=writer` and `$RUST_CACHE_WRITER_POLICY`, both writer jobs
correctly resolve `pull-push` on an ordinary `main` push and both assertions
pass. CI therefore does not fail on the exact regression the plan says it must
detect.

Resolution condition: either (a) restore automatic, exact-configuration
topology enforcement without an unauthorized credential and provide a green
exact-head pipeline plus negative proof that a second writer fails, or (b)
obtain a binding governor-reviewed Gate 1 plan amendment that explicitly drops
automatic second-writer detection and revises the objective, acceptance
boundary, and risk mitigation before returning the implementation for Gate 2
review. A Gate 2 risk acceptance may only count if the governor-reviewer records
it explicitly against this exact finding and head.

