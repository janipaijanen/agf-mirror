---
kind: handoff
from: coder-tester
to: governor-reviewer
model: codex-gpt-5
id: 20260621T164642Z-coder-tester-j5kp
timestamp: 2026-06-21T16:46:42Z
immutable: true
references-to: []
topic: docs-topbar-version-gate2
requires-ack: false
artifacts_owned:
  - docs/coordination/20260621T164642Z-coder-tester-j5kp.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review docs/topbar-version branch for Gate 2 approval
    - Merge if approved
  must_read:
    - MR_DESCRIPTION.md
    - agf/docs/build/hooks/release_info.py
    - agf/docs/site/overrides/partials/header.html
    - agf/docs/site/stylesheets/agf.css
  do_not_touch:
    - docs/coordination/
---

# Gate 2 handoff: docs/topbar-version

Branch: `docs/topbar-version`
MR: https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/71

## What changed

- Reused the existing release-info build hook to expose `version`,
  `commit`, `short_commit`, `branch`, and build timestamp through
  `config.extra.agf_release`.
- Added a Material header override that shows a compact top-bar badge:
  `v<version> <short-commit>`.
- Linked the badge to the generated release information page.
- Added CSS for desktop and mobile top-bar sizing.

## Verification

- `mkdocs build --strict`: clean.
- `git diff --check`: clean.
- Generated HTML includes `agf-header-version`, `v2.1.1`, and the short
  commit on the home page, docs overview, and release-info page.

## Review focus

- Confirm the top bar shows version plus short commit on generated pages.
- Confirm the badge links to `reference/release-info/`.
- Confirm the Material override is limited to the header partial.
