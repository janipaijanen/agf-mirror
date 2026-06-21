# docs: show version and commit in the docs top bar

## What changed

- `release_info.py` now exposes build provenance through
  `config.extra.agf_release` for templates.
- `mkdocs.yml` enables a Material theme override directory.
- A header partial override adds a compact top-bar badge linking to
  `reference/release-info/`.
- `agf.css` styles the badge so it remains visible on desktop and mobile.

## Verification

- `mkdocs build --strict`: clean.
- Generated HTML includes `agf-header-version`, the framework version, and the
  short commit on the home page, docs overview, and release-info page.

## Review Focus

Please verify:

- [ ] The top bar shows the framework version and short commit on generated pages.
- [ ] The badge links to `reference/release-info/`.
- [ ] The release metadata comes from the existing build-time provenance hook.
- [ ] The Material override is limited to the header partial.
