---
kind: decision-record
from: governor-reviewer
to: planner-architect,coder-tester
id: 20261022T041904Z-governor-reviewer-382s
timestamp: 2026-10-22T04:19:04Z
model: claude-sonnet-5-5
immutable: true
references-to:
  - 20261021T224603Z-governor-reviewer-1q2r
  - 20261021T231341Z-coder-tester-67jn
requires-ack: true
topic: mr3104-advisory-rows-waived-and-merge-order
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Governor decisions: MR !3104 advisory rows waived, MR !3102 merges before MR !3105

Jani, the human governor-reviewer, replied "waive both advisory rows, merge
!3102 first" in the Claude Code session, after the earlier Gate 1 decision
`1q2r` left those items open. Claude transcribes that reply here. The human's
message is the authority; Claude is its recorder, not a substitute governor.

**Decision 1: waive both advisory rows** on plan
`docs/plans/20261021T220000Z-retire-windows-support.md` (MR !3104). The
`infosec-sentinel` and `contribution-auditor` rows are waived by the governor
and no longer `pending`. Contribution-auditor note `r6ns` stays a provisional
bridge and is not treated as a clearance. With `1q2r` (Gate 1 approved at
`4ddb90bf`) and this record both published, the coder-tester may set the
plan's advisory rows to the waived status citing this record id, flip its
`status` to `approved`, and set `gate1_approved_by`, then re-run
`agov plan validate`. Changing the plan head invalidates `4ddb90bf` as the
approved head, so the governor-visible head should be named in the follow-up
note.

**Decision 2: merge order.** MR !3102 (Part 1 re-tagging) merges before MR
!3105 (Windows retirement). MR !3105 rebases onto `main` after !3102 and
resolves its overlap in `.gitlab-ci.yml` and
`tooling/rust/crates/agov/tests/ci_needs_implication.rs`.

**Scope correction.** The "Scope reminder for Gate 2" in `1q2r` was wrong
about the plan. Coder-tester note `67jn` is right: the plan's `affected_paths`
at `4ddb90bf` already lists `tooling/rust/crates/agov-mcp/src/main.rs`,
`tooling/rust/crates/agov/src/main.rs`, `CHANGELOG.md` and the rule-projection
YAML. Gate 2 still checks the MR !3105 diff against the plan's list.

**Not decided by this record:**

- The reply sets merge order. It is not a merge authorization. MR !3102 is
  Draft, its exact-head pipeline `failed`, and the nested-container blocker
  (`crun: mount proc to proc: Operation not permitted` in
  `agov:oci:mr-build-scan`) is unresolved. Gate 2 approval and a merge
  instruction for !3102 are still needed, and per `AGENTS.md` that
  authorization must be recorded before the merge runs.
- The nested-container blocker, and whether MR !3102 changes to address it.
- Closing or superseding the Windows migration part of MR !3100.
- Any release, tag, or runner deregistration.
