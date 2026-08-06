---
kind: handoff
from: governor-reviewer
to: coder-tester
id: 20260806T091214Z-governor-reviewer-1kjq
timestamp: 2026-08-06T09:12:14Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260805T135235Z-governor-reviewer-j86r.md
  - 20260805T135752Z-governor-reviewer-nqm8.md
  - 20260806T071815Z-coder-tester-n462.md
  - 20260806T074049Z-planner-architect-15j6.md
topic: pages-theme-hero-slice-2-implementation
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Implement only slice 2, Pages theme/hero, from the approved visual-refresh plan after the plan metadata correction merged in MR !1203 at `8063c23812b36b88cc8687247e4f49bfe28028e0`.
    - Use a new feature branch and linked worktree based on current `main`; run the plan-prescribed deterministic checks, full governed acceptance pipeline, and exact-head Gate 2 review.
    - Verify every README link intended for GitLab Pages against the deployed route before claiming the visual-refresh work complete; the currently broken Evidence and learnings URL must remain a recorded blocker or be corrected through authorized scope.
    - Stop and return to planning if governed `blue-light` fails WCAG AA against the actual Material dark background; do not invent a fallback token.
  must_read:
    - docs/plans/20260803T192952Z-pages-readme-visual-refresh-v251.md
    - docs/coordination/20260805T135235Z-governor-reviewer-j86r.md
    - docs/coordination/20260806T074049Z-planner-architect-15j6.md
    - README.md
    - agf/docs/site/reference/palette.md
  do_not_touch:
    - agf/docs/build/hooks/check_nav_inventory.py
    - navigation restructuring or any of the 42 page bodies
    - agf/docs/site/overrides/main.html
    - font binaries, font CDNs, or new color tokens
    - release branches, versions, tags, Releases, packages, OCI tags, or deployment
    - .claude/settings.local.json
---

# Human governor authorization: Pages theme/hero slice 2

This record transcribes the human governor's repeated active-session decision:
"approved. proceed." Codex is authorized to lead and implement the Pages
theme/hero slice under the already-approved plan. The plan metadata correction
has merged through MR !1203, and slice 1 completed through MR !1193.

Authorization is limited to the five slice-2 files named by the plan:

- `agf/docs/build/mkdocs.yml` for theme palette/features;
- `agf/docs/site/stylesheets/agf.css` for the hero and system-font type/spacing scale;
- `agf/docs/site/index.md` for approved hero markup/copy;
- `agf/docs/site/reference/palette.md` for measured contrast evidence; and
- `agf/docs/build/hooks/check_palette_contrast.py` for extended self-tests.

The live README-link audit found that the absolute Evidence and learnings URL
returns 404 because `/docs/` is missing. Slice 2 does not silently expand to a
README correction. The implementation and Gate 2 report must distinguish
theme/hero completion from full-workstream completion and must not claim all
README links resolve unless an authorized correction and deployed exact-SHA
verification exist.

Slice 3 navigation remains unauthorized until slice 2 receives its own Gate 2
disposition and the governor publishes a later fresh handoff.
