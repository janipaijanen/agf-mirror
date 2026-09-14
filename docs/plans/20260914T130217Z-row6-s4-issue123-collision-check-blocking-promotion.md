---
id: 20260914T130217Z-row6-s4-issue123-collision-check-blocking-promotion
status: approved
gate1_approved_by: governor-reviewer
gate1_approved_at: 2026-09-14T15:26:01Z
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
  - agf/rules/workflow/development-workflow-rule.md
  - .agf/gates.yaml
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

## Correction (2026-09-14, round 2) — independent Gate 1 review BLOCK

Independent review `20260914T133701Z-coder-tester-thsq` found four
mandatory blocking findings in the round-1 draft below and correctly
recommended BLOCK before governor Gate 1. All four are fixed in this
round-2 revision (the design below reflects the corrected state
throughout, not left as a separate historical section, since round 1
was never approved):

- **S4-G1-1 (material):** round 1 named `development-workflow-rule.md`
  as "existing pointer sufficient" without actually adding blocking-
  authority language to it -- the rule's current text requires an
  affected-files list, but never grants `agov plan collision-check` (or
  any check) permission to halt Gate 1, contradicting
  `tool-authority-boundary-rule.md`'s "a tool may block work only when a
  named rule... explicitly grants that check blocking authority." Round
  1 would have shipped a gate with no rule actually authorizing it to
  block. **Fixed:** `development-workflow-rule.md` is now in
  `affected_paths`, actually amended (see "Rule projection" below), and
  the manifest's `normative_owner` disposition changes from "existing
  pointer sufficient" to "updated."
- **S4-G1-2 (material):** round 1 registered no new gate in
  `.agf/gates.yaml` -- only companion touches to *existing* gates'
  tests. Without its own registered gate id, surfaces, and test paths,
  a later change to `plan_collision.rs`, its CLI flags, the schema
  field, or the CI job is never tied back to this gate's own negative
  test by `gate-change-check` itself. **Fixed:** `.agf/gates.yaml` is
  now in `affected_paths`; see "Gate registration" below for the new
  entry.
- **S4-G1-3 (material, the most substantive fix):** round 1's CI job
  relied on `collision-check`'s existing local-branch-mode ref
  enumeration (`git branch --list`/`for-each-ref` over
  `refs/heads/*`+`refs/remotes/*`), inherited unchanged from S3. A
  GitLab CI job's checkout does not reliably have every other open
  branch present as a local or remote-tracking ref -- a standard MR
  pipeline checkout fetches the pipeline's own ref, not the full
  multi-branch inventory S3's interactive/local-clone use case assumes.
  Round 1's blocking job could therefore pass cleanly regardless of
  real collisions (seeing no other refs) or behave inconsistently
  depending on runner checkout residue -- exactly the non-determinism a
  blocking gate cannot tolerate. **Fixed:** this slice's blocking scope
  is narrowed to **plan-vs-plan collisions only** -- see "Design:
  `--plans-only`" below. Ref-vs-plan and forge-vs-plan blocking (which
  both have the same underlying determinism problem, forge doubly so)
  are deferred to a future slice that defines and tests a deterministic,
  bounded ref-fetch contract first. Plan-vs-plan comparison needs no
  such fetch: every `docs/plans/*.md` file is already present, verbatim,
  in the same checkout the CI job runs in -- no external state, fully
  deterministic.
- **S4-G1-4 (material):** round 1's CI job used a bare `rules: changes:`
  entry with no pipeline-source condition. On a plan branch's first
  push (before GitLab registers the MR), this project's own documented
  first-push behavior produces both a `push`-sourced and a later
  `merge_request_event`-sourced pipeline for the same SHA; a bare
  `changes:` rule makes the new job eligible in both, duplicating a
  real Rust-build/test job for no additional Gate 2 evidence -- exactly
  the anti-pattern this project's `ci.skip` first-push guard and
  duplicate-pipeline discipline already exist to prevent. **Fixed:**
  the job now has exactly one rule,
  `if: $CI_PIPELINE_SOURCE == "merge_request_event"` combined with the
  path filter, and no other rule -- it never runs on a `push`-sourced
  pipeline, so no dual-graph duplication is possible. See "Design: CI
  job" below.

## Correction (2026-09-14, round 3) — independent re-review BLOCK

