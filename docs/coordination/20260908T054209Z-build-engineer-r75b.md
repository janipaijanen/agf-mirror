---
kind: review
from: build-engineer
to: governor-reviewer
id: 20260908T054209Z-build-engineer-r75b
timestamp: 2026-09-08T05:42:09Z
model: gpt-6-astra
immutable: true
references-to:
  - 20260908T042657Z-planner-architect-2g7g.md
  - 20260908T043532Z-contribution-auditor-hqp4.md
  - 20260907T200210Z-coder-tester-8j2h.md
topic: ruw4-implementation-exact-head-build-review
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# RUW-4 exact-head build review — CLEAR at `b4199c26`

Scope: CI topology, runner compatibility, artifact flow, test evidence, and
release-gate reliability for all 13 files in MR !2539 at exact head
`b4199c26abfd43090c44dc3c795331f4584de734`. Coverage: **COMPLETE**.
Recommendation: **CLEAR**. Blocking findings: 0. Advisory findings: 0.
Accepted-risk findings: 0. Authority: advisory only; governor-reviewer decides
Gate 2.

## Reviewer provenance

The governor directly assigned this Codex/GPT-6 session to take the
build-engineer role. This same session contributed the authentication-phase
correction and tests retained in !2539, so its review is not independent
corroboration of that portion. The implementation worktree and runner
configuration were not modified.

## Pipeline and topology evidence

- Canonical detached-MR pipeline `2828211506` passed at exact SHA `b4199c26`.
  All **22 mandatory jobs succeeded**; the two Windows packaging jobs were
  optional/manual by declared policy. No required job failed, skipped, or
  remained manual.
- GitLab CI lint of the exact `.gitlab-ci.yml` returned valid with zero errors
  and zero warnings.
- The new `release:namespace-check` is test-stage, `allow_failure: false`, and
  selected only for `merge_request_event` plus an `agf/VERSION` change. It has
  no tag, branch-push, schedule, or post-tag rule.
- Its only mandatory producer is `rust:tooling:preview` with artifacts enabled.
  That producer is selected on `agf/VERSION`, and pipeline-needs validation
  proves the producer/consumer implication. The adapter extracts
  `dist/agov-<version>-linux-x86_64.tar.gz`; it contains no `cargo build` or
  container-service path and preserves the command's non-zero exit.
- The job has no `dind` tag or service, so it follows the ordinary untagged
  private-runner lane. Pipeline `2828211506` confirms ordinary jobs on
  `priv runner`, while only the two declared OCI jobs used the `dind` runner.
- Preview job `16357688195` produced exact-head artifact
  `agov-rust-preview-b4199c26.zip` (3,690,164 bytes). The downstream
  `release:assets-verify-preview` job downloaded both required preview
  producers and passed. No second Rust or archive build is introduced by the
  namespace job.
- Successful execution writes only
  `dist/release-namespace-absence.json`, bound to source SHA, live target SHA,
  version, project identity, checkpoint time, and pipeline ID. Failed checks
  create no success evidence and block their release-metadata MR.

## Independent exact-head checks

- GitLab CI lint: pass, 0 errors / 0 warnings.
- `agov gate change-check`: pass.
- strict plan projection-check: pass.
- shell syntax and `git diff --check`: pass.
- namespace unit tests: 8 passed.
- real CLI and adapter integration: 6 passed.
- CI needs/routing/mutation tests: 12 passed.
- gate-change companion tests: 15 passed.
- canonical pipeline Rust workspace tests, clippy with warnings denied,
  governance validation, distribution validation, preview verification,
  OCI tests, and dependency checks: passed.

The preview job logged expected non-blocking warnings for absent Windows zip
artifacts because Windows packaging is an explicitly optional manual job. The
Trivy job logged workspace-inherited-version parse warnings but still inspected
`Cargo.lock` with zero reported vulnerabilities; mandatory `cargo deny check`
separately reported advisories, bans, licenses, and sources OK. Neither warning
is introduced by RUW-4 or hides a required-job failure.

## Evidence limit and maturity

The namespace job correctly did not run in !2539 because this implementation
MR does not change `agf/VERSION`. Its selection, needs, runner class, artifact
reuse, and blocking exit are exercised by deterministic tests; its real GitLab
API invocation will first run on the release-metadata MR. The implementation is
therefore **Level 4 Validated**, not Level 1 Planned and not yet Level 5
Deployable. Level 5 requires the supported release path/artifact, matching
documentation, and successful real namespace-job evidence. This corrects only
the maturity label in `hqp4`; its contribution CLEAR result is unchanged.
