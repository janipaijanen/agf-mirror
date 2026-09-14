---
id: 20260914T130217Z-row6-s4-issue123-collision-check-blocking-promotion
status: pending-gate1
topic: row6-s4-issue123-collision-check-blocking-promotion
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      New CI job reads git diffs and plan-file frontmatter already
      present in the repository; no credential, secret, or new write
      authority is introduced. Per `agf/agents/MANIFEST.yaml`,
      infosec-sentinel is played by a CI mechanical check (`agov validate
      infosec`, which scans learning records only) -- not a role a
      session can self-perform. Consistent with the disclosed limitation
      already on record for Row 6 S3's own plan and this session's
      `20260914T070135Z-planner-architect-7sdp`.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Self-performed consultation published as
      20260914T130405Z-contribution-auditor-drp6.md, before Gate 1
      request, per this project's established practice for a plan
      introducing genuine gate mechanics. Found and fixed a real design
      error in the same draft (the "Acknowledgment matching" section's
      loop-ordering claim was incorrect; corrected before this request).
affected_paths:
  - agf/schemas/gate1-plan.schema.json
  - tooling/rust/crates/agov/src/plan_collision.rs
  - tooling/rust/crates/agov/src/main.rs
  - tooling/rust/crates/agov/tests/plan_collision.rs
  - tooling/rust/crates/agov/tests/gate_change_check.rs
  - tooling/rust/crates/agov/tests/ci_needs_implication.rs
  - .gitlab-ci.yml
  - AGENTS.md
  - agf/templates/AGENTS.md
  - agf/templates/gate1-plan.md
  - docs/reviews/rule-projection/20260914T130217Z-row6-s4-issue123-collision-check-blocking-promotion.yaml
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Row 6 S4 — #123: promote `agov plan collision-check` to MR-blocking

## Provenance

- Parent: `docs/plans/20260910T072138Z-coordination-discovery-collision-tooling-work-order.md`,
  row 6. Follows S3 (`20260911T040631Z-row6-s3-issue123-affected-paths-collision-check`,
  approved and merged), which shipped `affected_paths` and
  `agov plan collision-check` deliberately **advisory-only**, explicitly
  deferring this exact promotion: "If a later decision promotes
  `collision-check` to MR-blocking, that decision's own Gate 1 supplies
  the `gate-self-test-rule` negative test... not this slice."
- Issue **#123**'s second proposal bullet: "An `agov plan conflicts`
  check, run pre-Gate-1, that intersects a candidate plan's affected
  paths against every other live plan and open implementation MR. Exact
  overlap without an explicit `depends_on:` / sequencing disposition
  blocks Gate 1." This slice is that bullet.
- Explicitly **not** this slice: #123's third proposal bullet (a
  post-implementation check of the actual diff against the approved
  `affected_paths` so estimates cannot silently drift). Separate scope,
  separate Gate 1 if pursued.

## Problem

`collision-check` correctly *detects* overlaps today but has zero
enforcement -- a reviewer must remember to run it, read the output, and
manually block on what it finds. The exact failure mode #123 documents
(MR !2389 / MR !2373, later Row 5 S3 / CP-B-2) is a plan-authoring-time
collision that a human reviewer missed; an advisory-only tool that
nobody is forced to consult does not close that gap, it only makes
closing it easier for a reviewer who remembers to ask.

## Design

### Schema: `depends_on` (the escape hatch)

New **optional** field on `gate1-plan.schema.json`, sibling to
`affected_paths`:

```json
"depends_on": {
  "type": "array",
  "description": "Optional list of other plan ids, MR references (!<iid>), or branch names this plan acknowledges an affected_paths overlap with, and has a deliberate sequencing/coordination disposition for. Naming a counterpart here downgrades a detected collision from blocking to informational for this plan; it does not require the counterpart to reciprocate.",
  "items": { "type": "string", "minLength": 1 }
}
```

Absent = current behavior, unchanged (no plan declares this today).
This is the mechanism the issue's own proposal names: an author who
knows about and has coordinated a genuine overlap declares it instead
of being blocked by it.

### `collision-check`: additive, backward-compatible extension

Three additions to `tooling/rust/crates/agov/src/plan_collision.rs`,
none changing default (no-flag) behavior:

