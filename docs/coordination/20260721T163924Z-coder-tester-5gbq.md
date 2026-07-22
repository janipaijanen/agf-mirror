---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260721T163924Z-coder-tester-5gbq
timestamp: 2026-07-21T16:39:24Z
model: codex-gpt-5
immutable: true
references-to: []
topic: persistent-agf-page-identity
requires-ack: false
artifacts_owned:
  - agf/docs/site/overrides/main.html
  - agf/docs/site/overrides/partials/header.html
  - agf/docs/site/stylesheets/agf.css
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review persistent AGF branding in the shared header and browser titles.
  must_read:
    - agf/docs/site/overrides/partials/header.html
    - agf/docs/site/overrides/main.html
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Persistent AGF page identity ready for Gate 2

The shared MkDocs templates now keep the AGF identity visible across the
entire site. At the top of a page the header shows `AGF`; after scrolling it
shows the linked brand-qualified title `AGF / <page title>`. Browser titles
use `AGF — <page title>`, placing the product identity before truncation in a
crowded tab bar.

This is a small presentation correction implemented under the fast-track
path: one shared theme component, its CSS, and the global title override. It
does not change navigation hierarchy or page content.

Accessibility and responsive details:

- `AGF` links to the site home and has an explicit accessible label;
- the visual slash is hidden from assistive technology;
- only the page-title portion ellipsizes on narrow screens;
- no JavaScript or per-page metadata is required.

Validation passed:

- strict MkDocs build;
- generated home, coordination-protocol, and contracts-and-validators pages
  inspected for the expected header markup and browser titles;
- `git diff --check`.
