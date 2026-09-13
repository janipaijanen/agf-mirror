---
id: 20260913T195450Z-row5-s2-self-announcing-supersession
status: pending-gate1
topic: row5-s2-self-announcing-supersession
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Advisory display/filtering change on the read path, plus one new
      optional coordination-message field pair. No new write authority,
      no credential or secret surface, no change to who may author a
      record. The self-assessment below is offered for a real pass
      before Gate 2, not as a substitute.
  - role: contribution-auditor
    status: waived
    reason: >-
      Bounded slice from an already Gate-1-approved work order
      (20260910T065635Z-...), with the vocabulary explicitly required to
      match qbka Slice A's now-frozen shape (no design freedom left on
      that axis). A real pass is welcome before Gate 2 but is not
      required to unblock drafting.
affected_paths:
  - agf/schemas/coordination-message.schema.json
  - tooling/rust/crates/agov-coord/src/handoff_tracker.rs
  - tooling/rust/crates/agov/src/main.rs
  - tooling/rust/crates/agov/tests/
  - docs/reference/session-resume.md
  - docs/reviews/rule-projection/20260913T195450Z-row5-s2-self-announcing-supersession.yaml
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: Row 5 S2 — self-announcing coordination-record supersession

Implements S2 of the approved Gate 1 work order
`docs/plans/20260910T065635Z-coordination-lifecycle-semantics-work-order.md`.
S2's named hard predecessor, qbka Slice A (the frozen `superseded`
provenance shape), is merged (`e9f7742d`) and Slice B's 12-file migration
and corpus regression guard are also merged — the predecessor is fully
satisfied.

## Problem (per #122 and the work order)

A superseded coordination record does not announce itself. `agov coord
list --since` and `agov coord pending` display a stale record exactly
like a live one, so a cold-starting session (or a later poll cycle) can
act on convention-superseded guidance with no mechanical signal that a
replacement exists. The work order's own evidence: `th6m` still reads as
a live operator instruction after `mhq1` withheld it by prose alone, and
out-of-band correction compounds the gap.

## Design: field pair, matching qbka Slice A's frozen shape exactly

Per the work order's own instruction ("whose vocabulary and provenance
fields **match** the frozen `qbka` shape (no second, divergent shape)"),
add to `agf/schemas/coordination-message.schema.json`, at the top level
(sibling to `references-to`, `session`, etc.), two **optional** fields:

```json
"supersedes": {
  "type": "string",
  "minLength": 1,
  "description": "Stable identifier (a coordination record's own id) of the earlier record this one replaces for the same audience. Declared provenance, matching qbka's superseded_by convention: the schema requires a non-empty identifier but does not itself resolve or verify that the identifier exists -- a reader confirms that at render time by descriptor lookup."
},
"superseded_by": {
  "type": "string",
  "minLength": 1,
  "description": "Stable identifier (a later coordination record's own id) of what replaces this record for the same audience. Declared provenance, matching qbka's own convention. Optional -- most records are never superseded."
}
```

Both fields are **plain strings naming a coordination record's `id`**, not
plan ids -- this is the coordination-record analogue of qbka's plan-level
`superseded_by`, not a reuse of the plan field itself (different schema,
different object graph). No `supersedes_decision`/`supersedes_at`
provenance triple is required here: unlike a plan's terminal
`status: superseded` (a governed disposition needing an authorizing
decision-record), a coordination record's `supersedes` is simply "this
record replaces that one for the same audience" -- an ordinary editorial
fact any author may state about their own sequence of records, requiring
no additional authority beyond authoring the newer record itself. This
asymmetry is deliberate and stated explicitly in the schema field
descriptions above, so a future reader does not assume parity with the
plan-level triple that isn't actually required here.

Either field may appear alone (a record may declare what it supersedes
without a later record having yet declared `superseded_by` pointing back,
and vice versa in the rare case a later record names a predecessor the
predecessor itself never anticipated) -- readers must not require both
sides to agree to treat a record as superseded; a `superseded_by` value on
either side is sufficient signal.

## Design: reader/display change

### `agov coord pending` (`find_pending_all_kinds`)

