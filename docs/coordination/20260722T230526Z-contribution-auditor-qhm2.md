---
kind: handoff
from: contribution-auditor
to: coder-tester
id: 20260722T230526Z-contribution-auditor-qhm2
timestamp: 2026-07-22T23:05:26Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260722T091729Z-governor-reviewer-m4tq
  - 20260722T230025Z-governor-reviewer-ksdg
topic: v240-workstream-d-class4-start
requires-ack: true
artifacts_owned:
  - agf/docs/site/docs/concepts/position.md
  - agf/docs/site/index.md
  - agf/docs/site/docs/concepts/coordination-protocol.md
  - agf/docs/site/docs/start/framework-map.md
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - acknowledge and claim Workstream D Class 4
    - implement the approved four-file inline Mermaid redesign from current main
    - publish a Gate 2 handoff with exact-SHA pipeline and visual evidence
  must_read:
    - docs/plans/v2.4.0-workstream-d-figure-system.md
    - agf/docs/site/reference/palette.md
    - docs/coordination/20260722T230025Z-governor-reviewer-ksdg.md
  do_not_touch:
    - agf/framework.md
    - agf/docs/agent-communication-0.26.0.md
    - docs/academic/governing-the-governors.tex
    - docs/academic/figures/
---

# Release handoff: begin Workstream D Class 4

MR !417 merged to `main` as `f997f8b22abc9cf4bfa21ae30cbce5b2b5d562ec`.
The Class 3 dependency is therefore cleared. The governor's `ksdg` decision
directs Classes 4 and 5 to continue as v2.4.0 release blockers, and the
approved Workstream D child plan sequences Class 4 before Class 5.

Coder-tester should now branch from current `main` and implement only the four
MkDocs inline Mermaid files in the approved Class 4 inventory. Apply the
shared semantic responsibility palette rather than Mermaid defaults, preserve
meaning without colour alone, and check whether the coordination-protocol
diagram duplicates the DOT figure on the same page. The two non-MkDocs
Mermaid documents remain excluded by the Gate 1 decision.

The Gate 2 handoff must include the plan's desktop, narrow/mobile, light,
dark, print/greyscale, and screen-reader-semantics checks; a manual visual
inspection of every rendered diagram; exact-SHA pipeline evidence; and an
explicit statement about whether any duplicate push pipeline was created.
Do not mix Class 5 prose labels, deferred Class 3 accessibility work, or
academic TeX changes into this slice.

After Class 4 merges, route Class 5 as the final Workstream D implementation
slice. Do not wait for the post-release taxonomy or marketing consultations.
