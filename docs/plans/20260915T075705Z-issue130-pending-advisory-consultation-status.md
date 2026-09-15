---
id: 20260915T075705Z-issue130-pending-advisory-consultation-status
status: pending-gate1
topic: issue130-pending-advisory-consultation-status
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Schema field addition and a semantic Rust validation check on
      already-present plan-frontmatter fields; no credential, secret,
      network, or new write authority introduced. Per
      `agf/agents/MANIFEST.yaml`, infosec-sentinel is CI-only and cannot
      be self-performed (consistent with the disclosed limitation
      already on record this session, e.g.
      `20260914T070135Z-planner-architect-7sdp`).
  - role: contribution-auditor
    status: consulted
    reason: >-
      Self-performed consultation published as
      20260915T075814Z-contribution-auditor-4pj7.md, before Gate 1
      request, per this project's established practice for a plan
      introducing gate mechanics. Found and fixed two real gaps in the
      same draft (a non-compiling code snippet, and a deferred
      rule-projection manifest that should have existed at Gate 1) --
      see that record for detail.
affected_paths:
  - agf/schemas/gate1-plan.schema.json
  - tooling/rust/crates/agov-coord/src/plan_validator.rs
  - agf/templates/gate1-plan.md
  - AGENTS.md
  - agf/templates/AGENTS.md
  - docs/reviews/rule-projection/20260915T075705Z-issue130-pending-advisory-consultation-status.yaml
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Issue #130 — a schema-true `pending` status for advisory consultation

## Provenance

- Issue **#130**: `advisory_consultation[].status` accepts only
  `consulted | waived | not-applicable`. A plan under active review has
  no truthful value for "requested but not yet answered" -- MR !2478 had
  to reuse `waived` with explanatory prose even though the reviews were
  mandatory and outstanding, confirmed by contribution audit
  `20260907T062346Z-contribution-auditor-a8pj` (finding A1): `waived`
  semantically means "deliberately skipped," so status-keyed tooling or
  a Gate 1 reviewer could misread an outstanding review as one that was
  never requested.
- **Fresh corroborating evidence, this session (2026-09-15):** drafting
  Row 6 S4's plan hit the identical gap directly -- the frontmatter
  initially declared `contribution-auditor: status: pending` (the
  truthful state at that point in drafting) and `agov plan validate`
  correctly rejected it, since `pending` is not a valid enum value
  today. The real consultation had to be performed and the field
  flipped to `consulted` before the file would validate at all -- which
  is the *right* outcome for that specific plan, but only because the
  validator happened to reject an invalid enum value, not because it
  understands "pending" as a real state. A plan author who instead
  reused `waived` (as MR !2478 did) would have passed validation while
  making the same false claim #130 already documents.

## Problem

There is no schema-true way to say "this row is requested and
outstanding" as opposed to "this row was consulted," "this row was
waived (deliberately skipped)," or "this row is not-applicable." Authors
currently have three bad options: (a) reuse `waived` with prose
explaining it isn't really waived (MR !2478's approach, actively
misleading to status-keyed tooling), (b) leave the plan simply unable to
validate until the real consultation happens (blocks progress on
drafting the rest of the plan around it), or (c) omit the row entirely
until ready (schema requires every declared role to have `status`, and
`required_roles()` requires both `infosec-sentinel` and
`contribution-auditor` to be present at all -- so omission isn't
actually legal either). None of the three lets an author write down the
truth.

## Design

### Schema: `pending` (new enum value)

Add `"pending"` to `advisory_consultation[].status`'s enum in
`agf/schemas/gate1-plan.schema.json`:

```json
"status": {
  "type": "string",
  "enum": [
    "consulted",
    "waived",
    "not-applicable",
    "pending"
  ]
}
```

Purely additive: every existing plan using the three prior values
remains valid unchanged. `reason` stays optional for `pending` (unlike
`waived`, which already requires it) -- a pending row's own status
already says everything required; naming the requesting coordination
record's id in `reason` is recommended practice, not enforced.

### Semantic validation: `pending` blocks Gate 1 approval (the actual fix)

The schema addition alone would let a plan sit at `status: approved`
with a `pending` consultation row forever -- exactly as useless as
today's `waived` misuse, just honestly labeled instead of dishonestly
labeled. The real fix is a new check in
`tooling/rust/crates/agov-coord/src/plan_validator.rs`'s
`validate_semantics_with_manifest` (the same function that already
enforces required-role presence and active-role status), mirroring its
existing pattern:

The existing function only extracts role *names* into `roles: Vec<String>`
(for the required-role-presence check) -- it does not currently retain
each row's `status` alongside its `role`. The new check needs both, so
it iterates `data.get("advisory_consultation")` a second time (matching
this function's own existing style of one focused loop per concern,
rather than restructuring the existing `roles` extraction):

