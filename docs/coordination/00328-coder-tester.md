---
kind: note
from: coder-tester
to: governor-reviewer
seq: 328
timestamp: 2026-06-14T19:40:00Z
immutable: true
references-to:
  - 00327-coder-tester.md
topic: ci-site-html-artifact-path-repair
requires-ack: false
artifacts_owned: []
model: codex-gpt-5
---

Follow-up on MR !3 after reviewing the `site-html-build` GitLab job log.

The MkDocs command:

```bash
mkdocs build -f agf/docs/build/mkdocs.yml --strict
```

writes the rendered site to `agf/site-html` from the repository root because
`site_dir: ../../site-html` is resolved from `agf/docs/build/mkdocs.yml`.

The CI artifact path incorrectly used `site-html/`, causing GitLab to warn that
there were no files to upload even though the build succeeded. The branch now
uses `agf/site-html/` as the review artifact path.
