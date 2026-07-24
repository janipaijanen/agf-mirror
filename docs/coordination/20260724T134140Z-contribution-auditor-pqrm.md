---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260724T134140Z-contribution-auditor-pqrm
timestamp: 2026-07-24T13:41:40Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260724T133934Z-coder-tester-k6bm
topic: agov-homepage-table-placement-review
requires-ack: true
artifacts_owned:
  - agf/docs/site/index.md
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - "Independently assess whether the full agov command/maturity table is correctly placed on the public homepage, and return KEEP, MOVE, or CONDENSE with evidence and a scoped recommendation"
  must_read:
    - agf/docs/site/index.md
    - agf/docs/site/docs/adopt/tooling-and-tokens.md
  do_not_touch:
    - agf/docs/site/index.md
    - agf/docs/site/docs/adopt/tooling-and-tokens.md
---

# Marketing/visitor-flow consultation: `agov` table placement

The governor asks whether the full `agov` command/maturity table on
<https://janipaijanen.gitlab.io/agents-govern-framework/> is in the right
place, and asks Codex to coordinate with Claude plus marketing-oriented and
friend feedback.

Current placement: the 14-row table appears under "Token savings with the Rust
tooling (optional)", after "How it works" and before "What you get". The same
topic has a dedicated adopter page, `docs/adopt/tooling-and-tokens.md`.

Codex's preliminary hypothesis is **CONDENSE on homepage, MOVE full inventory
to the dedicated tooling page**: the matrix is credible proof for technical
evaluators but interrupts the first-time visitor's value/audience/adoption
flow, and a wide three-column command table is costly on mobile. Treat this as
a hypothesis to challenge, not a decision.

Planner-architect/Claude: perform an independent review and return one of
KEEP/MOVE/CONDENSE. Evaluate:

1. the homepage's primary visitor question and desired next action;
2. whether detailed command coverage is proof needed before "What you get";
3. information scent and duplication with the tooling page;
4. narrow-screen scanning and cognitive load;
5. the smallest replacement that preserves technical credibility (for
   example, 3–4 representative capabilities plus a link);
6. whether this belongs in the already-governed v2.4.2 public-experience work
   rather than an isolated hotfix.

Separate observed page facts from marketing judgment. Do not claim feedback
from external "friends" unless the governor supplies their actual comments or
a named communication channel. If such feedback arrives, record it as
attributed qualitative input, not representative user research.

No page edit is authorized by this handoff. Return a review/recommendation for
governor disposition and Gate 1 scoping.
