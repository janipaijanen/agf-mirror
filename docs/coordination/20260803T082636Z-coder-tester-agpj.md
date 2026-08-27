---
artifacts_owned: []
from: coder-tester
handoff:
  do_not_touch:
  - .claude/settings.local.json
  must_read:
  - docs/plans/ai-regulation-requirements-gap-analysis.md
  - docs/sources.md
  next_agent: fact-checker
  status: ready
  task:
  - Bounded renewed review of F-REG-1 through F-REG-4 at new exact head
    4eb464aeaa830757814e22a4a74df58b7449316e -- confirm AI-1 now states
    Regulation (EU) 2026/1744's enacted dates (2027-12-02 Annex III,
    2028-08-02 Article 6(1)/Annex I), AI-3 no longer states an explicit
    NIS2 Art. 21 contractual flow-down duty, AI-4 no longer states PLD
    Art. 9 as a standing mandatory-disclosure obligation, and AI-5/AI-15
    correctly state MR !962 as merged.
  - Confirm docs/sources.md's AI Act entry (A-FACT-1) carries the same
    corrected dates so the plan does not re-inherit the stale status.
id: 20260803T082636Z-coder-tester-agpj
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260803T073119Z-fact-checker-aman.md
requires-ack: true
timestamp: 2026-08-03T08:26:36Z
to: fact-checker
topic: mr967-renewed-factcheck-request-post-fregfixes
---

# Renewed fact-check request: MR !967 post F-REG-1..4 fixes

`docs/plans/ai-regulation-requirements-gap-analysis.md` (MR !967) has
been revised at exact head `4eb464ae` addressing all four blocking
findings from `aman` (`20260803T073119Z-fact-checker-aman.md`):

- F-REG-1: AI-1 now cites Regulation (EU) 2026/1744 as enacted law
  (published in the Official Journal 2026-07-24, in force July 2026),
  with the correct 2027-12-02 / 2028-08-02 dates, replacing the stale
  "Omnibus VII agreed but not yet law" framing. `docs/sources.md`'s AI
  Act entry updated to match (A-FACT-1).
- F-REG-2: AI-3 no longer states NIS2 Art. 21 as requiring contractual
  security flow-down; contractual flow-down is now labelled an inferred
  implementation option, with Art. 21(2)(d)/(3)'s actual text quoted.
- F-REG-3: AI-4 no longer states PLD Art. 9 as a standing mandatory-
  disclosure obligation; now states the actual court-ordered,
  proportionality-bounded mechanism per Art. 9/10.
- F-REG-4: AI-5 and AI-15 now state MR !962 as merged, not
  forward-dependent.

Please independently re-verify each against the plan's current text and
primary sources rather than taking this summary on trust.
