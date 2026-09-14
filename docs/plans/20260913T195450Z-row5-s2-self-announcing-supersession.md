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
      optional coordination-message field. No new write authority, no
      credential or secret surface, no change to who may author a
      record. Round 2 added edge-validity checks (existence, temporal
      order, self-edge/cycle, same-audience, sender authority) because
      an earlier draft had none (S2-G1-F2). Round 3 closed a real
      authority-boundary gap those checks still had: S2-G1-R2-F2 found
      that "latest timestamp wins" could let a non-governor author
      displace a governor-reviewer's authoritative override with no
      governor-reviewer record in the loop. The round-3 chain-walk
      design applies the sender-authority check to the current chain
      leaf at every hop, structurally preventing that displacement
      rather than merely documenting the risk -- this remains
      "not-applicable" for a dedicated infosec-sentinel pass because no
      credential/secret surface exists, but the authority-boundary
      concern itself was real and is now mechanically closed, not
      argued away.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Round 1 waived this, which independent review (S2-G1-F4) correctly
      identified as exactly the risk the parent work order's consultation
      requirement exists to catch -- round 1's design did in fact diverge
      from qbka's shape in a way a real pass would have caught. Round 2
      consultation performed and published as
      20260914T012444Z-contribution-auditor-r2sj.md.
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

## Correction (2026-09-14, round 2) — independent Gate 1 review BLOCK

Independent review `20260914T010848Z-coder-tester-h8a7` found four
mandatory blocking findings in the round-1 design below and correctly
recommended BLOCK before any implementation started (none had). All four
are addressed in this round-2 revision; the round-1 "Design" sections
that follow this note are **superseded by the corrected design further
below** and kept only as the historical record of what was proposed and
rejected — do not implement the round-1 field pair or algorithm.

- **S2-G1-F1 (critical):** round 1 proposed a *two-way* field pair —
  `supersedes` on the later record and `superseded_by` on the earlier
  one. Coordination records are immutable and append-only; an earlier
  record cannot retroactively acquire a field after a later correction
  exists. This also contradicts a prior independent design refinement,
  `20260905T200607Z-coder-tester-hqps`, already on record: "the original
  immutable record cannot acquire a literal `superseded_by` field. Add
  `supersedes: [id, ...]` only on the later record and derive the
  reverse `superseded-by` edge in `agov`." Round 1 should have found and
  followed this — it did not. **Fixed:** the schema now defines exactly
  one new field, `supersedes` (an array, on the later record only); the
  reverse relationship is derived by readers, never written back.
- **S2-G1-F2 (critical):** round 1 dropped an obligation from the live
  queue whenever a named target merely *existed*, with no check for
  temporal order, self-edges, cycles, or same-audience/sender authority.
  A mistaken or hostile record could have silently hidden another role's
  live obligation. **Fixed:** see "Edge validity" below — an edge is
  honored only after existence, strict temporal order, no self-edge,
  same-audience, and sender-authority checks all pass; an invalid edge
  fails safe (the original record stays visible, with a diagnostic).
