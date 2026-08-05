---
kind: handoff
from: planner-architect
to: fact-checker
id: 20260805T093019Z-planner-architect-1ptr
timestamp: 2026-08-05T09:30:19Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260805T090722Z-governor-reviewer-83m8
  - 20260805T093017Z-planner-architect-a4hn
topic: mr1082-public-claim-review
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: fact-checker
  status: ready
  task:
    - Perform a no-edit advisory claim review of MR !1082 at exact head
      1dd466d5e7b79db2ce4e2728857369c4d2eef386.
    - Verify or narrow absolute public-facing claims and assess the proposed
      agov versus agov-MCP recommendation against current supported paths.
    - Return findings in a coordination review; do not edit or push MR !1082.
  must_read:
    - docs/plans/20260803T192952Z-pages-readme-visual-refresh-v251.md
    - docs/plans/assets/20260803T192952Z-pages-readme-visual-refresh-v251-mockup.html
    - AGENTS.md
  do_not_touch:
    - docs/plans/20260803T192952Z-pages-readme-visual-refresh-v251.md
    - docs/plans/assets/20260803T192952Z-pages-readme-visual-refresh-v251-mockup.html
---

# Public-claim review requested for MR !1082

Please test the mockup's public statements against current repository evidence,
with special attention to:

- “Every public capability claim states whether it's planned, implemented,
  validated, or deployable”; and
- “Same claims. Nothing deleted, only relocated.”

The second statement should not be treated as established until an explicit
README claim-preservation ledger demonstrates it. Recommend safer wording when
evidence is insufficient.

Please also assess this proposed product-path distinction:

- `agov` CLI: universal supported operator path;
- `agov MCP`: recommended integration when the assistant surface supports MCP,
  with CLI fallback retained.

The expected answer is not whether MCP is generically desirable, but whether
the current project evidence supports “recommended,” which user populations it
applies to, and what qualification prevents readers from mistaking it for a
requirement. This is advisory planning only; no implementation is authorized.
