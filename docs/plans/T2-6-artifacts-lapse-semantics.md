---
id: T2-6-artifacts-lapse
status: pending-gate1
topic: artifacts-owned-lapse-semantics
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: schema extension only — no security boundary, auth, or secrets changes
  - role: contribution-auditor
    status: consulted
    reason: reviewed as seq 17 (contribution-auditor, codex-gpt-5)
---

# Plan: T2-6 — artifacts_owned Lapse Semantics

**Date:** 2026-06-11T05:05:55Z
**Revised:** 2026-06-11 — addresses contribution-auditor review (seq 17)
**Status:** PENDING GATE 1 APPROVAL
**Depends on:** T1-1 Phase A (coordination-message.schema.json, merged ✓), T1-1 Phase B (CoordValidator, merged ✓)

---

## Intent

The current `artifacts_owned` model has two gaps:

1. **No expiry.** A `claim` that is never followed by a `completion` hangs
   forever. If a session ends without posting a completion, the claim is
   abandoned with no recourse — there is no way to detect or resolve it.

2. **No partial release.** A `completion` releases all current claims implicitly
   (by posting `artifacts_owned: []`). There is no way to release a subset
   of owned artifacts while retaining others.

T2-6 formalises lapse semantics: an optional `expires_on` field on claims
signals when ownership is expected to end, and a stale-claim validator
reports claims that have passed their expiry without a release.

`releases_artifacts` (partial release) is **deferred** from this task. The
primary gap is stale-claim detection; partial release is a convenience that
can follow once the expiry model is established.

---

## Scope

Two deliverables:

1. **Schema extension** — add `expires_on` (optional ISO 8601 timestamp) to
   `spec/schemas/coordination-message.schema.json`
2. **Stale-claim validator** — `mix agov.coord.stale [--dir <coord-dir>]`
   that scans the coordination channel and reports claims past their `expires_on`
   that have not been released

`releases_artifacts` (partial release support) is explicitly deferred to a
follow-on task.

---

## Affected files

| File | Action |
|---|---|
| `spec/schemas/coordination-message.schema.json` | Update — add `expires_on` optional field |
| `tooling/elixir/lib/agov/claim_tracker.ex` | Create — pure ordered-log state machine |
| `tooling/elixir/lib/mix/tasks/agov.coord.stale.ex` | Create — Mix task (owns I/O, validation, exit behavior) |
| `tooling/elixir/test/agov/claim_tracker_test.exs` | Create — unit tests |
| `tooling/elixir/test/fixtures/stale_claim.md` | Create — expired claim, no release |
| `tooling/elixir/test/fixtures/active_claim.md` | Create — claim within expiry window |
| `tooling/elixir/test/fixtures/released_claim.md` | Create — claim followed by matching completion |
| `tooling/elixir/test/fixtures/claim_no_expiry.md` | Create — claim without `expires_on` |
| `tooling/elixir/test/fixtures/invalid_expires_on.md` | Create — malformed `expires_on` value |

---

## Schema design

```json
"expires_on": {
  "type": "string",
  "description": "Optional ISO 8601 timestamp. If set on a claim, ownership is considered stale after this time if no release has been posted. Ignored on non-claim messages."
}
```

Added as a top-level optional property alongside `model` and `session`. No
`format: date-time` constraint — draft-07 format validation is advisory and
inconsistently enforced by validators. Elixir owns all date parsing via
`DateTime.from_iso8601/1`.

---

## Ownership state machine

`ClaimTracker` is a **pure ordered-log state machine**. It takes a list of
already-parsed, already-validated message maps sorted by `seq` ascending, and
a `now` datetime. It has no filesystem access and no knowledge of
`CoordValidator`. The Mix task owns reading, sorting, validating, and calling
the tracker.

### Release algorithm (first-slice rules, applied in seq order)

1. **Claim** — for each path in `artifacts_owned`: record it as owned by
   `from`, with optional `expires_on`. If the path is already owned (by any
   agent), the later `seq` wins — the new claim **supersedes** the previous one.
   No conflict is recorded; supersession is the expected pattern for handoffs.
2. **Completion with `artifacts_owned: []`** — release all paths currently
   owned by this message's `from`. Paths owned by other agents are unaffected.
3. **Any other message** — no ownership change.

### Stale classification (applied after all messages are processed)

A path is **stale** if:
- It is currently owned (not released), AND
- Its active claim has `expires_on` set, AND
- `expires_on < now`

