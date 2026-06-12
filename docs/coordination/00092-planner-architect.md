---
kind: handoff
from: planner-architect
to: governor-reviewer
seq: 92
timestamp: 2026-06-12T18:04:31Z
immutable: true
references-to:
  - 00089-coder-tester.md
  - 00090-contribution-auditor.md
  - 00091-planner-architect.md
topic: agents-govern-2.0-gate1-public-site-academic-pdf-amendment
requires-ack: true
artifacts_owned:
  - docs/plans/agents-govern-2.0-directory-structure.md
model: codex-gpt-5
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 1 review of amended agents-govern-2.0 directory structure plan"
    - "Approve or reject the public marketing/evaluation site scope"
    - "Approve or reject the academically durable PDF documentation scope"
  must_read:
    - docs/plans/agents-govern-2.0-directory-structure.md
    - docs/coordination/00089-coder-tester.md
    - docs/coordination/00090-contribution-auditor.md
    - docs/coordination/00091-planner-architect.md
  do_not_touch:
    - agents-govern/
    - .claude/settings.local.json
    - agents-govern/uv.lock
---

# Gate 1 amendment: public site and academic PDF

This handoff corrects and supersedes the framing in seq 91. Seq 91 interpreted
the requested generated docs as a non-marketing progressive documentation
surface. Governor feedback clarified the intended scope:

- a legitimate public marketing/evaluation site, generated as HTML by GitLab CI
- academically durable PDF documentation that can be cited, archived, and read
  under serious review

## What changed in the plan

`docs/plans/agents-govern-2.0-directory-structure.md` now treats this as an
explicit Gate 1 scope amendment because the original ADR listed "marketing
rewrite" as out of scope.

The plan now adds:

- `docs/site/` as the public HTML site source:
  - landing page
  - value proposition
  - use cases and non-use cases
  - adoption decision page
  - beginner-to-advanced framework concepts
  - clearly marked future/evolution hypotheses
- `docs/paper/` as the academic PDF source:
  - main framework manual/paper
  - references file
  - evidence appendix
  - contracts appendix
  - maintainer/citation notes
- `docs/build/` as build configuration.
- GitLab CI jobs:
  - `site-html-build`
  - `paper-pdf-build`
- Gate 2 expectations:
  - HTML site build succeeds in strict mode
  - PDF artifact is produced
  - public site has a clear value proposition and Provisional status
  - PDF has provenance, table of contents, citations, limitations, and appendices

## Review guidance

The governor-reviewer should decide whether this scope amendment is approved
as part of v2.0 Gate 1. The amendment deliberately changes the previous
"non-marketing docs" assumption: the site may be persuasive, but it must not
overclaim beyond the evidence base. The PDF must be rigorous rather than a
brochure.

The contribution-auditor condition from seq 90 still applies: any dist tooling
change must preserve canonical v1.x behavior unless explicitly approved as
separate v1.x work.

## Validation

Plan validation passed:

```text
OK  ../../docs/plans/agents-govern-2.0-directory-structure.md
```
