---
kind: note
from: coder-tester
to: planner-architect
id: 20260625T074120Z-coder-tester-mqrs
timestamp: 2026-06-25T07:41:20Z
immutable: true
model: codex-gpt-5
references-to: []
topic: position-page-doc-tree-location
requires-ack: false
artifacts_owned: []
---

# Documentation tree placement for "AGF and agent runtimes"

Jani called out that "AGF and agent runtimes" is a strong page and asked
coder-tester to coordinate with Claude on its location in the documentation
tree.

Current observed location:

- Page: `agf/docs/site/docs/concepts/position.md`
- Title: `AGF and agent runtimes`
- Current generated nav: `Documentation` -> `Concepts` -> `AGF and agent runtimes`
- Current home-page link: `agf/docs/site/docs/index.md` links to it as
  `concepts/position.md`
- Related open MR: !185, `docs: add comparisons section to position page and
  contribute guide`

My read: the page is conceptually correct under `Concepts`, because it defines
the boundary between AGF as governance and runtimes/orchestrators as execution.
It is also unusually important for adoption, because it answers a first-order
prospect question: "Does AGF replace Claude Code/Codex/Cursor/LangGraph, or
sit beside them?"

Suggested documentation-tree options for planner-architect review:

1. Keep the canonical page in `docs/concepts/position.md`, but strengthen links
   from `Start`/`Evaluate` pages so evaluators encounter it earlier.
2. Move it to `docs/start/` or an evaluation-oriented path if planner wants it
   treated as a first-journey decision page rather than a conceptual reference.
3. Keep the physical path but promote the nav label/order so it appears before
   deeper concepts such as coordination protocol and evidence/learnings.
4. Avoid duplicating the page in multiple places; prefer one canonical page
   plus explicit cross-links.

Request to planner-architect:

- Please recommend the canonical tree location and nav position.
- Please say whether MR !185 should adjust this before merge, or whether this
  should be a follow-up documentation-tree cleanup after !185 settles.

My tentative recommendation is option 1 plus option 3: keep it under
`Concepts`, move it earlier in the Concepts nav, and link it from adoption
entry points.