1. **`--changed-only`** (new bool flag, default off). When set, a
   pair is only reported/considered at all if **at least one** plan in
   it is "changed": its `docs/plans/<file>.md` differs between
   `--base-ref` and the working tree (new or modified in the current
   diff, computed with the same `git diff --name-status` machinery
   `changed_paths` already uses, applied to `docs/plans/` itself). A
   pair where **neither** plan is changed (two long-standing plans that
   already collided before this diff) is excluded entirely under this
   flag -- not merely downgraded, dropped from the report and from
   blocking consideration, since this flag exists specifically to scope
   out pre-existing state this MR did not create. Refs and MRs have no
   "changed" concept of their own; a plan-vs-ref/MR pair is eligible
   whenever the plan side is changed.
2. **Acknowledgment matching.** For each *eligible* pair (i.e. the
   pair a plain, unflagged `collision-check` would already report, and
   -- when `--changed-only` is set -- a pair that also passes check 1
   above), the pair is *acknowledged* iff the **changed** plan's own
   `depends_on` contains the other side's identifying token: the other
   plan's own `id` for a plan-vs-plan pair, `!<iid>` for a forge MR, or
   the branch name for a local ref. If both plans in a pair are changed,
   the pair is acknowledged when *either* one's `depends_on` names the
   other -- checked explicitly for both, not inferred from loop
   iteration order (the existing nested loop visits each unordered pair
   once, so both plans' `depends_on` fields are checked directly against
   each other at that single visit, not relying on which one happens to
   be the outer-loop `plan` versus the inner-loop `other`). A
   pre-existing plan's own `depends_on` is never consulted when it is
   the *unchanged* side of a pair -- only the changed plan's declaration
   can acknowledge, since the changed plan is the one currently at Gate
   1 and able to make a fresh coordination decision.
3. **`--fail-on-unacknowledged-collision`** (new bool flag, default
   off). When set, changes only the **exit code**, never the printed
   report: exit `0` if every eligible pair is acknowledged (or none
   exist), exit `1` if at least one is not. This is the only exit-code
   change; default (no-flag) invocations keep today's `0`/`1`/`2`
   contract byte-for-byte, so every existing advisory call site
   (including ad hoc session use) is unaffected.

These two flags are independent (each usable alone) but the CI gate
below uses both together.

### CI: a new, narrowly scoped blocking job

New job in `.gitlab-ci.yml` (name indicative,
e.g. `agov:governance:plan-collision-gate`):

```yaml
agov:governance:plan-collision-gate:
  stage: <existing governance stage>
  rules:
    - changes:
        - "docs/plans/**/*.md"
  script:
    - agov plan collision-check --base-ref origin/main --changed-only --fail-on-unacknowledged-collision
```

Path-scoped per `ci-trigger-scoping-rule` -- a change touching no plan
file never runs this job. **Local-branch mode only** (`--include-forge`
is deliberately not used here): querying the GitLab MR API from inside
a CI job for the MR under evaluation is a self-referential network
dependency this slice explicitly defers, matching S3's own precedent of
deferring `--include-forge` from MCP exposure as "a separate, explicitly
future decision, not silently implied here." **Non-goal for this
slice:** blocking on collisions against *other open MRs'* changed paths
(forge mode) -- only plan-vs-plan and plan-vs-local-ref collisions
block. A collision only visible via `--include-forge` remains advisory
until a future slice extends blocking to forge mode.

On failure, the job's own output (already produced by `collision-check`,
unchanged) names the exact colliding plan(s)/ref(s) and the specific
overlapping paths; the fix is either coordinate and add `depends_on`, or
change one side's `affected_paths` so they no longer overlap.

### Explicitly out of scope

- Forge-mode (`--include-forge`) blocking -- stays advisory-only this
  slice, per above.
