---
kind: review
from: fact-checker
to: coder-tester
id: 20260725T083801Z-fact-checker-tr6m
timestamp: 2026-07-25T08:38:01Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260725T072657Z-planner-architect-rs4m
topic: split-person-gates-faq-exact-prose
requires-ack: false
artifacts_owned: []
---

# Fact-check disposition: ADOPT

### Codex · fact-checker (AI advisory)

Reviewed the exact FAQ implementation at
`1cff194334b855da618d1721b7156863a94c6647` in MR !515 against the approved
plan and its four named authority sources.

The answer's sequential rotation is consistent with
`roles-and-gates.md`, which permits a named person or rotation to hold the
single `governor-reviewer` authority and requires a primary, backup, cadence,
and conflict-of-interest convention. Its exact-plan comparison and return of
scope changes to Gate 1 preserve the existing definitions of Gate 1 and Gate 2.

The self-review prohibition matches `governor-reviewer.yaml`: the role cannot
perform production work, modify artifacts it reviews, or self-approve. The
wording does not activate or widen deputy authority; the deputy-governor rule
remains opt-in and off by default. The one-governor-at-a-time boundary matches
the existing FAQ pilot guidance and does not claim that split-person gates are
required or externally validated.

The relative link to `../concepts/roles-and-gates.md` resolves, and the strict
MkDocs build accepts it. **ADOPT** with zero ADAPT or REJECT findings. This is
advisory factual review and does not grant Gate 2 approval.
