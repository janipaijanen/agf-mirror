---
kind: handoff
from: coder-tester
to: planner-architect
seq: 231
timestamp: 2026-06-13T19:13:09Z
immutable: true
references-to:
  - 00228-coder-tester.md
topic: shareable-workitems-mechanism
requires-ack: true
artifacts_owned:
  - docs/proposals/workitem-sharing-mechanism.md
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Review the shareable workitems proposal.
    - Argue whether docs/workitems manifests are the right mechanism.
    - Decide whether bundles should ever be committed or only externally attached.
    - Recommend when a workitem manifest becomes mandatory.
    - Include a position on untracking .claude/settings.local.json as separate hygiene.
  must_read:
    - docs/proposals/workitem-sharing-mechanism.md
  do_not_touch:
    - agents-govern/
    - .claude/settings.local.json
    - agents-govern/uv.lock
---

Governor requirement:

We must have a mechanism to share workitems that currently live in `.worktrees/`
or any equivalent local checkout directory. The physical directories themselves
are local and ignored, but complex work needs a portable representation for
colleagues and agents.

Coder-tester drafted `docs/proposals/workitem-sharing-mechanism.md`.

Core proposal:

- do not share `.worktrees/` directly;
- share branch commits through normal Git transport;
- share obligation and context through coordination entries;
- add tracked workitem manifests under `docs/workitems/<id>.yaml`;
- support optional bundles/patch transfer when no shared remote is available;
- later add Elixir tooling: `mix agov.workitem.*`.

Codex recommendation:

Make workitem manifests mandatory for complex or cross-agent work and optional
for trivial branches. A manifest should record branch, base branch, owner,
coordination links, expected dirty state, verification commands, and restore
instructions.

Planner-architect requested:

Please critique this with Claude's planning lens. The decision needed is whether
this becomes a Gate 1 follow-up plan, a rule, a schema/tooling slice, or all
three in stages.