```rust
// New check, alongside the existing required-role and
// role-active-status checks in validate_semantics_with_manifest:
let plan_status = data.get("status").and_then(serde_json::Value::as_str);
if matches!(plan_status, Some("approved" | "active" | "complete")) {
    let pending_rows = data
        .get("advisory_consultation")
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten()
        .filter(|entry| entry.get("status").and_then(serde_json::Value::as_str) == Some("pending"))
        .filter_map(|entry| entry.get("role").and_then(serde_json::Value::as_str));
    for role in pending_rows {
        errors.push((
            format!(
                "plan status is '{}' but advisory_consultation role '{role}' is still pending -- Gate 1 approval requires every required-role row to leave pending first",
                plan_status.unwrap()
            ),
            "#/advisory_consultation".to_string(),
        ));
    }
}
```

A plan at `status: pending-gate1` MAY have `pending` rows -- that is
exactly the state `pending` exists to represent (mid-drafting, review
requested, not yet answered). The block fires only once a plan claims
`approved`/`active`/`complete` while a required row is still `pending`
-- the literal MR !2478 failure mode #130 documents, now caught by the
validator itself rather than relying on plan-author self-discipline
("the plan self-blocks... in prose," per `a8pj`'s own observation).

### Non-goals

- No change to which roles are required (`infosec-sentinel`,
  `contribution-auditor` via `required_roles()`) -- unaffected.
- No change to `waived`'s existing `reason`-required behavior.
- No retroactive requirement that any existing plan add `pending` rows
  or otherwise change -- the three prior enum values keep their exact
  current meaning and validation behavior.
- No new coordination-message kind or field for "consultation requested"
  -- an author still uses an ordinary `handoff`/`note` to request a real
  review, same as today; this slice only fixes what the plan file itself
  is permitted to say about that request's outstanding state.

## Test strategy (gate-self-test-rule: this is gate mechanics)

`agov plan validate` is an existing mechanically-enforced blocking
check; this slice adds new blocking behavior to it, so it needs its own
negative/positive evidence, not merely a schema-shape test:

1. **Negative (the new check fires):** a plan with `status: approved`
   and one `advisory_consultation` row `status: pending` ->
   `agov plan validate` fails, naming the specific role and stating Gate
   1 approval requires it to leave pending first.
2. **Positive, mid-draft:** the same plan at `status: pending-gate1`
   with the identical `pending` row -> validates cleanly (pending is
   legitimate pre-Gate-1 state).
3. **Positive, flipped:** the same plan at `status: approved` with the
   row changed to `consulted` -> validates cleanly (matches this
   session's own real Row 6 S4 experience).
4. **Backward compatibility:** every existing enum value
   (`consulted`/`waived`/`not-applicable`) at every plan status,
   unmodified, continues to validate exactly as before -- re-run
   unchanged against the existing `plan_validator` test suite.
5. **`status: active`/`status: complete` also block on `pending`,** not
   only `approved` -- a plan that reaches implementation or completion
   with an unresolved required consultation is the same defect,
   whichever of the three post-Gate-1 statuses it currently carries.
6. Full workspace suite, clippy, `agov gate change-check` if
   `plan_validator.rs` is a registered gate surface (confirm at
   implementation time; it is not currently registered in
   `.agf/gates.yaml`, so this may be a net-new registration decision,
   not a companion-touch one -- to be resolved during implementation,
   not asserted here).
7. **Gate 2 evidence requirement:** the implementation MR's own
   description includes the reviewer-visible negative-test command and
   its output, tied to that MR's exact head.

## Rule projection

`AGENTS.md`'s "Gate discipline" section and `agf/templates/AGENTS.md`'s
identical copy currently say only "The `advisory_consultation` section
must declare a status for both `infosec-sentinel` and
`contribution-auditor`" -- true but silent on what `pending` means or
when it blocks. Both gain a one-line addition: `pending` is valid
pre-Gate-1 drafting state; a plan cannot reach `approved` while a
required row is still `pending`. `agf/templates/gate1-plan.md` gains a
comment noting `pending` as an available value alongside the existing
three, with the same one-line blocking-condition note.

Six-surface manifest at
`docs/reviews/rule-projection/20260915T075705Z-issue130-pending-advisory-consultation-status.yaml`,
validated (`agov plan projection-check --allow-planned --base-ref
origin/main`: OK). Normative owner: `AGENTS.md` itself, **existing
pointer sufficient** -- unlike Row 6 S4's brand-new CI gate, `agov plan
validate`'s blocking authority over Gate 1 plan-frontmatter semantics is
already established and already exercised today (the existing
required-role-presence and active-role-status checks already block
without their own dedicated per-check rule text); this slice adds a new
semantic check inside that same already-authorized command, not a new
independent gate.

## Rollback

The schema addition is purely additive (a new enum value; no existing
value's meaning changes). The validator addition is a new check with a
narrow trigger condition (`status` in `{approved, active, complete}` AND
some row `pending`) -- no plan using the three prior values at any
status is affected. Reverting removes the enum value and the check;
zero migration for any existing plan, since none can currently declare
`pending` at all.

## Decision requested at Gate 1

Approve (a) the `pending` schema enum value, (b) the new
`plan_validator.rs` semantic check blocking `approved`/`active`/`complete`
while a required consultation row remains `pending`, (c) the test
strategy above as satisfying `gate-self-test-rule` for this
gate-mechanics change, and (d) the rule-projection disposition to be
finalized in the six-surface manifest at implementation time.
