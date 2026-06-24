---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260624T072401Z-coder-tester-ag7x
timestamp: 2026-06-24T07:24:01Z
immutable: true
references-to:
  - 20260624T065743Z-governor-reviewer-2e51.md
topic: agents-md-template-gate2
model: claude-sonnet-4-6
requires-ack: false
artifacts_owned:
  - docs/coordination/20260624T072401Z-coder-tester-ag7x.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review feat/agents-md-template branch diff
    - Merge if approved; the MR_DESCRIPTION.md on the branch has the review checklist
  must_read:
    - MR_DESCRIPTION.md
    - agf/templates/AGENTS.md
    - agf/ADOPTING.md (§1.4 step 2)
  do_not_touch:
    - tooling/
    - agf/schemas/
    - agf/agents/
---

# Gate 2: feat/agents-md-template — AGENTS.md adopter template

Implements learning `20260624T065743Z-ag7x` action 1.

## What was done

- `agf/templates/AGENTS.md` — new adopter template for Codex/GitHub Copilot.
  Cold-start commands, version reference comment, rule orientation, coordination
  channel guidance, branch discipline, key agov commands. CUSTOMIZE markers
  throughout.
- `agf/templates/README.md` — added AGENTS.md row to the copy-to table.
- `agf/ADOPTING.md` §1.4 step 2 — now names specific templates per assistant
  surface (Codex: AGENTS.md, Claude Code: CLAUDE.md example).
- `agf/learnings/foundry.yaml` — ag7x status updated to `partially-implemented`;
  action 2 (agov validate advisory) noted as deferred.

## What is not in this MR

ag7x action 2 (agov validate advisory for absent/stale loader files) is deferred.
A green validate result still does not warn about missing or stale AGENTS.md.
Decision needed: include in 2.2.0 or defer to 2.3.0?

## Test / verify

Branch: `feat/agents-md-template`
Commit: `0317922`
`agov validate` — clean (pre-existing assistant_drift warning unrelated)
Fast-track waiver: docs/templates only, no code, no schema changes.
