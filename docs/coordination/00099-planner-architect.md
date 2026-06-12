---
kind: handoff
from: planner-architect
to: governor-reviewer
seq: 99
timestamp: 2026-06-12T18:48:33Z
immutable: true
references-to:
  - 00098-planner-architect.md
topic: agents-govern-2.0-gate1-docs-toolchain-decision
requires-ack: true
artifacts_owned:
  - docs/plans/agents-govern-2.0-directory-structure.md
model: codex-gpt-5
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 1 review of amended agents-govern-2.0 directory structure plan"
    - "Approve or reject documentation source/toolchain decision: Markdown + MkDocs + Pandoc"
  must_read:
    - docs/plans/agents-govern-2.0-directory-structure.md
    - docs/coordination/00098-planner-architect.md
  do_not_touch:
    - agents-govern/
    - agents-govern/agents/MANIFEST.yaml
    - .claude/settings.local.json
    - agents-govern/uv.lock
---

# Gate 1 amendment: documentation toolchain decision

This handoff refines seq 98 by making the HTML/PDF documentation source
toolchain explicit.

## Recommendation

Use:

- **Markdown** as canonical prose source.
- **MkDocs + MkDocs Material** for GitLab Pages HTML.
- **Pandoc** for the academic PDF/manual from `docs/paper/*.md`.
- **No reStructuredText/Sphinx** for v2.0 unless a future venue explicitly
  requires it.

## Rationale

Markdown is already the framework's dominant authoring format. The current
`agents-govern/pyproject.toml` already has a `docs` dependency group for
MkDocs, MkDocs Material, MkDocs Awesome Pages, MkDocs Gen Files, and
PyMdown Extensions. The changelog records repeated strict `mkdocs build`
checks, so MkDocs is the lowest-friction HTML path.

Pandoc is the right fit for the academic PDF because it can build from
Markdown sources, consume `references.bib`, and produce a durable review
artifact without forcing the GitLab Pages site to become the PDF source.

Raw Markdown-to-HTML scripts would recreate navigation, theme, search, link
checking, and extension concerns already covered by MkDocs. reStructuredText
would add a second prose source format without current repository evidence.

## Plan change

`docs/plans/agents-govern-2.0-directory-structure.md` now records:

- Markdown as canonical source.
- MkDocs strict mode for HTML/GitLab Pages.
- Pandoc for `docs/paper/` PDF output.
- reStructuredText/Sphinx out of scope for v2.0.
- a drift risk for multiple source formats, mitigated by the Markdown + MkDocs
  + Pandoc split.

## Validation

Plan validation passed:

```text
OK  ../../docs/plans/agents-govern-2.0-directory-structure.md
```
