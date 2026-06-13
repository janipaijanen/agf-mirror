---
kind: handoff
from: coder-tester
to: contribution-auditor
seq: 216
timestamp: 2026-06-13T17:51:34Z
immutable: true
references-to:
  - 00215-planner-architect.md
topic: agf-v2-layout-s1-contribution-audit
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Review branch feat/agf-v2-layout before Gate 2.
    - Check that agf/ is a coherent adopter-facing single directory.
    - Check that agents-govern/ bootstrap content is not shipped under agf/.
    - Check that retiring agents-govern-2.0/ and spec/ matches seq 215.
  must_read:
    - docs/plans/agf-v2-layout-s1.md
    - docs/coordination/00215-planner-architect.md
  do_not_touch:
    - agents-govern/
    - .claude/settings.local.json
    - agents-govern/uv.lock
---

Implementation branch is ready for the required contribution-auditor advisory
review before Gate 2.

Branch:

- `feat/agf-v2-layout`

Implementation commit:

- `2fa75b0 feat: establish agf v2 layout`

Summary:

- created canonical `agf/` adopter-facing directory;
- moved agent definitions, docs, rules, schemas, skills, project-specific
  examples, framework document, adopting guide, changelog, and license under
  `agf/`;
- retired `agents-govern-2.0/`;
- retired `spec/rules-template/` and `spec/schemas/`;
- updated Elixir schema defaults from `../../spec/schemas` to
  `../../agf/schemas`;
- updated `AGENTS.md` and validator docs for the new schema path.

Verification:

- pre-retirement `mix test`: 125 passed;
- post-retirement `mix test`: 125 passed;
- final `mix test`: 125 passed;
- branch-scoped `mix agov.coord.visibility --base-branch master --branch
  feat/agf-v2-layout` exits 2 only because of pre-existing base-branch sequence
  collisions at seq 39, 188, and 211; it did not report new branch-local
  obligations from this branch before this handoff was created.

Implementation note for review:

The approved plan table named `spec/rules-template/` and `spec/schemas/` as
sources for `agf/rules/` and `agf/schemas/`. During implementation,
coder-tester also preserved adopter-facing `agents-govern-2.0/rules-template/`,
`agents-govern-2.0/schemas/`, `agents-govern-2.0/skills/`, and
`agents-govern-2.0/project-specific-examples/` under `agf/`, because retiring
`agents-govern-2.0/` without moving those referenced assets would have produced
a broken adopter distribution.
