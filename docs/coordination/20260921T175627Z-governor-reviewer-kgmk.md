---
kind: decision-record
from: governor-reviewer
to: all
id: 20260921T175627Z-governor-reviewer-kgmk
timestamp: 2026-09-21T17:56:27Z
immutable: true
topic: gate1-approval-assistant-rule-drift-resolution
references-to:
  - 20260921T040743Z-assistant-rule-drift-resolution
requires-ack: false
artifacts_owned: []
---

<!-- Filed into the coordination channel by the claude-sonnet-5 session at
the governor's explicit request ("gate approved. proceed", 2026-09-21,
live session) to transcribe a live Gate 1 decision into a durable,
governor-authored coordination record, per AGENTS.md's "Conversational
decisions and bridge records" section. The decision itself is the
governor's. -->

# Gate 1 approval: assistant-rule-drift-resolution (issue #135)

Gate 1 approved for `docs/plans/20260921T040743Z-assistant-rule-drift-resolution.md`
at commit `0043d7a3` on branch `plan/assistant-rule-drift-resolution` (MR
!3085, exact-head pipeline green).

The approved plan resolves foundry's assistant-rule drift (issue #135) by
converting the 49 symlink-safe `.claude/rules/` findings (7 missing + 28
front-matter-only-differ + 14 confirmed-stale-differ) to relative symlinks
pointing at their `agf/rules/` canonical counterpart, keeping 4 files as
deliberate hand-maintained loader-projection copies
(`workflow/simplicity-rule.md`, `workflow/development-workflow-rule.md`,
`always/improvement-propeller-rule.md`,
`always/universal-polling-and-autonomous-continuation-rule.md`), and
leaving the 2 foundry-local-only files untouched
(`always/coordination-message-rule.md`,
`always/coordination-sequence-guard-rule.md`). This resolves the
`agf/rules/README.md` policy conflict the plan originally flagged as
unresolved, based on Claude Code's documented symlink support
(`code.claude.com/docs/en/memory.md`).

`contribution-auditor` advisory consultation is waived for this plan on
the same basis as `20260920T115342Z-governor-reviewer-3es7` and
`20260920T194655Z-governor-reviewer-kgd8`: Codex (the project's
contribution-auditor player) remains token-dormant this session, and the
governor has directed the planner-architect session to proceed without a
separate contribution-audit round for this rename/consolidation-shaped,
non-authority-expanding change.

Implementation (Gate 2) may proceed on this basis. Gate 2 merge itself
remains a separate binding governor-reviewer decision.
