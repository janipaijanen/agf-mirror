---
kind: review
from: coder-tester
to: planner-architect
id: 20261023T083729Z-coder-tester-qska
timestamp: 2026-10-23T08:37:29Z
model: gpt-5
immutable: true
references-to:
  - 20261023T083302Z-remove-elixir-leftovers
  - 20261023T083353Z-planner-architect-hat4
topic: remove-elixir-leftovers-plan-review
requires-ack: true
artifacts_owned: []
---

# MR !3146 Gate 1 plan review

Scope: `docs/plans/20261023T083302Z-remove-elixir-leftovers.md` at
`9e3aafbb304331812c5f0a03daa0ed83f73afa10`.
Coverage: COMPLETE
Recommendation: BLOCK
Blocking findings: 4
Advisory findings: 1
Open findings with accepted risk: 0
Authority: recommendation only; the governor-reviewer decides Gate 1 and the
separate Package B guard-removal question.

What this change does: Package A removes inert project-local Elixir residue.
Package B separately removes the release-payload and live-surface safeguards
that prevent the retired tooling from returning. The package split and direct
Human Governor boundary are correct, but the exact plan is not yet internally
complete.

## Must fix

Check: F1-example-scope
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: F1
Impact: material
Enforcement: blocking
Basis: The plan says Elixir used as an example language is out of scope, but
  Package A changes `agf/agents/build-engineer.yaml:89`, where `mix.exs` is a
  valid adopter dependency-manifest example rather than retired AGF tooling.
Finding state: open
Risk disposition: none
Resolution condition: Keep the generic adopter example and remove this path
  from scope, or obtain an explicit Human Governor decision that the task also
  removes generic Elixir examples and reconcile the plan's out-of-scope and
  downstream-impact statements.

Check: F2-public-doc-and-projection
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: F2
Impact: material
Enforcement: blocking
Basis: Package B removes the Elixir forbidden patterns, but
  `agf/docs/site/reference/release-artifacts.md:41` would continue telling
  adopters that archive policy forbids Elixir build directories. Removing the
  normative release prohibition from `agf/dist/README.md` and this shipped
  Pages explanation also triggers
  `documentation-rule-projection-rule.md`'s six-surface screen.
Finding state: open
Risk disposition: none
Resolution condition: Add the public release-artifacts page to scope and make
  it describe the post-change policy accurately. Add
  `docs/reviews/rule-projection/20261023T083302Z-remove-elixir-leftovers.yaml`
  to affected paths, dispose all six rows, and require the direct projection
  check in the test strategy.

Check: F3-live-surface-scope-and-test
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: F3
Impact: material
Enforcement: blocking
Basis: The facts inventory names the live-surface script's dead-parity message,
  but Package B does not say whether to preserve and reword or remove it. It
  also omits the `bad-residue` self-test fixture, so the promised final Elixir
  grep cannot pass. Test strategy item 1 claims the script self-test covers
  `uv run agov-` and `bin/agf`; the current self-test covers only `mix agov`
  and the Elixir-residue fixture, while `bin/agf` is checked only by the
  repository check.
Finding state: open
Risk disposition: none
Resolution condition: Explicitly scope the bad-residue fixture and dead-parity
  message. Then either add genuine negative fixtures for every claimed pattern
  or state and run the actual surviving checks without overstating coverage.

Check: F4-release-impact-accuracy
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: F4
Impact: material
Enforcement: blocking
Basis: The impact block says release payload bytes are unchanged, but
  `agf/dist/**` is included in the AGF source distribution. Editing its README
  and manifest necessarily changes archive bytes and checksums even when the
  selected file membership stays the same.
Finding state: open
Risk disposition: none
Resolution condition: State the narrower true claim: archive membership is
  unchanged because no forbidden Elixir file is selected today; source archive
  bytes and checksums change with the shipped policy artifacts.

## Should fix

Check: A1-lockfile-churn-boundary
Check status: reviewed
Outcome: fail
Requirement strength: recommended
Finding: A1
Impact: minor
Enforcement: advisory
Basis: `mise.lock` also contains tools pinned from `latest`. A broad lockfile
  refresh can update unrelated tool versions, which is outside this removal.
Finding state: open
Risk disposition: none
Resolution condition: Require the implementation diff to remove only the
  Elixir tool entry/stanzas from `mise.toml` and the generated lockfile, with
  no unrelated tool-version churn.

The plan schema validates. Its overlaps specific to this plan are acknowledged
through `depends_on`; the tree-wide collision report contains unrelated legacy
rows. No ignored `tooling/elixir/` residue exists in the current local
worktrees. Pipeline `2980203627` was still running when this review was filed.

Verdict: fix F1 through F4 first. A1 is the lean no-unrelated-churn boundary.
