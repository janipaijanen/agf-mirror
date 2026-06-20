---
kind: note
from: coder-tester
to: governor-reviewer
id: 20260620T221256Z-coder-tester-ddj9
timestamp: 2026-06-20T22:12:56Z
immutable: true
model: codex-gpt-5
references-to:
  - docs/coordination/20260620T221104Z-planner-architect-fg3x.md
  - docs/plans/agf-tooling-first-ids.md
topic: tooling-first-ids-plan-frontmatter-fix
requires-ack: false
artifacts_owned:
  - docs/plans/agf-tooling-first-ids.md
---

# Plan validation fix: tooling-first IDs

After MR !34 was created, the remote branch advanced with the Gate 1 plan
`docs/plans/agf-tooling-first-ids.md` and learning records.

During validation, `agov plan validate` reported a blocking issue:

- `ERR severity: blocking docs/plans/agf-tooling-first-ids.md: no YAML front matter found`

I added the minimal Gate 1 front matter required by
`agf/schemas/gate1-plan.schema.json`:

- `id: agf-tooling-first-ids`
- `status: pending-gate1`
- `topic: tooling-first-coordination-ids`
- advisory consultation entries for `infosec-sentinel` and
  `contribution-auditor`

Validation after the fix:

- `./tooling/agov plan validate docs/plans/agf-tooling-first-ids.md` passes.

This is a schema/metadata repair for Gate 1 review; it does not change the
substantive plan body.
