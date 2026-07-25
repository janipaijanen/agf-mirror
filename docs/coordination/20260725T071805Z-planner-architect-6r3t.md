---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260725T071805Z-planner-architect-6r3t
timestamp: 2026-07-25T07:18:05Z
model: codex-gpt-5
immutable: true
references-to: []
topic: framework-map-full-size-figure-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/framework-map-full-size-figure.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Review and decide Gate 1 for the framework-map full-size figure plan at exact plan commit 13fa639da651a471f093588e6896ac80679e368a"
  must_read:
    - docs/plans/framework-map-full-size-figure.md
    - "GitLab issue #64"
  do_not_touch:
    - release-2.4.2
    - agf/docs/site/docs/start/framework-map.md
    - agf/docs/figures/src/framework-map.dot
    - agf/docs/site/reference/figures/framework-map.svg
---

# Framework-map full-size figure plan ready for Gate 1

Issue #64 records a direct user finding that the published framework map is
unreadable at article width and offers no full-size inspection path.

The proposed corrective slice is deliberately small:

- replace the inline-only Mermaid graph with a governed DOT/SVG figure;
- make the image and an explicit text link open the standalone SVG for normal
  browser zoom;
- preserve the page's prose, node guide, edge guide, and claim boundaries;
- verify keyboard, 390px, direct-SVG, and 200%-zoom behavior; and
- append one bounded learning record so future dense orientation diagrams do
  not treat responsive shrinking or scrolling alone as readability evidence.

The plan adds no JavaScript, plugin, site-wide figure retrofit, technical
contract, or accessibility-conformance claim. It explicitly excludes the
already-frozen v2.4.2 release and sequences this as a later patch decision.

Plan validation and `git diff --check` pass at exact commit
`13fa639da651a471f093588e6896ac80679e368a`.

Governor-reviewer: record APPROVE or BLOCK for Gate 1. No implementation may
start while the plan remains `pending-gate1`.
