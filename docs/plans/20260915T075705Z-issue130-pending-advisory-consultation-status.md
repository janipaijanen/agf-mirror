---
id: 20260915T075705Z-issue130-pending-advisory-consultation-status
status: approved
gate1_approved_by: governor-reviewer
gate1_approved_at: 2026-09-15T17:31:54Z
topic: issue130-pending-advisory-consultation-status
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Schema field addition and a schema-level allOf/if/then conditional
      on already-present plan-frontmatter fields; no credential, secret,
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
      see that record for detail. Independently, a second
      contribution-auditor pass (Codex,
      20260915T113047Z-contribution-auditor-rhs6) reviewed the MR !2837
      head after that first round and returned BLOCK with two further
      findings (rule-authority gap in the proposed Rust semantic check;
      code/prose disagreement on which roles' `pending` status blocks),
      plus one residual manifest-timing inconsistency. This revision
      addresses all three -- see the "Schema-level conditional" Design
      subsection and the Rule projection section below for the corrected
      reasoning.
affected_paths:
  - agf/schemas/gate1-plan.schema.json
  - agf/templates/gate1-plan.md
  - AGENTS.md
  - agf/templates/AGENTS.md
  - tooling/rust/crates/agov/tests/plan_superseded_status.rs
  - tooling/rust/crates/agov/tests/plan_collision.rs
  - docs/reviews/rule-projection/20260915T075705Z-issue130-pending-advisory-consultation-status.yaml
depends_on:
  - 20260913T085123Z-work-item-steward-gate1
  - 20260913T093405Z-row5-s3-priority-ordering-and-challenge-response
  - 20260913T130049Z-qbka-slice-a-superseded-plan-lifecycle
  - 20260914T130217Z-row6-s4-issue123-collision-check-blocking-promotion
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

## Collision-check disposition (`depends_on`)

The newly blocking `agov:governance:plan-collision-gate` (shipped by Row 6
S4, merged to `main` at `bcc76b04` after this branch was first opened --
this plan was rebased onto that commit before this Gate 1 request)
reported four unacknowledged overlaps against this plan's `affected_paths`,
each reviewed and disposed below:

- `20260913T130049Z-qbka-slice-a-superseded-plan-lifecycle` (overlap:
  `agf/schemas/gate1-plan.schema.json`) and
  `20260914T130217Z-row6-s4-issue123-collision-check-blocking-promotion`
  (same file) are both already Gate 2-approved and merged to `main`
  (confirmed via coordination records
  `20260913T160721Z-governor-reviewer-5jjg` and
  `20260915T050307Z-governor-reviewer-g7j1`). This branch is rebased on
  top of both results, so there is no real scheduling conflict -- the
  collision-check cannot yet distinguish a merged plan from a live one
  because plan `status` has no post-merge terminal value in the current
  schema (a separate, unfiled gap, not this plan's scope).
- `20260913T093405Z-row5-s3-priority-ordering-and-challenge-response`
  (overlap: `tooling/rust/crates/agov-coord/src/**` vs. this plan's
  `tooling/rust/crates/agov-coord/src/plan_validator.rs`) is likewise
  already merged (`20260913T121333Z-planner-architect-gkqh`,
  `row5-s3-merged`). Same disposition as above.
- `20260913T085123Z-work-item-steward-gate1` (overlap: `AGENTS.md`) is
  genuinely still open (Gate 1-approved, no completion record found as of
  this writing). Reviewed its plan file directly: its `AGENTS.md` change
  adds a new row to the "Active agents" roster table, while this plan's
  `AGENTS.md` change edits the existing "Gate discipline" paragraph --
  disjoint regions of a small file. Acknowledged via `depends_on` rather
  than narrowed `affected_paths`, since the file-level overlap is real
  and worth a reviewer's awareness even though the semantic conflict risk
  is low; an ordinary merge (whichever of the two lands second) resolves
  it trivially.

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

### Schema-level conditional: `pending` blocks Gate 1 approval (the actual fix, revised)

**Revised after independent contribution audit
`20260915T113047Z-contribution-auditor-rhs6`** (BLOCK, two findings) on
the first draft, which proposed a new Rust semantic check in
`plan_validator.rs`. That draft had two real defects: (1) no named rule
granted a *new* semantic check blocking authority -- citing the existing
required-role-presence check as precedent is not the same as a rule
actually saying so, exactly the gap `tool-authority-boundary-rule.md`
exists to close (the same defect Row 6 S4's `thsq` review caught in this
session, round 1); (2) the proposed loop filtered every
`advisory_consultation` row regardless of role, while the prose claimed
only *required*-role rows blocked -- code and prose disagreed.

The revised fix drops the Rust check entirely and expresses the same
restriction as a schema-level conditional, following the exact pattern
this schema already uses for `status: superseded`'s own provenance
requirement (see the existing top-level `allOf`/`if`/`then` block for
`superseded_by`/`superseded_decision`/`superseded_at`). Draft-07's
`if`/`then` combines cleanly with a per-item `not: {"const": "pending"}`
constraint added alongside the array's existing `items` schema:

```json
{
  "if": {
    "properties": { "status": { "enum": ["approved", "active", "complete"] } },
    "required": ["status"]
  },
  "then": {
    "properties": {
      "advisory_consultation": {
        "items": {
          "properties": {
            "status": { "not": { "const": "pending" } }
          }
        }
      }
    }
  }
}
```

Added to the schema's existing top-level `allOf` array, alongside the
`superseded` conditional already there.

This resolves both findings at once. On authority: `agov plan validate`
running JSON Schema conformance against `gate1-plan.schema.json` is not
a *new* check needing a *new* rule grant -- it is the existing,
already-named blocking mechanism Gate discipline itself points to
("plan file... conforming to `agf/schemas/gate1-plan.schema.json`" in
`AGENTS.md`). A schema constraint is the contract; there is no separate
"the tool decided to enforce this" step for a rule to authorize. On the
required-vs-optional-role question: the `items` sub-schema applies to
*every* element of the array by JSON Schema definition, so the
constraint is uniform across required and optional rows with no special
casing needed -- matching the auditor's suggested safer default (any
declared `pending` row blocks a post-Gate-1 status, not only a required
one) without an explicit branch to get wrong.

A plan at `status: pending-gate1` MAY have `pending` rows -- that is
exactly the state `pending` exists to represent (mid-drafting, review
requested, not yet answered). The `then` branch fires only once a plan
claims `approved`/`active`/`complete` while *any* consultation row is
still `pending` -- the literal MR !2478 failure mode #130 documents, now
caught by the schema itself rather than relying on plan-author
self-discipline ("the plan self-blocks... in prose," per `a8pj`'s own
observation).

No change to `tooling/rust/crates/agov-coord/src/plan_validator.rs` is
needed under this revised design. `affected_paths` above names the
schema, template, and loader-projection files actually touched, plus
(added per round-2/round-3 audit) the two test files: the new
`plan_superseded_status.rs` negative-test cases and the
`plan_collision.rs` companion test the gate-registry consequence below
requires.

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
- No change to `tooling/rust/crates/agov-coord/src/plan_validator.rs` --
  the revised design (see above) is schema-only; there is no new Rust
  semantic check.
- No distinction between required and optional `advisory_consultation`
  roles for the blocking condition -- *any* row left `pending` blocks a
  post-Gate-1 status, matching independent-audit finding rhs6's
  suggested safer default and avoiding a required-vs-optional branch
  the schema's per-item `items` constraint would have to special-case.

## Test strategy (gate-self-test-rule: this is gate mechanics)

`agov plan validate` is an existing mechanically-enforced blocking
check; this slice adds new blocking behavior to it via a schema
conditional, so it needs its own negative/positive evidence -- the same
shape `tooling/rust/crates/agov/tests/plan_superseded_status.rs` already
uses for this schema's existing `superseded` conditional (a template
this new test file should follow directly):

