---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260913T171642Z-governor-reviewer-hs6j
timestamp: 2026-09-13T17:16:42Z
immutable: true
references-to:
  - 20260903T215909Z-coder-tester-67kq
requires-ack: false
topic: qbka-slice-b-two-supersession-ratifications
artifacts_owned: []
---

# Ratifying two qbka Slice B legacy-plan supersessions

Jani: "I'll ratify both now" (in response to a proposal describing each
disposition below).

## `docs/plans/plan-validator-advisory-role-allowlist.md`

Retroactively ratifying its `status: superseded` disposition. Historical
record: I directly committed `e3d301fd838d315bb74f79a739164fba7089b269`
(2026-07-26) marking this plan superseded, with the commit message
explaining the older, never-approved, contradictory plan was superseded by
`docs/plans/issue30-plan-validator-role-status-diagnostics.md` (Gate 1
approved 2026-07-24T10:10:00Z, implemented via MR !463). The only existing
coordination record referencing this, `20260726T173950Z-planner-architect-
mdkj`, is `kind: completion` from `planner-architect` and explicitly states
it exercises no Gate authority. This record supplies the missing
governor-reviewer authority for that already-executed disposition.

`superseded_by`: `issue30-plan-validator-role-status-diagnostics`

## `docs/plans/agf-coord-channel-fsm.md`

Declaring this plan superseded. It proposed refactoring the Elixir
`Agov.ClaimTracker` into a pure `Agov.ClaimFSM` module. The entire Elixir
tooling tree was later removed (`docs/plans/remove-elixir-tooling.md`,
implemented, `tooling/elixir/` no longer exists), making this plan's
proposal permanently inapplicable. No prior record declared this
disposition.

`superseded_by`: `remove-elixir-tooling`
