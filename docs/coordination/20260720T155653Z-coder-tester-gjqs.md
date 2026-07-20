---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260720T155653Z-coder-tester-gjqs
timestamp: 2026-07-20T15:56:53Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260720T152331Z-coder-tester-prjn.md
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/235
topic: mr-description-mechanism-hygiene
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Assess whether the branch-local `MR_DESCRIPTION.md` mechanism should be documented, replaced, or supported by tooling.
    - Fold this into the agent improvement-proposal norm work if Gate 1 approval makes that a natural fit; otherwise keep it as a small separate process-improvement proposal.
    - Preserve the boundary that branch-local MR description helpers must not become silent repository baggage or drift from the visible GitLab MR description.
  must_read:
    - docs/coordination/20260720T152331Z-coder-tester-prjn.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Handoff: decide how `MR_DESCRIPTION.md` should work

The governor asked whether Codex had proposed a change after noting that the
`MR_DESCRIPTION.md` mechanism felt odd. Codex had not yet filed that proposal;
this handoff records it as an improvement candidate.

Observation:

- `MR_DESCRIPTION.md` is useful as branch-local scratch text for a merge
  request body.
- It is easy for the file and the actual GitLab MR description to drift.
- It can become confusing repository baggage if committed and merged without a
  clear rule saying whether it is temporary, durable, or generated.
- Updating GitLab manually from the file works, but it depends on agent memory
  and is not transparent to a cold-starting agent unless documented.

Recommended options to evaluate:

1. Treat `MR_DESCRIPTION.md` as a temporary branch helper only, and require the
   GitLab MR body to be updated explicitly whenever the file changes.
2. Stop committing `MR_DESCRIPTION.md`; use coordination records plus the
   GitLab MR body as the durable surfaces.
3. Add lightweight tooling or documentation that validates/syncs the MR body
   from a known branch-local description file.

Codex preference: option 2 for ordinary work. Coordination records and the
GitLab MR body are already the durable, visible surfaces. If a branch-local
description file remains useful for Claude/Codex collaboration, option 1 is
acceptable, but the file should be documented as temporary and should normally
be removed before merge unless the project intentionally adopts it.

This is a good concrete example for the proposed agent improvement-proposal
norm: the agent notices workflow friction, routes it separately, and does not
expand the active MR without approval.
