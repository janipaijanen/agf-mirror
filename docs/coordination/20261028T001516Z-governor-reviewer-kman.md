---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20261028T001516Z-governor-reviewer-kman
timestamp: 2026-10-28T00:15:16Z
model: gpt-5
immutable: true
references-to:
  - 20261027T234948Z-governor-reviewer-rmps
  - 20261028T000402Z-contribution-auditor-ajra
  - 20261028T000412Z-infosec-sentinel-s7g1
topic: Gate 1 v2.12 release publication plan
requires-ack: false
artifacts_owned:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3323
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/pipelines/2996882260
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1 approved: v2.12.0 release publication plan

Decision: APPROVE Gate 1 for plan
`20261027T235807Z-v212-release-publication` at MR !3323 exact head
`516cfb9549ec2799c212d0b341799f1dfe1e1fe8`.

Contribution-auditor record `20261028T000402Z-contribution-auditor-ajra` and
InfoSec record `20261028T000412Z-infosec-sentinel-s7g1` are CLEAR with complete
coverage and no findings. Exact-head pipeline `2996882260` passed, including
the blocking plan-collision gate. All fourteen overlaps involving this plan are
acknowledged; historical plans without `affected_paths` remain an explicit
coverage limitation, not evidence of an undispositioned known overlap.

Approved procedure: standard current-main release; final construction-base
capture; derived and reconciled A/B/C version-cell allowlists; governed branch
`release-2.12.0`; separate human confirmations before version editing and final
changelog wording; dirty-draft validation and committed-tree identity; current
CI-rule job matrix; preflight and immediate pre-push four-namespace absence
checks; exact merge-parent/tree proof; anonymous released-byte verification;
and a separate post-publication status-sync MR.

This decision authorizes one preparatory external mutation after the plan merge:
create, if absent, and read back only the exact `v2.12.0` protected-tag rule with
tag creation restricted to Maintainers (access level 40). A pre-existing
nonmatching rule stops the release.

This decision does not authorize the version bump, release branch, metadata MR
merge, tag, package, GitLab Release, OCI publication, Pages deployment,
announcement, or status-sync edit. The version bump and final changelog wording
remain explicit human-confirmation gates; exact-head Gate 2 and all later
fail-closed publication checks remain mandatory.
