# AGF nutshell flyer — source

A one-page HTML orientation sheet ("AGF in a Nutshell"), rendered to
`agf/docs/figures/agf-nutshell-flyer.png` (and the byte-identical site copy
at `agf/docs/site/reference/figures/agf-nutshell-flyer.png`).

Same convention as the Graphviz diagrams in `agf/docs/figures/src/*.dot`:
source lives here, the rendered output is committed alongside it, and
nothing regenerates automatically — a human or agent re-renders and commits
through a normal reviewed MR when the flyer content or `agf/VERSION`
changes. CI only verifies the committed PNG isn't stale (see
`agf:flyer:verify` in `.gitlab-ci.yml`).

## Files

- `template.html` — the full page, with `__FONT_*__` and `__AGF_VERSION__`
  / `__BUILD_DATE__` tokens in place of the embedded font data and the
  footer's provenance line.
- `fonts/*.b64` — base64-encoded IBM Plex Serif/Sans/Mono `.woff2` files,
  substituted into the `__FONT_*__` tokens at build time.
- `build_flyer.py` — substitutes the tokens and writes a self-contained
  HTML file. No dependencies beyond the Python standard library.

## Regenerating the PNG

Two steps: generate the HTML, then screenshot it with a headless browser
(any Chromium-based one works; this was authored against Google Chrome /
Chromium's `--headless --screenshot` flag).

```bash
cd agf/docs/figures/src/agf-nutshell-flyer
python3 build_flyer.py -o /tmp/agf-nutshell-flyer.html
chromium --headless --disable-gpu --no-sandbox --window-size=1600,1131 \
  --screenshot=../../agf-nutshell-flyer.png file:///tmp/agf-nutshell-flyer.html
cp ../../agf-nutshell-flyer.png ../../../site/reference/figures/agf-nutshell-flyer.png
```

`build_flyer.py --version X.Y.Z --date YYYY-MM-DD` overrides the defaults
(`agf/VERSION` and today's UTC date) if you need a specific stamp.

## Editing the content

Edit `template.html` directly — it's a normal (large) HTML file with
`__FONT_*__`/`__AGF_VERSION__`/`__BUILD_DATE__` placeholders where
`build_flyer.py` substitutes real values. Don't hand-edit a generated
output file; edit the template and regenerate.
