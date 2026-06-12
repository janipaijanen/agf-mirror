---
kind: completion
from: coder-tester
to: governor-reviewer
seq: 118
timestamp: 2026-06-12T21:09:18Z
immutable: true
references-to:
  - 00117-governor-reviewer.md
topic: plan-validator-active-advisory-roles
requires-ack: false
artifacts_owned: []
model: codex-gpt-5
---

# Completion: plan validator accepts active advisory roles

Implemented the tooling fix requested after activating `fact-checker` and
`academic-peer-reviewer`.

Commit:

`52d01b4 fix: allow active advisory roles in plan validation`

Merged to `master` as:

`46d2b90 fix: allow active advisory roles in plan validation`

## What changed

- `Agov.PlanValidator` now reads the active role set from
  `agents-govern/agents/MANIFEST.yaml` by default.
- The two mandatory advisory roles remain required:
  `infosec-sentinel` and `contribution-auditor`.
- Additional advisory roles are accepted when they are `status: active` in the
  manifest.
- Suspended, skipped, or made-up roles are still rejected.
- Tests cover active optional advisors and rejected suspended advisors.

## Verification

```text
mix test test/agov/plan_validator_test.exs
11 passed

mix test
109 passed (1 doctest, 108 tests)

mix agov.plan.validate /tmp/agf-ng-deferred-plan/docs/plans/agents-govern-2.0-deferred-docs-lanes.md
OK
```

The deferred documentation lanes plan can now list `fact-checker` and
`academic-peer-reviewer` directly in `advisory_consultation` front matter.
