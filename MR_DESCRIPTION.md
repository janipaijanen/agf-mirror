# docs: merge Value Proposition into Home; switch to Material theme

## Summary

- Merges `agf/docs/site/value.md` content into `agf/docs/site/index.md` as one reader entry point
- Deletes `agf/docs/site/value.md` and removes its nav entry
- Switches MkDocs theme from `mkdocs` to `material` with automatic dark/light mode via `prefers-color-scheme`

## Why

The Home and Value Proposition pages covered the same reader intent: what AGF is
and why a team would adopt it. Per `marketing-tone-rule`, each claim should be
stated once in canonical form. The merged page preserves the value content and
organizes it as: what AGF is, the situation, what you get, what it costs, who
benefits, where to go next, and evidence.

Use Cases and Evolution were assessed and kept separate. Use Cases has a
distinct job; Evolution mixes shipped capabilities with hypotheses and stays
separate under `implementation-honesty-rule`.

The Material theme was already declared in `agents-govern/pyproject.toml`
(`mkdocs-material>=9.5,<10`) but was not active. CI now installs the MkDocs
Material and docs plugin dependencies needed by the site build job.

## Changes

### `agf/docs/site/index.md`

- Absorbs the Value Proposition page content.
- Keeps current Home framing from main, including the file-based coordination
  model for single-assistant and multi-assistant operation.
- Adds cost and beneficiary sections from the value page.
- Preserves the scope disclaimer that AGF is not a replacement for review
  culture.

### `agf/docs/site/value.md`

- Deleted. Content now lives in `index.md`.

### `agf/docs/build/mkdocs.yml`

- Removes the `Value proposition: value.md` nav entry.
- Switches `theme.name` from `mkdocs` to `material`.
- Adds automatic light/dark palette configuration and manual toggle.
- Adds `navigation.top` and `content.code.copy`.

### `.gitlab-ci.yml`

- Installs the MkDocs docs dependency set required by the Material theme in CI.

## Validation

- `mise exec -- uv run --project agents-govern --group docs mkdocs build -f agf/docs/build/mkdocs.yml --strict`
- `git diff --check`

## Checklist

- [x] No nav entry remains for deleted `value.md`
- [x] Value Proposition content is preserved in `index.md`
- [x] CI installs the docs dependencies required for the configured theme
- [x] No `Co-authored-by` or AI-tool trailers in commits