A second independent review, `20260914T143145Z-coder-tester-rr33`,
confirmed all four round-2 findings are resolved but found two further
residual, real problems in the corrected design. Both fixed here; the
design sections below reflect the corrected state throughout.

- **S4-G1-R2-1 (material):** `--changed-only` still needed a base ref to
  diff against, and round 2 quietly assumed a local `refs/heads/main`
  would exist in the CI job's own checkout. The actual runner log for
  this MR's own pipeline shows a **detached-HEAD checkout with no local
  `main` ref at all** -- the same fact this project's own existing
  `cf_resolve_base` fail-closed contract
  (`.gitlab/scripts/changed-files.sh`, already used by the
  `agov:governance:validate` job) was written to handle. Worse, round
  2's exit-code contract meant an unresolvable base silently produced
  "no changed plan -> exit 0," letting the gate pass without ever
  evaluating the candidate plan. **Fixed:** the CI job now sources and
  calls the repository's existing `cf_resolve_base` function to fetch
  and resolve `$CI_MERGE_REQUEST_TARGET_BRANCH_NAME` deterministically,
  passes the resolved SHA explicitly as `--base-ref`, and **fails
  closed (exits non-zero) if the base cannot be resolved at all** --
  never silently degrading to a clean pass. See "Design: CI job" below.
- **S4-G1-R2-2 (material):** the plan said `--fail-on-unacknowledged-collision`
  changes only the exit code, "never the printed report" -- but the
  unchanged report still literally prints `(advisory only)` and no
  `severity: blocking` line, even while the process exits non-zero.
  `tool-authority-boundary-rule.md` requires blocking output to declare
  its level; undeclared output is advisory by that rule's own default.
  A job that blocks the MR while its own evidence still says "advisory
  only" is self-contradictory. **Fixed:** blocking mode now emits
  mode-sensitive severity labels -- see "Design: severity labeling"
  below. Default (no-flag) output is completely unchanged.

