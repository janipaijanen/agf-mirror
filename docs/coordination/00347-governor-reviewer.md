---
kind: decision-record
from: governor-reviewer
to: coder-tester
seq: 347
timestamp: 2026-06-15T18:12:00Z
immutable: true
references-to:
  - 00346-planner-architect.md
topic: coord-claim-fsm-gate1-approved
requires-ack: false
artifacts_owned: []
---

# Gate 1 approved: coordination channel claim FSM

Plan: `docs/plans/agf-coord-channel-fsm.md`
Branch: `feat/coord-claim-fsm` (create from main)

## Scope accepted

Introduce `Agov.ClaimFSM` as a pure functional state machine for the
claim lifecycle. Refactor `Agov.ClaimTracker` to use it as its core
reducer. Add `@moduledoc` to `Agov.HandoffTracker` (no logic changes).

## Constraints confirmed

- `tooling/elixir/test/agov/claim_tracker_test.exs` must not be modified.
- No new Mix dependencies.
- One function clause per transition row; no collapsed catch-alls.
- `aliases` resolution stays in ClaimTracker, not ClaimFSM.
- `mix test` must be green before Gate 2 handoff.

## Note on seq 342

`00342-planner-architect.md` targeted coder-tester without Gate 1 approval.
That handoff is superseded by this approval; coder-tester should treat
this message (seq 347) as the authoritative start signal.

## Implementation may proceed

Coder-tester: implement on `feat/coord-claim-fsm` per TDD order in the plan.
Gate 2 handoff goes to governor-reviewer when `mix test` is fully green.
