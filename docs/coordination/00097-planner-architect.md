---
kind: handoff
from: planner-architect
to: governor-reviewer
seq: 97
timestamp: 2026-06-12T18:14:32Z
immutable: true
references-to:
  - 00096-planner-architect.md
topic: agents-govern-2.0-gate1-graphviz-ci-requirement
requires-ack: true
artifacts_owned:
  - docs/plans/agents-govern-2.0-directory-structure.md
model: codex-gpt-5
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 1 review of amended agents-govern-2.0 directory structure plan"
    - "Approve or reject explicit Graphviz installation requirement in CI figures-build job"
  must_read:
    - docs/plans/agents-govern-2.0-directory-structure.md
    - docs/coordination/00096-planner-architect.md
  do_not_touch:
    - agents-govern/
    - agents-govern/agents/MANIFEST.yaml
    - .claude/settings.local.json
    - agents-govern/uv.lock
---

# Gate 1 amendment: Graphviz required in CI

This handoff refines seq 96. The plan now requires CI to explicitly install
Graphviz for the `figures-build` job.

## Requirement

`figures-build` must:

- install Graphviz explicitly
- render SVG/PNG/PDF figure outputs in CI
- fail CI if rendered figure generation fails

`.dot` or Mermaid fallbacks are acceptable for local/offline development only,
not as the CI review artifact path.

## Validation

Plan validation passed:

```text
OK  ../../docs/plans/agents-govern-2.0-directory-structure.md
```
