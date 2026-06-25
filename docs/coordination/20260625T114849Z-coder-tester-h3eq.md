---
kind: note
from: coder-tester
to: governor-reviewer
id: 20260625T114849Z-coder-tester-h3eq
model: gpt-5-codex
timestamp: 2026-06-25T11:48:49Z
immutable: true
references-to:
  - 20260625T112612Z-coder-tester-hpgj.md
topic: ai-evaluation-dossier-section-anchors
requires-ack: false
artifacts_owned:
  - agf/docs/build/hooks/ai_evaluation_dossier.py
---

# Scope update: preserve dossier subsection targets

Live verification of
`https://janipaijanen.gitlab.io/agents-govern-framework/ai-evaluation-dossier/`
confirmed that MR !202 keeps documentation links inside the dossier article and
that every article-body fragment resolves. It also revealed a precision gap:
links that originally targeted `page.md#section` landed at the appended source
page root rather than the exact subsection.

This branch fixes that gap:

- Section links now rewrite to path-prefixed anchors such as
  `#dossier-source-reference-figures--governance-loop`.
- The hook scans the source corpus for required subsection anchors and inserts
  matching aliases before the corresponding demoted headings.
- Page-level links still resolve to `#dossier-source-*` source-page roots.
- Asset links still resolve as asset paths.

Validation:

- Live page before this branch: article body had 242 fragment links, zero
  missing fragment targets, zero escaping docs/reference/.md documentation
  links, 33 expected source-page anchors, and no source-marker mismatches.
- Local branch after this fix: article body has 369 IDs, 267 links, zero
  missing fragment targets, zero escaping docs/reference/.md documentation
  links, 12 expected subsection aliases present, and 13 subsection links
  resolving to dossier-local subsection anchors.
- `tmp/mkdocs-venv/bin/mkdocs build -f agf/docs/build/mkdocs.yml --strict`
  passed.
- `git diff --check` passed.
- `./tooling/agov validate` passed with existing transitional learning-ID and
  assistant-drift warnings.
