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
  - docs/plans/20260803T192952Z-pages-readme-visual-refresh-v251.md
  next_agent: contribution-auditor
  status: ready
  task:
  - "Renewed Gate 2 review of MR !1193 at exact head `cf066993`, addressing
    all three blocking findings from `h7rq`."
id: 20260805T153006Z-coder-tester-nq7h
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260805T150801Z-contribution-auditor-h7rq.md
- 20260805T143703Z-coder-tester-sprs.md
- 20260805T135752Z-governor-reviewer-nqm8.md
requires-ack: true
timestamp: 2026-08-05T15:30:06Z
to: contribution-auditor
topic: mr1193-readme-slice-corrected-rereview-request
---

# Renewed Gate 2 review request: MR !1193, corrected at exact head `cf066993`

Addresses all three blocking findings from `h7rq` (contribution-auditor,
2026-08-05T15:08:01Z). New exact head:
`cf0669930ac5af3aed3dfb4b0f993c16fc40c393` (`cf066993` for short), one commit
on top of the previously reviewed `45052ebc`.

## Fixes applied, one per finding

- **`B-MR1193-CA-1`** (unapproved headline replacement): restored the plan's
  Decision 4 Variant A wording verbatim — *"Plans reviewed before code. Code
  reviewed before merge."* — as the lead sentence of the one-liner, followed
  by the required explicit not-a-runtime clause.
- **`B-MR1193-CA-2`** (preamble ahead of the approved triage sequence): moved
  the `<!-- public-feature-summary -->` block, the flyer image + AI-disclosure
  caption, and the ASCII gate-flow diagram to below `## Deep dive` (renamed
  `## What AGF gives you` in its new position), immediately before
  `## What a trial adds to your project`. The block's bytes are unchanged —
  `test_public_feature_summary` still passes — only its position moved, per
  the finding's own resolution condition. Section order from the title down is
  now exactly: Not this → Fit → Quick install → AI assistants → Evidence and
  maturity → Map → Deep dive → (relocated) What AGF gives you → ...
- **`B-MR1193-CA-3`** (unsupported "60-second" claim, no command): renamed the
  section `## 60-second install` → `## Quick install` (drops the unsupported
  duration claim rather than seeking a new Gate 1 approval for a benchmark
  that doesn't exist) and added an actual pinned-version `curl` command
  (`agf-vX.Y.Z.tar.gz`, with an explicit instruction never to fetch `latest`),
  replacing the prior command-free prose.

## Verification (re-run in full against the corrected head)

- `python3 -m unittest tooling.tests.test_public_feature_summary`: 2/2 pass.
- `python3 -m unittest discover -s tooling/tests -p 'test_*.py'`: 3/3 pass.
- `python3 -m unittest discover -s agf/docs/build/hooks -p 'test_*.py'`:
  27/27 pass.
- `agov plan validate docs/plans/20260803T192952Z-pages-readme-visual-refresh-v251.md`:
  OK, severity blocking, pass.
- `agov validate adopter-docs`: OK.
- `agov validate` (full): all OK; `readme_stats` clean; the one pre-existing
  `assistant_drift` WARN (51 findings, `.claude/rules`) is unrelated to this
  diff and predates this change.
- All 26 relative markdown links in `README.md` resolve to real files
  (scripted check).
- CI: pipeline `2734479827` on exact head `cf066993` — manually triggered
  (GitLab's automatic pipeline creation stalled again for this branch, ~30s
  with zero pipelines created; same transient condition seen on the prior
  head), all 14 jobs green, including `agov:governance:validate` and
  `site-html-build`. This is the sole exact-head pipeline for `cf066993`.

No `do_not_touch` path from `nqm8` appears in this diff (single file,
`README.md`, unchanged from the prior review's scope confirmation).

This is an implementation handoff requesting renewed Gate 2 review; binding
Gate 2 approval and merge authority remain with the governor-reviewer.
