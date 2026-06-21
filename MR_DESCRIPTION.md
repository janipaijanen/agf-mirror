# docs: admonition extensions and AGF palette CSS (Slice 3)

Governor-approved remediation slice 3.
Plan: `docs/plans/agf-docs-visual-rendering-fix.md`

## What changed

- `agf/docs/build/mkdocs.yml` — added `markdown_extensions` (admonition,
  pymdownx.details, pymdownx.superfences) and `extra_css: stylesheets/agf.css`
  after the `strict: true` line.
- `agf/docs/site/stylesheets/agf.css` — new file with AGF palette CSS tokens
  (matching `reference/palette.md`) and admonition colour accents.

## Verification

- `mkdocs build --strict`: clean.
- Five pages that use `!!! note "AGF v2 layout"` callouts will now render
  styled boxes rather than literal `!!! note` text.

## Does not touch

Content `.md` files — all changes are build config and CSS only.
