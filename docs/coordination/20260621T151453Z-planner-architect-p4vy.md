---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260621T151453Z-planner-architect-p4vy
timestamp: 2026-06-21T15:14:53Z
immutable: true
references-to:
  - 20260621T143538Z-coder-tester-drq5.md
  - 20260621T151007Z-planner-architect-n2fq.md
topic: docs-visual-rendering-fix-slice3
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Add markdown_extensions and extra_css to agf/docs/build/mkdocs.yml
    - Create agf/docs/site/stylesheets/agf.css with palette tokens and figure scroll
  must_read:
    - docs/plans/agf-docs-visual-rendering-fix.md
    - agf/docs/build/mkdocs.yml
    - agf/docs/site/reference/palette.md
  do_not_touch:
    - agf/docs/site/docs/start/first-governed-change.md
    - agf/docs/site/docs/index.md
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Handoff: Docs visual rendering fix (Slice 3 of 4)

Governor-approved remediation slice 3. Implement after Slices 1 and 2 are
merged (they are — merged at `ecd241d`).

## What to implement

### 1. `agf/docs/build/mkdocs.yml`

After the `strict: true` line, add:

```yaml
markdown_extensions:
  - admonition
  - pymdownx.details
  - pymdownx.superfences

extra_css:
  - stylesheets/agf.css
```

The `docs_dir` is `../site`, so `stylesheets/agf.css` resolves to
`agf/docs/site/stylesheets/agf.css`.

### 2. `agf/docs/site/stylesheets/agf.css` (new file)

```css
/* AGF palette tokens — matches reference/palette.md */
:root {
  --agf-blue-mid:   #0077BB;
  --agf-blue-light: #99CCEE;
  --agf-green-mid:  #009944;
  --agf-amber:      #EE7700;
  --agf-red-mid:    #CC3311;
  --agf-grey-mid:   #888888;
}

/* Mobile figure scroll — ready for future use; no existing pages require
   wrapping in this slice */
.figure-scroll {
  overflow-x: auto;
  -webkit-overflow-scrolling: touch;
}

/* Admonition colour accents using AGF palette */
.md-typeset .admonition.note,
.md-typeset details.note {
  border-color: var(--agf-blue-mid);
}
.md-typeset .admonition.note > .admonition-title,
.md-typeset details.note > summary {
  background-color: color-mix(in srgb, var(--agf-blue-light) 30%, transparent);
}
```

## Pages that use admonitions (verify all render correctly)

After the build, confirm these five pages show styled callout boxes — not literal
`!!! note` text:

- `docs/concepts/roles-and-gates.md`
- `docs/concepts/evidence-and-learnings.md`
- `docs/operate/review-and-release.md`
- `docs/advanced/customizing-the-framework.md`
- `docs/operate/day-to-day.md`

All five use `!!! note "AGF v2 layout"` at or near line 8.

## Validation before Gate 2

```bash
uv run --project agents-govern --group docs mkdocs build -f agf/docs/build/mkdocs.yml --strict
```

Must pass with zero errors or warnings.

Note the admonitions extension and Material theme CSS changes are non-breaking:
strict mode only fails on broken links, not rendering.

## Branch

`docs/visual-rendering-fix`

## Do not touch

`agf/docs/site/docs/start/first-governed-change.md` and all other `.md` content
files — no content changes in this slice.
