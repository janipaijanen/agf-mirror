---
id: 20260911T040631Z-row6-s3-issue123-affected-paths-collision-check
status: pending-gate1
topic: row6-s3-issue123-affected-paths-collision-check
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Adds an optional `affected_paths` array to `gate1-plan.schema.json`
      and a new read-only `agov plan collision-check` command that reads
      plan files and open-MR changed-path metadata (via the forge API or a
      local git diff) to report overlaps. InfoSec confirms this command
      is read-only (no plan/MR mutation), makes no forge write calls, and
      the new schema field cannot be used to smuggle content outside the
      plan-file structure (it is a bounded array of path-glob strings,
      schema-validated).
  - role: contribution-auditor
    status: consulted
    reason: >-
      Bounded per the row-6 work order's scope guard. Confirms `affected_
      paths` is optional and additive (a plan with only prose "affected
      files" remains valid), that `collision-check` ships **advisory**
      (report + non-zero exit for CI visibility) rather than MR-blocking
      in this slice, and that promoting it to blocking is a separate,
      gate-self-tested decision — not implied by this slice.
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
| `agf/schemas/gate1-plan.schema.json` | New **optional** `affected_paths` property: array of strings, each a path or a glob (e.g. `agf/schemas/coordination-message.schema.json`, `tooling/rust/crates/agov/src/main.rs`). Absent = current prose-only plans remain fully valid (backward compatible). No `required` addition. |
| `tooling/rust` new `agov plan collision-check [--base-ref <ref>]` | Reads every plan file with `status: pending-gate1` or `status: approved` that declares `affected_paths`, and the changed-path set of every open MR (via the forge API, or `git diff --name-only <merge-base>...<mr-head>` for a locally-fetched MR branch). Reports every pair whose `affected_paths` (or MR changed-paths) intersect, naming both plans/MRs and the shared path(s). **Advisory**: non-zero exit when an overlap is found (for CI visibility), no merge block in this slice. |
| `agov plan validate` | Unaffected — a plan with no `affected_paths` still validates exactly as today. A plan that declares `affected_paths` gets a light shape check (array of non-empty strings) as part of the existing schema validation, no new blocking behaviour beyond that. |
| `AGENTS.md` "Gate discipline" + `agf/templates/gate1-plan.md` | One-line pointer: a plan MAY declare `affected_paths` for machine-checkable overlap detection; `agov plan collision-check` reports (does not block) overlaps. |

## Explicitly out of scope (parent work order guard)

No MR-blocking gate in this slice — promoting `collision-check` to
blocking is a separate, `gate-self-test-rule`-covered decision requiring
its own evidence and negative test, not implied here. No requirement that
every plan retroactively adds `affected_paths` — prose-only plans stay
valid indefinitely.

## Not a gate (this slice) — required fixtures instead

1. **Backward compatible.** Every currently-approved plan (none of which
   declare `affected_paths`) still passes `agov plan validate` unchanged.
2. **Overlap detected.** Two synthetic plans declaring an overlapping
   `affected_paths` entry → `collision-check` reports both, names the
   shared path, non-zero exit.
3. **No overlap, clean.** Two plans with disjoint `affected_paths` →
   zero exit, no report.
4. **MR-side overlap.** A plan's `affected_paths` intersects an open MR's
   actual changed-path set → reported the same way as plan-vs-plan.
5. **Malformed `affected_paths` (non-string entry, empty array item)** →
   schema validation fails closed with a clear error, not silently
   ignored.

If a later decision promotes `collision-check` to MR-blocking, that
decision's own Gate 1 supplies the `gate-self-test-rule` negative test
(a genuine collision still blocks after the change) — not this slice.

## Rule-projection screen

`docs/reviews/rule-projection/20260911T040631Z-row6-s3-issue123-affected-paths-collision-check.yaml`
— normative owner: `always/uncertainty-and-scope-rule.md` (`updated` —
its "list every module that will change" prose gains a structured,
optional machine-checkable form) or `development-workflow-rule.md`'s
Phase 2 plan-file guidance (`existing pointer sufficient` if the reviewer
judges the uncertainty-scope rule is the better single owner — pick one,
not both); adopter explanation: `not applicable` (internal governance
tooling, no adopter-facing Pages claim); runbook: `not applicable`;
AI loaders: `AGENTS.md` + `agf/templates/gate1-plan.md` (`updated`,
one-line pointer), `.claude/CLAUDE.md` `not applicable`; validation/drift:
`advisory only` (the command itself, non-blocking); distribution: schema
+ `agov` + the plan template ship in the AGF artifact.

## Rollback

`affected_paths` is optional-additive on the schema; `collision-check` is
a new, standalone, read-only command. Revert the MR → command removed,
schema field removed, zero effect on any existing plan. No migration.

## Decision requested at Gate 1

Approve (a) the optional `affected_paths` schema field, (b) `agov plan
collision-check` shipping **advisory**, not blocking, (c) the 5 fixtures,
(d) the rule-projection screen (owner chosen at implementation between
the two named candidates). Then coder-tester implements; own Gate 2.
