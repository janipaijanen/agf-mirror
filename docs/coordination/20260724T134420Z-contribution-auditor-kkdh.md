---
kind: handoff
from: contribution-auditor
to: fact-checker
id: 20260724T134420Z-contribution-auditor-kkdh
timestamp: 2026-07-24T13:44:20Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260724T134140Z-contribution-auditor-pqrm
topic: homepage-agov-table-marketing-and-audience-review
requires-ack: true
artifacts_owned:
  - agf/docs/site/index.md
  - agf/docs/site/docs/adopt/tooling-and-tokens.md
handoff:
  next_agent: fact-checker
  status: ready
  task:
    - "Review the homepage agov table through the active fact-checker's marketing-copy, audience, and factual-claim mandate, then return KEEP, MOVE, or CONDENSE with evidence"
  must_read:
    - docs/coordination/20260724T134140Z-contribution-auditor-pqrm.md
    - agf/docs/site/index.md
    - agf/docs/site/docs/adopt/tooling-and-tokens.md
  do_not_touch:
    - agf/docs/site/index.md
    - agf/docs/site/docs/adopt/tooling-and-tokens.md
---

# Marketing and audience review of homepage `agov` table

The governor clarifies that the requested consultation means the project's
marketing-oriented roles and adjacent governed reviewers. In this manifest,
the active `fact-checker` explicitly owns factual review of marketing and
evaluation copy. There is no separate marketing role; `documentation-writer`
is skipped and academic review is not triggered by this placement question.

Fact-checker: independently screen the table and its surrounding prose for:

1. fit with first-time visitor intent and the homepage's desired next action;
2. factual accuracy and support for maturity, availability, and token-savings
   claims;
3. whether the full inventory strengthens credibility or obscures the primary
   value proposition;
4. mobile/narrow-screen comprehension and accessibility of the table format;
5. duplication or contradiction with the dedicated tooling-and-tokens page;
6. the minimum homepage evidence that preserves trust if the full table moves.

Return **KEEP**, **MOVE**, or **CONDENSE**, distinguishing observed facts from
marketing judgment. Route the result to planner-architect/Claude so it can be
synthesized with the independent placement review requested in `pqrm` and
returned to the governor for disposition. Do not edit either page under this
handoff.
