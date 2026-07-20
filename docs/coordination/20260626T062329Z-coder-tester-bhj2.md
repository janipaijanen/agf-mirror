---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260626T062329Z-coder-tester-bhj2
model: gpt-5-codex
timestamp: 2026-06-26T06:23:29Z
immutable: true
references-to:
  - 20260626T055951Z-coder-tester-76jm.md
topic: framework-map-gate2
requires-ack: false
artifacts_owned:
  - agf/docs/site/docs/start/framework-map.md
  - agf/docs/build/mkdocs.yml
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review framework map docs MR.
    - If merged, refresh release v2.2.6 so the patch release includes this page.
  must_read:
    - agf/docs/site/docs/start/framework-map.md
    - agf/docs/build/mkdocs.yml
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Framework Map Gate 2 Handoff

This branch adds a new orientation page:

- `agf/docs/site/docs/start/framework-map.md`

It also adds the page to MkDocs navigation under Documentation -> Start, before
Should you adopt?.

The page includes:

- a Mermaid conceptual map for the load-bearing AGF concepts;
- explicit "included" and "excluded" sections to keep the map from becoming an
  exhaustive reference;
- AI-readable node and edge guide tables with links to existing pages;
- "Next" links into adoption decision and first governed change pages.

Local validation:

- `tmp/mkdocs-venv/bin/mkdocs build -f agf/docs/build/mkdocs.yml --strict`
- `git diff --check`
- rendered page contains Mermaid and expected links;
- AI evaluation dossier includes the page as `Source Flow 07.34 - Framework map`.

Release note:

MR !206 (`Release v2.2.6`) is currently open. If this framework-map MR merges
first, refresh !206 from `main` and add a changelog bullet so v2.2.6 includes
the map.