- **S2-G1-F3 (material):** round 1 touched only `coord list` and `coord
  pending`, explicitly leaving `agov handoff next` — the actual primary
  surface for handoff obligations — unchanged, so a superseded handoff
  would have stayed actionable there. **Fixed:** the design below shares
  one resolution function across `agov handoff next`, `agov coord
  status`, `agov coord pending`, and `agov coord list`, per
  `hqps`'s own explicit list ("`list`, `pending`, `status`, and `handoff
  next`").
- **S2-G1-F4 (material):** round 1 waived contribution-auditor
  consultation despite the parent work order requiring it at this exact
  gate to confirm no divergence from qbka's shape — and round 1 *did*
  diverge, undetected, precisely because that consultation was skipped.
  **Fixed:** `advisory_consultation` above now reflects a real
  consultation (`20260914T012444Z-contribution-auditor-r2sj`), performed
  against this corrected design.

## Correction (2026-09-14, round 3) — independent re-review BLOCK

A second independent re-review, `20260914T042829Z-coder-tester-kjj8`,
confirmed all four round-1 findings above are resolved, but found two
new, real blocking findings in round 2's own edge-validity semantics.
Both are fixed in this round-3 revision; the round-2 "Edge validity"
subsection further below is corrected in place (not superseded
separately) since only its checks change, not the overall one-way-field
architecture that round 2 already got right.

- **S2-G1-R2-F1 (material):** round 2's edge-validity checks verified
  only that a candidate successor `S` names the predecessor in its own
  `supersedes` array. `agf/rules/workflow/coordination-publication-rule.md`'s
  "Lifecycle and supersession semantics" normatively requires a
  correction to be a record that **references and supersedes** the
  earlier one -- round 2 implemented "supersedes" alone, silently
  dropping the causal-link half of that requirement. A record could
  therefore retire another's live obligation without ever citing it in
  `references-to`, splitting graph/audit traversal (which uses
  `references-to`) from queue-resolution semantics (which would have
  used `supersedes` alone). **Fixed:** edge validity now also requires
  `S.references-to` to contain the predecessor's id, alongside
  `S.supersedes` containing it -- both, not either.
- **S2-G1-R2-F2 (critical):** round 2's "latest timestamp wins" rule for
  multiple valid correctors could let a lower-authority author displace
  a governor-reviewer's authoritative override. Concretely: if
  governor-reviewer publishes override `G` against predecessor `X`, and
  `X`'s original author later publishes a further direct correction `A`
  also naming `X`, round 2 would report `A` (later timestamp) as
  authoritative -- even though `A`'s author never satisfied the
  sender-authority check against `G`, only against `X`. This let an
  ordinary author silently override a governor decision with no
  governor-reviewer record in the loop. **Fixed:** resolution is now a
  **chain walk to the current leaf**, not a flat "latest direct
  candidate" scan -- see the corrected "Edge validity" subsection below.
  A corrector must name the *current leaf* of the chain (not
  necessarily the original record) in both `supersedes` and
  `references-to`; the sender-authority check therefore applies to the
  leaf's own author at each hop, so once governor-reviewer becomes the
  leaf, only governor-reviewer (or a fresh governor-reviewer record) can
  extend the chain further. Equal-timestamp direct candidates at the
  same hop are broken by lexicographically smallest id, deterministically.

## Problem (per #122 and the work order)

A superseded coordination record does not announce itself. `agov coord
list --since` and `agov coord pending` display a stale record exactly
like a live one, so a cold-starting session (or a later poll cycle) can
act on convention-superseded guidance with no mechanical signal that a
replacement exists. The work order's own evidence: `th6m` still reads as
a live operator instruction after `mhq1` withheld it by prose alone, and
out-of-band correction compounds the gap.

## ROUND 1 DESIGN (superseded — kept for historical record only; do not implement)

The sections immediately below, through "## Corrected design (round 2)",
describe the rejected round-1 proposal. They are preserved verbatim as
the record of what independent review found and blocked; skip to
"## Corrected design (round 2)" for what to actually build.

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

*(End of round-1 design — superseded. Corrected design follows.)*

## Corrected design (round 2)

### Schema: one field, one-way, on the later record only

Add exactly one new field to `agf/schemas/coordination-message.schema.json`,
top level (sibling to `references-to`):

```json
"supersedes": {
  "type": "array",
  "minItems": 1,
  "items": { "type": "string", "minLength": 1 },
  "description": "Stable ids of one or more earlier coordination records this one replaces for the same audience. Written only on the later (superseding) record -- coordination records are immutable and append-only, so an earlier record can never acquire a field after the fact. Readers derive the reverse relationship; see coordination-publication-rule.md's lifecycle and supersession semantics, which requires a correction to both reference and supersede the record it replaces -- an id listed here MUST also appear in this record's own references-to for the edge to be honored. An entry is honored only after edge-validity checks (existence and matching references-to citation, temporal order, no self-edge, same audience, sender authority against the current chain leaf) pass -- see agov-coord's supersession module."
}
```

No `superseded_by` field is added anywhere. No `..._decision`/`..._at`
provenance triple: unlike a plan's terminal `status: superseded` (a
governed disposition), a coordination record's `supersedes` is an
ordinary editorial fact about the author's own record sequence,
requiring no additional authority beyond authoring the newer record --
subject to the sender-authority check below, which is the actual
authority boundary.

### Edge validity (shared logic, new module) -- corrected per round 3

New `tooling/rust/crates/agov-coord/src/supersession.rs` (or a section
of `handoff_tracker.rs` if that proves the more natural home at
implementation time), exposing two functions: a single-hop check and a
chain walk built on it.

```rust
/// Returns true iff `s` is a valid direct superseder of `target`
/// (a single hop). All six checks below must pass.
fn is_valid_direct_edge(s: &MessageDescriptor, target: &MessageDescriptor) -> bool

