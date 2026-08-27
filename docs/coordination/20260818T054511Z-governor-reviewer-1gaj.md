---
kind: decision-record
from: governor-reviewer
to: all
id: 20260818T054511Z-governor-reviewer-1gaj
timestamp: 2026-08-18T05:45:11Z
model: human-governor
immutable: true
references-to:
  - 20260818T053711Z-coder-tester-brn2.md
  - 20260818T053050Z-contribution-auditor-31mk.md
topic: cl3-restricted-corpus-approved
requires-ack: false
artifacts_owned: []
---

# CL-3 restricted initial commercial corpus approved

This record transcribes the human governor's binding instruction in the active
session: “Approved. proceed as proposed.” It approves the exact recommendation
presented in `brn2` and summarized to the governor after the independent review
chain reached CLEAR.

## Binding CL-3 outcome

- **Selected model:** option (a), a restricted initial commercial corpus.
- **Initially eligible corpus:** the 142 files classified
  `Licensor-authored` in MR !1723's independently reviewed inventory at exact
  head `d13bb9084fce9a9b1c7c08bb735e0fe6d08d7e98`.
- **Current exclusion:** the 49 `generated/vendor` rows—all internal
  `.claude/rules` or `.claude/commands` projections/loaders—are excluded from
  the initial commercial corpus pending affirmative qualified-counsel
  clearance. Their internal origin does not by itself supply the separately
  evidenced commercial-relicensing basis required for inclusion.
- **External/unknown evidence:** the bounded inventory found 0 confirmed
  third-party or externally sourced files and 0 `unknown` items under its
  stated technical rules. This remains technical evidence, not a legal
  conclusion.
- **Counsel condition:** qualified-counsel review is required before any real
  buyer offer or agreement execution.

## Authorized next stage

CL-4 **Gate 1 planning** is authorized. The plan must define an immutable
binding between the commercial licence/agreement text and the exact restricted
corpus, including an auditable treatment of the 142 included paths and 49
excluded paths. This decision does not pre-approve that plan or authorize CL-4
implementation before Gate 1 approval.

A future CLA or other explicit inbound relicensing mechanism may be proposed as
a separate contributor-policy/product decision if broader prospective coverage
becomes valuable. It is not selected or authorized here and would not
retroactively clear the current 49 excluded projections.

## Boundaries unchanged

This decision does not approve MR !1712 or MR !1723 at Gate 2, authorize any
merge, offer a commercial licence, execute an agreement, or provide legal
advice. Those actions remain separately gated.
