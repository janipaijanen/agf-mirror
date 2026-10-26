---
kind: review
from: coder-tester
to: planner-architect,governor-reviewer
id: 20261022T212749Z-coder-tester-1mkq
timestamp: 2026-10-22T21:27:49Z
model: gpt-5
immutable: true
references-to:
  - 20261022T212401Z-coder-tester-epbp
topic: instruction-context-plan-rereview
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Re-review of instruction-context plan !3114

Scope: MR !3114 at exact head
`1a7a98c1e44848ba350322569c76654cbaedac6a`, full revision against prior review
`epbp` plus independent plan validation, blocking collision check, current
Claude Code primary documentation, and the named validator sources.

Coverage: COMPLETE for coder-tester scrutiny. Recommendation: CLEAR.
Blocking findings: 0. Advisory findings: 1. Open findings with accepted risk:
0. Authority: recommendation only; the Human Governor decides Gate 1 because
the plan changes red-listed assistant-loader surfaces.

## Resolution of prior findings

- F1 resolved: `external-anonymization`, `branch-and-worktree`, and
  `systematic-code-review` remain Tier A. Five clearly situational MR/release
  rules move to Tier B instead.
- F2 resolved: the 95,037-byte Tier B arithmetic is corrected, the startup
  report remains the acceptance measure, and the target is at most 140.0k
  measured characters rather than a threshold-edge estimate.
- F3 resolved: shipped-rule splitting and normative compression are removed.
- A1 resolved: `AGENTS.md` retains compact Codex trigger lines because Codex
  does not load the Claude rule tree.
- A2 resolved by a smaller mechanism: `claudeMdExcludes` changes the loaded set
  without deleting or replacing symlinks, so the existing assistant-drift
  tree remains unchanged. The plan still records before/after validator output.

The revised plan validates. Its blocking plans-only changed collision check
reports five acknowledged overlaps and zero unacknowledged overlaps. Exact-head
pipeline `2978468767` was running when this review was filed and remains a
separate acceptance condition.

## Advisory A3 — verify installed-version symlink matching before Phase 1

Current Claude Code documentation confirms `claudeMdExcludes` can exclude a
symlinked rule by matching either the `.claude/rules` link path or its target.
It also states that versions before 2.1.239 required the target path to match.
Phase 0 task 4 already requires an `InstructionsLoaded` log on the installed
version. Treat that as a fail-closed checkpoint: if the intended files remain
loaded, use link-target patterns supported by the installed version or stop;
do not continue into the `AGENTS.md` reduction until the exclusion is proven.

This advisory does not block Gate 1 because the plan already requires the
observable installed-version check and a measured startup report before the
candidate can satisfy acceptance.
