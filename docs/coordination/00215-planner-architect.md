---
kind: decision-record
from: planner-architect
to: coder-tester
seq: 215
timestamp: 2026-06-13T20:50:00Z
immutable: true
references-to:
  - 00213-planner-architect.md
topic: agf-v2-layout-s1-gate1-approved
requires-ack: true
artifacts_owned:
  - docs/plans/agf-v2-layout-s1.md
---

# Gate 1 Approved: AGF v2.0 Layout S1

Governor answered the three Gate 1 questions directly (2026-06-13).
Plan updated to `status: gate1-approved`. Proceed to implementation.

## Gate 1 answers (governor)

**Q1 — Content boundary:** All agent YAMLs from `agents-govern-2.0/agents/`.
Downstream adopters decide which agents to commission or modify.

**Q2 — Sync vs. copy:** Retire `spec/rules-template/` immediately in S1.
`agf/rules/` is the sole canonical source from S1 onwards.

**Q3 — `agents-govern-2.0/` retirement:** Retire immediately in S1.
`agf/` supersedes `agents-govern-2.0/` in the same slice.

## Hard constraint (governor, verbatim)

`agf/` MUST NOT ship `agents-govern/` content in any circumstances.
`agents-govern/` stays in git history; it is foundry-internal and never
part of the adopter distribution.

## Scope change from draft plan

S1 is now larger than originally drafted:

- `spec/rules-template/` retired in S1 (was: S2)
- `spec/schemas/` retired in S1 (was: implied S2 by extension)
- `agents-govern-2.0/` retired in S1 (was: S2)
- Tooling default `AGOV_SCHEMA_DIR` must be updated in S1 because
  `spec/schemas/` is being removed (was: not in scope)

## Implementation sequence (from updated plan)

1. Create `agf/` and populate from source dirs
2. Update `tooling/elixir` `AGOV_SCHEMA_DIR` default → `../../agf/schemas`
3. `mix test` — must pass (125 tests)
4. `git rm -r spec/rules-template/ spec/schemas/ agents-govern-2.0/`
5. `mix test` — must still pass
6. Update `AGENTS.md`
7. `mix agov.coord.visibility --base-branch master` — clean

## Branch

Create `feat/agf-v2-layout` from current master tip.

## Gate 2 requirement

Contribution-auditor advisory review required before Gate 2 merge (this is an
adopter-facing distribution change). Flag in the Gate 2 handoff.
