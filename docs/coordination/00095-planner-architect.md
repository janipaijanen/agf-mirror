---
kind: handoff
from: planner-architect
to: governor-reviewer
seq: 95
timestamp: 2026-06-12T18:09:55Z
immutable: true
references-to:
  - 00093-planner-architect.md
  - 00094-planner-architect.md
topic: agents-govern-2.0-gate1-downstream-impact-exit-docs
requires-ack: true
artifacts_owned:
  - docs/plans/agents-govern-2.0-directory-structure.md
model: codex-gpt-5
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 1 review of amended agents-govern-2.0 directory structure plan"
    - "Approve or reject mandatory early end-user docs for downstream impact and exit/decommission"
  must_read:
    - docs/plans/agents-govern-2.0-directory-structure.md
    - docs/coordination/00094-planner-architect.md
  do_not_touch:
    - agents-govern/
    - agents-govern/agents/MANIFEST.yaml
    - .claude/settings.local.json
    - agents-govern/uv.lock
---

# Gate 1 amendment: downstream impact and exit docs

This handoff refines seq 94. The progressive end-user HTML documentation must
explain adoption consequences and reversibility early, before advanced topics.

## Required pages

The amended plan now requires these GitLab Pages end-user docs:

- `docs/site/docs/adopt/downstream-impact.md`
- `docs/site/docs/adopt/exit-and-decommission.md`

## Required coverage

`downstream-impact.md` must explain:

- files/directories added or changed in Layout A and Layout B
- CI, hook, validation, and review-gate changes
- human/agent role obligations introduced by adoption
- ongoing maintenance costs: learning records, upgrades, review overhead,
  coordination hygiene, and version pinning
- boundaries: what the framework does not control, enforce, or guarantee
- risk tradeoffs and signs that adoption is too heavy for the project

`exit-and-decommission.md` must explain:

- whether to downgrade compliance, pause usage, or remove adoption
- how to remove copied agent definitions, rules, skills, hooks, CI jobs,
  coordination conventions, and local learning files where appropriate
- how to preserve audit history without pretending the project remains governed
- how to verify removal with search checks for `agov`, `agents-govern`, copied
  rule paths, CI includes, and assistant command/skill references
- how to record the exit decision in project history

## Review guidance

These pages should be treated as adoption safety material, not optional
reference appendices. A project should understand downstream burden and exit
path before it commits to using the framework.

## Validation

Plan validation passed:

```text
OK  ../../docs/plans/agents-govern-2.0-directory-structure.md
```
