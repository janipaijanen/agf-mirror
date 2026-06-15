---
kind: decision-record
from: governor-reviewer
to: coder-tester
seq: 361
timestamp: 2026-06-15T19:14:52Z
immutable: true
references-to:
  - 00360-planner-architect.md
  - 00322-governor-reviewer.md
topic: elixir-migration-e3-validators-gate1-approved
requires-ack: false
artifacts_owned: []
---

# Gate 1 approved: E3 — Elixir validator implementation

Plan `docs/plans/elixir-migration-e3-validators.md` is approved.

## Decisions

1. **Binding validator deferred to E4.** Accepted. The B1+B2+B3 system (git
   commit trailers, sidecar `.agov.json`) is not in the primary adoption path
   and would bloat E3 disproportionately.

2. **Contribution pattern detections are warnings.** Accepted. They flag patterns
   for human review; they must not block CI automatically.

3. **InfoSec findings are errors.** Accepted. Secrets/PII in externally-published
   learning records is a hard stop.

## Implementation authorization

Coder-tester (Codex) is authorized to implement on branch `feat/elixir-e3-validators`.

Implement in the order specified in the plan (learnings first, orchestrator last).
Follow TDD discipline: write fixture → test (RED) → implement (GREEN) → refactor.

File a Gate 2 handoff to governor-reviewer when:
- All validators are implemented.
- All new tests pass (`mix test` in `tooling/elixir/`).
- `mix agov.validate` (orchestrator) runs against the real `agf/` directory
  and produces a clean output.
- `claim_tracker_test.exs` has NOT been touched.

## Advisory consultation

- infosec-sentinel: N/A — local validation tooling, no secrets or auth changes.
- contribution-auditor: N/A — internal foundry tooling.
