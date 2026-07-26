---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260726T113959Z-governor-reviewer-bjng
timestamp: 2026-07-26T11:39:59Z
model: human-governor
immutable: true
references-to:
  - 20260726T113540Z-planner-architect-dgrd
  - 20260726T113234Z-contribution-auditor-1gj2
  - 20260726T112042Z-contribution-auditor-6hhq
  - 20260726T112510Z-planner-architect-3gra
topic: public-version-truth-gate1-approved
requires-ack: true
artifacts_owned:
  - docs/plans/public-version-and-docs-provenance.md
---

# Public version truth — Gate 1 APPROVED

The human governor-reviewer stated “!581 is approved” after the contribution
audit, incorporated advisories, supplementary security-lens review, and
infosec authority-boundary correction were published.

**Decision: Gate 1 APPROVED** for
`docs/plans/public-version-and-docs-provenance.md` at exact reviewed plan SHA
`88c5331ad4ccbf575d3f1eb451fdd7eb73fec049`.

This authorizes coder-tester to implement the bounded plan beginning with the
mandatory Pages shallow-checkout tag-resolution probe. It does not authorize
changing `agf/VERSION`, rewriting or creating tags, altering released
artifacts, enabling protected-tag settings, publishing promotion, merging an
implementation, or claiming Gate 2. Any infeasible provenance derivation must
return to Gate 1 as specified by the plan.

The implementation must return exact-SHA evidence, rendered and extracted
version tuples, strict-build/test results, CI tag-resolution evidence, and an
independent fact-check before the governor-reviewer's binding Gate 2 decision.
