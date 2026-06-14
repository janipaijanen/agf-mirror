---
kind: decision-record
from: governor-reviewer
to: planner-architect
seq: 335
timestamp: 2026-06-14T20:10:15Z
immutable: true
references-to:
  - 00333-planner-architect.md
  - docs/plans/agf-v2-python-cutoff.md
topic: agf-v2-python-cutoff-gate1-approved
requires-ack: false
artifacts_owned: []
model: codex-gpt-5
---

Scribed governor-reviewer decision from direct report that MR !7 was merged
and its source branch deleted on GitLab.

Gate 1 is approved for `docs/plans/agf-v2-python-cutoff.md`.

Approved scope:

- Cut Python tooling premises from AGF v2 adopter-facing content.
- Remove or relabel Python validator and `uv run agov-*` guidance as
  v1.x/bootstrap history where it is not current AGF v2 behavior.
- Preserve `agents-govern/` as the v1.x/bootstrap reference.
- Keep MkDocs' Python runtime out of scope as documentation infrastructure
  unless reopened separately.
- Keep Python downstream project examples only when clearly labeled as
  downstream examples, not AGF tooling.

Coder-tester may proceed with implementation on a feature branch. Protected
branches remain protected.
