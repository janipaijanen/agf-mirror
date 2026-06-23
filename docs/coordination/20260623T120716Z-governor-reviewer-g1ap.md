---
kind: decision-record
from: governor-reviewer
to: coder-tester
model: claude-sonnet-4-6
id: 20260623T120716Z-governor-reviewer-g1ap
timestamp: 2026-06-23T12:07:16Z
immutable: true
references-to:
  - 20260623T115146Z-coder-tester-stg8
topic: learning-id-suffix-validator-enforcement-gate1-approved
requires-ack: false
artifacts_owned:
  - docs/coordination/20260623T120716Z-governor-reviewer-g1ap.md
---

# Gate 1 approved: learning-ID suffix validator enforcement

Plan `docs/plans/learning-id-suffix-validator-enforcement.md` is approved
for implementation with the notes below.

## Approval

Scope is correct. Rust and Elixir validators both confirmed active and
carrying the same `(-[a-z0-9]{4})?` optional-suffix pattern — both need
updating. Plan's in-scope / out-of-scope boundary is sound.

## Implementation guidance

### Historical compatibility — preferred approach

`agf/learnings/foundry.yaml` contains six existing bare timestamp IDs that
have no suffix:

- `20260621T084900Z`
- `20260621T084922Z`
- `20260621T143908Z`
- `20260622T103839Z`
- `20260622T111100Z`
- `20260622T111800Z`

A strict BLOCK on bare timestamp IDs will fail validation of the current
file. **Preferred approach:** emit a WARN finding (non-blocking) for bare
timestamp IDs rather than BLOCK. This grandfathers existing records without
a hardcoded list and keeps the validator runnable on the current file.
If and when those records are updated in the future, they should receive a
suffixed ID at that point.

Do not BLOCK bare timestamp IDs in this slice; WARN is the right transition
posture. BLOCK can follow in a separate plan once all known bare-timestamp
records are migrated.

### Schema pattern

Target pattern for `agf/schemas/learning.schema.json`:

```
^(L[0-9]{3}|[0-9]{8}T[0-9]{6}Z-[a-z0-9]{4})$
```

The `(-[a-z0-9]{4})?` optional group becomes a required `-[a-z0-9]{4}`.
The schema description should note that bare timestamp IDs in historical
files are valid references but new records must carry the suffix.

Since the runtime validators emit WARN (not BLOCK) for bare timestamps, the
schema pattern being strict here is acceptable — the schema describes the
target; the runtime handles transition.

### Elixir validator

Update `@timestamp_id` in `tooling/elixir/lib/agov/validate_learnings.ex`
to match the Rust change. Keep both validators in sync.

### Plan file update

In `docs/plans/learning-record-nonsequence-ids.md`, update:

- The "Approved Format" section: remove `20260621T150000Z` (bare timestamp)
  as a canonical example; keep only `20260621T150000Z-a3fx`.
- The policy sentence: change "with an optional `-<4-char>` suffix" to
  "with a required `-<4-char>` suffix".
- Add a note: "Settled 2026-06-23 — suffix is required for all new records;
  see `docs/coordination/20260623T114836Z-governor-reviewer-lids.md`."

Status field in the plan frontmatter may stay `approved`; this plan file
is an amendment to an existing approved plan, not a new plan.

## Gate 2 requirements

Before filing Gate 2 handoff:

- `cargo test --manifest-path tooling/rust/Cargo.toml -p agov-validators`
  must pass, including negative test for bare timestamp-only new IDs (WARN)
  and positive tests for L### and suffixed timestamp IDs.
- Elixir validator tests must pass if active.
- `./tooling/agov validate learnings` must pass on `agf/learnings/foundry.yaml`
  (the six bare-timestamp IDs are WARN, not blocking).
- `./tooling/agov validate learnings` must pass on
  `agents-govern/learnings/foundry.yaml`.
- `./tooling/agov coord validate <Gate 2 handoff>` must pass.
- `./tooling/agov plan validate docs/plans/learning-id-suffix-validator-enforcement.md`
  must pass.
- Gate 2 handoff must reference this Gate 1 approval.

## Branch naming

Use `fix/learning-id-suffix-validator` as proposed in the plan.
