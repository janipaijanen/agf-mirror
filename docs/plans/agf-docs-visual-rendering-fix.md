---
id: agf-docs-visual-rendering-fix
status: approved
topic: docs-visual-rendering-fix
gate1_approved_by: "governor-reviewer"
gate1_approved_at: "2026-06-21T15:12:58Z"
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: documentation-only; no secrets, auth, or security boundary changes
  - role: contribution-auditor
    status: not-applicable
    reason: documentation-only; no code path, role definition, or governance structure changes
---

# Plan: AGF docs visual rendering fix (Slice 3 of 4)

## Intent

Fix the P1 broken-admonitions issue and add the minimal CSS that expresses the
documented AGF palette in the site (P3). Mobile figure scroll wrapping is
included as a lightweight CSS-only improvement.

Scope is intentionally narrow: this slice does not redesign navigation, rewrite
copy, or reorganise diagrams. The admonitions fix is the blocking item; palette
and mobile scrolling are safe additions with no doc-content changes.

## Affected files

| File | Change |
|---|---|
| `agf/docs/build/mkdocs.yml` | Add `markdown_extensions` and `extra_css` |
| `agf/docs/site/stylesheets/agf.css` | New file — palette tokens, figure scroll |

No markdown content files change in this slice. The `!!! note` admonition blocks
already exist on five pages; adding the extension makes them render correctly.

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

`admonition` makes `!!! note` blocks render as designed.
`pymdownx.details` allows `??? note` collapsible variants (none currently exist,
but the extension is low-cost and expected by Material theme).
`pymdownx.superfences` allows fenced code blocks inside admonitions (none
currently exist, but prevents a silent rendering gap if they are added).

### 2. `agf/docs/site/stylesheets/agf.css` (new file)

Create with these three sections only — no marketing changes, no layout rewrites:

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

/* Figure scroll wrapper — allows wide DOT SVGs to scroll on mobile
   without breaking desktop layout.
   Usage in markdown: wrap <figure> with a <div class="figure-scroll">
   Note: existing pages use plain ![]() — this CSS is ready for future
   use; no existing pages require wrapping in this slice. */
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

The `color-mix()` function is supported in all modern browsers (Chrome 111+,
Firefox 113+, Safari 16.2+). If the target audience includes older browsers, the
title background line can be simplified to a solid light fill instead.

## Migration needed

No. The admonitions already render (without the extension they appear as literal
text). Adding the extension enables correct rendering with no content changes.

## Test strategy

After both changes:

```bash
uv run --project agents-govern --group docs mkdocs build -f agf/docs/build/mkdocs.yml --strict
```

Must pass with zero errors or warnings.

Human verification: open the built site and confirm that `!!! note "AGF v2 layout"`
on `roles-and-gates`, `evidence-and-learnings`, `review-and-release`,
`customizing-the-framework`, and `day-to-day` renders as a styled callout box,
not as literal `!!! note` text.

## Risks

Low. Only two files change, both are additive (no existing content removed). The
extension list is the Material MkDocs standard set; no third-party plugins.

## Rollback

Revert the two lines added to `mkdocs.yml` and delete `stylesheets/agf.css`.
The admonition blocks return to literal text (original state).

## Branch

`docs/visual-rendering-fix`
