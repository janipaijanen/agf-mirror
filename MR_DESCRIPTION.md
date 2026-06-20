# docs: merge Value Proposition into Home; switch to Material theme

## Summary

- Merges `agf/docs/site/value.md` content into `agf/docs/site/index.md` — one entry point instead of two
- Deletes `agf/docs/site/value.md` and removes its nav entry
- Switches MkDocs theme from `mkdocs` (basic) to `material` with automatic dark/light mode via `prefers-color-scheme`

## Why

The Home and Value Proposition pages covered the same reader intent (what is AGF and why adopt it?) with overlapping content. Per `marketing-tone-rule`, each claim is stated once in canonical form; the duplication invited drift. The merged page preserves all content from both sources, organised as: what AGF is → the situation → what you get → what it costs → who benefits → where to go next → evidence.

Use Cases and Evolution pages were assessed and kept separate: Use Cases is distinct in purpose; Evolution mixes hypothesis with shipped capabilities and must stay separate per `implementation-honesty-rule`.

The Material theme was already declared in `agents-govern/pyproject.toml` (`mkdocs-material>=9.5,<10`) but never activated. No new packages required. The `prefers-color-scheme` palette reads the user's OS/browser preference automatically; a toggle allows manual override.

## Changes

### `agf/docs/site/index.md`
- Absorbed all content from `value.md`
- Structure: what AGF is and is not → the situation → what you get → what it costs → who benefits → where to go next → evidence
- Removed duplication: "Why this exists" and "The situation" unified into one narrative
- Scope disclaimer ("not a replacement for review culture") preserved verbatim from value.md

### `agf/docs/site/value.md`
- Deleted (`git rm`) — all content now lives in `index.md`

### `agf/docs/build/mkdocs.yml`
- Removed `- Value proposition: value.md` nav entry
- `theme.name` changed from `mkdocs` to `material`
- Added `palette` block: `prefers-color-scheme: light` → `default`/indigo; `prefers-color-scheme: dark` → `slate`/indigo; both with manual toggle
- Added `features: [navigation.top, content.code.copy]`

## Verification

- `mkdocs build --strict` passes (0.27s, no warnings, no broken links)
- Dark/light mode switches automatically on browser preference; toggle visible in header

## Checklist

- [x] No new nav entry added for deleted `value.md`
- [x] All `value.md` content preserved in `index.md`
- [x] No new package dependencies
- [x] Strict-mode build passes
- [x] No `Co-authored-by` or AI-tool trailers in commits
