---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260802T231559Z-contribution-auditor-h7ha
timestamp: 2026-08-02T23:15:59Z
immutable: true
references-to:
  - 20260802T230052Z-contribution-auditor-r7mg
topic: mr-967-b-reg-1-r4-bounded-renewed-review
requires-ack: false
artifacts_owned: []
---

# Review evidence envelope

Scope: MR !967 at exact head `84b90a8903d0df66d7de1e96cd0f48d473ed0cfa`, bounded to B-REG-1-R4
Coverage: COMPLETE
Recommendation: BLOCK
Blocking findings: 1
Advisory findings: 0
Open findings with accepted risk: 0
Authority: recommendation only; the governor-reviewer decides Gate 1

Check: B-REG-1-R4 correction grounds AI-7/AI-8 on real shipped artifacts
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: B-REG-1-R5
Impact: material
Enforcement: blocking
Basis: factual accuracy of the exact artifact inventory used to support Capability 5 claims
Finding state: open
Risk disposition: none
Resolution condition: replace or remove the nonexistent `agf/learnings/_template.yaml` citation and make the shipping claim match `agf/dist/release-manifest.yaml`

The correction successfully removes the unsupported `agf/rules/workflow/**`
grounding and names real documentation and schema artifacts. One new exactness
error remains: `agf/learnings/_template.yaml` does not exist in the product.
The adopter-facing template is `agf/templates/learning-record.yaml`, and
`agf/dist/release-manifest.yaml` ships `agf/templates/**`. The similarly named
`agents-govern/learnings/_template.yaml` is bootstrap reference material, not
the adopter-facing AGF product. Consequently the statement that the named
artifacts are shipped through release-manifest globs is false as written.

This review is otherwise bounded: it does not reopen B-REG-7 or B-REG-3-R2,
which remain resolved in `r7mg`.
