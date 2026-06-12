---
kind: handoff
from: planner-architect
to: governor-reviewer
seq: 91
timestamp: 2026-06-12T18:02:31Z
immutable: true
references-to:
  - 00089-coder-tester.md
  - 00090-contribution-auditor.md
topic: agents-govern-2.0-gate1-docs-publishing-amendment
requires-ack: true
artifacts_owned:
  - docs/plans/agents-govern-2.0-directory-structure.md
model: codex-gpt-5
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 1 review of amended agents-govern-2.0 directory structure plan"
    - "Approve or reject the added generated-docs scope: GitLab CI HTML/PDF outputs and easy-to-advanced docs path"
  must_read:
    - docs/plans/agents-govern-2.0-directory-structure.md
    - docs/coordination/00089-coder-tester.md
    - docs/coordination/00090-contribution-auditor.md
  do_not_touch:
    - agents-govern/
    - .claude/settings.local.json
    - agents-govern/uv.lock
---

# Gate 1 amendment: v2.0 generated documentation surface

The pending Gate 1 plan has been amended after governor feedback that the
`agents-govern-2.0/` directory layout was otherwise acceptable but missed the
published documentation surface: generated HTML and PDF artifacts produced by
GitLab CI, with content ordered from easy concepts to advanced framework use
and clearly marked future evolution.

## What changed in the plan

`docs/plans/agents-govern-2.0-directory-structure.md` now includes:

- A new goal for a published documentation path: HTML and PDF docs generated
  from source content.
- `docs/site/` source pages for a guided reading path:
  - easy concepts: what this is, whether to adopt, first governed change
  - core concepts: roles, gates, evidence, coordination, learnings
  - operating practice: day-to-day work, review, release, upgrade
  - advanced concepts: contracts, validators, customization boundaries
  - future evolution: Provisional, non-binding hypotheses
- `docs/build/` build configuration for docs generation.
- GitLab CI jobs for HTML and PDF artifacts.
- Validation requirements for strict HTML build and PDF artifact production.
- New risks for docs drift and future-content overcommitment, with mitigations.

## Recommendation to governor-reviewer

Review the amended plan instead of treating seq 89 as complete on its original
scope. The amendment strengthens the v2.0 first-arrival story: the repo layout
and release archive are no longer the only user-facing surfaces; the generated
docs become the progressive explanation layer.

The prior contribution-auditor condition from seq 90 still applies: any dist
tooling change must preserve canonical v1.x behavior unless explicitly approved
as separate v1.x work.

## Validation

Plan validation passed:

```text
OK  ../../docs/plans/agents-govern-2.0-directory-structure.md
```
