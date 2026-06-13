---
kind: handoff
from: coder-tester
to: contribution-auditor
seq: 217
timestamp: 2026-06-13T17:56:00Z
immutable: true
references-to:
  - 00215-planner-architect.md
topic: agf-v2-layout-s1-contribution-audit-bridge
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Review branch feat/agf-v2-layout before Gate 2.
    - Treat branch-local seq 216 as the detailed contribution-audit handoff.
    - Confirm whether the branch is ready for governor-reviewer Gate 2.
  must_read:
    - docs/plans/agf-v2-layout-s1.md
    - docs/coordination/00215-planner-architect.md
  do_not_touch:
    - agents-govern/
    - .claude/settings.local.json
    - agents-govern/uv.lock
---

Master-visible bridge for the AGF v2.0 Layout S1 contribution-auditor review.

Branch:

- `feat/agf-v2-layout`

Branch-local detailed handoff:

- `docs/coordination/00216-coder-tester.md`

Implementation commits:

- `2fa75b0 feat: establish agf v2 layout`
- `e00399b coordination: request agf layout contribution audit`

Summary:

- creates canonical `agf/` adopter-facing directory;
- retires `agents-govern-2.0/`;
- retires `spec/rules-template/` and `spec/schemas/`;
- moves Elixir schema default to `../../agf/schemas`;
- preserves adopter-facing rules, schemas, skills, and examples under `agf/`;
- does not touch `agents-govern/`.

Known verification note:

Branch-scoped `mix agov.coord.visibility --base-branch master --branch
feat/agf-v2-layout` exits 2 because of pre-existing base-branch sequence
collisions at seq 39, 188, and 211. It did not report new branch-local
obligations from `feat/agf-v2-layout` before branch-local seq 216 was created.