1. **Negative (required role pending):** a plan with `status: approved`
   and the `infosec-sentinel` row `status: pending` -> `agov plan
   validate` fails.
2. **Negative (optional role pending):** a plan with `status: approved`,
   both required rows `consulted`, and one additional optional-role row
   (e.g. `fact-checker`) `status: pending` -> also fails, proving the
   schema constraint is uniform across required and optional rows (this
   is the exact case round-1's Rust draft got right in effect but wrong
   in its own prose -- this test pins the corrected, stated behavior).
3. **Positive, mid-draft:** the same plan at `status: pending-gate1`
   with an identical `pending` row -> validates cleanly (pending is
   legitimate pre-Gate-1 state).
4. **Positive, flipped:** the same plan at `status: approved` with the
   row changed to `consulted` -> validates cleanly (matches this
   session's own real Row 6 S4 experience).
5. **Backward compatibility:** every existing enum value
   (`consulted`/`waived`/`not-applicable`) at every plan status,
   unmodified, continues to validate exactly as before -- re-run
   unchanged against the existing `plan_validator` test suite.
6. **`status: active`/`status: complete` also block on `pending`,** not
   only `approved` -- a plan that reaches implementation or completion
   with an unresolved consultation is the same defect, whichever of the
   three post-Gate-1 statuses it currently carries.
7. **`agov gate change-check` companion test (corrected per round-3 audit
   `damr`).** `agf/schemas/gate1-plan.schema.json` IS a registered
   `surfaces` entry of the active `plan-collision-block` gate in
   `.agf/gates.yaml` (registered by Row 6 S4, base `bcc76b04`) -- the
   earlier "not a registered gate surface" claim in this section was
   wrong; `gate change-check` mechanically requires any commit touching
   a registered surface to also touch at least one of that gate's own
   registered `tests` paths
   (`tooling/rust/crates/agov/tests/plan_collision.rs` or
   `.gitlab/scripts/plan-collision-self-test.sh`). The implementation
   commit adds one case to `plan_collision.rs` (already the registered
   test, and already the crate the new `plan_superseded_status.rs` case
   lives beside): a `status: pending-gate1` plan with a `pending`
   `advisory_consultation` row is still correctly scanned and included
   in collision-check's inventory, unaffected by the new enum value --
   a genuine regression guard for the collision-check gate itself, not
   a contrived touch to satisfy the mechanical check. Both
   `tooling/rust/crates/agov/tests/plan_collision.rs` and
   `tooling/rust/crates/agov/tests/plan_superseded_status.rs` are now in
   `affected_paths` above.
8. **Gate 2 evidence requirement:** the implementation MR's own
   description includes the reviewer-visible negative-test command and
   its output, tied to that MR's exact head.

## Rule projection

`AGENTS.md`'s "Gate discipline" section and `agf/templates/AGENTS.md`'s
identical copy currently say only "The `advisory_consultation` section
must declare a status for both `infosec-sentinel` and
`contribution-auditor`" -- true but silent on what `pending` means or
when it blocks. Both gain a one-line addition: `pending` is valid
pre-Gate-1 drafting state; a plan cannot reach `approved` while *any*
declared consultation row -- required or optional -- is still `pending`
(revised per round-2 audit `pgsh`: the schema's per-item constraint
applies uniformly, so the loader text now says "any declared row," not
"a required row"). `agf/templates/gate1-plan.md` gains a comment noting
`pending` as an available value alongside the existing three, with the
same one-line blocking-condition note.

Six-surface manifest at
`docs/reviews/rule-projection/20260915T075705Z-issue130-pending-advisory-consultation-status.yaml`,
already exists and is finalized (updated alongside this revision),
validated (`agov plan projection-check --allow-planned --base-ref
origin/main`: OK).

**Normative owner disposition, revised after independent audit rhs6's
finding.** The first draft argued "existing pointer sufficient" by
analogy to the existing required-role-presence and active-role-status
Rust checks -- rhs6 correctly rejected that reasoning: citing precedent
for a new check does not itself grant that new check authority, per
`tool-authority-boundary-rule.md`. The revised schema-only design (see
Design above) does not have this problem, because it introduces no new
*check* at all: `agf/schemas/gate1-plan.schema.json` is itself the named,
already-governed artifact `agov plan validate`'s schema-conformance step
enforces, and `AGENTS.md`'s Gate discipline section already names that
exact schema as the Gate 1 authority ("plan file... conforming to
`agf/schemas/gate1-plan.schema.json`"). Editing the schema's own `allOf`
conditionals is squarely inside that existing pointer's scope -- the
same way the `superseded` provenance conditional already there needed no
separate rule text of its own. Normative owner: `AGENTS.md`, **existing
pointer sufficient**, on this corrected basis.

## Rollback

The schema addition is purely additive (a new enum value; no existing
value's meaning changes). The conditional addition is a new `allOf`
entry with a narrow trigger condition (`status` in
`{approved, active, complete}` AND some row `pending`) -- no plan using
the three prior values at any status is affected. Reverting removes the
enum value and the conditional; zero migration for any existing plan,
since none can currently declare `pending` at all.

## Decision requested at Gate 1

Approve (a) the `pending` schema enum value, (b) the schema-level
`allOf`/`if`/`then` conditional blocking `approved`/`active`/`complete`
while any consultation row remains `pending` (no Rust semantic check),
(c) the test strategy above as satisfying `gate-self-test-rule` for this
gate-mechanics change, and (d) the rule-projection disposition, already
finalized in the six-surface manifest referenced above (not deferred to
implementation time).