/// Walks the chain of valid direct edges starting at `candidate`,
/// hop by hop, always re-targeting the *current leaf* (never the
/// original `candidate` once a valid hop has been taken). Returns the
/// final leaf if at least one valid hop exists; `None` if `candidate`
/// has no valid direct superseder at all, meaning it stays live.
pub fn valid_superseder<'a>(
    candidate: &'a MessageDescriptor,
    all: &'a [MessageDescriptor],
) -> Option<&'a MessageDescriptor>
```

Single-hop checks in `is_valid_direct_edge`, all mandatory, evaluated
against `target` (the current chain leaf at that hop, not necessarily
the original candidate -- see "Chain walk" below):

1. **Existence + causal reference (S2-G1-R2-F1 fix):** a candidate
   successor `S` names `target.id` in **both** its own `supersedes`
   array **and** its `references-to` array (id/filename forms
   normalized through the existing reference resolver), and `S` is
   present among `all` (i.e. resolvable on the reader's current ref --
   same branch or `main`). Both fields must agree; `supersedes` alone,
   without the matching `references-to` citation, is not honored -- this
   restores the coordination-publication-rule's own "references and
   supersedes" wording, which round 2 had silently dropped.
2. **Strict temporal order:** `S.timestamp > target.timestamp`. This
   alone makes cycles structurally impossible at a single hop (a cycle
   would require an earlier record to supersede a later one); the chain
   walk below additionally prevents a cycle across hops by construction
   (each hop strictly increases in timestamp).
3. **No self-edge:** `S.id != target.id`.
4. **Same audience:** `S.to == target.to`. Supersession replaces a
   record for the same intended audience, per the work order and the
   coordination-publication-rule's own framing; a record addressed to a
   different role cannot supersede this one.
5. **Sender authority:** `S.from == target.from` (the current leaf's own
   author correcting their own record -- the common case) **or**
   `S.from == "governor-reviewer"` (an authoritative override), matching
   the existing `reply_evidence` precedent (`next_agent` or
   `governor-reviewer`) already established in this codebase for an
   analogous "who may act on this record" question. Checking this
   against `target` (the current leaf) rather than the original
   candidate is exactly the S2-G1-R2-F2 fix: once governor-reviewer
   becomes a leaf, only governor-reviewer (or a further governor-reviewer
   record) satisfies this check for the next hop.

### Chain walk (S2-G1-R2-F2 fix)

`valid_superseder` does not pick "the latest of all direct candidates
against the original record." It walks forward one hop at a time:

1. Set `leaf = candidate`.
2. Find every `S` in `all` for which `is_valid_direct_edge(S, leaf)` is
   true.
3. If none: stop. Return `leaf` if `leaf != candidate` (at least one hop
   was taken), else `None` (candidate stays live).
4. If one or more: pick the one with the **earliest** `timestamp`
   (ties broken by lexicographically smallest `id`, deterministic and
   stable) as the next `leaf`. This is the corresponding fix for
   round 2's undefined equal-timestamp behavior. Go to step 2.

A corrector must always name the *current leaf* -- not the original
record -- in both `supersedes` and `references-to` to take effect. A
record naming only the original root once a leaf already exists fails
the existence check at that hop and is not honored, exactly the
fail-safe behavior this design already guarantees for any other invalid
edge. This is what prevents the governor-authority bypass: `A`
(original author) cannot retroactively displace `G` (governor's
override) by targeting the original record, because after `G` becomes
the leaf, only an edge targeting `G` itself is evaluated, and only `G`'s
own author (`governor-reviewer`) or a fresh governor-reviewer record
passes the sender-authority check at that hop.

**Fail-safe:** any hop's checks failing means that edge is not honored
-- the walk simply stops there and returns the current leaf (or `None`
if no hop was ever valid). No record is ever hidden by an invalid or
unauthorized edge; this function raises no error and blocks nothing. It
is a pure, advisory read-time derivation.

### One shared resolution point across all four surfaces

Per S2-G1-F3's resolution condition and `hqps`'s explicit list, this
must reach `list`, `pending`, `status`, and `handoff next` --
not a per-command reimplementation (the exact mistake Row 5 S4's own
S4-F1 finding caught in a different feature; this design deliberately
avoids repeating it):

- **`agov/src/main.rs`'s `handoff_resolved`** (already the single shared
  function behind `pending_handoffs`, which in turn backs `agov handoff
  next`, `agov coord status`, and the `--roles` list views): extend it
  to also treat a `valid_superseder` result as resolving, alongside its
  existing `ack`/`completion`/`decision-record` check. This one change
  automatically and correctly covers `handoff next`, `coord status`, and
  the list-by-role views in a single place -- exactly the shared-point
  fix S4-F1 needed and didn't originally have.
- **`agov-coord/src/handoff_tracker.rs`'s `find_pending_all_kinds`**
  (the separate, non-handoff-kind surface backing `agov coord pending`):
  extend analogously, using the same `valid_superseder` function, so
  both parallel resolution paths share one edge-validity implementation
  even though they remain two call sites (matching this codebase's
  existing two-implementation split between handoff-kind and
  other-kind resolution -- not itself in this slice's scope to unify).
- **`agov/src/main.rs`'s `coord_list`:** does not drop rows (it is a
  chronological audit log, not a live-obligation queue). For each row
  whose `valid_superseder` result is `Some(s)`, append `superseded — see
  <s.id>` to the rendered line (text and JSON). An unresolvable or
  invalid edge leaves the row unannotated, unchanged from today.

### Non-goals (unchanged from round 1, still correct; check count updated to six per round 3)

- Out-of-band MR-comment instructions (routed to row 6 / #123).
- Auto-resolving `requires-ack` from a superseding record alone --
  `supersedes` is a lifecycle-display/resolution annotation sharing the
  same resolving-kind precedent as `ack`/`completion`/`decision-record`,
  not a new grant of ack-clearing authority to arbitrary record kinds; a
  `note` or `review` record with a *valid* `supersedes` edge resolves
  the superseded record specifically because supersession is now a
  recognized terminal state for that predecessor, exactly parallel to
  how those three kinds already terminate a handoff today. This is a
  deliberate, bounded extension of "what counts as resolved" -- not an
  unbounded one, since all six edge-validity checks still gate it, per
  hop, at every hop of the chain walk.
- Rewriting or validating an author's supersession *claim* itself beyond
  the six mechanical checks -- a claim that passes all six checks but
  is still substantively wrong (e.g. governor-reviewer supersedes the
  wrong record by mistake) is a review finding, corrected by a further
  new record, never by editing history.
- `agov coord graph`/`graph-index` -- unchanged, separate scope.

### Test strategy (round 3, supersedes round 2's list below it)

In addition to round 1's positive/fail-safe/annotation tests (still
valid in spirit, rewritten against the real one-way array field,
`is_valid_direct_edge`, and the `valid_superseder` chain walk):

1. **Each of the six single-hop edge-validity checks gets its own
   negative test:** non-existent target, `supersedes` without matching
   `references-to` citation (S2-G1-R2-F1's own regression case),
   non-later timestamp (equal and earlier), self-edge, different `to`
   audience, and unauthorized sender (neither the current leaf's own
   author nor `governor-reviewer`) -- each must leave the candidate
   fully live/unannotated, exactly as if `supersedes` were absent.
2. **Positive, all six checks pass, single hop:** candidate excluded
   from `handoff next`, `coord status`, `coord pending`, and the
   `--roles` views; annotated (not excluded) in `coord list`.
3. **Governor-authority chain tests (S2-G1-R2-F2's own regression
   cases), all four required by the finding:**
   - **Author-before-governor:** original author supersedes `X` first,
     governor-reviewer's later record names `X` (not the author's
     record) -> governor's record is invalid at that hop (wrong leaf
     target) and does not resolve; a *further* governor-reviewer record
     correctly targeting the author's record as leaf does resolve.
   - **Governor-before-author:** governor-reviewer supersedes `X`
     first; the original author's later record also names `X` (not
     governor's record) -> author's record is invalid (fails
     sender-authority against the governor-authored leaf) and does not
     displace governor's override; `X` remains resolved by governor's
     record, not the author's later one.
   - **Equal timestamps:** two direct correctors of the same leaf at
     identical timestamps -> resolution picks the lexicographically
     smallest id, deterministically, same result on every run and every
     surface.
   - **Authorized extension of the winning leaf:** a further valid
     record correctly targeting the current leaf (whichever party
     authored it, provided sender-authority passes against the leaf)
     extends the chain correctly.
4. **Shared-function regression:** a single test asserting `handoff
   next`, `coord status`, `coord pending`, and the `--roles` list view
   all agree on the same candidate's resolved leaf for the same
   fixture -- the literal test S4-F1's absence would have caught for
   that feature, applied here pre-emptively.
5. Full workspace suite, clippy, corpus regression, `agov gate
   change-check` with the companion touch to `tests/gate_change_check.rs`
   if `main.rs`/`handoff_tracker.rs` are registered gate-change-check
   surfaces (confirmed pattern from Row 5 S4's own implementation).

### Rollback (round 2)

The new field is additive; the resolution function is pure and advisory.
Reverting restores today's behavior with no migration. No existing
record's meaning changes since no record currently sets `supersedes`.
