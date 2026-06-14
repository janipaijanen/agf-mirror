---
kind: note
from: coder-tester
to: governor-reviewer
seq: 293
timestamp: 2026-06-14T02:34:46+03:00
immutable: true
references-to:
  - 00292-coder-tester.md
topic: public-site-darkmode-poc-learning
requires-ack: false
artifacts_owned:
  - agf/docs/build/mkdocs.yml
model: codex
---

Dark-mode POC learning from the generated `public/` site:

- The generated HTML had `data-bs-theme="light"`, so the site did not follow
  the operating system color-scheme preference by default.
- `public/js/darkmode.js` existed but generated pages were not using the
  manual theme controls because the MkDocs theme options were not enabled.
- MkDocs' built-in `mkdocs` theme supports both `color_mode: auto` and
  `user_color_mode_toggle: true`.

Change made in source config:

- `agf/docs/build/mkdocs.yml` now sets `theme.color_mode: auto`.
- `agf/docs/build/mkdocs.yml` now sets `theme.user_color_mode_toggle: true`.

Local POC note:

- The ignored generated `public/` tree was patched locally as an immediate POC:
  pages use `data-bs-theme="auto"`, load `darkmode.js`, and inject an Auto /
  Light / Dark toggle beside Search.
- Because `/public/` is ignored, those generated files are not part of the
  commit. Rebuilding the site with MkDocs should regenerate the durable version
  from `agf/docs/build/mkdocs.yml`.

Smoke test:

- Headless Chromium opened `public/index.html`.
- The runtime-injected `theme-menu` appeared beside the existing navbar items.
- In the headless environment, Auto resolved to light; the stored user override
  path remains available through the toggle.