Two non-blocking advisory corrections from the same review are also
applied: the projection manifest's two remaining "two new flags"
references (`adopter_facing_explanation` and `distribution` rows -- round
2 only updated `validation_drift_evidence`) are corrected to three, and
`--plans-only --include-forge` behavior is now explicitly specified and
tested: `--plans-only` takes precedence and silently suppresses forge
querying regardless of `--include-forge` (documented explicitly, per the
review's accepted alternative to rejecting the combination outright).

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

Four additions to `tooling/rust/crates/agov/src/plan_collision.rs`, none
changing default (no-flag) behavior:

1. **`--plans-only`** (new bool flag, default off; S4-G1-3 fix). When
   set, `collision-check` skips ref/branch enumeration and forge
   querying entirely -- it never calls `git branch --list`/
   `for-each-ref` or the GitLab API, regardless of `--include-forge`.
   Only the plan-vs-plan comparison runs, over `docs/plans/*.md` files
   already present in the current checkout. This is what makes the
   check deterministic inside a CI job's own checkout: no external ref
   state is consulted, so the same commit always produces the same
   result regardless of runner fetch depth or branch-inventory residue.
2. **`--changed-only`** (new bool flag, default off). When set, a
   pair is only reported/considered at all if **at least one** plan in
   it is "changed": its `docs/plans/<file>.md` differs between
   `--base-ref` and the working tree (new or modified in the current
   diff, computed with the same `git diff --name-status` machinery
   `changed_paths` already uses, applied to `docs/plans/` itself). A
   pair where **neither** plan is changed (two long-standing plans that
   already collided before this diff) is excluded entirely under this
   flag -- not merely downgraded, dropped from the report and from
   blocking consideration, since this flag exists specifically to scope
   out pre-existing state this MR did not create. (With `--plans-only`
   also set, "eligible" below means plan-vs-plan pairs only; refs/MRs
   are not enumerated at all in that mode, so their own "changed"
   concept, or lack of one, is moot.)
3. **Acknowledgment matching.** For each *eligible* pair (i.e. the
   pair a plain, unflagged `collision-check` would already report, and
   -- when `--changed-only` is set -- a pair that also passes check 2
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
4. **`--fail-on-unacknowledged-collision`** (new bool flag, default
   off). When set, changes only the **exit code**, never the printed
   report: exit `0` if every eligible pair is acknowledged (or none
   exist), exit `1` if at least one is not. This is the only exit-code
   change; default (no-flag) invocations keep today's `0`/`1`/`2`
   contract byte-for-byte, so every existing advisory call site
   (including ad hoc session use) is unaffected.

These flags are independent (each usable alone) but the CI gate below
uses `--plans-only`, `--changed-only`, and
`--fail-on-unacknowledged-collision` together.

### CI: a new, narrowly scoped blocking job

New job in `.gitlab-ci.yml` (name indicative,
e.g. `agov:governance:plan-collision-gate`), reusing this repository's
existing fail-closed base-resolution contract
(`.gitlab/scripts/changed-files.sh`'s `cf_resolve_base`, the same
function `agov:governance:validate` already uses -- S4-G1-R2-1 fix):

```yaml
agov:governance:plan-collision-gate:
  stage: <existing governance stage>
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
      changes:
        - "docs/plans/**/*.md"
  script:
    - . "$CI_PROJECT_DIR/.gitlab/scripts/changed-files.sh"
    - |
      target_ref="${CI_MERGE_REQUEST_TARGET_BRANCH_NAME:?target branch required in an MR pipeline}"
      if ! base="$(cf_resolve_base "$target_ref")"; then
        echo "ERR: could not resolve comparison base against '$target_ref'; failing closed (this gate never silently passes an unevaluated plan)." >&2
        exit 1
      fi
    - agov plan collision-check --plans-only --changed-only --fail-on-unacknowledged-collision --base-ref "$base"
```

Path-scoped per `ci-trigger-scoping-rule` -- a change touching no plan
file never runs this job. **MR-pipeline-source-only (S4-G1-4 fix):**
exactly one rule, gated on `$CI_PIPELINE_SOURCE == "merge_request_event"`
combined with the path filter; there is no second rule for `push`-sourced
pipelines, so the job is structurally absent from a `push` pipeline's
graph -- a plan branch's first push (before GitLab registers the MR)
never triggers this job at all, and no dual-graph duplication is
possible once the MR pipeline does run it.

**Deterministic, fail-closed base resolution (S4-G1-R2-1 fix):** a real
GitLab MR pipeline checks out a **detached HEAD** -- there is no local
`refs/heads/main` to default to, contrary to round 2's assumption. The
job explicitly fetches and resolves `$CI_MERGE_REQUEST_TARGET_BRANCH_NAME`
via `cf_resolve_base` (fetch + `git rev-parse` against the fetched
remote-tracking ref, already fail-closed on empty/unset predefined SHA
variables -- the exact condition round 2 missed) and passes the
resolved SHA explicitly as `--base-ref`. **If the base cannot be
resolved at all, the job exits non-zero (blocks) rather than silently
treating "no changed plan found" as a clean pass** -- an unresolvable
base means the candidate plan was never evaluated, which must never be
indistinguishable from "evaluated, no collision."

**Plan-vs-plan only this slice (S4-G1-3 fix, unchanged from round 2):**
`--plans-only` means no ref/branch enumeration and no forge querying
occur -- only `docs/plans/*.md` files are compared against each other.
This needs the resolved `--base-ref` only to determine *which plan
files changed* (a plain `git diff --name-status`, not a multi-branch
inventory), which is why S4-G1-R2-1's fix is a base-resolution fix, not
a reintroduction of the ref-enumeration determinism problem S4-G1-3
already closed. **`--plans-only` combined with `--include-forge`:**
`--plans-only` takes precedence and silently suppresses forge querying
regardless of `--include-forge` -- tested explicitly (see "Test
strategy" below), not left as an unspecified interaction.
**Non-goal for this slice:** blocking on collisions against a local
branch's or another open MR's changed paths (both `collision-check`'s
existing local-ref mode and `--include-forge`) -- both remain fully
advisory-only until a future slice defines and tests a deterministic,
bounded ref-fetch contract for a CI checkout.

On failure, the job's own output names the exact colliding plans and
the specific overlapping paths, now labeled per "Design: severity
labeling" below; the fix is either coordinate and add `depends_on`, or
change one side's `affected_paths` so they no longer overlap.

### Design: severity labeling (S4-G1-R2-2 fix)

Round 2's claim that `--fail-on-unacknowledged-collision` "changes only
the exit code, never the printed report" is corrected: it was
self-contradictory with `tool-authority-boundary-rule.md`'s requirement
that blocking output declare its level, since S3's unchanged report
literally prints `(advisory only)` and carries no `severity: blocking`
line even while the process would exit non-zero.

**Default (no-flag) output: completely unchanged**, still ending in
`Possible overlaps within the coverage above: N (advisory only)`, still
carrying no per-line severity label -- this preserves S3's own advisory
contract byte-for-byte, matching this plan's own backward-compatibility
requirement.

**Under `--fail-on-unacknowledged-collision`: mode-sensitive labels,
matching this project's existing `agov coord validate`/`agov plan
validate` convention** (`OK`/`ERR severity: blocking`,
`WARN severity: advisory`):

- Each reported `Possible overlap: ...` line for an *eligible,
  unacknowledged* pair (the ones that would cause exit `1`) gains a
  trailing `severity: blocking` marker.
- Each reported overlap line for an *eligible, acknowledged* pair
  (cleared by `depends_on`) gains `severity: informational (acknowledged
  via depends_on)` instead.
- The final summary line changes from `(advisory only)` to state the
  real split, e.g.
  `N unacknowledged collision(s) -- severity: blocking; M acknowledged`.
- `Unassessed`/`Invalid (excluded)`/`Unavailable` lines keep their
  existing, already-distinct labels (per S3's own contract) -- this
  slice does not change what those three categories mean, only adds
  the new blocking/acknowledged distinction to actual overlap lines.

This is purely additive to the report's *text* under the new flag; it
does not change what counts as a collision, an acknowledgment, or an
eligible pair -- those are the flags already specified above.

### Rule authority (S4-G1-1 fix)

`agf/rules/workflow/development-workflow-rule.md`'s Phase 2 ("Write a
Plan File First") gains a new subsection, e.g. "Affected-paths collision
blocking," stating explicitly: a plan whose `affected_paths` overlaps
another live (`pending-gate1`/`approved`) plan's `affected_paths`, with
neither plan's `depends_on` naming the other, is blocked at Gate 1 by
the `agov:governance:plan-collision-gate` CI job; the job checks
plan-vs-plan overlap only (not local branches or open MRs, which remain
advisory); the fix is `depends_on` or a non-overlapping
`affected_paths` edit. This is the named rule
`tool-authority-boundary-rule.md` requires before any check may block --
the manifest's `normative_owner` disposition is **updated**, not
"existing pointer sufficient," since round 1 asserted the pointer was
already sufficient without actually adding this text.

### Gate registration (S4-G1-2 fix)

New entry in `.agf/gates.yaml`, alongside `gate-change-check` and
`ci-needs-implication`:

```yaml
  - id: plan-collision-block
    status: active
    surfaces:
      - agf/schemas/gate1-plan.schema.json
      - agf/rules/workflow/development-workflow-rule.md
      - .gitlab-ci.yml
      - tooling/rust/crates/agov/src/plan_collision.rs
      - tooling/rust/crates/agov/src/main.rs
    tests:
      - tooling/rust/crates/agov/tests/plan_collision.rs
```

This ties every behavior-bearing surface (the schema field, the
authorizing rule, the CI job definition, and the command implementation)
to this gate's own negative-test evidence via `gate-change-check`
itself -- a later change to any of these five files must also touch
`tests/plan_collision.rs`, exactly the discipline `gate-change-check`
already enforces for the two existing registered gates.

### Explicitly out of scope

- **Ref-mode and forge-mode (`--include-forge`) blocking** -- both stay
  advisory-only this slice, deferred pending a deterministic, bounded
  CI ref-fetch contract (S4-G1-3). Only plan-vs-plan collisions block.
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
   entry naming it -> `--plans-only --changed-only
   --fail-on-unacknowledged-collision` exits `1`.
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
5. **`--plans-only` determinism (S4-G1-3 regression):** with local
   branches/refs present in the test fixture's git repository that
   would otherwise report a collision under plain `collision-check`,
   `--plans-only` produces the identical result whether or not those
   refs exist -- proving ref/branch state is never consulted under this
   flag, the property that makes the CI job safe against incomplete
   runner checkouts.
6. **CI-rule regression (S4-G1-4, first-push/MR-pair):** a fixture
   pipeline-config test (companion to this project's existing
   first-push-duplication coverage) proving a plan-changing commit's
   first push produces the new job only in a `merge_request_event`
   pipeline graph, never in the paired `push` graph.
7. **Fail-closed base resolution (S4-G1-R2-1 regression):** a job-level
   fixture where `cf_resolve_base` cannot resolve the target (fetch
   failure, unset `CI_MERGE_REQUEST_TARGET_BRANCH_NAME`, or an empty
   rev-parse result) -> the job exits non-zero and the printed error
   names the unresolvable target -- proving the gate blocks rather than
   silently treating an unevaluated plan as a clean pass. A second
   fixture confirms the resolved-base happy path still finds a real,
   known collision (i.e. the fail-closed path and the working path are
   both exercised, not just one).
8. **Severity labeling (S4-G1-R2-2 regression):** under
   `--fail-on-unacknowledged-collision`, an unacknowledged eligible
   collision's report line carries `severity: blocking`; an acknowledged
   one carries `severity: informational (acknowledged via depends_on)`;
   the summary line no longer says `(advisory only)`. The exact same
   fixture run **without** the flag reproduces S3's original
   `(advisory only)` text byte-for-byte, proving default output is
   truly unchanged.
9. **`--plans-only --include-forge` interaction:** with both flags
   passed, forge querying does not occur (no GitLab API calls) --
   `--plans-only` takes precedence, tested explicitly rather than left
   implicit.
10. **Default (no-flag) invocations unaffected:** re-run every existing
    `plan_collision.rs` fixture unmodified, with none of the three new
    flags passed -> byte-identical exit codes and report text to before
    this slice (0/1/2 contract preserved).
11. **Companion touch** to `tests/gate_change_check.rs` (registering the
    new `plan-collision-block` gate id) and, if the new job trips
    `ci-needs-implication`'s own surface checks,
    `tests/ci_needs_implication.rs` -- `.gitlab-ci.yml` and `main.rs` are
    registered shared surfaces for both existing gates per
    `.agf/gates.yaml`.
12. Full workspace suite, clippy, corpus regression, `agov gate
    change-check`.
13. **Gate 2 evidence requirement:** the implementation MR's own
    description must include the reviewer-visible negative-test
    command and its output, tied to that MR's exact head, per
    `gate-self-test-rule` -- not merely "tests pass in CI," a
    transcript a reviewer can read directly.

## Rule projection

Introduces genuinely new MUST/blocking language (a real gate, not
advisory display) -- six-surface manifest required at
`docs/reviews/rule-projection/20260914T130217Z-row6-s4-issue123-collision-check-blocking-promotion.yaml`.
Normative owner: `agf/rules/workflow/development-workflow-rule.md`,
**actually updated** with the new "Affected-paths collision blocking"
subsection (see "Rule authority" above) -- not merely an existing
pointer, per S4-G1-1's fix. `AGENTS.md` + `agf/templates/AGENTS.md` gate
discipline sections and `agf/templates/gate1-plan.md` need the
`depends_on` field and the corrected (plan-vs-plan-only) blocking
behavior documented, matching S3's precedent of updating both loader
copies together.

## Rollback

The new job is additive (`.gitlab-ci.yml` gains a job; nothing existing
changes). Reverting removes the job, the three new flags, and the new
`.agf/gates.yaml` entry; `depends_on` reverts to an unused optional
schema field with zero effect (no plan is required to have declared it);
`development-workflow-rule.md`'s new subsection reverts to its prior
text. `collision-check`'s default advisory behavior is untouched
throughout, so rollback carries no migration for any existing plan or
caller.

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

Approve (a) the `depends_on` schema field, (b) the three new
`collision-check` flags (`--plans-only`, `--changed-only`,
`--fail-on-unacknowledged-collision`) and their exit-code contract, (c)
the new path-scoped, MR-pipeline-only, plan-vs-plan-only blocking CI
job, including its fail-closed `cf_resolve_base`-based target
resolution, (d) the mode-sensitive severity-labeling contract for
blocking-mode output, (e) the new `development-workflow-rule.md`
subsection actually granting this check blocking authority, (f) the new
`plan-collision-block` entry in `.agf/gates.yaml`, (g) that ref-mode
blocking, forge-mode blocking, and the post-implementation drift check
are explicitly out-of-scope follow-ups, not silently implied here, and
(h) the test strategy above (including the Gate 2 reviewer-visible
negative-test transcript requirement) as satisfying
`gate-self-test-rule` for this gate-mechanics change.
