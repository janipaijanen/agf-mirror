---
kind: handoff
from: governor-reviewer
to: contribution-auditor
id: 20260801T151748Z-governor-reviewer-r73p
timestamp: 2026-08-01T15:17:48Z
model: codex-gpt-5
immutable: true
references-to: []
topic: four-way-test-engineering-lens
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Independently critique whether the Rotary Four-Way Test should inform AGF documentation and feature decisions.
    - Assess the proposed use as a non-binding reflection lens rather than a four-yes acceptance gate.
    - Recommend wording and safeguards for evidence, uncertainty, dissent, affected parties, trade-offs, and decision authority.
    - Identify attribution, licensing, trademark, or scope questions that should be verified before quoting or naming the test in public project material.
  must_read:
    - AGENTS.md
    - agf/rules/always/tool-authority-boundary-rule.md
  do_not_touch:
    - MR !894
    - docs/plans
    - agf/rules
    - README.md
    - agf/docs
---

# Independent consultation: Four-Way Test as an engineering lens

The human governor proposed that documentation and feature work be guided by
the Rotarians' Four-Way Test: truth, fairness to those concerned, goodwill and
better relationships, and benefit to those concerned.

Codex's preliminary view is that these values are useful, but the four
questions should not become a binding requirement that every answer be an
unqualified “yes.” Engineering decisions often involve uncertainty,
conflicting interests, necessary criticism, migration cost, and benefits that
are unevenly distributed.

The proposed adaptation is:

1. Are factual support, uncertainty, and evidence strength visible?
2. Have affected parties, burdens, exclusions, and plausible harms been
   considered fairly?
3. Is the communication candid, respectful, and constructive, including when
   it records disagreement or a blocking finding?
4. Is the expected benefit proportionate to cost, risk, complexity, and the
   available alternatives?

When an answer is “no” or uncertain, the record should explain the trade-off
and identify the authorized decision-maker rather than mechanically reject the
work.

Please provide an independent contribution-auditor critique. Do not implement
or modify any plan, rule, documentation, MR !894, or issue state. Publish the
response as a validated coordination review so the governor can compare both
analyses before deciding whether planning work is warranted.
