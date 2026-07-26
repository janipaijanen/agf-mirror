---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260726T231937Z-coder-tester-qs4b
timestamp: 2026-07-26T23:19:37Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260726T184011Z-governor-reviewer-t3q7
  - 20260726T225024Z-contribution-auditor-7a65
  - 20260726T231121Z-infosec-sentinel-p3hs
topic: agov-binary-freshness-final-gate2
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review MR !638 at exact implementation SHA 3beea54826fac04de7ddb5c51aa6459e2a493b76 and approve or block Gate 2.
    - If approving, explicitly record the human-directed Windows evidence deferral and the non-blocking infosec follow-up warning.
  must_read:
    - docs/plans/agov-binary-freshness-and-release-integrity.md
    - docs/coordination/20260726T225024Z-contribution-auditor-7a65.md
    - docs/coordination/20260726T231121Z-infosec-sentinel-p3hs.md
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/638
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/pipelines/2707056136
  do_not_touch:
    - rust:tooling:preview:windows-package
    - rust:tooling:build:windows:preview
    - agf/VERSION
    - Git tags and release artifacts
---

# MR !638 ready for human Gate 2

MR !638 is frozen at exact implementation SHA
`3beea54826fac04de7ddb5c51aa6459e2a493b76`, targeting `main`. No commit has
been added since the exact-head evidence and independent reviews were
produced. Pipeline `2707056136` passed.

## Implemented boundary

The change rejects stale, dirty, malformed, and provenance-incompatible
repo-local `agov` binaries before execution; embeds full source provenance in
both `agov` and `agov-mcp`; invalidates cached provenance when tracked
freshness-sensitive sources change; and verifies packaged binary bytes and
embedded provenance before archive creation.

The actual stale source build observed in this session (`agov 2.2.6`, commit
`7dd69050fb1f`) was rejected by the new wrapper. The published negative test
confirms that provenance failure occurs before archive creation. Local Rust
workspace tests, clippy, wrapper freshness tests, adopter-documentation and
infosec validators, and `git diff --check` passed. The measured local overhead
was approximately 10.1 ms per `agov --version` invocation and 20.5 ms (about
3.0%) per `agov session resume` invocation; these are local samples, not
cross-machine benchmarks.

## Exact-head package evidence

Exact job `15541210058` passed and its immutable job artifact was independently
downloaded. Both checksum sidecars match the archive bytes. Native execution
and deterministic marker inspection agree on version `2.4.4`, exact source
SHA `3beea54826fac04de7ddb5c51aa6459e2a493b76`, `dirty:false`, and
`source_dirty:false`.

- Linux x86_64 archive SHA-256:
  `72ab9adb401293af84bf45c2af7a0ab46b24bbd4d09997a744271f181d625d5e`
- Linux aarch64 archive SHA-256:
  `e5e06cccfe452e412f3323464b8cc8c1e49660fdca55c1bc55325cd6b26974f9`

The human governor commented on MR !638: "DEFER Windows builds and checks
until further notice." Both Windows jobs remain manual and were never started.
No Windows package provenance is claimed. If Gate 2 is approved, the decision
must preserve this as an explicit governed deferral rather than describing the
unrun target as passing or waived evidence.

## Independent reviews

- Contribution-auditor `20260726T225024Z-contribution-auditor-7a65`: CLEAR at
  the exact implementation SHA, conditional on the Gate 2 decision explicitly
  recording the Windows deferral.
- Infosec-sentinel `20260726T231121Z-infosec-sentinel-p3hs`: CLEAR at the exact
  implementation SHA. It independently inspected local candidate probing,
  scoped dirty-state derivation, override containment, embedded-provenance
  verification, and fail-before-archive behavior.

The infosec review records one non-blocking follow-up warning: CI currently
asserts `AGOV_BUILD_DIRTY=false` and `AGOV_BUILD_SOURCE_DIRTY=false` by clean
checkout/job-ordering convention rather than mechanically checking the
freshness-sensitive paths immediately before those exports. This is
pre-existing and not a blocker for MR !638, but should be considered for a
separate hardening task.

## Requested decision

Approve or block Gate 2 for exact SHA
`3beea54826fac04de7ddb5c51aa6459e2a493b76`. Approval may authorize merging
only that SHA into `main`; it does not authorize a release branch, version
bump, tag, release artifact, or either Windows job.
