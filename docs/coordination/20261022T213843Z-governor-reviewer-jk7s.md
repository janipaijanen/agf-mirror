---
kind: decision-record
from: governor-reviewer
to: planner-architect,coder-tester
id: 20261022T213843Z-governor-reviewer-jk7s
timestamp: 2026-10-22T21:38:43Z
model: human-governor
immutable: true
references-to:
  - 20261022T213244Z-planner-architect-brpp
  - 20261022T212922Z-planner-architect-rhqs
topic: instruction-context-plan-gate1
requires-ack: false
artifacts_owned:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3114
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1 approved for the instruction-context reduction plan

The Human Governor directly approved Gate 1 for plan
`20261022T205113Z-instruction-context-reduction` in MR !3114 at exact head
`3bfa9603ac30edc779263438ef6e0ca091cd1188`. This record ratifies bridge
records `brpp` and `rhqs`, including the direct waiver of the
`infosec-sentinel` and `contribution-auditor` advisory rows.

## Approved scope

- Task 0 evidence already recorded in the plan.
- Foundry-local Phase 0: configure `claudeMdExcludes`, retain every rule
  symlink, add the on-demand rule index and projection coverage evidence, and
  measure the actual loaded set.
- Phase 1: move reference material out of `AGENTS.md` verbatim while retaining
  the always-needed obligations and compact Codex trigger lines.
- Acceptance target: at most 140.0k characters in the measured Claude startup
  report.

No shipped `agf/rules/**` text, normative compression, validator/gate change,
release, or deployment is authorized.

The Phase 0 `InstructionsLoaded` evidence is a fail-closed checkpoint. Phase 1
must not begin unless the intended exclusions are proven on the installed
Claude Code version. Current documentation notes that versions before 2.1.239
require symlink-target matching; the implementation must use patterns proven
by the installed-version evidence or stop.

Official coder-tester records `1mkq` and `kjhh` in coordination MR !3117 give
CLEAR for the substantive plan and the exact waiver-row delta. Exact-head plan
pipeline `2978476401` passed. The plan validates and its blocking collision
check reports five acknowledged overlaps and zero unacknowledged overlaps.

This is the Human Governor's own decision, not an exercise of delegation
artifact `qpda`; the delegated-decision count remains 0 of 5. Gate 2 and merge
of the implementation remain with the Human Governor because the affected
assistant-loader paths are on the delegation red list.
