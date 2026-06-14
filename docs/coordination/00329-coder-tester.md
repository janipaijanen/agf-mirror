---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 329
timestamp: 2026-06-14T19:48:47Z
immutable: true
references-to:
  - 00328-coder-tester.md
topic: mkdocs-site-html-artifact-path-gate2
requires-ack: true
artifacts_owned: []
model: codex-gpt-5
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review GitLab MR !4 for the MkDocs review artifact path fix.
    - Confirm the `site-html-build` artifact path should be `agf/site-html/`.
    - Merge to `main` if approved, deleting the source branch.
  must_read:
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/4
    - .gitlab-ci.yml
    - docs/coordination/00328-coder-tester.md
  do_not_touch:
    - main
    - master
    - .env
    - agents-govern/
---

Gate 2 handoff for the MkDocs HTML review artifact path repair.

MR: https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/4

The CI log showed MkDocs building to `agf/site-html`, while GitLab tried to
upload `site-html/` from the repository root. This branch changes the
`site-html-build` artifact path to `agf/site-html/`.

Validation completed locally:

- `agov coord validate docs/coordination/00328-coder-tester.md`
- `YamlElixir.read_from_file!("../../.gitlab-ci.yml")`
- `git diff --check`
- `mkdocs build -f agf/docs/build/mkdocs.yml --strict`
- verified generated `agf/site-html/index.html` exists

Generated local site output was removed after validation.
