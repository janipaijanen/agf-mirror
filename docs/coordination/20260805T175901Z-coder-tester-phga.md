---
artifacts_owned: []
from: coder-tester
handoff:
  do_not_touch:
  - agf/docs/build/mkdocs.yml
  - agf/docs/site/stylesheets/agf.css
  - agf/docs/site/index.md
  - agf/docs/site/reference/palette.md
  - agf/docs/build/hooks/check_palette_contrast.py
  - agf/docs/build/hooks/check_nav_inventory.py
  must_read:
  - README.md
  next_agent: contribution-auditor
  status: ready
  task:
  - "Focused re-review of the `B-MR1193-CA-3` residual only, at new exact
    head `d8f38c24`."
id: 20260805T175901Z-coder-tester-phga
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260805T163551Z-contribution-auditor-ksqm.md
- 20260805T153006Z-coder-tester-nq7h.md
- 20260805T150801Z-contribution-auditor-h7rq.md
requires-ack: true
timestamp: 2026-08-05T17:59:01Z
to: contribution-auditor
topic: mr1193-readme-slice-b3-residual-rereview
---

# Focused re-review request: MR !1193, `B-MR1193-CA-3` residual only, exact head `d8f38c24`

Addresses the sole residual finding from `ksqm` (contribution-auditor,
2026-08-05T16:35:51Z): the install command used a GitHub-style release-
download URL that returns HTTP 404. `B-MR1193-CA-1` and `B-MR1193-CA-2`
were already confirmed resolved in that same review and are not reopened
here.

New exact head: `d8f38c249e1de69f3a1b6072e60ed68cd09e803d` (`d8f38c24` for
short), one commit on top of the previously reviewed `cf066993`.

## Fix applied

Replaced the invalid GitHub-style route with the actual GitLab
generic-package-registry URL pattern (tag path `vX.Y.Z`, filename
`agf-X.Y.Z.tar.gz`, no `v` prefix on the filename — matching the real
asset naming `ksqm` identified via the Releases API):

```bash
curl -LO https://gitlab.com/api/v4/projects/janipaijanen%2Fagents-govern-framework/packages/generic/agf/vX.Y.Z/agf-X.Y.Z.tar.gz
```

## Verification

- Independently reproduced `ksqm`'s finding first: confirmed the prior
  command 404s (`.../-/releases/download/v2.4.11/agf-v2.4.11.tar.gz`).
- Verified the corrected command end-to-end, not just a HEAD probe:
  substituted the current stable release (`v2.4.11`), ran the exact `curl
  -LO` command from the README, confirmed a 1069K `gzip compressed data`
  file was downloaded, and confirmed `tar tzf` lists real `agf/` contents
  (`agf/ADOPTING.md`, `agf/CHANGELOG.md`, `agf/LICENSE`, `agf/VERSION`,
  ...). Scratch file removed after verification, not committed.
- `python3 -m unittest tooling.tests.test_public_feature_summary`: 2/2
  pass.
- `python3 -m unittest discover -s tooling/tests -p 'test_*.py'`: 3/3
  pass.
- `python3 -m unittest discover -s agf/docs/build/hooks -p 'test_*.py'`:
  27/27 pass.
- `agov plan validate docs/plans/20260803T192952Z-pages-readme-visual-refresh-v251.md`:
  OK, severity blocking, pass.
- `agov validate adopter-docs`: OK.
- `agov validate` (full): `readme_stats` OK; the one pre-existing
  `assistant_drift` WARN (51 findings, `.claude/rules`) is unrelated and
  predates this change.
- All 26 relative markdown links in `README.md` resolve to real files
  (scripted check).
- CI: pipeline `2734876660` on exact head `d8f38c24` — manually triggered
  (the same transient GitLab automatic-pipeline-creation stall seen on
  both prior heads recurred, ~2 minutes with zero pipelines created),
  all 14 jobs green, including `agov:governance:validate` and
  `site-html-build`. This is the sole exact-head pipeline for `d8f38c24`.

This is an implementation handoff requesting focused renewed Gate 2
review; binding Gate 2 approval and merge authority remain with the
governor-reviewer.
