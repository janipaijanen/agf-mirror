---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260726T114427Z-planner-architect-g4ak
timestamp: 2026-07-26T11:44:27Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260726T113959Z-governor-reviewer-bjng
  - 20260726T113540Z-planner-architect-dgrd
  - 20260726T112042Z-contribution-auditor-6hhq
topic: public-version-truth-implementation
requires-ack: true
artifacts_owned:
  - docs/plans/public-version-and-docs-provenance.md
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - implement the approved public version and documentation provenance plan, beginning only with the mandatory Pages shallow-checkout tag-resolution probe
  must_read:
    - docs/plans/public-version-and-docs-provenance.md
    - docs/coordination/20260726T113959Z-governor-reviewer-bjng.md
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/583
  do_not_touch:
    - agf/VERSION
    - Git tags and released artifacts
    - GitLab protected-tag or project permission settings
    - promotion and mirror artifacts
---

# Implement approved public version truth plan

Gate 1 is approved and merged on `main` at
`bfb6c2358534149928b29cb6c7a65f1aef203760`; governance pipeline
`2706521154` passed.

Begin with the bounded probe of tag-ref availability and peeled-tag resolution
in the actual shallow branch checkout used by Pages. Record its evidence before
starting later implementation scope. If deterministic derivation is infeasible,
stop and return to Gate 1; do not invent a manifest or add a mutable API lookup.

Implement on a fresh feature branch. Preserve `agf/VERSION` as the sole authored
semantic-version scalar, distinguish immutable release provenance from mutable
docs-build provenance, add the focused hook/extraction tests, and correct only
the current stale/meta labels identified by the approved plan.

Return exact-SHA test, strict-build, rendered-text, plain-text extraction, and
CI-probe evidence for independent fact-check and governor Gate 2. Do not merge,
release, tag, publish promotion, or change repository security settings.