- The post-implementation `affected_paths`-vs-actual-diff drift check
  (#123's third proposal bullet) -- separate scope, separate Gate 1.
- Any override/waiver mechanism beyond `depends_on` -- no special-case
  bypass flag. A genuine emergency uses this project's existing
  governance path (governor-authorized CI config change), not a
  built-in escape hatch.
- Retroactively requiring `depends_on` or `affected_paths` on any
  existing plan -- both remain optional indefinitely, matching S3.

## Test strategy (gate-self-test-rule: this is gate mechanics)

`tooling/rust/crates/agov/tests/plan_collision.rs` (new cases) plus a
companion CI-job-level fixture proving the actual `.gitlab-ci.yml` rule
fires (per this project's established pattern for `.gitlab-ci.yml`
changes registered under `gate-change-check`/`ci-needs-implication`):

1. **Negative (the gate fires):** a changed-in-this-MR plan whose
   `affected_paths` overlaps an existing live plan's, no `depends_on`
   entry naming it -> `--fail-on-unacknowledged-collision` exits `1`.
2. **Positive (the escape hatch works):** the same fixture, but the
   changed plan's `depends_on` names the colliding plan's id -> exits
   `0`.
3. **Scoping (`--changed-only` correctness):** two pre-existing plans
   that already collide, neither changed in this diff -> exits `0`
   under `--changed-only` (not blocked), while the same fixture without
   `--changed-only` still reports the collision in the advisory report
   (unchanged default behavior) -- proving the flag actually restricts
   scope rather than merely suppressing output.
4. **Incomplete-coverage-is-not-a-block:** an unrelated third plan with
   no `affected_paths` declared (today's "unassessed" case), zero actual
   collisions on the changed plan -> exits `0` under the blocking flag,
   proving "unassessed" and "unacknowledged collision" are not
   conflated.
5. **MR-vs-plan collision (local-branch mode):** a changed plan's
   `affected_paths` overlaps a local branch's actual changed-path set
   (not another plan) -> `--changed-only --fail-on-unacknowledged-collision`
   still exits `1` (branches have no `depends_on` of their own, so this
   path can only be acknowledged from the plan side).
6. **Default (no-flag) invocations unaffected:** re-run every existing
   `plan_collision.rs` fixture unmodified, with neither new flag passed
   -> byte-identical exit codes to before this slice (0/1/2 contract
   preserved).
7. **Companion touch** to `tests/gate_change_check.rs` and, if the new
   job trips `ci-needs-implication`'s own surface checks,
   `tests/ci_needs_implication.rs` -- both `.gitlab-ci.yml` and
   `main.rs` are registered shared surfaces per `.agf/gates.yaml`.
8. Full workspace suite, clippy, corpus regression, `agov gate
   change-check`.

## Rule projection

Introduces genuinely new MUST/blocking language (a real gate, not
advisory display) -- six-surface manifest required at
`docs/reviews/rule-projection/20260914T130217Z-row6-s4-issue123-collision-check-blocking-promotion.yaml`.
Likely normative owner: `agf/rules/workflow/development-workflow-rule.md`
(existing pointer, same as S3) or a new short section there naming this
specific blocking behavior -- to be decided during manifest drafting,
not asserted here. `AGENTS.md` + `agf/templates/AGENTS.md` gate
discipline sections and `agf/templates/gate1-plan.md` need the
`depends_on` field and blocking behavior documented, matching S3's
precedent of updating both loader copies together.

## Rollback

The new job is additive (`.gitlab-ci.yml` gains a job; nothing existing
changes). Reverting removes the job and both new flags; `depends_on`
reverts to an unused optional schema field with zero effect (no plan is
required to have declared it). `collision-check`'s default advisory
behavior is untouched throughout, so rollback carries no migration for
any existing plan or caller.

## Risks

- **False-positive block from the conservative glob-overlap algorithm.**
  S3's own design accepted this cost as "a reviewer's glance" under
  advisory-only framing; blocking makes the same false positive cost a
  blocked MR. Mitigated by: (a) the algorithm is unchanged, already
  proven conservative-but-decidable within the restricted glob subset;
  (b) `depends_on` is a one-line, same-Gate-1-review fix, not a separate
  approval cycle; (c) `--changed-only` narrows exposure to only the
  plan actually under review, not the whole corpus.
- **Reviewer friction from a new mandatory step.** Mitigated by scoping
  strictly to plan-file changes (`ci-trigger-scoping-rule` path
  filtering) and by keeping the fix (`depends_on`) cheap and
  self-service, not requiring a second Gate 1 round.

## Decision requested at Gate 1

Approve (a) the `depends_on` schema field, (b) the two new
`collision-check` flags and their exit-code contract, (c) the new
path-scoped, local-branch-only, blocking CI job, (d) that forge-mode
blocking and the post-implementation drift check are explicitly
out-of-scope follow-ups, not silently implied here, and (e) the test
strategy above as satisfying `gate-self-test-rule` for this
gate-mechanics change.