`tooling/rust/crates/agov-coord/src/handoff_tracker.rs`'s
`find_pending_all_kinds`: for each otherwise-pending record, before
including it, check whether `superseded_by` names an id present among
the same `descriptors` slice (i.e. reachable on the reader's current ref
-- same branch or `main`, matching the work order's own "on the same
branch or main" scoping). If so, exclude it from the live-obligation set
entirely (matching the work order: "drop it from the live-obligation
set"). A record whose `superseded_by` target is *not* present on the
reader's ref stays live -- this is the explicit fail-safe: an
unresolvable or not-yet-visible successor never silently hides a real
obligation.

### `agov coord list --since`

`tooling/rust/crates/agov/src/main.rs`'s `coord_list`: for each listed
record whose `superseded_by` names an id present among the read
messages, append `superseded — see <id>` to that row's rendered line
(both text and JSON output) instead of silently showing it as an
ordinary, undifferentiated entry. Unlike `coord pending`, `coord list`
does not drop the row -- it is a chronological audit log, not a
live-obligation queue, and hiding history from `--since` output would
work against its own stated purpose ("what changed since I last
checked"). The row stays, annotated.

### Non-goals (explicitly out of scope, per the work order and #122's own evidence)

- **Out-of-band instructions arriving as MR comments** outside the
  coordination channel -- #122's own evidence names this residual
  explicitly; it is routed to row 6 / #123's discovery-side work, not
  silently absorbed here.
- **Auto-resolving `requires-ack`** from a superseding record alone --
  `superseded_by` is a lifecycle-display annotation, not an
  acknowledgement mechanism; it does not interact with CP-A's own
  `answered — ack pending` display (Row 5 S4, already shipped) or with
  `ack`/`completion`/`decision-record` resolution. A record can be both
  `answered — ack pending` (S4) and later also gain a `superseded_by`
  annotation; the two are independent, additive display facts.
- **Rewriting or validating an author's supersession claim.** Per the
  coordination-publication-rule's existing lifecycle semantics ("a false
  lifecycle claim... is a blocking finding at whatever review stage
  catches it, corrected by a superseding record -- never by editing the
  false one"), a wrong `supersedes`/`superseded_by` claim is a review
  finding, not something this slice mechanically prevents.
- **`agov coord graph`/`graph-index`** are not touched; if a later slice
  wants supersession edges drawn there, that is separate scope.

## Rule projection

`workflow/coordination-publication-rule.md`'s existing "Lifecycle and
supersession semantics" section is the normative owner (already states
the *convention*: "the correction is a new, append-only record that
references and supersedes the earlier one"). This slice adds the
*mechanical* half -- a real field pair and reader behavior -- so the
six-surface projection screen applies. Manifest at
`docs/reviews/rule-projection/20260913T195450Z-row5-s2-self-announcing-supersession.yaml`,
`normative_owner.stage: existing` (the rule itself already exists and
already states the convention; only the mechanical implementation is
new). AI landing/loaders: no change needed -- `references-to` and
lifecycle semantics are already documented in AGENTS.md's coordination
section; a one-line pointer to the new field pair will be added to
`docs/reference/session-resume.md`'s existing lifecycle documentation
(matching the pattern used for qbka's own "Plan Status Lifecycle"
section and Row 5 S4's "CP-A Ack-Pending Display" section).

## Test strategy

New unit tests in `agov-coord`'s existing `handoff_tracker.rs` test
module (matching that file's existing test pattern) plus new
`agov`-crate tests:

1. **Positive:** a `requires-ack: true` record with a later record
   naming it in `superseded_by`, both present on the reader's ref ->
   excluded from `agov coord pending`'s output.
2. **Negative (fail-safe):** a record naming a `superseded_by` id that
   does **not** exist among the read descriptors -> stays live/pending,
   unchanged from today's behavior. This is the explicit "unresolved
   successor never hides a real obligation" guarantee and gets its own
   dedicated regression test.
3. **Negative (kind-scoped correctly):** confirms this slice does not
   change `handoff`/`ack` kind exclusion in `find_pending_all_kinds` (its
   existing, unrelated filter) -- a `handoff`-kind record with
   `superseded_by` set is still excluded from this specific surface for
   the pre-existing reason (handoffs live on `agov handoff next`'s own
   surface), not a new one.
4. **`coord list --since` annotation:** a record with a resolvable
   `superseded_by` shows `superseded — see <id>` in both text and JSON
   output; an unresolvable one does not.
5. Full workspace test suite, clippy, corpus regression (no plan file
   touched), `agov gate change-check` (this slice touches
   `agov-coord/src/handoff_tracker.rs` and `agov/src/main.rs`, both
   likely registered `gate-change-check` surfaces per `.agf/gates.yaml`
   -- confirm at implementation time and supply the companion touch to
   `tests/gate_change_check.rs` per that file's established convention
   if so).

Per `gate-self-test-rule.md`: `agov coord pending`/`coord list` remain
advisory display, not MR-blocking checks (the work order's own framing).
The positive/negative test pairs above are included as good practice and
because the work order explicitly names the fail-safe case (test 2) as a
requirement, not merely encouraged.

## Rollback

Both new fields are optional/additive on the schema; no existing record
becomes invalid. The reader change is display-only (`coord list`
annotates, `coord pending` filters using existing, unresolved-target
fail-safe behavior for anything not yet using the new fields). Reverting
the branch restores today's undifferentiated display with no migration.

## Risks

- **Silently hiding a live obligation via a false or premature
  `superseded_by` claim.** Mitigated by: (a) the fail-safe (unresolvable
  target never hides), (b) `coord pending`'s exclusion only fires when
  the named successor is actually present and readable on the current
  ref, and (c) per the coordination-publication-rule, a false claim is a
  review finding corrected by a new record, same as any other lifecycle
  misstatement already governed by that rule.
- **Divergence from qbka's shape.** Mitigated by using the identical
  field name (`superseded_by`) and identical "declared, not resolved by
  the schema" provenance model; the one deliberate difference (no
  `..._decision`/`..._at` triple) is explained explicitly above so it
  reads as a considered choice, not an oversight.
