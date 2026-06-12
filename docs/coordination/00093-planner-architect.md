---
kind: handoff
from: planner-architect
to: governor-reviewer
seq: 93
timestamp: 2026-06-12T18:06:25Z
immutable: true
references-to:
  - 00089-coder-tester.md
  - 00090-contribution-auditor.md
  - 00091-planner-architect.md
  - 00092-planner-architect.md
topic: agents-govern-2.0-gate1-gitlab-pages-end-user-docs-amendment
requires-ack: true
artifacts_owned:
  - docs/plans/agents-govern-2.0-directory-structure.md
model: codex-gpt-5
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 1 review of amended agents-govern-2.0 directory structure plan"
    - "Approve or reject the GitLab Pages site scope: marketing/evaluation plus progressive end-user HTML documentation"
    - "Approve or reject the academically durable PDF documentation scope"
  must_read:
    - docs/plans/agents-govern-2.0-directory-structure.md
    - docs/coordination/00089-coder-tester.md
    - docs/coordination/00090-contribution-auditor.md
    - docs/coordination/00092-planner-architect.md
  do_not_touch:
    - agents-govern/
    - .claude/settings.local.json
    - agents-govern/uv.lock
---

# Gate 1 amendment: GitLab Pages end-user docs

This handoff refines seq 92. The to-be GitLab site is not only a public
marketing/evaluation surface; it must also include progressive HTML
documentation for end users.

## What changed in the plan

`docs/plans/agents-govern-2.0-directory-structure.md` now defines
`docs/site/` as the GitLab Pages source with two connected lanes:

- **Marketing/evaluation lane** — landing page, value proposition, use cases,
  non-use cases, evidence, adoption status, and Provisional status.
- **Progressive end-user documentation lane** — docs index, adoption decision,
  first governed change, install/layout guidance, concepts, day-to-day
  operation, review/release/upgrade, advanced contracts/validators, and
  customization boundaries.

The plan keeps `docs/paper/` as the academically durable PDF source, with
provenance, citations, limitations, evidence appendix, and contract appendix.

## Review guidance

Governor-reviewer should review the current plan rather than seq 91 or seq 92
in isolation. The current Gate 1 question is:

- Is the `agents-govern-2.0/` directory structure approved?
- Is the GitLab Pages site approved as both marketing/evaluation content and
  progressive end-user HTML documentation?
- Is the academic PDF/manual track approved?

The contribution-auditor condition from seq 90 still applies: any dist tooling
change must preserve canonical v1.x behavior unless explicitly approved as
separate v1.x work.

## Validation

Plan validation passed:

```text
OK  ../../docs/plans/agents-govern-2.0-directory-structure.md
```
