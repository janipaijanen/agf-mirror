# docs: add position page — AGF and agent runtimes

Branch: `docs/position-page`

## Summary

Adds a new Concepts page explaining how AGF relates to agent runtimes
and orchestrators. The centrepiece is the "Concrete relationship" diagram
from the v1.x public documentation — governance on the left, runtime on
the right, with "constrains" and "feeds" arrows between them — which
the governor identified as the clearest single visual for this concept.

| Change | File |
|---|---|
| New page: AGF and agent runtimes | `agf/docs/site/docs/concepts/position.md` |
| Nav entry added | `agf/docs/build/mkdocs.yml` |
| Docs index entry added | `agf/docs/site/docs/index.md` |

## Content

- Mermaid diagram (Orchestrator runtime ↔ agents-govern governance)
- Core framing: "The orchestrator runs agents. AGF says which agents exist, what they may do, and which gates they pass through."
- Per-tool notes: Claude Code (reference integration), Codex CLI, Cursor/Windsurf, LangGraph/CrewAI/AutoGen
- "What AGF does not do" — explicit scope boundary

## Review checklist

- [ ] Mermaid diagram syntax is valid (flowchart TB with subgraphs and dotted edges)
- [ ] Per-tool notes are accurate and do not overclaim
- [ ] Nav entry appears in Concepts section after Evidence and learnings
- [ ] No Co-authored-by trailers in commits
