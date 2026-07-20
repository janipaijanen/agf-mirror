---
artifacts_owned: []
from: fact-checker
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - tooling/elixir/mix.lock
  must_read:
  - docs/plans/docs-audience-boundary-proposal.md
  - docs/coordination/20260720T135454Z-fact-checker-ag3k.md
  - docs/coordination/20260720T141116Z-planner-architect-75sq.md
  next_agent: planner-architect
  status: ready
  task:
  - Correct or remove the stale audience phrase-count rows in docs/plans/docs-audience-boundary-proposal.md; the headline 34/22/20/2 count is fixed, but the supporting phrase table still contains outdated row counts.
id: 20260720T141554Z-fact-checker-qr8m
immutable: true
kind: handoff
model: codex-gpt-5
references-to:
- 20260720T141116Z-planner-architect-75sq.md
- 20260720T135454Z-fact-checker-ag3k.md
requires-ack: false
timestamp: 2026-07-20T14:15:54Z
to: planner-architect
topic: docs-audience-boundary-proposal-fact-check-followup
---

# Fact-check follow-up: supporting phrase-count table still stale

The second correction in `20260720T141116Z-planner-architect-75sq.md` fixes
the headline claim requested by fact-checker: 34 nav pages, 22 with
`Audience:` clauses, 20 human-only, and 2 that mention `AI assistants`.

However, while confirming that fix I found the supporting phrase-count rows in
the "Corrected counts" table are still stale. Current plan rows say:

- `"developers" (alone or combined)`: 9
- `"administrators"`: 3
- `"governors"`: 3
- `"evaluators"`: 5
- `"adopters"`: 4
- `"AI assistants"`: 2

Independent extraction of the current top-of-page `Audience:` clauses from all
34 nav pages gives:

- `"developers"`: 12
- `"administrators"`: 7
- `"governors"`: 7
- `"evaluators"`: 4
- `"adopters"`: 4
- `"AI assistants"`: 2

The headline 34/22/20/2 claim is now correct, but the stale supporting row
counts are still evidence drift and should be corrected before final Gate 1.

Smallest unblock: either update those phrase rows to the extracted counts
above, or remove the phrase-count rows entirely and keep only the exhaustive
34/22/20/2 population statement plus the list of the two AI-assistant pages.

After the edit, rerun `agov plan validate` and route back for final
fact-checker confirmation.
