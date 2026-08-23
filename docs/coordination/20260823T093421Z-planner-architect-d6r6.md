---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - .claude/settings.local.json
  must_read:
  - agf/skills/poll-and-coordinate/SKILL.md
  - agf/docs/site/docs/operate/poll-and-coordinate.md
  next_agent: contribution-auditor
  status: ready
  task:
  - Independently review MR !1790 (exact head
    a85eded73aac1d9f7efb52ac127dbab5c5c0c5da) and publish CLEAR or
    concrete findings.
id: 20260823T093421Z-planner-architect-d6r6
immutable: true
kind: handoff
model: claude-sonnet-5
references-to: []
requires-ack: false
timestamp: 2026-08-23T09:34:21Z
to: contribution-auditor
topic: poll-and-coordinate-skill-ready
---

# poll-and-coordinate skill ready for review

Human governor requested a single agf-level skill/command for the
coordination-poll cycle, after this session repeatedly hand-wrote poll
instructions inline and after two real poll-process gaps surfaced this
session — recorded as learning `20260823T081929Z-gkhn` in
`agf/learnings/foundry.yaml` (not a coordination record, so not linked
via `references-to`; mentioned here for context).

Published MR !1790, exact head
`a85eded73aac1d9f7efb52ac127dbab5c5c0c5da`:

- New `agf/skills/poll-and-coordinate/SKILL.md`, matching the
  `upgrade`/`learning` skill shape, operationalizing — not duplicating —
  the existing canonical `agf/docs/site/docs/operate/poll-and-
  coordinate.md`.
- Patched that canonical doc with a "Common pitfalls" subsection naming
  both failure modes concretely (stale checkout vs. fetched ref;
  default-paginated MR listing).
- `agov validate` clean end to end, including a pre-existing
  `readme_stats` drift (skill count 7→8) this MR's own addition
  triggered and fixed in the same commit.

Requesting your independent review.
