---
id: agf-coord-channel-fsm
title: Coordination Channel — Explicit Claim FSM
created: 2026-06-14T20:10:27Z
status: superseded
branch: feat/coord-claim-fsm
superseded_by: remove-elixir-tooling
superseded_decision: 20260913T171642Z-governor-reviewer-hs6j.md
superseded_at: 2026-09-13T17:16:42Z
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: Superseded before implementation or real consultation occurred; consultation is moot.
  - role: contribution-auditor
    status: not-applicable
    reason: Superseded before implementation or real consultation occurred; consultation is moot.
---

**Correction (2026-09-13, qbka Slice B):** this plan proposed refactoring
the Elixir `Agov.ClaimTracker` into a pure `Agov.ClaimFSM` module. It was
never approved (`gate-1-pending`, an invalid enum spelling besides) and the
entire Elixir tooling tree was later removed
(`docs/plans/remove-elixir-tooling.md`, implemented) — `tooling/elixir/`
no longer exists in this checkout. The proposal is permanently
inapplicable; marked `superseded` per `hs6j`. See
`docs/plans/20260913T171837Z-qbka-slice-b-legacy-plan-migration.md`.

# Coordination channel: explicit claim FSM

## Intent

Make the coordination channel's claim lifecycle explicit as a pure
functional state machine (`Agov.ClaimFSM`), then refactor
`Agov.ClaimTracker` to use it as its core reduction step.

The FSM becomes the authoritative, readable specification for what
valid claim/completion sequences mean. Tests against it can be written
as a transition table — exhaustive, declarative — rather than
scenario-by-scenario unit tests against the full tracker.

`HandoffTracker` is out of scope: its resolution condition is
graph-reachability (BFS over message reference chains), not a
per-message FSM. That algorithm is correct as-is; adding a module
docstring with the state diagram is sufficient documentation there.

## State machine (ClaimFSM)

One FSM instance per artifact path. States:

```
:unclaimed
{:active, entry}    # claimed, not expired
{:stale, entry}     # claimed, expires_on in the past
{:released, entry}  # completed by the owning role
```

Entry: `%{from, seq, claimed_at, expires_on}`.

Events:

```
{:claim, from, seq, claimed_at, expires_on}
{:complete, from}     # completion covering this path (blanket or explicit)
{:tick, now}          # advance time; re-classify active → stale if needed
```

Transition table:

| State           | Event                              | Next state              |
|-----------------|------------------------------------|-------------------------|
| `:unclaimed`    | `{:claim, ...}`                    | `{:active\|:stale, e}` |
| `{:active, _}`  | `{:claim, ...}`                    | supersede → `active\|stale` |
| `{:stale, _}`   | `{:claim, ...}`                    | supersede → `active\|stale` |
| `{:active, e}`  | `{:complete, e.from}`              | `{:released, e}`        |
| `{:stale, e}`   | `{:complete, e.from}`              | `{:released, e}`        |
| `{:active, e}`  | `{:complete, other}`               | `{:active, e}` (no-op) |
| `{:stale, e}`   | `{:complete, other}`               | `{:stale, e}` (no-op)  |
| `{:released,_}` | any                                | `{:released, _}` (terminal) |
| `:unclaimed`    | `{:complete, _}`                   | `:unclaimed` (no-op)   |
| `{:active, e where expires_on: nil}` | `{:tick, _}` | `{:active, e}` (no-op) |
| `{:active, e}` | `{:tick, now}` where now > e.expires_on | `{:stale, e}` |
| `{:stale, _}`  | `{:tick, _}`                       | unchanged              |

Claim supersession: a later `:claim` event for the same path always
overwrites the current entry, regardless of which role sent it or
whether the current state is `:active` or `:stale`. The `seq` on the
new entry establishes which claim is authoritative.

`aliases` resolution (matching `completion.from` to `claim.from` via
a canonical-name map) is handled by `ClaimTracker` before emitting
`:complete` events, not inside `ClaimFSM`. The FSM only compares
strings.

## Affected files

| File | Action |
|------|--------|
| `tooling/elixir/lib/agov/claim_fsm.ex` | **create** — pure FSM module |
| `tooling/elixir/lib/agov/claim_tracker.ex` | **modify** — use ClaimFSM as core reducer |
| `tooling/elixir/test/agov/claim_fsm_test.exs` | **create** — transition table tests |
| `tooling/elixir/test/agov/claim_tracker_test.exs` | unchanged (must stay green) |
| `tooling/elixir/lib/agov/handoff_tracker.ex` | **modify** — add `@moduledoc` with state diagram |

## Migration needed

No. Internal refactor only; no public API or schema changes.

## Test strategy

TDD:

1. Write `claim_fsm_test.exs` covering every row in the transition
   table. Each test is one input state + one event → expected next
   state. All must fail (RED) before `claim_fsm.ex` exists.
2. Implement `ClaimFSM` to GREEN.
3. Refactor `ClaimTracker` to use `ClaimFSM`. Run existing
   `claim_tracker_test.exs` without modification — must stay GREEN.
4. Add HandoffTracker `@moduledoc`.

No new dependencies. StreamData (property-based) is a natural
follow-on if the FSM proves stable, but is not in this slice.

## Risks

- Behavioral regression during ClaimTracker refactor. Mitigated by:
  the existing `claim_tracker_test.exs` suite covers all cases
  (stale/active/released, supersession, partial completion, aliases,
  invalid expires_on) and must pass before the MR is filed.

## Rollback

`git revert` the refactor commit. `ClaimFSM` can stay (it is additive
and unused if ClaimTracker is reverted).

## What done looks like

- `mix test` green (148 + new FSM tests).
- `claim_fsm.ex` contains the full transition table with one function
  clause per row; no business logic in `claim_tracker.ex` that
  duplicates what the FSM handles.
- A reviewer can read the FSM module and verify correctness against
  this plan's transition table without reading ClaimTracker.
