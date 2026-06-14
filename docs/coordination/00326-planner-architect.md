---
kind: handoff
from: planner-architect
to: coder-tester
seq: 326
timestamp: 2026-06-14T20:12:00Z
immutable: true
references-to: []
topic: implement-coord-claim-fsm
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - "Implement Agov.ClaimFSM and refactor Agov.ClaimTracker to use it"
  must_read:
    - "docs/plans/agf-coord-channel-fsm.md"
    - "tooling/elixir/lib/agov/claim_tracker.ex"
    - "tooling/elixir/test/agov/claim_tracker_test.exs"
  do_not_touch:
    - "tooling/elixir/test/agov/claim_tracker_test.exs"
---

# Implement coordination channel claim FSM

Plan: `docs/plans/agf-coord-channel-fsm.md`
Branch: `feat/coord-claim-fsm` (create from main)

## What to build

A pure functional state machine module (`Agov.ClaimFSM`) that makes
the coordination channel's claim lifecycle explicit, then refactor
`Agov.ClaimTracker` to use it as its core reducer.

## FSM specification

Four states per artifact path:

```
:unclaimed
{:active, entry}    # claimed, not expired
{:stale, entry}     # claimed, expires_on in the past
{:released, entry}  # completed by the owning role
```

Three events:

```
{:claim, from, seq, claimed_at, expires_on}
{:complete, from}
{:tick, now}
```

Full transition table is in the plan file.

## Implementation order (TDD)

1. Write `tooling/elixir/test/agov/claim_fsm_test.exs` — one test per
   transition table row. All must fail (RED) before the module exists.
2. Create `tooling/elixir/lib/agov/claim_fsm.ex` — one function
   clause per transition row. GREEN.
3. Refactor `tooling/elixir/lib/agov/claim_tracker.ex` to use
   `ClaimFSM` as its core reducer. `claim_tracker_test.exs` must
   remain GREEN without modification.
4. Add `@moduledoc` to `tooling/elixir/lib/agov/handoff_tracker.ex`
   documenting the pending/resolved states and graph-reachability
   resolution. No logic changes there.

## Constraints

- `tooling/elixir/test/agov/claim_tracker_test.exs` must not be
  modified. It is the regression guard for ClaimTracker's behaviour.
- No new Mix dependencies.
- `mix test` must end at 148 tests + new FSM tests, all green.
- One function clause per transition row in ClaimFSM — no collapsed
  catch-all clauses that hide cases.
- `aliases` resolution stays in ClaimTracker, not in ClaimFSM. The
  FSM compares strings only.

## Done when

- `mix test` green.
- A reviewer can read `claim_fsm.ex` and verify each clause against
  the plan's transition table without opening `claim_tracker.ex`.
- Gate 2 handoff filed back to governor-reviewer.
