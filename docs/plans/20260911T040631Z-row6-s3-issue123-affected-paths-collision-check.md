---
id: 20260911T040631Z-row6-s3-issue123-affected-paths-collision-check
status: pending-gate1
topic: row6-s3-issue123-affected-paths-collision-check
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Per `agf/agents/MANIFEST.yaml`, foundry's infosec-sentinel is "ci
      (agov validate infosec, on commit)" — a mechanical check, not a
      separate reviewer session. Ran the actual command, precisely named
      (corrected 2026-09-11, RNP-1 round 2 — the prior wording said "ran
      against this plan," which overstated its scope):
      `agov validate infosec` invokes `ValidateCommand::Infosec` with no
      plan-path argument (`main.rs:6883-6887`), which dispatches
      `agov_validators::infosec::validate_all(&agov_core::learnings_dir())`
      — it scans **learning records only**, not this plan file, its
      branch, or any API security contract. Result: no findings in the
      learning corpus. This run provides **no plan-security consultation
      whatsoever**; it is cited only as the mechanical CI check this
      project runs on every commit, not as evidence about this plan's
      own design. The read-only/no-write/no-credential reasoning for
      `agov plan collision-check` below is author reasoning, not a
      screening attestation, and remains an outstanding item for an
      actual infosec-sentinel or governor security consultation to
      confirm or correct.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Corrected 2026-09-11 (RNP-1, RNP-3, RNP-4, two rounds). Independent
      review now exists and is cited: `20260911T121946Z-contribution-auditor-6pq1`
      (Codex, `/root/independent_plan_review`, session distinct from this
      plan's author session `session_01L2r2kfLcdnYqGcTny6kyuH`), advisory
      BLOCK on RNP-1, RNP-3, RNP-4, RNP-5; recheck
      `20260911T124355Z-contribution-auditor-gtsg` found RNP-1's infosec
      wording still overstated and RNP-3's overlap algorithm still had a
      false negative (embedded `**`), both now resolved (see
      "Corrections" below). `status: consulted` reflects that the review
      happened and is cited, not that its findings are already cleared —
      clearance is the reviewer's own recheck, not a self-assessment.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Row 6 S3 — #123: structured affected-paths + `agov plan collision-check` (Gate 1)

## Provenance

- Parent: `docs/plans/20260910T072138Z-coordination-discovery-collision-tooling-work-order.md`
  (v2.11 IN row 6, Gate 1 `tsbh` / ratified `q862`), slice **S3**.
  Governor "Keep all in v2.11" (2026-09-10) confirms row 6 stays a hard
  cut precondition.
- Issue **#123**: a Gate 1 plan's affected files are prose, not a
  normalised, queryable set; nothing checks overlap across pending or
  approved plans and active MRs. Evidence: MR !2389 ↔ MR !2373 both slated
  to modify the same two Rust test files, surfaced only by manual
  independent review (`4ghp` finding A3), not by any plan field or `agov`
  check.
- **Fresh corroborating evidence, this session (2026-09-10):** the Row 5
  S3 (!2027) slice draft first claimed the `urgency` coordination-schema
  field, which the already-approved CP-B work order had assigned to
  CP-B-2 — a two-plan collision on one field, caught only by a manual
  read during drafting (`p2rk`). Exactly the failure class #123 names.

## Change

| File | Change |
|---|---|
| `agf/schemas/gate1-plan.schema.json` | New **optional** `affected_paths` property: array of strings, each a path or a glob (e.g. `agf/schemas/coordination-message.schema.json`, `tooling/rust/crates/agov/src/main.rs`). Absent = current prose-only plans remain fully valid (backward compatible). No `required` addition. Glob syntax is a **restricted subset** of `agov-validators/src/gate_change.rs`'s existing `glob_regex` dialect — see "Glob syntax restriction (RNP-3 round 2)" below for why the full dialect cannot be used as-is. |
| `tooling/rust` new `agov plan collision-check [--base-ref <ref>] [--include-forge]` | See "Collision-check contract (RNP-3)" below for the full specification. |
| `agov plan validate` | Unaffected — a plan with no `affected_paths` still validates exactly as today. A plan that declares `affected_paths` gets a light shape check (array of non-empty strings, valid glob per `glob_regex`) as part of the existing schema validation, no new blocking behaviour beyond that. |
| `AGENTS.md` "Gate discipline" + `agf/templates/AGENTS.md` "Gate discipline" + `agf/templates/gate1-plan.md` | One-line pointer (identical on both `AGENTS.md` copies, per RNP-4 round 2): a plan MAY declare `affected_paths` for machine-checkable overlap detection; `agov plan collision-check` reports (does not block) overlaps. |

### Glob syntax restriction (RNP-3 round 2)

Round 1 claimed the segment-walk overlap algorithm below had "no false
negatives by construction" against the *full* `glob_regex` dialect. The
recheck (`gtsg`) disproved this with a concrete, independently-verified
counterexample: `glob_regex("src/a**/x.rs")` compiles to
`^src/a(?:.*/)?x\.rs$` (embedded `**` inside a segment, absorbing a
slash-containing suffix mid-pattern), which **does** match literal
`src/ab/c/x.rs` — but the segment walk (which only special-cases a
segment that equals `**` *exactly*) compares `a**` against `ab` as two
plain literal-ish segments and rejects the pair. Confirmed independently
(Python `re.fullmatch` against the same translation) before writing this
correction.

**Resolution:** `affected_paths` accepts a **validated subset** of
`glob_regex`'s dialect, not the full dialect: `**` may appear **only as a
complete path segment** — preceded and followed by `/` or a string
boundary (`foo/**`, `**/foo`, `foo/**/bar`, or `**` alone) — never
embedded inside a segment (`a**`, `**b`, `pre**post` are all **rejected**
at schema-validation time with a clear error, not silently accepted with
undefined overlap behaviour). This is not an arbitrary restriction: every
existing `**` usage in this codebase's actual `.agf/gates.yaml` entries
(`tooling/rust/crates/agov/tests/fixtures/gate-change/**`, `agf/rules/**`,
etc.) already only uses `**` as a whole standalone segment — the
restriction codifies existing practice, it does not remove a construct
anyone currently relies on. Within this restricted subset, the segment
walk below is complete (no false negatives): every construct the subset
permits is exactly the set of cases the walk's compatibility rule
already handles correctly (literal-equal, `*`/`?`-bearing single-segment
wildcards, and whole-segment `**` absorption).

### Collision-check contract (RNP-3)

**Glob overlap ("can these two patterns ever match the same concrete
path, including a path that does not exist yet"):** two patterns overlap
if, walking their `/`-separated segments together, every corresponding
position is *compatible* — segments are literal-equal, or at least one
side's segment contains `*`/`?` (a wildcard segment is compatible with
any concrete single segment, hence with any other segment including
another wildcard), or one side's current segment is `**` (which absorbs
the remainder of the other side's path, including zero segments, making
the rest trivially compatible). Restricted to the subset above, this is a
**conservative, decidable, witness-free, no-false-negative** check — it
never needs to enumerate concrete paths (so it correctly reports
`new/**` as overlapping `new/file.rs` before either exists — Codex's
original example), and it may report some pairs as "possible overlap"
that no real file will ever instantiate (acceptable: this command is
advisory-only, never MR-blocking, so false positives cost a reviewer's
glance, not a blocked merge).

**Input sources, exact semantics:**
- **Plans**: every `docs/plans/*.md` with `status: pending-gate1` or
  `status: approved` that declares `affected_paths`. Plans without the
  field are **not** silently treated as conflict-free — they are listed
  in the report under "unassessed (no `affected_paths` declared)",
  distinct from "assessed, no overlap found." A plan's absence of the
  field is never presented as evidence of no conflict.
- **Local-branch mode (default, no network):** `--base-ref <ref>`
  (default: the local default branch, same convention as `agov session
  resume --base-branch`) sets the merge-base comparison point. For every
  ref in the **exact same ref universe S2 establishes**
  (`refs/heads/*` + `refs/remotes/<remote>/*`, excluding symbolic refs —
  reused for consistency, not redefined here), compute its changed-path
  set via `git diff --name-status <merge-base>...<ref-head>`. Renames
  (`R100 old new` and similar) contribute **both** the old and new path
  to that ref's changed-path set — conservative, since a plan's
  `affected_paths` naming the pre-rename path should still flag against
  a branch that renamed it away.
- **Forge mode (`--include-forge`, opt-in, network):** additionally
  queries the GitLab MR API for every open MR's changed-path set,
  identified by **MR IID + exact head SHA** (never a cached list — each
  invocation re-fetches), following pagination fully (`page`/`per_page`
  or `Link` header, whichever the forge client already uses elsewhere in
  this codebase). On **any** API error, timeout, or incomplete pagination
  for **any** MR, that MR is marked `unavailable`, and the overall report
  is marked `INCOMPLETE` — never silently degraded to "0 open-MR
  collisions found." An `INCOMPLETE` result is a distinct outcome from
  "checked, clean," both in the human-readable report and in the exit
  code below.
- **Coverage metadata is always stated explicitly** in the report header:
  which mode ran, how many plans/refs/MRs were inspected, and how many
  (if any) were unassessed or unavailable. "0 collisions" is never
  printed without that qualifying scope.
- **`--base-ref`'s exact scope (RNP-3 round 2):** it affects only the
  **diff computation** for each enumerated ref/MR (the merge-base used
  by `git diff --name-status <merge-base>...<ref-head>`, and the
  equivalent forge-side base for MR diffs). It never filters or narrows
  **which plans** are inventoried — plan inventory is always "every
  `docs/plans/*.md` with `status: pending-gate1` or `status: approved`,"
  independent of `--base-ref`.
- **Forge response completeness (RNP-3 round 2):** a truncated,
  collapsed, or otherwise incomplete-looking diff response from the forge
  API (fewer changed paths than the MR's own stated file-count metadata,
  or a diff response the client detects as size-limited) is treated as
  `unavailable` for that MR — the same as an outright API error — never
  as "this MR touches zero paths." Every retrieved diff is bound to the
  **exact head SHA** requested; if the forge reports the MR's head SHA
  changed between the initial MR-list fetch and the per-MR diff fetch
  (a live push during the scan), that MR is re-fetched once at its new
  head, or marked `unavailable` if it changes again — never silently
  diffed against a stale head.
- **Malformed plan files (RNP-3 round 2):** a `docs/plans/*.md` file that
  fails `agov plan validate` (bad front matter, schema violation) is
  excluded from the inventory and listed in the report as
  `invalid (excluded)` with the validation error — distinct from both
  "assessed, no overlap" and "unassessed (no `affected_paths`)". It does
  not silently reduce coverage without being named.
- **MCP exposure (RNP-3 round 2):** `agov-mcp` exposes `collision-check`
  in local-branch mode only (no network calls from the MCP adapter,
  consistent with this project's existing MCP tools — `--include-forge`
  is CLI-only in this slice; adding it to MCP is a separate, explicitly
  future decision, not silently implied here).

**Exit codes:** `0` = full coverage achieved, zero overlaps found;
`1` = at least one overlap found (regardless of coverage completeness —
a found overlap is reported even from an otherwise-incomplete scan);
`2` = no overlap found, but coverage was incomplete (some ref/MR
`unavailable`, or plans exist with no `affected_paths` declared) — a
caller must not treat `2` as equivalent to `0`.

## Explicitly out of scope (parent work order guard)

No MR-blocking gate in this slice — promoting `collision-check` to
blocking is a separate, `gate-self-test-rule`-covered decision requiring
its own evidence and negative test, not implied here. No requirement that
every plan retroactively adds `affected_paths` — prose-only plans stay
valid indefinitely.

## Not a gate (this slice) — required fixtures instead

1. **Backward compatible.** Every currently-approved plan (none of which
   declare `affected_paths`) still passes `agov plan validate` unchanged.
2. **Overlap detected, existing paths.** Two synthetic plans declaring an
   overlapping literal `affected_paths` entry → `collision-check` reports
   both, names the shared path, exit `1`.
3. **Future-path overlap (RNP-3's core case).** One plan declares
   `new/**`, another declares `new/file.rs` — neither path exists in the
   working tree yet → still reported as overlapping (the segment-walk
   check never touches the filesystem), exit `1`.
4. **No overlap, full coverage.** Two plans with disjoint `affected_paths`,
   local-branch mode, no unassessed plans → exit `0`, report states full
   coverage.
5. **Unassessed plan is not clean.** A third plan with no `affected_paths`
   declared exists alongside two disjoint-path plans → report lists it
   under "unassessed," exit `2` (not `0` — its absence is never silent
   conflict-freedom).
6. **MR-side overlap, local mode.** A plan's `affected_paths` intersects
   a local-branch's actual changed-path set (via `git diff --name-status`)
   → reported the same way as plan-vs-plan, exit `1`.
7. **Rename handling.** A local branch renames a file a plan's
   `affected_paths` names → both the pre- and post-rename path are in
   that branch's changed-path set; the plan still flags as overlapping.
8. **Forge-mode pagination and failure.** `--include-forge` against a
   fixture with more MRs than one API page, and separately against a
   fixture where one MR's changed-path fetch fails → the paginated case
   still finds every MR; the failure case marks that MR `unavailable` and
   the overall result `INCOMPLETE` (exit `2`), never silently reported as
   clean.
9. **Malformed `affected_paths` (non-string entry, empty array item,
   invalid glob per `glob_regex`)** → schema validation fails closed with
   a clear error, not silently ignored.
10. **Rejected embedded `**` (RNP-3 round 2).** `affected_paths` entries
    `src/a**/x.rs`, `pre**post/file.rs` → schema validation fails closed
    with an error naming the restriction (`**` only as a whole path
    segment), not silently accepted with undefined overlap behaviour.
11. **Forge head-change mid-scan (RNP-3 round 2).** An MR's head SHA
    changes between the MR-list fetch and its diff fetch → re-fetched
    once at the new head, or marked `unavailable` on a second change —
    never diffed against the stale head.
12. **Malformed plan excluded, named (RNP-3 round 2).** A
    `docs/plans/*.md` file that fails `agov plan validate` → excluded
    from the inventory, listed as `invalid (excluded)` with its
    validation error, distinct from "unassessed."

If a later decision promotes `collision-check` to MR-blocking, that
decision's own Gate 1 supplies the `gate-self-test-rule` negative test
(a genuine collision still blocks after the change) — not this slice.

## Rule-projection screen

`docs/reviews/rule-projection/20260911T040631Z-row6-s3-issue123-affected-paths-collision-check.yaml`
(corrected 2026-09-11, two rounds — round 1: manifest created, one owner
named instead of two deferred candidates; round 2 (RNP-4 recheck): fixed
an adopter-disposition contradiction): normative owner
`agf/rules/workflow/development-workflow-rule.md` (existing pointer
sufficient — its Phase 2 "affected files list" prose already covers this
field's purpose); adopter explanation not applicable; runbook
`agf/templates/gate1-plan.md` updated; AI loaders `AGENTS.md` **and**
`agf/templates/AGENTS.md` both updated (`.claude/CLAUDE.md` not
applicable via import) — round 1 left the adopter template as an "open
question," which round 2 found self-contradictory against the manifest's
own distribution/runbook rows (the template already ships this exact
capability), so it is decided here: both loaders get the identical
pointer; validation/drift advisory only; distribution confirmed for the
schema and both templates (root `AGENTS.md` is foundry-local, not
shipped; `agov` itself ships via the separate tooling archive, not the
`agf/` framework artifact).
`agov plan projection-check --allow-planned --base-ref origin/main` on
this manifest: OK.

## Rollback

`affected_paths` is optional-additive on the schema; `collision-check` is
a new, standalone, read-only command. Revert the MR → command removed,
schema field removed, zero effect on any existing plan. No migration.

## Corrections (2026-09-11, RNP review — two rounds)

Round 1 (`6pq1`) found this plan's original text incomplete in four
ways. Round 2 (`20260911T124355Z-contribution-auditor-gtsg`) rechecked
and found RNP-1 and RNP-3 still partial, RNP-4 partial in a new way not
visible in round 1. Status after both rounds:

- **RNP-1** (unsupported consultation attestations, then: infosec wording
  overstated the mechanical check's actual scope) — **resolved round 2**:
  `advisory_consultation.infosec-sentinel` now names the exact command
  (`ValidateCommand::Infosec` / `validate_all(&learnings_dir())`) and
  states plainly it provides no plan-security consultation.
- **RNP-3** (undefined glob/overlap/coverage contract, then: the overlap
  algorithm had a false negative on embedded `**`, plus several
  completeness gaps) — **resolved round 2**: restricted `affected_paths`'
  accepted glob syntax to whole-segment `**` only (verified
  counterexample independently before fixing), added fixture 10 for the
  rejection, and closed the remaining completeness gaps (`--base-ref`
  scope, forge truncation/mid-scan head-change, malformed-plan handling,
  MCP exposure) with fixtures 11-12.
- **RNP-4** (deferred owner, missing manifest, then: adopter-template
  disposition self-contradicted the manifest's own distribution row) —
  **resolved round 2**: owner decided at Gate 1
  (`development-workflow-rule.md`, round 1), manifest added and validated
  (round 1), adopter-template contradiction fixed by extending the same
  pointer to `agf/templates/AGENTS.md` (round 2).
- **RNP-5** (stale independence claim) — resolved round 1, see below.

### RNP-5: shared-surface correction

Same correction as S2's sibling plan: row-5 S4 (!2681) and row-6 S1
(!2683/!2675) have since merged into `main` and touched
`tooling/rust/crates/agov/src/main.rs` and `AGENTS.md` — files this
slice's own "Change" table also edits (command registration in `main.rs`,
the Gate-discipline pointer in `AGENTS.md`). Logical independence is not
absence of shared files. **Implementation-order note for coder-tester:**
rebase onto current `main` (already containing S1/S4) before
implementing; add this slice's specific `main.rs` command and `AGENTS.md`
pointer on top of current content, not by reverting or overwriting S1/S4.

## Decision requested at Gate 1

Approve (a) the optional `affected_paths` schema field and its
restricted-subset `glob_regex`-based syntax (whole-segment `**` only),
(b) the full collision-check contract above (glob-overlap algorithm,
input sources, `--base-ref` scope, forge completeness/head-change
handling, malformed-plan handling, MCP exposure, coverage/exit-code
semantics), (c) `agov plan collision-check` shipping **advisory**, not
blocking, (d) the 12 fixtures, (e) the rule-projection manifest (owner
decided: `development-workflow-rule.md`; both `AGENTS.md` loaders
updated). Then coder-tester implements against current `main`; own
Gate 2.
