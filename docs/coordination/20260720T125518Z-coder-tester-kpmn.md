---
artifacts_owned: []
from: coder-tester
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - tooling/elixir/mix.lock
  must_read:
  - agf/docs/build/mkdocs.yml
  - agf/docs/site/docs/index.md
  - agf/docs/site/docs/concepts/roles-and-gates.md
  - agf/docs/site/docs/start/first-governed-change.md
  - agf/docs/site/docs/adopt/downstream-impact.md
  - agf/docs/site/docs/adopt/mcp-server.md
  next_agent: planner-architect
  status: ready
  task:
  - Prepare a Gate 1 documentation proposal for cleaning up AGF docs audience boundaries and information architecture.
  - Rewrite the rough governor prompt into a planner-ready brief for third-year engineering bachelor students as the calibration audience.
  - Define what humans are expected to understand and decide, what AI agents are expected to execute and record, and what tooling/CI validates mechanically.
  - Classify which documentation is for humans, which documentation humans provide to AI assistants, which pages are AI/operator procedure, and which pages are reference material.
  - Include generated graph/diagram requirements that make the responsibility boundaries and documentation map visually obvious.
  - Pull in advisory roles in the proposal as appropriate, especially contribution-auditor for audience/scope and fact-checker or academic-peer-reviewer for conceptual clarity.
  - Do not implement the site rewrite yet; file the proposal and a coordination response with recommended next action.
id: 20260720T125518Z-coder-tester-kpmn
immutable: true
kind: handoff
model: codex-gpt-5
references-to: []
requires-ack: false
timestamp: 2026-07-20T12:55:18Z
to: planner-architect
topic: docs-audience-boundary-proposal
---

# Handoff: documentation audience-boundary proposal

The governor-reviewer reports that the documentation site is not yet carrying
the intended message. It currently implies that human adopters should become
familiar with AGF tooling and framework ceremonies in a way that overstates the
human obligation. The desired boundary is sharper:

- humans understand the accountability model, make binding decisions, choose
  adoption boundaries, and review evidence;
- AI agents perform the repetitive framework clerk work, prepare plans,
  create coordination records, run local tooling, and summarize evidence;
- `agov` and CI validate record shape and configured contracts, but do not
  approve governance decisions;
- some docs are for humans to read directly, while other docs are operational
  instructions humans give to AI agents or expect AI agents to follow.

## Improved prompt for Claude/planner-architect

Prepare a documentation proposal for AGF that explains the framework to a
third-year engineering bachelor student who understands software engineering,
Git, CI, tests, and code review, but has not internalized AGF's roles, gates,
coordination channel, or agent/tool boundary.

The proposal must make the site less confusing by separating four audiences and
uses:

1. Human adopter/governor pages: what humans decide, what evidence they should
   expect, what risks and costs adoption introduces, and what they do not need
   to personally run.
2. AI agent/operator instructions: what the assistant is expected to do with
   AGF files, roles, rules, plans, coordination messages, and tooling.
3. Shared conceptual pages: the mental model both humans and AI agents can use
   to coordinate, written at undergraduate engineering level.
4. Reference pages: command, schema, maturity, environment, and release details
   for readers who need precision after the concept is clear.

The proposal should explicitly correct the current drift where "adopter" can
sound like "human must learn every ceremony and CLI command." The intended
message is: humans own judgment and authority; AI agents and deterministic
tools carry much of the ceremony; humans need enough literacy to supervise,
verify, and revoke.

## Expected proposal outcomes

Produce one Gate 1-ready proposal or plan that covers:

- a short problem statement with examples of the current audience confusion;
- a responsibility boundary matrix for humans, AI agents, deterministic tools,
  CI, and repository artifacts;
- a proposed documentation taxonomy: human-facing guide, AI-facing operational
  instructions, shared concepts, reference, and audit/coordination records;
- a recommended navigation or page-map update for the MkDocs site;
- suggested rewrites or new pages, with scope split into small implementation
  MRs;
- which existing pages should be moved, renamed, reframed, or left alone;
- which pages should contain prompts that humans give to AI assistants;
- validation criteria for future implementation, including strict MkDocs build
  and rendered diagrams.

## Diagram requirements

Use generated diagrams where they improve comprehension. At minimum, propose:

- a human/AI/tooling responsibility swimlane;
- a documentation map by audience and use;
- a Gate 1/Gate 2 lifecycle diagram that labels the owner of each action;
- an adoption flow showing the human decision points versus AI/tooling clerk
  work.

Prefer Mermaid or existing generated SVG workflow if that fits the docs build.
The proposal should say where each graph belongs and what confusion it is meant
to remove.

## Agent collaboration to pull in

Primary role: `planner-architect`, played by Claude.

Recommended advisory pulls:

- `contribution-auditor`: check that the proposal separates product docs from
  project-specific governance evidence and avoids overstating adoption claims.
- `fact-checker` or `academic-peer-reviewer`: check whether the undergraduate
  explanation is technically precise without becoming ceremonial jargon.
- `coder-tester`: later implementation after Gate 1 approval.
- `governor-reviewer`: binding approval of the audience boundary and the
  eventual site structure.

`documentation-writer` is currently skipped in the manifest, so do not assume it
can own this unless governor-reviewer reactivates the role.

## Acceptance criteria

The proposed docs structure should let a third-year engineering bachelor
student explain, after a short reading path:

- why AGF separates planner, implementer, and governor authority;
- what humans must decide and what they may delegate to AI;
- why AI agents can draft and validate evidence but cannot approve gates;
- what `agov` validates mechanically and what it cannot decide;
- which docs they should read as a human adopter and which docs are meant to be
  loaded into or followed by AI agents;
- where generated graphs sit in the site and what each graph clarifies.

Non-goal: do not rewrite the documentation site in this handoff. The immediate
deliverable is a proposal with a small, reviewable implementation plan.
