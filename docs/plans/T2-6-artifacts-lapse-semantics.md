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
    status: not-applicable
    reason: incremental and well-scoped extension to an existing schema field; governor-reviewer Gate 1 and Gate 2 review is sufficient oversight
---

# Plan: T2-6 — artifacts_owned Lapse Semantics

**Date:** 2026-06-11T05:05:55Z
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
| `tooling/elixir/lib/agov/claim_tracker.ex` | Create — ownership state machine (scan messages, track claim/release lifecycle) |
| `tooling/elixir/lib/mix/tasks/agov.coord.stale.ex` | Create — Mix task |
| `tooling/elixir/test/agov/claim_tracker_test.exs` | Create — unit tests |
| `tooling/elixir/test/fixtures/stale_claim.md` | Create — expired claim fixture |
| `tooling/elixir/test/fixtures/released_claim.md` | Create — claim released by completion |
| `tooling/elixir/test/fixtures/active_claim.md` | Create — claim within expiry window |
| `tooling/elixir/test/fixtures/claim_completion_pair.md` | Create — completion that releases the claim |

---

## Schema design

```json
"expires_on": {
  "type": "string",
  "format": "date-time",
  "description": "Optional ISO 8601 timestamp. If set on a claim, ownership is considered stale after this time if no release has been posted."
}
```

Added as a top-level optional property alongside `model` and `session`.
Meaningful only on `claim` kind; present on other kinds is ignored by the
validator (no `if/then` constraint — schema owns shape, Elixir owns semantics).

No `required` change — `expires_on` is always optional.

---

## Ownership model

The stale-claim validator implements this state machine per artifact path:

```
UNCLAIMED
  → on claim message with path in artifacts_owned: CLAIMED (with optional expires_on)
  → on completion message with empty artifacts_owned (same agent): RELEASED
CLAIMED
  → if expires_on is set and now > expires_on and no release: STALE
  → on completion with empty artifacts_owned (same agent): RELEASED
STALE
  → reported by validator; remains stale until explicitly released
RELEASED
  → terminal; path is free to be claimed again
```

**Agent identity** for release matching: the `from` field of the completion
message must match the `from` field of the most recent claim for that path.
A completion from a different agent does not release another agent's claim.

---

## Implementation plan

### Step 1 — Update schema

Add `expires_on` optional string property to `coordination-message.schema.json`.
No other schema changes.

### Step 2 — Write tests FIRST (TDD RED)

Fixtures:
- `stale_claim.md` — `claim` with `expires_on` in the past, no completion
- `active_claim.md` — `claim` with `expires_on` in the future
- `released_claim.md` — `claim` followed by matching `completion`
- `claim_completion_pair.md` — completion that releases `stale_claim.md`'s artifacts

Tests for `ClaimTracker`:
- Stale claim (expired, no release) → reported in stale list
- Active claim (not expired) → not reported
- Released claim (completion from same agent) → not reported
- Claim with no `expires_on` → never reported as stale (expiry is opt-in)
- Completion from different agent → does not release claim
- Multiple claims for same path → most recent claim is active

### Step 3 — Implement ClaimTracker (GREEN)

`lib/agov/claim_tracker.ex`:

1. `scan(messages)` — takes a list of parsed message maps (already validated),
   returns `%{stale: [...], active: [...], released: [...]}`
2. `scan_dir(coord_dir)` — reads and validates all `.md` files from the
   coordination directory, calls `scan/1`
3. Stale = `expires_on` present, parsed as DateTime, before `DateTime.utc_now()`,
   and no matching release found

ClaimTracker does **not** call `CoordValidator` directly — it receives already-
parsed maps. The Mix task handles file reading and validation before passing
to ClaimTracker.

### Step 4 — Implement Mix task

`mix agov.coord.stale [--dir <coord-dir>]`

Default coord dir: `../../docs/coordination` (same as `agov.handoff.next`).
Run from `tooling/elixir/`.

Output format:
```
STALE  path/to/file  claimed-by  claimed-at  expired-at
```

Exit codes:
- `0` — no stale claims
- `1` — one or more stale claims found
- `2` — error (directory not found, parse failure)

---

## Test strategy

| Tier | What | Where |
|---|---|---|
| Unit | `ClaimTracker.scan/1` — stale, active, released, no-expiry, wrong-agent, multi-claim | `test/agov/claim_tracker_test.exs` |
| Manual | `mix agov.coord.stale` against `docs/coordination/` | Run from `tooling/elixir/`; expect no stale claims on current channel |

TDD order: schema update → fixtures → tests → RED → ClaimTracker → GREEN → task.

---

## Risks

| Risk | Mitigation |
|---|---|
| `expires_on` format validation in ex_json_schema | `format: date-time` is advisory in draft-07; parse validation done in Elixir with `DateTime.from_iso8601/1` |
| Clock skew between agents | `expires_on` is advisory — stale means "worth reviewing", not "automatically invalid" |
| Real coordination channel has no `expires_on` claims yet | Validator exits 0 cleanly (no stale claims); feature is opt-in |
| Partial-release pressure | Explicitly deferred — note in this plan prevents scope creep |
