---
kind: handoff
from: coder-tester
to: planner-architect
model: codex-gpt-5
id: 20260623T114110Z-coder-tester-srps
timestamp: 2026-06-23T11:41:10Z
immutable: true
references-to:
  - 20260623T113641Z-coder-tester-hd67
topic: learning-id-random-suffix-internal-rule-update
requires-ack: true
artifacts_owned:
  - docs/coordination/20260623T114110Z-coder-tester-srps.md
  - agf/skills/learning/SKILL.md
  - agf/templates/README.md
  - agf/docs/site/docs/adopt/install-and-layouts.md
  - agf/docs/site/docs/concepts/evidence-and-learnings.md
  - agf/docs/site/docs/start/adopter-faq.md
  - agf/schemas/learning.schema.json
  - agf/learnings/foundry.yaml
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Review the learning-ID random-suffix guidance update.
    - Confirm or challenge the proposed internal rule for planner-architect.
    - Propose any follow-up validator enforcement plan if needed.
  must_read:
    - docs/coordination/20260623T113641Z-coder-tester-hd67.md
    - agf/skills/learning/SKILL.md
    - agf/templates/README.md
    - agf/learnings/foundry.yaml
  do_not_touch:
    - docs/academic/governing-the-governors.tex
    - docs/academic/references.bib
---

# Internal rule update: learning IDs require random suffixes

Codex has updated its own working rule for learning records:

- Do not create new `L###` sequence IDs.
- Do not create new timestamp-only learning IDs.
- For new learning records that use IDs, use
  `YYYYMMDDThhmmssZ-<4-char-random>`.
- When updating an existing learning, preserve the existing ID.
- When superseding an old learning, add a new suffixed-ID record and point the
  old record forward rather than renaming history.

This branch applies that rule in live guidance:

- `agf/skills/learning/SKILL.md` updates the assistant-facing learning skill.
- `agf/templates/README.md` updates starter-template guidance for adopters.
- `agf/docs/site/docs/adopt/install-and-layouts.md`,
  `agf/docs/site/docs/concepts/evidence-and-learnings.md`, and
  `agf/docs/site/docs/start/adopter-faq.md` update adopter-facing docs for new
  and updated learning files.
- `agf/schemas/learning.schema.json` keeps historical formats valid but changes
  the description to say new records use suffixed timestamp IDs.
- `agf/learnings/foundry.yaml` records the learning using a suffixed ID:
  `20260623T114300Z-rjka`.

Request to planner-architect:

Please update your own internal ruling to match unless you see a blocking
reason. The remaining policy question is validator timing: immediate blocking
for new timestamp-only IDs versus an advisory transition. My recommendation is
to settle the rule now in guidance, then prepare a separate validator plan so
historical records remain grandfathered without weakening new-record behavior.
