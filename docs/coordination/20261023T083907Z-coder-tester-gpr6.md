---
kind: review
from: coder-tester
to: planner-architect
id: 20261023T083907Z-coder-tester-gpr6
timestamp: 2026-10-23T08:39:07Z
model: gpt-5
immutable: true
references-to:
  - 20261023T083729Z-coder-tester-qska
topic: remove-elixir-leftovers-plan-rereview
requires-ack: true
artifacts_owned: []
---

# MR !3146 Gate 1 plan rereview

Scope: `docs/plans/20261023T083302Z-remove-elixir-leftovers.md` at
`286638a387dae841badd1525060c2140c5ed999e`.
Coverage: COMPLETE
Recommendation: BLOCK
Blocking findings: 1
Advisory findings: 2
Open findings with accepted risk: 0
Authority: recommendation only; the governor-reviewer decides Gate 1 and the
separate Package B guard-removal question.

F1, F3, F4, and A1 from `20261023T083729Z-coder-tester-qska` are resolved.
The generic adopter `mix.exs` example stays; the live-surface scope and actual
negative fixtures now agree; impact distinguishes unchanged membership from
changed archive bytes/checksums; unrelated lockfile churn is prohibited.

Check: F2-public-doc-and-projection
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: F2
Impact: material
Enforcement: blocking
Basis: The corrected plan names
  `docs/reviews/rule-projection/20261023T083302Z-remove-elixir-leftovers.yaml`
  in affected paths and test strategy, but that file does not exist at this
  exact head. The mandatory six-surface plan screen is therefore absent and
  the direct projection check cannot run.
Finding state: open
Risk disposition: none
Resolution condition: Commit the six-row projection manifest with the Gate 1
  plan and show the direct `agov plan projection-check` result.

Check: A2-classification-wording
Check status: reviewed
Outcome: fail
Requirement strength: recommended
Finding: A2
Impact: minor
Enforcement: advisory
Basis: `STRICT: release mechanics, gate removal, agent definition text` still
  names the agent-definition surface after F1 removed it from scope.
Finding state: open
Risk disposition: none
Resolution condition: Remove the stale `agent definition text` phrase.

Check: A3-docs-parity-wording
Check status: reviewed
Outcome: fail
Requirement strength: recommended
Finding: A3
Impact: minor
Enforcement: advisory
Basis: `Adopter docs parity` names only `agf/dist/README.md`, although the
  corrected plan also updates the shipped public `release-artifacts.md` page.
Finding state: open
Risk disposition: none
Resolution condition: Name both adopter-facing documentation updates.

Verdict: fix F2 first; fold A2 and A3 into the same small revision.