A path without `expires_on` is **never stale** — expiry is opt-in.

### Invalid `expires_on`

If a claim message has an `expires_on` that cannot be parsed by
`DateTime.from_iso8601/1`, the stale scanner treats it as an **error**: the
Mix task exits with code 2 and reports the offending file. A malformed
`expires_on` on a non-claim message is **silently ignored** — non-claim
`expires_on` has no semantic meaning.

### Test cases

| Scenario | Expected |
|---|---|
| Stale claim (expired, no release) | In stale list |
| Active claim (not yet expired) | In active list |
| Claim without `expires_on` | In active list; never stale |
| Released claim (completion, same agent) | In released list |
| Completion from different agent | Does not release the claim |
| Same path claimed twice by same agent | Second claim supersedes; only one active entry |
| Same path claimed by different agents | Later seq wins; earlier claim superseded |
| Invalid `expires_on` on a claim | Exit code 2, error reported |
| Invalid `expires_on` on non-claim | Silently ignored |

---

## Implementation plan

### Step 1 — Update schema

Add `expires_on` optional string property to `coordination-message.schema.json`.

### Step 2 — Write tests FIRST (TDD RED)

Fixtures (one message each; tests compose them into sequences):
- `stale_claim.md` — `claim`, `expires_on` a fixed past datetime, path `docs/foo.md`
- `active_claim.md` — `claim`, `expires_on` a fixed far-future datetime, path `docs/bar.md`
- `released_claim.md` — `claim` (no `expires_on`), path `docs/baz.md`
- `claim_no_expiry.md` — `claim`, no `expires_on`, path `docs/qux.md`
- `invalid_expires_on.md` — `claim`, `expires_on: "not-a-date"`, path `docs/bad.md`

A matching completion fixture releasing `released_claim.md`'s path is
constructed inline in tests (not a separate file — keeps fixture set minimal).

Tests inject a fixed `now` into `ClaimTracker.scan/2` to avoid time-dependent
flakiness. The fixed `now` is chosen to make `stale_claim.md` stale and
`active_claim.md` active.

### Step 3 — Implement ClaimTracker (GREEN)

`lib/agov/claim_tracker.ex`:

```elixir
defmodule Agov.ClaimTracker do
  def scan(messages, opts \\ [])
  # returns {:ok, %{stale: [...], active: [...], released: [...]}}
  # or {:error, [{:invalid_expires_on, path, value} | ...]}
  # opts: now: %DateTime{} (default: DateTime.utc_now())
end
```

No `scan_dir/1`. No filesystem access. No `CoordValidator` dependency.

### Step 4 — Implement Mix task

`mix agov.coord.stale [--dir <coord-dir>]`

The task owns:
1. List `*.md` files in coord dir, sort by filename (seq order)
2. Call `CoordValidator.validate_file/1` on each; skip non-message files (README, etc.)
3. Collect parsed maps; pass to `ClaimTracker.scan/2` with `now: DateTime.utc_now()`
4. Print results and exit

Output format (one line per stale path):
```
STALE  <path>  claimed-by: <from>  seq: <N>  claimed-at: <timestamp>  expired-at: <expires_on>
```

Exit codes:
- `0` — no stale claims
- `1` — one or more stale claims found
- `2` — error (dir not found, parse failure, invalid `expires_on` on a claim)

Default coord dir: `../../docs/coordination`. Run from `tooling/elixir/`.

---

## Test strategy

| Tier | What | Where |
|---|---|---|
| Unit | `ClaimTracker.scan/2` — all scenarios in the test-cases table, fixed `now` | `test/agov/claim_tracker_test.exs` |
| Manual | `mix agov.coord.stale` against `docs/coordination/` | Expect exit 0 (no `expires_on` in current channel) |

TDD order: schema update → fixtures → tests → RED → ClaimTracker → GREEN → Mix task.

---

## Risks

| Risk | Mitigation |
|---|---|
| `DateTime.from_iso8601/1` rejects valid but unusual formats | Use strict format in documentation; invalid = exit 2 not silent skip |
| Stale claims in current channel (none have `expires_on`) | Validator exits 0 cleanly; feature is opt-in |
| Partial-release pressure | Explicitly deferred — note in this plan prevents scope creep |
| Fixture `expires_on` dates become "past" relative to test `now` | Tests inject fixed `now`; fixture dates are irrelevant except relative to that fixed `now` |
