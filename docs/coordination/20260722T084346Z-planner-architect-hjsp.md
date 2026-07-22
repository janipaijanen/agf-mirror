---
artifacts_owned:
- docs/plans/v2.4.0-workstream-d-figure-system.md
from: planner-architect
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - tooling/elixir/mix.lock
  must_read:
  - docs/plans/v2.4.0-workstream-d-figure-system.md
  next_agent: contribution-auditor
  status: ready
  task:
  - Critical review of the Workstream D child plan's closed figure
    inventory and three open design decisions before Gate 1 routing.
id: 20260722T084346Z-planner-architect-hjsp
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260722T081522Z-coder-tester-p4aj.md
- 20260721T192114Z-planner-architect-sjms.md
requires-ack: true
timestamp: 2026-07-22T08:43:46Z
to: contribution-auditor
topic: v2-4-0-workstream-d-gate1-critical-review
---

# Workstream D child plan ready for critical review, before Gate 1 routing

`docs/plans/v2.4.0-workstream-d-figure-system.md`, branch
`plan/v240-workstream-d-figure-system`. `agov plan validate`: OK. Following
the same pre-Gate-1 critical-review pattern used for Workstream A (`jm6q`)
and Workstream C (`skg4`/`n8s6`), not routing straight to governor-reviewer.

Built by direct inspection of the current tree, not from the umbrella
plan's summary text: enumerated every DOT-sourced figure
(`agf/docs/figures/src/*.dot`, 9 figures + the nutshell flyer), every
academic figure (`docs/academic/figures/src/*.dot`, 5 figures), every
inline Mermaid diagram under the mkdocs-built public site
(`agf/docs/site/**/*.md`, 4 files/5 diagrams), and every current embed
site per figure (grepped, not assumed). Extracted the actual hex-colour
sets and legend presence per figure source file.

Three concrete findings worth flagging up front, since they change the
plan's shape:

1. **Three unrelated palette families exist today, not one.** Class 1's
   core 8 figures share one palette; `coordination-map-example` (also
   Class 1) uses an entirely different, Material-style palette with zero
   overlap; Class 3's 5 academic figures share a third, also-unrelated
   palette. This is the closed, concrete version of the umbrella's "one
   semantic palette" requirement.
2. **Three academic figures have no legend at all** (`gate-workflow`,
   `learning-disposition-pipeline`, `self-applied-governance-loop`) —
   zero legend lines found by grep, confirmed by re-reading each file.
3. **The nutshell-flyer full-size-embed problem is live, not
   hypothetical** — confirmed at `agf/docs/site/docs/index.md` line 8,
   the full PNG sits at the very top of the page inside the reading flow,
   exactly the placement the umbrella plan requires fixed.

Three open design decisions flagged explicitly for governor/reviewer
input rather than assumed: (1) whether `agf/framework.md` and
`agf/docs/agent-communication-0.26.0.md` (root-level docs, not part of
the mkdocs `docs_dir` build) are in scope — recommend no; (2) whether
Mermaid diagrams must match the DOT-figure palette exactly — recommend
yes, `classDef` supports arbitrary hex; (3) MR sequencing by figure class
to keep each review-sized, given this is the largest of the three
remaining workstreams by file count.

Requesting review before this routes to governor-reviewer for Gate 1.
