---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260720T152331Z-coder-tester-prjn
timestamp: 2026-07-20T15:23:31Z
model: codex-gpt-5
immutable: true
references-to:
  - docs/plans/docs-audience-boundary-proposal.md
  - 20260720T143255Z-governor-reviewer-s6hg.md
  - 20260720T150722Z-coder-tester-pp5e.md
topic: agent-improvement-proposal-ruleish-followup
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Prepare a small Gate 1 proposal for an AGF rule-ish norm that lets agents surface improvement ideas when they notice oddness or friction.
    - Keep the norm non-blocking so agents SHOULD propose improvements, but MUST NOT widen the active MR without authorization.
    - Cover improvements for smoothness, transparency, logical consistency, token saving, process clarity, documentation clarity, tooling ergonomics, and other AGF-scope reasons.
    - Define the routing shape as quick note, review, lesson-proposal, or Gate 1 plan depending on impact and scope.
    - Preserve authority boundaries so AI agents propose and supply evidence while governor-reviewer decides whether to approve or prioritize changes.
  must_read:
    - docs/plans/docs-audience-boundary-proposal.md
    - agf/docs/site/docs/operate/day-to-day.md
    - agf/docs/site/docs/concepts/roles-and-gates.md
    - agf/rules/always/tool-authority-boundary-rule.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Handoff: propose an agent improvement-suggestion norm

The governor-reviewer clarified a useful operating norm during the docs
audience-boundary implementation:

> if anything is odd you and any other agent can propose changes to make things
> more smooth, transparent, logical, token saving, etc (what are in our agf
> scope at least), but also for any other reason. this should be marked as a
> rule-ish?

Codex agrees this should be recorded as a rule-ish AGF norm, but as a
non-blocking `SHOULD`, not as a hard `MUST`.

Recommended shape:

- agents SHOULD surface oddness, friction, duplication, confusing workflow,
  avoidable token spend, unclear authority boundaries, brittle CI/tooling, or
  documentation gaps they notice while doing assigned work;
- agents SHOULD keep the active task narrow and route broader improvements as
  a coordination note, review, lesson proposal, or Gate 1 plan instead of
  quietly expanding the current MR;
- the proposal SHOULD name the observed issue, likely benefit, affected
  artifact(s), whether the idea is AGF-scope or project-local, and any cheap
  evidence;
- AI agents MUST NOT treat an improvement idea as approval to change rules,
  gates, roles, or merge policy; governor-reviewer approval remains binding;
- token-saving ideas SHOULD prefer deterministic tooling, compact summaries,
  and existing `agov` surfaces before adding more ceremony.

Candidate landing places:

- a small rule under `agf/rules/always/`, probably named around continuous
  improvement or improvement proposals;
- a short operational paragraph in `agf/docs/site/docs/operate/day-to-day.md`;
- possibly a learning template hint if the proposal turns into a repeated
  evidence pattern.

This is separate from the active MR-2 audience relabeling branch; do not fold
it into that implementation MR.
