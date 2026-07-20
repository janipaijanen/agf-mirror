---
id: agf-nutshell-flyer-in-repo
status: approved
topic: agf-nutshell-flyer-in-repo
gate1_approved_by: "human governor; approved in chat when selecting the CI-generation approach"
gate1_approved_at: "2026-07-20T19:09:49Z"
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: static visual asset + CI job pulling a pinned public image; no
      secrets, auth, or new attack surface beyond normal CI dependency pinning
  - role: contribution-auditor
    status: not-applicable
    reason: single-contributor infra/docs addition, no incoming external
      contribution to audit
  - role: fact-checker
    status: not-applicable
    reason: no comparative or measured claims introduced; the flyer's own
      content was already fact-checked/reviewed earlier this session
---

# Plan: bring the AGF nutshell flyer into the repo, CI-verified

## Problem statement

The "AGF in a Nutshell" one-pager currently exists only as a Claude Artifact
(a scratchpad HTML file with embedded base64 fonts, never committed). The
human governor wants it: (a) committed as a first-class repo asset, (b) its
PNG embedded in the root `README.md` and the docs site's first page, and
(c) kept fresh — verified as part of the release process, not silently
allowed to drift (e.g. showing a stale AGF version number forever).

## Precedent already in this repo

The 8 existing Graphviz diagrams (`agf/docs/figures/src/*.dot`) are **not**
regenerated in CI. Both the `.dot` source and the pre-rendered `.svg` output
are committed (dual-copied to `agf/docs/figures/` and
`agf/docs/site/reference/figures/`); CI only validates the MkDocs build
references them correctly. The human governor confirmed (chat, when asked)
this flyer should follow the same convention rather than adding an
auto-commit-from-CI bot, which would be in tension with
`merge-request-workflow-rule` (no automated commits to main outside a
reviewed MR).

## Design

**Source (committed):** `agf/docs/figures/src/agf-nutshell-flyer/`
- `build_flyer.py` — generator script (adapted from the Artifact version),
  takes `--version` and `--date` (default: read `agf/VERSION`, current UTC
  date) and writes the final self-contained HTML.
- `fonts/*.b64` — the 5 base64-encoded IBM Plex font files the template
  embeds (Serif 600/700, Sans variable, Mono 400/500).

**Output (committed, dual-copy convention):**
- `agf/docs/figures/agf-nutshell-flyer.png`
- `agf/docs/site/reference/figures/agf-nutshell-flyer.png`
(byte-identical, same convention as every other figure)

The intermediate HTML is not committed — same as the `.dot` → `.svg` case,
only source + final rendered asset live in the tree.

**CI job (new):** `agf:flyer:verify`, mirroring `agf:dist:validate`'s rule
shape (MR + branch + tag, path-scoped to the flyer's own source directory
and `agf/VERSION` so it doesn't run on unrelated changes):
- Uses a pinned headless-browser image (first new browser-capable CI
  dependency in this repo — chosen and pinned by exact tag during
  implementation, not `:latest`).
- Regenerates the PNG fresh from the committed source, diffs it against the
  committed PNG.
- **Fails the pipeline if they differ** — this is a verify-only gate, not an
  auto-commit. A stale flyer (e.g. after a version bump) must be
  re-rendered and committed by a human/agent through a normal reviewed MR,
  exactly like updating any other diagram.

## Affected files

- New: `agf/docs/figures/src/agf-nutshell-flyer/build_flyer.py`, `fonts/*.b64`
- New: `agf/docs/figures/agf-nutshell-flyer.png`,
  `agf/docs/site/reference/figures/agf-nutshell-flyer.png`
- Modified: `.gitlab-ci.yml` (new `agf:flyer:verify` job)
- Modified: `README.md` (embed the PNG, developer-facing placement — a
  visual overview near the top, not replacing the developer-focused body
  per `documentation-audience-rule`)
- Modified: `agf/docs/site/docs/index.md` (embed the PNG on the docs site's
  first page)
- Modified flyer content itself: license footer corrected to the canonical
  dual-license phrasing already used at `agf/docs/site/index.md` ("CC BY-SA
  4.0, with separate commercial licensing available. Tooling code: MIT.")
  — was previously stating only CC BY-SA 4.0, which underclaimed.

## Test strategy

- `mkdocs build -f agf/docs/build/mkdocs.yml --strict` after the docs/index.md
  and figure changes.
- Visual inspection of the rendered PNG at normal size before committing
  (per `visible-artifact-verification-rule`), plus confirm README/docs
  pages actually display it (not just that the file exists).
- Local dry-run of `build_flyer.py` + render step before writing the CI job,
  so the CI job's command is known-working, not guessed.
- Both SVG-equivalent copies (`agf/docs/figures/*.png` and
  `agf/docs/site/reference/figures/*.png`) confirmed byte-identical.

## Downstream-impact preview

Adding `agf:flyer:verify` does not touch `agf/rules/**` (not a
downstream-impact-rule trigger path), but is noted here for transparency:
ADDITIVE — a new CI job, path-scoped, does not change any existing job's
behavior or any adopter-facing contract.

## Risks

- Pinned headless-browser CI image adds real weight (image pull time) to
  the pipelines it runs on — mitigated by path-scoping so it only runs when
  the flyer source or `agf/VERSION` actually changes, plus tag pipelines
  (release, low frequency).
- Font-embedding makes the generator script's font asset files
  non-trivial in size (a few hundred KB total) — acceptable, matches the
  Artifact version's existing size budget.

## Rollback

Revert the commit(s); no schema, no data migration, no other job depends on
`agf:flyer:verify`.

## Next: release

Once this lands, the human governor asked to cut a release with a minor
version bump (`agf/VERSION` 2.2.6 → 2.3.0, per `semver-no-carry-rule`),
following `release-via-merge-request-rule`: release branch off main with the
VERSION/CHANGELOG bump only, MR, merge, tag the merge commit, push the tag.
