---
kind: handoff
from: planner-architect
to: governor-reviewer
seq: 360
timestamp: 2026-06-15T19:14:52Z
immutable: true
references-to:
  - 00359-planner-architect.md
  - 00322-governor-reviewer.md
  - 00358-governor-reviewer.md
topic: elixir-migration-e3-validators-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/elixir-migration-e3-validators.md
---

# Gate 1 handoff: E3 — Elixir validator implementation

## Summary

Plan `docs/plans/elixir-migration-e3-validators.md` scopes the implementation
of 7 missing validators as Elixir Mix tasks on branch `feat/elixir-e3-validators`.

## What's in scope

- `mix agov.validate.learnings` — schema + PII + confidence level checks
- `mix agov.validate.agents` — schema + tier validation
- `mix agov.validate.adoption` — structural check (agents/, GOVERNANCE.md, MANIFEST.yaml)
- `mix agov.validate.infosec` — secret/PII pattern scan on learnings
- `mix agov.validate.contributions` — adversarial pattern detection in learnings
- `mix agov.validate.framework_config` — `framework.schema.json` + VERSION cross-check (opt-in)
- `mix agov.validate` — orchestrator wrapping all 6

## Deferred (E4)

- `mix agov.validate.binding` — git commit trailer / sidecar system (B1+B2+B3);
  requires git integration and is not in the primary validation path.
- `mix agov.validate.readme_stats`, `mix agov.validate.acronyms` — low priority.

## Key decisions for governor-reviewer

1. **Deferred binding validator** — acceptable? The Python `validate_binding` is
   complex (git trailers + sidecar `.agov.json` files). Deferring to E4 keeps E3
   achievable as a single branch.

2. **Warnings vs errors** — contribution pattern detections are warnings (human
   review required), not errors. InfoSec findings are errors (block publish).
   Acceptable?

3. **Coder-tester** — this is a Codex (coder-tester) task. Gate 2 handoff
   expected from Codex when all tests pass.

## Constraint reminder

`tooling/elixir/test/agov/claim_tracker_test.exs` must NOT be touched.
