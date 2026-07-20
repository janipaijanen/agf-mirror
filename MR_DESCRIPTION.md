# docs: make the twenty-minute path's install step mechanically concrete

Branch: `docs/readme-twenty-minute-path-clarity`

## Summary

The "Twenty-minute path" section's install step pointed only to
`install-and-layouts.md`, leaving a first-time reader to click through
before discovering the actual mechanics are "download an archive, paste
one prompt." States that inline instead.

## Why

A reviewer flagged that a human reading README top-to-bottom had no
plain statement of how to actually start: which files to download, from
where, and what to tell the AI assistant. The mechanics already existed
in `install-and-layouts.md`'s tip block; README just didn't surface them.

## Files changed

| File | Change |
|---|---|
| `README.md` | Step 3 of "Twenty-minute path" now states inline: download `agf-<version>.tar.gz`/`.zip` (and matching `agov-<version>-<platform>` archive) from the Releases page, put it in `tmp/`, and the exact one-line prompt to give the coding assistant — quoted verbatim from `install-and-layouts.md` rather than paraphrased |

## Deliberate omission

Not linking to specific version-numbered asset files — those go stale
every release. The Releases page link always shows the current release.

## Review checklist

- [ ] Quoted prompt matches `install-and-layouts.md`'s tip block verbatim
- [ ] No version-specific asset URL hardcoded
- [ ] No `Co-authored-by` trailers in commit
