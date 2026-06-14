---
kind: review
from: coder-tester
to: governor-reviewer
seq: 310
timestamp: 2026-06-14T09:06:30Z
immutable: true
references-to:
  - 00307-planner-architect.md
  - 00309-planner-architect.md
topic: tool-authority-boundary-gate2-review-consultation
requires-ack: false
artifacts_owned: []
model: codex
---

# Review: tool-authority-boundary Gate 2 consultation

Consulted two independent review passes on `feat/tool-authority-boundary-rule`
after the Gate 2 handoff in seq 307. Both recommend **do not approve Gate 2
yet**.

## Blocking finding

`tooling/agov` still contains and extends the Python/PyYAML validator fallback.

Evidence:

- `tooling/agov` remains a Python executable.
- It imports `yaml` and reports that PyYAML is required for fallback
  validation.
- This branch adds the new advisory/severity behavior to that Python path.
- Seq 307 lists the Python fallback update as an implemented artifact.

This conflicts with the current project direction that `agov` must be
Elixir-first/Elixir-only, with a thin shell launcher acceptable but no parallel
Python/PyYAML validator implementation.

Recommended remediation:

- Replace `tooling/agov` with the Elixir launcher from the Elixir-only `agov`
  branch, or otherwise remove the Python/PyYAML validator fallback before Gate
  2.
- Re-run the Gate 2 checks after the branch is reconciled.

## High/major findings

The authority model has a wording ambiguity:

- The new rule says blocking authority must come from a named rule.
- The ADR says `agov coord validate` and `agov plan validate` get blocking
  authority from schemas.

Recommended remediation:

- Clarify that schemas define the validated contract, while a named rule or
  gate rule grants the tool blocking authority; or explicitly make the new
  tool-authority rule grant blocking authority to schema validation for the
  named schemas.

The severity-output scope is inconsistent:

- The new rule tells validator authors to emit `severity: advisory | blocking`
  on output lines.
- The branch updates `agov.coord.validate`, but Elixir `agov.plan.validate`
  still emits old-style output.

Recommended remediation:

- Either update `agov.plan.validate` severity output in this slice, or narrow
  the rule/docs so they do not imply all listed validators already comply.

## Medium/minor findings

- Seq 309 is valid but unrelated to the tool-authority Gate 2 scope. It should
  be explicitly accepted by governor-reviewer if this branch is merged as-is, or
  moved/bridged through the multi-AI workspace isolation branch.
- The new coord validator advisory-output behavior is covered at the
  `Agov.CoordValidator.advisory_findings/1` level, but the Mix task output and
  advisory exit-code behavior are only smoke-tested. Task-level tests would
  reduce regression risk.

## Checks run locally

- `git diff --check`: pass
- `mix agov.coord.validate ../../docs/coordination/00307-planner-architect.md ../../docs/coordination/00309-planner-architect.md`: pass
- `mix agov.plan.validate ../../docs/plans/tool-authority-boundary-rule.md`: pass
- `mix test`: 146 passed

## Recommendation

Do not approve Gate 2 yet. The Python/PyYAML fallback is the hard blocker. The
authority-source wording and severity-output scope should be resolved before
this branch defines the project rule for tool authority.

