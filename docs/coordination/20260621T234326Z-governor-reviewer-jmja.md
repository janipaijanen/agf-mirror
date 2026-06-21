---
kind: decision-record
from: governor-reviewer
to: all
id: 20260621T234326Z-governor-reviewer-jmja
timestamp: 2026-06-21T23:43:26Z
immutable: true
references-to:
  - 20260621T215556Z-coder-tester-md2b.md
topic: learning-threshold-governor-effort-gate2-approved
requires-ack: false
artifacts_owned:
  - docs/coordination/20260621T234326Z-governor-reviewer-jmja.md
decision:
  outcome: approved
  branch: docs/learning-threshold-governor-effort
  mr: "!93"
---

# Gate 2 retroactive approval — docs/learning-threshold-governor-effort

Retroactive Gate 2 review of `docs/learning-threshold-governor-effort`, merged
at `1be20c0`. Fast-track basis: four existing docs pages updated. No rules,
schemas, agents, nav, or security boundary changes. Reviewing against `md2b`
handoff `20260621T215556Z-coder-tester-md2b.md`.

## Scope

Four artifacts:
- `agf/docs/site/docs/concepts/evidence-and-learnings.md`
- `agf/docs/site/docs/start/adopter-faq.md`
- `agf/docs/site/docs/start/should-you-adopt.md`
- `agf/docs/site/docs/adopt/downstream-impact.md`

## Review

**evidence-and-learnings.md**

Added "When evidence becomes a learning" threshold table, "Who owns which
records" ownership table, and anatomy + anonymization sections. The threshold
table (normal iteration → do not record; first commissioning → record; repeated
friction → record; etc.) is practical and appropriately graded. The ownership
table clearly separates upstream vs. adopter-owned records and states the
upgrade behavior. Correct.

**adopter-faq.md**

Substantial new content added: governor time estimates table, learning-record
write/escalate threshold table, draft learning guidance, upstream contribution
workflow, and multi-human/AI coordination guidance. All content is:
- Honest about effort ("Budget real human attention")
- Appropriately qualified ("planning estimates, not service-level promises")
- Consistent with existing docs content in other pages
- Correct on the substance (e.g., "Gates without a governor become ceremony")

The new "What should our first week look like?" and "What should we avoid in
the first week?" FAQs are well-scoped and grounded. Correct.

**should-you-adopt.md**

Two changes:
1. "No new infrastructure" clarification — "It does not mean zero human effort.
   The governor role is real: someone must read gate evidence, accept or reject
   risk, and stop work when the trail is not good enough." Directly addresses
   the Lumo gap observation. Accurate.
2. Governor attention row in commitments table (5-15 minutes for non-trivial
   first-trial; more for high-risk). Honest and appropriately bounded. Correct.

**downstream-impact.md**

Governor effort estimate added to the gate-cycle cost section. Learning
records ongoing obligation updated to include threshold ("Do not record every
ordinary bugfix; do record first commissioning evidence, repeated friction,
rule catches, rule workarounds, and near-misses"). Consistent with
evidence-and-learnings.md content. Correct.

## Consistency check

The governor time estimate (5-15 minutes for non-trivial first-trial change)
appears in both `adopter-faq.md` and `downstream-impact.md` with identical
wording. No drift. The learning-record threshold table in `adopter-faq.md` and
the shortened version in `evidence-and-learnings.md` are consistent.
Message-integrity rule (one canonical form, no paraphrase drift) satisfied.

## Decision

**Approved.** All four pages are accurate, internally consistent, and directly
address the Lumo external review's legitimate gap observations on governor
effort and learning-record thresholds. Acks Gate 2 handoff
`20260621T215556Z-coder-tester-md2b.md`.
