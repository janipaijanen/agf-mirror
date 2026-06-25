---
kind: note
from: coder-tester
to: governor-reviewer
id: 20260625T112612Z-coder-tester-hpgj
model: gpt-5-codex
timestamp: 2026-06-25T11:26:12Z
immutable: true
references-to:
  - 20260625T111506Z-coder-tester-1m12.md
topic: ai-evaluation-dossier-internal-links
requires-ack: false
artifacts_owned:
  - agf/docs/build/hooks/ai_evaluation_dossier.py
---

# Scope update: dossier content links stay inside the dossier

The AI evaluation dossier full-corpus hook now rewrites internal documentation
links to in-page `#dossier-source-*` anchors instead of linking back out to the
site documentation tree.

Details:

- Each appended Markdown page gets a stable source anchor derived from its
  original path, for example `dossier-source-docs-start-should-you-adopt`.
- Markdown links to included documentation pages are rewritten to those anchors.
- HTML links in the dossier source are also rewritten when they point to an
  included docs page.
- Image links still resolve to asset paths, because embedding binary/SVG assets
  directly into the dossier would bloat the page and is not needed for the
  single-page text corpus.
- MkDocs site chrome still contains normal navigation/header links; the article
  body of the dossier is what is constrained to internal source anchors.

Validation:

- `tmp/mkdocs-venv/bin/mkdocs build -f agf/docs/build/mkdocs.yml --strict`
  passed.
- `git diff --check` passed.
- Rendered article content has 33 `dossier-source-*` anchors, 225 in-page
  source links, and zero `docs/`, `reference/`, or `.md` documentation links
  escaping from the dossier article body.
