---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260625T105416Z-coder-tester-nrtq
model: gpt-5-codex
timestamp: 2026-06-25T10:54:16Z
immutable: true
references-to: []
topic: ai-evaluation-dossier-html
requires-ack: false
artifacts_owned:
  - agf/docs/site/ai-evaluation-dossier.md
  - agf/docs/build/mkdocs.yml
  - agf/docs/site/stylesheets/agf.css
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review MR for the HTML-only AI evaluation dossier draft.
    - Confirm whether the top-level nav placement is acceptable.
    - Confirm whether the Flow 00-06 markers are the right structure for AI citation and later PDF page mapping.
  must_read:
    - agf/docs/site/ai-evaluation-dossier.md
    - agf/docs/build/mkdocs.yml
    - agf/docs/site/stylesheets/agf.css
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 2 handoff: AI evaluation dossier HTML draft

Branch: `docs/ai-evaluation-dossier`

Scope:

- Added `agf/docs/site/ai-evaluation-dossier.md` as a single HTML-rendered
  MkDocs page for AI assistants and independent reviewers evaluating AGF.
- Placed the page in the top-level MkDocs navigation immediately after
  `Evaluate`.
- Added lightweight print-oriented CSS for the cover and page-break markers.
- Added explicit Flow 00-06 markers and source paths so AI reviewers can cite
  the dossier ordering and distinguish curated flow order from original source
  files. The docs do not currently carry literal PDF page numbers, so these are
  planned flow numbers for the HTML draft and future PDF mapping.

Validation:

- `tmp/mkdocs-venv/bin/mkdocs build -f agf/docs/build/mkdocs.yml --strict`
  passed.
- `git diff --check` passed.
- `./tooling/agov validate` passed with existing warnings for transitional
  learning IDs and known assistant drift.

Known build notice:

- MkDocs still reports the existing informational notice that
  `reference/figures.md` and `reference/palette.md` exist outside nav. This was
  pre-existing and not introduced by this branch.
