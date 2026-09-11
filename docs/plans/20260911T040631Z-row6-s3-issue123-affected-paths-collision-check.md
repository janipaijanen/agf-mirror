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
      separate reviewer session. Ran `agov validate infosec`: no
      findings. The command is read-only (no plan/MR mutation, no forge
      write calls); the new schema field is a bounded array of
      path-glob strings, schema-validated, and cannot smuggle content
      outside the plan-file structure — independently re-checkable
      against the live schema/source. Corrected 2026-09-11 (RNP-1): the
      prior wording ("InfoSec confirms ...") implied an independent
      reviewer attestation this plan did not actually have.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Corrected 2026-09-11 (RNP-1, RNP-3, RNP-4): the prior wording
      claimed contribution-auditor "confirms" scope-boundedness without
      citing a reviewer or session. Independent review now exists and is
      cited: `20260911T121946Z-contribution-auditor-6pq1` (Codex,
      `/root/independent_plan_review`, session distinct from this plan's
      author session `session_01L2r2kfLcdnYqGcTny6kyuH`), advisory BLOCK
      on RNP-1, RNP-3, RNP-4, RNP-5. This revision resolves all four for
      S3 (see "Corrections (2026-09-11, RNP review)" below); the
      reviewer rechecks this corrected exact head before any Gate 1
      decision. `status: consulted` reflects that the review happened
      and is cited, not that its findings are already cleared.
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
| `agf/schemas/gate1-plan.schema.json` | New **optional** `affected_paths` property: array of strings, each a path or a glob (e.g. `agf/schemas/coordination-message.schema.json`, `tooling/rust/crates/agov/src/main.rs`). Absent = current prose-only plans remain fully valid (backward compatible). No `required` addition. Glob syntax is exactly `agov-validators/src/gate_change.rs`'s existing `glob_regex` dialect (already used for gate `surfaces`/`tests` matching) — `**` (any depth incl. zero), `*` (one path segment), `?` (one non-slash character), everything else literal — reused rather than a new dialect invented for this field. |
| `tooling/rust` new `agov plan collision-check [--base-ref <ref>] [--include-forge]` | See "Collision-check contract (RNP-3)" below for the full specification. |
| `agov plan validate` | Unaffected — a plan with no `affected_paths` still validates exactly as today. A plan that declares `affected_paths` gets a light shape check (array of non-empty strings, valid glob per `glob_regex`) as part of the existing schema validation, no new blocking behaviour beyond that. |
| `AGENTS.md` "Gate discipline" + `agf/templates/gate1-plan.md` | One-line pointer: a plan MAY declare `affected_paths` for machine-checkable overlap detection; `agov plan collision-check` reports (does not block) overlaps. |

### Collision-check contract (RNP-3)

**Glob overlap ("can these two patterns ever match the same concrete
path, including a path that does not exist yet"):** two patterns overlap
if, walking their `/`-separated segments together, every corresponding
position is *compatible* — segments are literal-equal, or at least one
side's segment contains `*`/`?` (a wildcard segment is compatible with
any concrete single segment, hence with any other segment including
another wildcard), or one side's current segment is `**` (which absorbs
the remainder of the other side's path, including zero segments, making
the rest trivially compatible). This is a **conservative, decidable,
witness-free** check — it never needs to enumerate concrete paths (so it
correctly reports `new/**` as overlapping `new/file.rs` before either
exists — Codex's exact example), and it may report some pairs as
"possible overlap" that no real file will ever instantiate (acceptable:
this command is advisory-only, never MR-blocking, so false positives cost
a reviewer's glance, not a blocked merge — false negatives would be the
dangerous direction, and this design has none by construction: any pair
that could concretely overlap is compatible at every segment).

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

If a later decision promotes `collision-check` to MR-blocking, that
decision's own Gate 1 supplies the `gate-self-test-rule` negative test
(a genuine collision still blocks after the change) — not this slice.

## Rule-projection screen

`docs/reviews/rule-projection/20260911T040631Z-row6-s3-issue123-affected-paths-collision-check.yaml`
(corrected 2026-09-11, RNP-4 — the manifest now exists and names one
owner, not two candidates deferred to implementation time):
normative owner `agf/rules/workflow/development-workflow-rule.md`
(existing pointer sufficient — its Phase 2 "affected files list" prose
already covers this field's purpose); adopter explanation not applicable;
runbook `agf/templates/gate1-plan.md` updated; AI loaders `AGENTS.md`
updated (`.claude/CLAUDE.md` not applicable via import;
`agf/templates/AGENTS.md` left as an open adopter-facing question, not
silently applied); validation/drift advisory only; distribution confirmed
for the schema and plan template (root `AGENTS.md` is foundry-local, not
shipped — corrected from the original plan's imprecise "agov ships in
the AGF artifact" claim: `agov` itself ships via the separate tooling
archive, not the `agf/` framework artifact).
`agov plan projection-check --allow-planned --base-ref origin/main` on
this manifest: OK.

## Rollback

`affected_paths` is optional-additive on the schema; `collision-check` is
a new, standalone, read-only command. Revert the MR → command removed,
schema field removed, zero effect on any existing plan. No migration.

## Corrections (2026-09-11, RNP review)

Independent review `20260911T121946Z-contribution-auditor-6pq1` found
this plan's original text incomplete in four ways, all resolved above:

- **RNP-1** (unsupported consultation attestations) — corrected
  `advisory_consultation` block.
- **RNP-3** (undefined glob/overlap/coverage contract) — the full
  "Collision-check contract" section above, plus fixtures 3, 5, 7, 8.
- **RNP-4** (deferred owner, missing manifest) — owner decided at Gate 1
  (`development-workflow-rule.md`), structured manifest added and
  validated, framework-vs-tooling distribution claim corrected.
- **RNP-5** (stale independence claim) — see below.

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
`glob_regex`-based syntax, (b) the full collision-check contract above
(glob-overlap algorithm, input sources, coverage/exit-code semantics),
(c) `agov plan collision-check` shipping **advisory**, not blocking,
(d) the 9 fixtures, (e) the rule-projection manifest (owner decided:
`development-workflow-rule.md`). Then coder-tester implements against
current `main`; own Gate 2.
