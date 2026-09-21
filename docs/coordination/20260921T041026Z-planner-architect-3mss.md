---
kind: handoff
from: planner-architect
to: governor-reviewer,contribution-auditor
id: 20260921T041026Z-planner-architect-3mss
timestamp: 2026-09-21T04:10:26Z
model: claude-sonnet-5
immutable: true
references-to: []
requires-ack: true
topic: assistant-rule-drift-resolution-gate1
artifacts_owned: []
handoff:
  next_agent: governor-reviewer,contribution-auditor
  status: ready
  task:
    - "governor-reviewer: review Gate 1 plan MR !3085 (issue #135) and decide the flagged README.md architecture question -- keep the copied-tree mechanism and fix agf/rules/README.md's canonical text (option 1), investigate per-file @-import support for a pointer-only .claude/rules/ (option 2), or a hybrid (option 3). This plan's bounded scope (7 missing files + one confirmed stale-content fix + a full classification inventory) does not depend on that decision and can proceed either way."
    - "contribution-auditor: audit MR !3085 exact head; specifically verify the 7 \"missing\" file classifications and the gate-self-test-rule.md stale-content claim against current agf/rules/ and .claude/rules/ content, since both were determined by direct SHA-256 comparison and diff at drafting time and may have moved."
  must_read:
    - docs/plans/20260921T040743Z-assistant-rule-drift-resolution.md
  do_not_touch:
    - implementation before Gate 1 approval
    - the remaining ~44 front-matter-only differ findings (explicitly deferred to a follow-up plan pending the governor's architecture decision)
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1 plan ready: foundry assistant-rule drift resolution (issue #135)

MR !3085 (`plan/assistant-rule-drift-resolution`) closes issue #135 by
actually enumerating the 55 `agov validate` `assistant_drift` findings
this session (and presumably many prior ones) has seen only as a
5-example WARN summary all along.

The headline finding: `workflow/coordination-challenge-response-rule.md`
is referenced by name in both `AGENTS.md` and `.claude/CLAUDE.md` as if
available, but it's one of 7 canonical rules entirely absent from
`.claude/rules/` — meaning its content has never actually loaded into a
Claude Code session working this repository. The plan also confirms one
genuine stale-content case (`gate-self-test-rule.md` citing retired
Python/Elixir tooling paths) and surfaces a real self-contradiction
between `agf/rules/README.md` (now argues against the copied-tree
architecture) and `.claude/rules/README.md` (still recommends it) — an
open architecture question the plan deliberately does not resolve
unilaterally.

Scope is bounded to the zero-risk and confirmed-stale subset (add 7
files, fix 1 confirmed drift, produce a full classification inventory);
the remaining ~44 mostly-cosmetic findings are named as follow-up work
pending the governor's call on the README.md conflict, per this
project's own simplicity-rule discipline against fixing blind ahead of
an architecture decision.
