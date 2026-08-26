---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260810T201201Z-contribution-auditor-8knp
timestamp: 2026-08-10T20:12:01Z
model: codex-gpt-5
immutable: true
references-to: []
topic: coordination-to-learning-candidate-discovery-issue
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Assess issue #107 as the focused future Gate 1 lane for governed coordination-to-learning candidate discovery and draft creation; schedule it without interrupting the active diligence and v2.5.1 lanes.
  must_read:
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/107
    - agf/learnings/foundry.yaml
    - agf/rules/always/agent-improvement-proposal-rule.md
  do_not_touch:
    - .claude/settings.local.json
---

# Focused coordination-to-learning tooling issue filed

Issue #107 now captures the unimplemented part of learning `L009`: discover
structured learning candidates in coordination, disposition them explicitly,
and generate a governed learning draft from exact coordination evidence.

The issue intentionally keeps automatic learning promotion out of scope.
Candidate discovery is advisory and deterministic-first; contribution-auditor
and governor-reviewer retain classification and Gate authority. It also keeps
this work separate from issue #105's arbitrary-file sensitive-material scanner
and notes issue #46 only as adjacent incident-to-learning governance work.

This is roadmap intake, not implementation authority. Planner-architect should
decide an appropriate planning window after the current diligence and release
follow-up lanes settle, inventory the existing `lesson-proposal` and Learning
Review conventions, and bring a bounded plan through Gate 1 before tooling or
schema changes begin.
