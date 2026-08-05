---
kind: handoff
from: planner-architect
to: fact-checker
id: 20260805T100621Z-planner-architect-4egj
timestamp: 2026-08-05T10:06:21Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260805T095054Z-fact-checker-jm2g.md
  - 20260805T090722Z-governor-reviewer-83m8.md
topic: mr1082-claims-rereview
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: fact-checker
  status: ready
  task:
    - Re-review B-MR1082-FC-1, A-MR1082-FC-1, and the incorporated CLI/MCP framing at exact head 52a9668715f4e559ad8dead6f081171a7f3d06f8.
    - Record CLEAR or remaining exact findings in coordination without editing MR 1082.
  must_read:
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/1082
    - docs/plans/20260803T192952Z-pages-readme-visual-refresh-v251.md
    - docs/plans/assets/20260803T192952Z-pages-readme-visual-refresh-v251-mockup.html
    - docs/coordination/20260805T095054Z-fact-checker-jm2g.md
  do_not_touch:
    - MR !1082 plan and mockup files; review only
    - production Pages files and root README.md
---

# MR !1082 public-claims re-review at exact head `52a96687`

The first review is acknowledged and its findings are addressed:

- **B-MR1082-FC-1:** the README evidence table is explicitly four rows,
  preserving mechanical installation, initial commissioning, governor
  attention, and exit duration; the mockup shows and counts all four.
- **A-MR1082-FC-1:** the absolute "Every public capability claim" copy is
  softened to "AGF's publication standard is to state...".
- **CLI/MCP confirmation:** the adopted AI-assistant route states that the CLI
  is universal and agov MCP is recommended for MCP-capable assistants, and
  requires a link to the existing Level 4 status/limitations page.

MR !1082's sole exact-head pipeline `2733474495` passed. This is a no-edit
advisory re-review. It does not authorize Gate 1 submission or implementation.
