---
kind: handoff
from: coder-tester
to: governor-reviewer
model: codex-gpt-5
id: 20260621T181158Z-coder-tester-hbgq
timestamp: 2026-06-21T18:11:58Z
immutable: true
references-to:
  - 20260621T175123Z-coder-tester-7s4d
topic: coordination-lifecycle-mermaid-gate2
requires-ack: true
artifacts_owned:
  - MR_DESCRIPTION.md
  - docs/coordination/20260621T181158Z-coder-tester-hbgq.md
  - agf/docs/build/mkdocs.yml
  - agf/docs/site/docs/concepts/coordination-protocol.md
  - agf/docs/figures/figures.md
  - agf/docs/site/reference/figures.md
  - agf/docs/figures/src/coordination-lifecycle.dot
  - agf/docs/figures/coordination-lifecycle.svg
  - agf/docs/site/reference/figures/coordination-lifecycle.svg
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review docs/coord-lifecycle-mermaid branch for Gate 2 approval.
    - Confirm the coordination lifecycle should be Mermaid rather than a static SVG figure.
    - Merge into main if approved so Pages removes the static lifecycle SVG from the figure reference.
  must_read:
    - MR_DESCRIPTION.md
    - agf/docs/site/docs/concepts/coordination-protocol.md
    - agf/docs/site/reference/figures.md
  do_not_touch:
    - unrelated worktrees
---

# Gate 2 handoff: coordination lifecycle Mermaid replacement

Branch: `docs/coord-lifecycle-mermaid`
MR: https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/76

## Summary

The previously merged `coordination-lifecycle.svg` figure is removed from the
Graphviz figure inventory and replaced by an embedded Mermaid `sequenceDiagram`
in the coordination protocol page. The MkDocs Mermaid custom fence is enabled
so the block renders as a diagram instead of highlighted code.

## Verification

- `mkdocs build --strict`: passed.
- Built HTML contains a `class="mermaid"` block for the lifecycle sequence.
- `git diff --check`: passed.
- `agov coord validate docs/coordination/20260621T181158Z-coder-tester-hbgq.md`: passed.
- `rg` confirmed no remaining docs reference to `coordination-lifecycle.svg` or
  `coordination-lifecycle.dot` outside this handoff and MR text.

## Review focus

- Confirm Mermaid is acceptable for this lifecycle sequence.
- Confirm Mermaid renders as a diagram in MkDocs.
- Confirm removing the direct SVG URL is intended.
- Confirm the figure inventory still reads coherently after removing this
  static asset.
