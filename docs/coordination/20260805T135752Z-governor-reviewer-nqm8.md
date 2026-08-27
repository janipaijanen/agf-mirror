---
kind: handoff
from: governor-reviewer
to: coder-tester
id: 20260805T135752Z-governor-reviewer-nqm8
timestamp: 2026-08-05T13:57:52Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260805T135235Z-governor-reviewer-j86r.md
  - 20260805T135749Z-governor-reviewer-pe4g.md
requires-ack: true
topic: mr1082-readme-implementation-slice
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - "Implement only slice 1 of the approved MR !1082 plan: the root README structure and claim-preserving copy."
    - Use a new feature branch and linked worktree under .worktrees/ based on current main; do not reuse the merged plan branch.
    - Run the plan-prescribed validation and the repository's full governed acceptance pipeline, then publish an exact-head Gate 2 handoff.
  must_read:
    - docs/plans/20260803T192952Z-pages-readme-visual-refresh-v251.md
    - docs/coordination/20260805T135235Z-governor-reviewer-j86r.md
    - docs/coordination/20260805T132830Z-contribution-auditor-rrsm.md
    - docs/coordination/20260805T132835Z-coder-tester-ep1p.md
    - docs/coordination/20260805T132842Z-fact-checker-jmgn.md
  do_not_touch:
    - agf/docs/build/mkdocs.yml
    - agf/docs/site/stylesheets/agf.css
    - agf/docs/site/index.md
    - agf/docs/site/reference/palette.md
    - agf/docs/build/hooks/check_palette_contrast.py
    - agf/docs/build/hooks/check_nav_inventory.py
---

# Implementation handoff: MR !1082 slice 1 — root README

The human governor-reviewer's binding Gate 1 approval `j86r` authorizes the
approved three-slice Pages/README visual-refresh implementation. Begin with
slice 1 only: the root `README.md`.

Implement the approved Decision 5/5a structure and acceptance boundaries:

- preserve the quick-start route while making the audience routes explicit;
- add a separately labeled AI-assistant route to `AGENTS.md`;
- state the `agov` CLI as the universal supported path and recommend agov MCP
  for MCP-capable assistants, linking to its maturity/limitations page;
- preserve all four evidence qualifiers: mechanical installation, initial
  commissioning, governor attention, and exit duration; and
- avoid importing superseded draft #1's three-row evidence table or any
  production Pages/theme/navigation changes.

This slice is independently revertible and must receive its own MR, exact-head
CI, evidence-backed review, and binding Gate 2 decision before the ordered
theme/hero slice begins. Release planning, version changes, tags, deployment,
and release remain unauthorized.
