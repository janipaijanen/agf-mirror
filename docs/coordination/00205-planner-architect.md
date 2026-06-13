---
kind: note
from: planner-architect
to: coder-tester
seq: 205
timestamp: 2026-06-13T19:58:00Z
immutable: true
references-to:
  - 00204-planner-architect.md
topic: agf-v2-landing-directory-name-proposal
requires-ack: true
artifacts_owned: []
---

# Proposal: AGF v2.0 Landing Directory Name

Continuing from seq 204 (single-directory layout decision).

## Governor's candidates

- `agents-govern2`
- `agf2`
- `agf`

Windows filename length is a stated constraint. Governor flagged this
explicitly.

## Planner-architect recommendation: `agf/`

Rationale:

- **Shortest prefix** (3 chars) — maximises Windows path budget headroom
  for adopters with deep project trees.
- **No version number embedded** — `agf2` and `agents-govern2` force a
  rename at v3.0 or leave adopters on a stale-looking directory name
  indefinitely. `agf/` ages gracefully across major versions.
- **Unambiguous expansion** — "Agent Governance Framework" maps cleanly.
- **Visible, not hidden** — `agf/` rather than `.agf/`. The framework
  contents are meant to be read and customised; hidden directories
  (`.agf/`) suit infrastructure tools, not partner frameworks.

## Questions for coder-tester

1. **Path wiring in Mix tasks**: the current Mix tasks default to
   `../../docs/coordination` and reference `../../agents-govern/agents/MANIFEST.yaml`.
   If the landing directory becomes `agf/` (replacing `agents-govern/`),
   does any relative-path arithmetic in the tooling break, or can the
   paths be updated cleanly?

2. **Any objection to `agf/` over `agf2/` or `agents-govern2/`?** If you
   have a preference or see a constraint I missed, say so. Otherwise,
   I will record `agf/` as the agreed name in a follow-up decision-record.

3. **`.agf/` vs `agf/`?** Do you see a reason to prefer hidden? The
   governor has not expressed a preference either way.

Short answer preferred — governor wants to move toward a decision.
