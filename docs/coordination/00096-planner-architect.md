---
kind: handoff
from: planner-architect
to: governor-reviewer
seq: 96
timestamp: 2026-06-12T18:12:55Z
immutable: true
references-to:
  - 00093-planner-architect.md
  - 00094-planner-architect.md
  - 00095-planner-architect.md
topic: agents-govern-2.0-gate1-figures-visual-system
requires-ack: true
artifacts_owned:
  - docs/plans/agents-govern-2.0-directory-structure.md
model: codex-gpt-5
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 1 review of amended agents-govern-2.0 directory structure plan"
    - "Approve or reject CI-regenerated figures for v2.0 HTML/PDF documentation"
    - "Approve or reject a v2.0 visual system and semantic color palette"
  must_read:
    - docs/plans/agents-govern-2.0-directory-structure.md
    - agents-govern/src/agents_govern/graph.py
    - agents-govern/docs/agent-communication-0.26.0.md
  do_not_touch:
    - agents-govern/
    - agents-govern/agents/MANIFEST.yaml
    - .claude/settings.local.json
    - agents-govern/uv.lock
---

# Gate 1 amendment: figures and visual system

This handoff refines seq 95. The v2.0 GitLab Pages site and academic PDF need
reusable figures and a consistent visual system.

## Inventory finding

Current checkout:

- No committed SVG/PNG/PDF figure assets were found under `agents-govern/`.
- Existing reusable figure source exists as
  `agents-govern/docs/agent-communication-0.26.0.md`, a generated Mermaid
  communication graph.
- Existing reusable generator exists in
  `agents-govern/src/agents_govern/graph.py`; it supports Graphviz SVG/PNG/PDF
  output plus Mermaid output through `agov-validate-agents --graph`.
- Local shell does not currently have Graphviz `dot`; CI must install Graphviz
  if regenerated SVG/PNG/PDF figures are required. Otherwise the generator can
  fall back to `.dot` or Mermaid source.

## Plan amendment

`docs/plans/agents-govern-2.0-directory-structure.md` now adds:

- `docs/site/assets/design-system.md`
- `docs/site/assets/styles/palette.css`
- `docs/site/assets/styles/palette.json`
- `docs/site/assets/figures/README.md`
- figure source pages for:
  - agent communication
  - governance feedback loop
  - Gate 1/Gate 2 pipeline
  - adoption impact map
  - exit/decommission flow
  - progressive docs learning path
  - evidence lifecycle
- `docs/build/figures.yml`
- a `figures-build` CI job expectation

## Recommendation

Yes, v2.0 should have its own palette, but as documentation infrastructure,
not decorative branding. It should be a small semantic token set that works
across HTML, SVG, PNG, and PDF.

Minimum requirements:

- colorblind-safe
- distinguishable in grayscale/print
- semantic token names instead of ad hoc hex values
- meaning preserved without color through labels, shapes, and line styles
- modest enough for academic PDF, distinctive enough for public GitLab Pages

Every figure should have orienting text before the diagram and should be
included in CI artifacts so the rendered site/PDF can be inspected.

## Validation

Plan validation passed:

```text
OK  ../../docs/plans/agents-govern-2.0-directory-structure.md
```
