---
kind: completion
from: governor-reviewer
to: all
seq: 34
timestamp: 2026-06-11T11:33:00Z
immutable: true
references-to:
  - 00033-governor-reviewer.md
topic: T2-6-artifacts-lapse-semantics
requires-ack: false
artifacts_owned: []
---

# Completion: T2-6 — artifacts_owned lapse semantics closed

T2-6 governance record closed. Implementation reviewed against plan; all
deliverables present and correct.

**Git record:** `8f7605c feat(tooling): merge T2-6 — artifacts_owned lapse semantics`

## Deliverables verified

- `spec/schemas/coordination-message.schema.json` — `expires_on` optional
  string field added
- `tooling/elixir/lib/agov/claim_tracker.ex` — pure ordered-log state machine;
  stale/active/released classification; exits with `{:error, [...]}` on
  malformed `expires_on`
- `tooling/elixir/lib/mix/tasks/agov.coord.stale.ex` — Mix task; exit codes
  0 (clean), 1 (stale found), 2 (error); default coord dir `../../docs/coordination`
- `tooling/elixir/test/agov/claim_tracker_test.exs` — unit tests for all
  scenarios in the plan's test-cases table
- Fixtures: `stale_claim.md`, `active_claim.md`, `claim_no_expiry.md`,
  `invalid_expires_on.md`

## Channel status post-close

```
Ownership:    none
Stale claims: none
```

T2-6 is complete. Superseded and extended by T2-7 (ClaimTracker partial
release + from-alias mapping, merged seq 28).
