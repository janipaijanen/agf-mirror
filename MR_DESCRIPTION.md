# docs: add comparisons section to position page and contribute guide

Branch: `docs/contribute-and-comparisons`

## Summary

Two additions from the v1.x docs review (coordination `20260625T060132Z-planner-architect-k82k`,
reviewed and prioritised by Codex in `20260625T062351Z-coder-tester-j6t4`):

| Change | File |
|---|---|
| New section: "How AGF relates to other frameworks" + Mermaid figure | `agf/docs/site/docs/concepts/position.md` |
| New page: Contribute (three upstream paths) | `agf/docs/site/contribute.md` |
| Nav entry: Contribute | `agf/docs/build/mkdocs.yml` |
| Docs index entry: Contribute | `agf/docs/site/docs/index.md` |

## Content

### Position page addition

"How AGF relates to other frameworks" — Mermaid diagram showing how
agents-govern relates to: agent runtimes (governs), CI/CD (provides
audit artifacts), SDLC governance (narrower scope within), ML governance
(different axis), AI policy/ethics (implemented within). Four brief
paragraphs covering each category.

### Contribute page

Three upstream contribution paths with thresholds:

1. **Submit a learning** — single-adopter acceptable; file an issue
2. **Propose a rule** — Provisional by default (1 adopter); Recommended
   requires 2+ independent corroboration
3. **Propose an agent** — 2+ adopter evidence required; propose a rule
   first if single-adopter

Anonymization requirement, independent-adopter evidence gaps, links to
Evidence and learnings and the framework issue tracker.

## Review checklist

- [ ] Mermaid diagram in position page renders (flowchart LR with dotted edges)
- [ ] Position page comparisons text is accurate and does not overclaim
- [ ] Contribute page evidence thresholds match framework rules
- [ ] Nav entry appears at top level between Documentation and Evolution
- [ ] No Co-authored-by trailers in commits
