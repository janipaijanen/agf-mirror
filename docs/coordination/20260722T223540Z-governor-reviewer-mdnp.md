---
kind: decision-record
from: governor-reviewer
to: all
id: 20260722T223540Z-governor-reviewer-mdnp
timestamp: 2026-07-22T22:35:40Z
model: human-jani
immutable: true
references-to:
- 20260722T091729Z-governor-reviewer-m4tq.md
- 20260722T213741Z-coder-tester-hgn2.md
topic: v240-academic-tex-low-priority-scope
requires-ack: true
artifacts_owned: []
---

# Decision: academic TeX embedding is low priority and must not delay v2.4.0

The governor directs the release team to advance v2.4.0 through immediate
release-blocking work. The academic TeX paper and its page-layout/toolchain
work are the lowest priority in this release sequence.

For MR !417:

1. Remove the `role-communication-topology.pdf` `figure*` block from
   `docs/academic/governing-the-governors.tex`.
2. Leave a clear TeX source comment at that location stating that the role
   communication topology image is intended there for possible future
   restoration. Preserve the surrounding explanatory prose.
3. Do not spend v2.4.0 release time adding a LaTeX toolchain or proving paper
   pagination/page fit for the omitted image. Absence of a paper compilation is
   not a Gate 2 blocker under this decision.
4. The underlying DOT/SVG/PDF artifact remains in the Class 3 palette slice.
   Its semantic correctness, palette compliance, regeneration, and reasonable
   standalone visual quality remain reviewable; paper-specific placement and
   pagination do not.

This is a priority/scope clarification, not a deferral of Workstream D Classes
4 and 5. After the focused Class 3 corrections, continue those remaining
release blockers before post-release marketing, prose, and learning work.
