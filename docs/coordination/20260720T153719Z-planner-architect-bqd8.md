---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260720T153719Z-planner-architect-bqd8
timestamp: 2026-07-20T15:37:19Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260720T152331Z-coder-tester-prjn.md
topic: agent-improvement-proposal-norm-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/agent-improvement-proposal-norm.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Read docs/plans/agent-improvement-proposal-norm.md for Gate 1.
    - Decide SHOULD vs. MUST strength (Question 1 in the plan).
    - "Decide landing directory: agf/rules/always/ vs. agf/rules/workflow/ (Question 2)."
    - Decide whether project-local (non-AGF-scope) observations are explicitly in scope (Question 3).
    - File a decision-record; if approved, the implementation (one small MR) can start immediately.
  must_read:
    - docs/plans/agent-improvement-proposal-norm.md
    - docs/coordination/20260720T152331Z-coder-tester-prjn.md
  do_not_touch:
    - docs/audience-operator-relabeling-mr2 branch (unrelated, already merged as !231)
---

# Gate 1 handoff: agent improvement-proposal norm

Claim: Prepared per the handoff from Codex
(`20260720T152331Z-coder-tester-prjn.md`), which recorded the
governor-reviewer's in-session directive that agents may propose
improvements when they notice friction, and asked planner-architect to
draft a small Gate 1 proposal.

Plan summary: one new SHOULD-strength (Provisional) rule,
`agf/rules/always/agent-improvement-proposal-rule.md` (full exhibit text
in the plan), plus one paragraph added to
`agf/docs/site/docs/operate/day-to-day.md`. No new coordination message
kind — routes through the existing `note` / `review` / `lesson-proposal` /
Gate 1 `plan` kinds. A contribution-auditor self-review is included in the
plan (no blocking findings; main scope-creep risk addressed by an explicit
"idea ≠ approval" boundary clause).

Three open questions for Gate 1 are listed at the end of the plan
(strength, landing directory, AGF-scope-only vs. broader scope). Recommend
reading those before deciding.
