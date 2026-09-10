---
id: 20260910T072138Z-coordination-discovery-collision-tooling-work-order
status: approved
gate1_approved_by: 20260910T122454Z-planner-architect-tsbh
gate1_approved_at: 2026-09-10T12:24:54Z
topic: coordination-discovery-collision-tooling-work-order-v211-row6
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      This work order commits `agov` and Gate-1-plan-schema changes: a
      path-contract alignment between `agov coord visibility` and
      `agov-coord::channel_reader::read_channel` (S1), an optional
      branch-local scan for `agov handoff next` (S2, CP-C), and a
      structured `affected_paths` field plus an `agov plan collision-check`
      command (S3, #123). InfoSec must confirm at this Gate 1 that: the
      path-contract change cannot be used to hide a live obligation by
      committing it just outside the scanned set (the gate-self-test
      negative fixture below is the control); the branch-local scan does
      not surface coordination content from unrelated or hostile branches
      as if addressed to the reader; and the collision-check reads plan
      metadata only, never executes plan content. Each slice returns for
      its own exact-head InfoSec review.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Discovery/visibility tooling touches the audit trail's completeness.
      Contribution-auditor must confirm that S1 makes the two readers
      agree without either of them dropping a genuinely misplaced live
      record; that S2 is a bounded addition and not an `agov handoff next`
      selection-logic redesign; and that S3's collision-check is advisory
      (reports overlap) unless a later decision makes it blocking, in
      which case `gate-self-test-rule` applies. Each slice returns for its
      own exact-head audit.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Coordination discovery and collision tooling — Gate 1 work order (v2.11 IN row 6)

## Provenance and authority

- **v2.11 scope plan** `docs/plans/20260909T021841Z-v211-release-scope.md`
  row 6 (IN by `er78`; `sh3m` approved the expanded scope): "#123
  structured affected paths / advisory plan conflicts plus `jprj` CP-C,
  **bounded** to the branch-local obligation visibility and archive-path
  correction already evidenced by B4; it is **not** an open-ended
  `handoff next` redesign."
- **`jprj`** §C (CP-C) — `agov handoff next` reads only the base branch, so
  a handoff on an unmerged branch is invisible to it (only
  `agov coord visibility` surfaces it).
- **Issue #123** — a Gate 1 plan's affected files are prose, not a
  queryable set; nothing checks overlap across pending/approved plans and
  active MRs. Evidence: MR !2389 ↔ MR !2373 both slated to modify the same
  two Rust test files, surfaced only by manual review `4ghp` finding A3.
- **Fresh corroboration (this session, 2026-09-10):** the Row 5 work order
  (MR !2658) claimed the `urgency` schema field that the Gate-1-approved
  CP-B work order already assigns to CP-B-2 — a two-plan collision on one
  schema field, caught only by a manual read (`p2rk`).

Gate 1 **work order**: fixes membership, sequence, and slice boundaries.
Authorizes no `agov`/schema change by itself — each slice returns for its
own Gate 1 detail, exact-head advisory reviews, and binding Gate 2.

## Members (bounded)

| Member | Gap |
|---|---|
| **S1 — channel path-contract** | `agov coord visibility` scans recursively (`git ls-tree -r`, `git diff --name-only <base>...HEAD`); `agov-coord::channel_reader::read_channel` scans one directory non-recursively (`std::fs::read_dir`). They disagree on what "the coordination channel" is. The B4 archive at `docs/coordination-archive/` was the visible instance; the divergence is general and latent. |
| **S2 — CP-C branch-local obligation visibility** | `agov handoff next <role>` reads only `main`; a ready handoff on an unmerged branch is invisible to it. |
| **S3 — #123 structured affected-paths + collision check** | No structured, queryable affected-path set on Gate 1 plans; no primitive intersects those sets across pending/approved plans and active MRs. |

**Explicitly out of scope:** any redesign of `agov handoff next`'s
selection or ranking logic; session-level addressing (#121, in row 5);
supersession semantics (#122, in row 5); the `urgency` field (CP-B-2).

## `5jde` / `jprj` disposition alignment

`jprj` §C recommended folding CP-C into #123's `agov` tooling slice —
this work order does exactly that (S2 alongside S3), and adds S1 because
the B4 incident showed the path-contract itself is unspecified, not just
the archive location.

## B4 status note

The specific B4 regression (39 false hidden-obligation results from the
archive) is **already resolved** — MR !2581 merged (2026-09-09,
`36c21dba`) relocating the archive to `docs/coordination-archive/`, which
the recursive scans still match but which is outside `docs/coordination/`.
S1 does not re-fix B4; it specifies the contract both readers must share
so the next archive/nesting case is handled by design, not by relocation.

## Ordering constraint

All three slices are independent; none blocks another. S1 and S2 are
`agov`-surface changes; S3 adds a plan-schema field and a new `agov`
subcommand. Recommended order by value: **S1 → S2 → S3** (S1 removes a
live correctness gap; S3 is the largest).

## Implementation slices (each its own Gate 1 detail + exact-head reviews + Gate 2)

### S1 — coordination-channel path contract (single definition, both readers)
- **Contract:** "the coordination channel" is exactly the set of
  `*.md` files **directly** under `docs/coordination/` (non-recursive), by
  both `agov coord visibility` and `agov-coord::channel_reader::read_channel`.
  Nested subdirectories and any sibling path (`docs/coordination-archive/`,
  etc.) are **not** the channel.
- **Change:** make `agov coord visibility` filter its recursive git output
  to that exact set (path depth check), matching `read_channel`. Document
  the contract in one place both cite.
- **gate-self-test (row 6 obligation):** the implementing MR includes a
  negative fixture — a **valid, live** top-level obligation record placed
  where it should not be (a nested `docs/coordination/<sub>/…` path, and a
  malformed sibling path) — and proves the tooling still **flags it as a
  misplaced live obligation**, i.e. the exclusion narrows to the archive
  case and does not silently swallow a genuinely mislocated record. A
  positive fixture proves a correctly-placed record is seen by both
  readers identically.
- **Rule projection:** the channel definition is currently implicit in
  `AGENTS.md` ("immutable Markdown files in `docs/coordination/`") and
  `coordination-publication-rule.md`. S1 makes it explicit; six-surface
  manifest owed at S1 Gate 1 (likely `coordination-publication-rule.md`
  as owner, `AGENTS.md` + loaders as pointers).

### S2 — CP-C: branch-local obligation visibility for `agov handoff next`
- **Change:** `agov handoff next <role>` gains an **opt-in** flag
  (e.g. `--include-branch-local`) that additionally scans branch-local
  coordination records the same way `agov coord visibility` does, so a
  ready handoff on an unmerged branch is not invisible. Default behaviour
  (base-branch only) is unchanged.
- **Bounded:** no change to how `handoff next` *selects* or *ranks* the
  next item; it only widens the candidate set when the flag is set. Not a
  redesign.
- **Not a gate.** Advisory queue display. No `gate-self-test-rule`
  trigger. Fixtures: a branch-local ready handoff appears with the flag
  and does not without it.

### S3 — #123: structured affected-paths + `agov plan collision-check`
- **Schema:** add an **optional** `affected_paths` array (path globs) to
  `gate1-plan.schema.json`. Absent = current prose-only behaviour
  (backward compatible); present = machine-checkable.
- **`agov`:** `agov plan collision-check [--base-ref <ref>]` intersects
  `affected_paths` across all `pending-gate1` / `approved` plan files and
  the changed-path sets of open MRs, and reports every overlap with the
  plan/MR pair and the shared paths. **Advisory** (report + non-zero exit
  on overlap for CI visibility), not an MR-blocking gate in this slice.
- **gate-self-test:** only if a later decision makes `collision-check`
  MR-blocking. If it ships advisory, the fixture set is a positive
  (two plans sharing a path → reported) and a negative (disjoint plans →
  clean) pair.
- **Rule projection:** `development-workflow-rule.md` / `uncertainty-and-
  scope-rule.md` name the "list every module that will change" obligation
  in prose; S3 adds the structured form. Six-surface manifest owed at S3
  Gate 1; `affected_paths` field may need a `documentation-rule-
  projection-rule` note since it is plan-schema normative.

## Affected files (work-order level — indicative; each slice's own Gate 1 is authoritative)

- `tooling/rust/crates/agov-coord/src/channel_reader.rs` — S1
- `tooling/rust/` `agov coord visibility` command crate — S1
- `tooling/rust/` `agov handoff next` command crate — S2
- `tooling/rust/` new `agov plan collision-check` subcommand — S3
- `agf/schemas/gate1-plan.schema.json` — S3 (`affected_paths`, optional)
- `tooling/rust/` tests — per slice (positive + negative fixtures; S1's
  misplaced-live-record negative fixture is mandatory per row 6)
- `agf/rules/workflow/coordination-publication-rule.md` — S1 (channel-def owner)
- `docs/reviews/rule-projection/<slice-id>.yaml` — one per slice as triggered
- `AGENTS.md` / `agf/templates/AGENTS.md` / `.claude/CLAUDE.md` — pointers

## Decomposition (each its own gate)

| Slice | Deliverable | Gate path |
|---|---|---|
| S1 | Shared channel path contract; `visibility` filtered to match `read_channel`; **mandatory** misplaced-live-record negative fixture | own Gate 1 → exact-head reviews → Gate 2 |
| S2 | `agov handoff next --include-branch-local` opt-in scan + fixtures | own Gate 1 → Gate 2 |
| S3 | `affected_paths` optional plan-schema field + `agov plan collision-check` (advisory) + fixtures | own Gate 1 → Gate 2; gate-self-test only if later made blocking |

## Risks and controls

- **S1's exclusion hides a genuinely misplaced live record.** Control: the
  mandatory negative fixture proves a nested/mislocated live obligation is
  still flagged; the contract is "directly under `docs/coordination/`",
  not "anything not in the archive".
- **S2 surfaces coordination from unrelated/hostile branches.** Control:
  opt-in flag; InfoSec reviews the branch-selection surface at S2 Gate 1;
  it widens candidates for *display*, grants no authority.
- **S3 collision-check produces noise and is ignored.** Control: advisory
  first, non-zero exit for CI visibility, tuned by real overlap rate
  before any proposal to make it blocking (which would be a separate
  gate-self-tested decision).
- **`affected_paths` drifts from the prose file list.** Control: S3 Gate 1
  decides whether `agov plan validate` warns on a plan that has prose
  "affected files" but no `affected_paths`; not blocking in this slice.

## Rollback

S1 is a filter tightening — revert restores prior (over-broad) visibility
output. S2 and S3 are opt-in / optional-additive — reverting each slice's
MR restores prior behaviour with no migration. No slice rewrites plans or
coordination records.

## Test and acceptance strategy

Per slice: `agov` command tests with and without the new flag/field
proving backward-compatible defaults; S1's positive (both readers agree on
a correct record) and mandatory negative (misplaced live record still
flagged) fixtures; S2's with/without-flag branch-local visibility
fixtures; S3's overlap-reported / disjoint-clean fixtures. No slice merges
without its exact-head advisory reviews and binding Gate 2.

## Rule-projection screen (work-order level)

`not applicable` for this parent plan — it sequences slices and changes no
normative text itself. S1 (channel definition) and S3 (`affected_paths`
plan-schema field) are the likely per-slice triggers and carry their own
`docs/reviews/rule-projection/<slice-id>.yaml` six-surface manifest at
their Gate 1. S2 is an `agov`-surface change with loader-pointer impact
only.

## Decision requested at Gate 1

Approve (a) the three-member bounded set (S1 channel path contract, S2
CP-C branch-local visibility, S3 #123 structured affected-paths +
collision-check), (b) that S3's `collision-check` ships **advisory**, not
blocking, in this work order, (c) the explicit out-of-scope list (no
`handoff next` redesign; #121/#122/`urgency` belong elsewhere), (d) that
S1 does not re-fix B4 but specifies the shared contract, and (e) that each
slice retains its own Gate 1 detail and binding Gate 2. This work order
grants no `agov`/schema change authority on its own.
