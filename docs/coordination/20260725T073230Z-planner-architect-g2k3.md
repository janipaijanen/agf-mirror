---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260725T073230Z-planner-architect-g2k3
timestamp: 2026-07-25T07:32:30Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260725T071805Z-planner-architect-6r3t
topic: public-mermaid-full-size-figures-gate1-revision
requires-ack: true
artifacts_owned:
  - docs/plans/framework-map-full-size-figure.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Review and decide Gate 1 for the complete Mermaid-to-openable-SVG plan at exact revised plan commit 121705e99102fb6adea246da04a20425f285696e"
  must_read:
    - docs/plans/framework-map-full-size-figure.md
    - "GitLab issue #64"
    - 20260725T071805Z-planner-architect-6r3t
  do_not_touch:
    - release-2.4.2
    - "the five public Mermaid blocks before Gate 1 approval"
---

# Revised complete Mermaid figure plan ready for Gate 1

This handoff supersedes `6r3t`. No Gate 1 approval was recorded and no
implementation began under the original framework-map-only scope.

The governor identified the same viewport failure on the position page and
expanded issue #64 to all public Mermaid diagrams. The revised plan closes the
inventory at five diagrams across four pages:

1. homepage governed-change flow;
2. runtime/governance boundary;
3. AGF relative to adjacent governance categories;
4. Gate 1-to-Gate 2 coordination sequence; and
5. framework concept map.

Each becomes a governed DOT/SVG asset with a linked image and an explicit,
keyboard-reachable full-size SVG link. The plan also removes the unused Mermaid
custom fence, browser JavaScript/CDN import, and Mermaid-only CSS after the
inventory reaches zero; updates the figure gallery/palette reference; and
records one bounded learning.

Global center-column widening is explicitly rejected: it harms prose measure
and does not solve standalone inspection at narrow widths. No lightbox,
JavaScript replacement, plugin, unrelated-figure retrofit, or v2.4.2 release
change is in scope.

Plan validation and `git diff --check` pass at exact revised plan commit
`121705e99102fb6adea246da04a20425f285696e`.

Governor-reviewer: record APPROVE or BLOCK for this revised Gate 1. No
implementation may begin while the plan remains `pending-gate1`.
