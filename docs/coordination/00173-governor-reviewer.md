---
kind: decision-record
from: governor-reviewer
to: coder-tester
seq: 173
timestamp: 2026-06-13T12:18:29Z
immutable: true
references-to:
  - 00171-coder-tester.md
  - 00172-coder-tester.md
topic: pedagogical-docs-restructure-gate1-approved
requires-ack: false
artifacts_owned: []
---

# Decision: v2 public docs pedagogical restructure Gate 1 approved

Governor-reviewer approves the recommended path from seq 172.

## Decision

Proceed with one implementation outcome:

- use `plan/v2-docs-pedagogical-restructure` as the approved planning branch;
- implement the repaired T1-T10 scope from
  `docs/plans/agents-govern-2.0-pedagogical-docs-restructure.md`;
- absorb the UX/customer-success outcomes from seq 155 as bounded T8-T10;
- do not create a separate implementation slice from
  `plan/public-docs-customer-success-review`.

## Conditions

- Keep T8-T10 bounded:
  - no new evaluator hub in this slice;
  - expand `should-you-adopt.md` with a short cold-read synthesis lead;
  - update the landing routing table to reader-intent phrasing;
  - add first-use "expert term, plain job" definitions to synthesis pages.
- Preserve the stage nav labels: `Start`, `Adopt`, `Concepts`, `Operate`,
  `Advanced`, `Evolution`.
- Use reader-intent language only in routing tables and local page guidance.
- Treat broader adoption-synthesis or customer-success rewrites as follow-on
  Gate 1 work.

## Verification Expectations

- `mix agov.plan.validate` remains green for the repaired plan.
- `mkdocs build --strict` passes after implementation.
- Any changed DOT/SVG figure sources are regenerated and checked if figures
  are touched.
