---
id: 20260913T130049Z-qbka-slice-a-superseded-plan-lifecycle
status: approved
gate1_approved_by: governor-reviewer
gate1_approved_at: 2026-09-13T13:04:13Z
topic: qbka-slice-a-provenance-bound-superseded-plan-status
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Self-assessed at drafting time: this changes only
      `agf/schemas/gate1-plan.schema.json` (a governance-artifact schema,
      not a runtime data/credential surface) and the `agov plan validate`
      Rust validator. No network access, no credential, no new trust
      boundary — matches `67kq`'s own infosec framing for this review's
      scope. Real infosec-sentinel confirmation still welcome before
      Gate 2, but no plausible finding surface was identified in
      drafting.
  - role: contribution-auditor
    status: waived
    reason: >-
      Not yet requested. This plan closely follows `67kq`'s own detailed
      recommendation (`docs/coordination/20260903T215909Z-coder-tester-67kq.md`),
      itself a contribution-auditor-style independent assessment of the
      plan corpus — a second full pass is welcome before Gate 2, focused
      on whether this plan's provenance-field design actually satisfies
      `67kq`'s four numbered requirements and whether Slice A/Slice B
      scoping is drawn in the right place.
affected_paths:
  - agf/schemas/gate1-plan.schema.json
  - tooling/rust/crates/agov/src/main.rs
  - tooling/rust/crates/agov/tests/
  - agf/templates/gate1-plan.md
  - docs/reference/session-resume.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: qbka Slice A — provenance-bound `superseded` plan status

**Gate 1: APPROVED** — `20260913T130413Z-governor-reviewer-td7j`
("gate 1 approved."). Fulfills the planning obligation from
`20260903T221643Z-coder-tester-qbka` (transcribing and extending
`20260903T215909Z-coder-tester-67kq`'s independent census and
recommendation), and clears **Row 5 S2**'s hard predecessor named in
`docs/plans/20260910T065635Z-coordination-lifecycle-semantics-work-order.md`
("S2 → qbka Slice A"). This is Slice A only — the schema, validator,
and lifecycle-semantics half. Slice B (migrating the twelve currently
invalid legacy plan files) is a **separate, later** plan, not this
one's scope (see "Not in this slice" below).

## Problem, re-verified at drafting time

`67kq`'s 2026-09-03 census found 12 invalid plan files out of 191 at
`main`. Re-run today (`agov plan validate` across all 222 current
`docs/plans/*.md` files, 31 more plans have been added since):
**still exactly 12 invalid files**, same shape as originally found:

```
docs/plans/agents-govern-2.0-session-continuity-s1.md
docs/plans/agf-coord-channel-fsm.md
docs/plans/agf-deprecated-field-detection.md
docs/plans/agf-v2-layout-s1.md
docs/plans/docs-assessment-2026-06-21.md
docs/plans/elixir-migration-e3-validators.md
docs/plans/gate1-role-authority-tooling-retarget.md
docs/plans/plan-validator-advisory-role-allowlist.md
docs/plans/rc1-sequence.md
docs/plans/release-pages-artifacts-policy.md
docs/plans/remove-elixir-tooling.md
docs/plans/validate-contributions-readme-stats.md
```

The gap has sat unaddressed for 10 days despite being named as a hard
predecessor for other approved work (Row 5 S2). This plan closes the
schema/validator half now; Slice B's actual 12-file migration follows
immediately after as its own plan, per `67kq`'s own slice split.

## Design: `superseded` as a provenance-bound terminal status

Per `67kq`'s four numbered requirements, add to
`agf/schemas/gate1-plan.schema.json`:

```json
"status": {
  "type": "string",
  "enum": ["pending-gate1", "approved", "active", "complete", "superseded"]
},
"superseded_by": {
  "type": "string",
  "minLength": 1,
  "description": "Stable identifier (plan id, or the id of the replacement work item/work order) of what replaced this plan. Declared provenance, matching gate1_approved_by's own convention -- see correction below."
},
"superseded_decision": {
  "type": "string",
  "minLength": 1,
  "description": "Filename or id of the governor-reviewer coordination decision-record authorizing this disposition. Declared provenance, matching gate1_approved_by's own convention -- see correction below."
},
"superseded_at": {
  "type": "string",
  "minLength": 1,
  "format": "date-time",
  "description": "UTC disposition timestamp (RFC3339), matching gate1_approved_at's own convention."
}
```

**Correction (2026-09-13, post-Gate-1, pre-Gate-2 — Codex independent review
`QBKA-A1`/`QBKA-A2`):** the original wording above ("Not a free-text
explanation -- a resolvable identifier"; "Only a governor-reviewer record
satisfies this") overstated the mechanical guarantee. The shipped schema
enforces a non-empty string (and, for `superseded_at`, RFC3339 `format:
date-time` — added in this correction; the original draft above only
required non-empty text and a genuinely malformed value like
`definitely-not-rfc3339` would have validated). It does **not** resolve
`superseded_by` against real plan/work-item ids, and does **not**
authority-check that `superseded_decision` actually names a real
governor-reviewer record. This was always the intended scope — "matches
the existing gate1_approved_by/gate1_approved_at convention's own
authority model" is accurate read as "same *unverified declarative
string* convention," since `gate1_approved_by`/`gate1_approved_at` are
themselves plain unverified strings in this schema, not resolved or
authority-checked either. The field descriptions now say this explicitly
("Declared provenance... does not itself resolve or verify...") instead
of implying mechanical resolution. Building real resolution/authority-
checking tooling for either field remains out of scope for Slice A; a
future plan may propose it if a real need surfaces.

Plus a top-level conditional requirement (same `if`/`then` pattern this
schema already uses for `advisory_consultation[].status: waived`):

```json
"allOf": [
  {
    "if": {
      "properties": { "status": { "const": "superseded" } },
      "required": ["status"]
    },
    "then": {
      "required": ["superseded_by", "superseded_decision", "superseded_at"]
    }
  }
]
```

This directly satisfies `67kq`'s four requirements: (1) a non-empty
stable replacement identifier (`superseded_by`); (2) the governor
decision authorizing it (`superseded_decision`); (3) a UTC disposition
timestamp (`superseded_at`); (4) the schema's own `if`/`then` plus
`agov plan validate`'s existing JSON-Schema-driven validation rejects a
`status: superseded` plan missing any of the three fields, or with any
of them empty — no code change needed in the validator beyond picking
up the new schema, since `agov plan validate` already validates against
the schema file directly (confirmed: it uses the same
`jsonschema` crate/pattern as `agov coord validate`).

## Lifecycle transition semantics (per `67kq`'s request, previously undefined)

The schema enumerates values but this project has never written down
which transitions are valid. Documenting explicitly, in
`docs/reference/session-resume.md` (the existing session-resume
reference doc) and `agf/templates/gate1-plan.md` (the plan template):

```
pending-gate1 -> approved       (governor Gate 1 decision)
pending-gate1 -> superseded     (governor decides the plan will not proceed;
                                  replaced by a different governed vehicle
                                  before ever reaching Gate 1)
approved      -> active         (implementation begins)
approved      -> superseded     (governor decides to replace an approved-
                                  but-not-yet-implemented plan)
active        -> complete       (implementation merged, Gate 2 passed)
active        -> superseded     (governor decides to abandon in-flight
                                  implementation in favor of a replacement)
complete      -> (terminal, no further transition)
superseded    -> (terminal, no further transition)
```

**Not a valid transition**: `complete -> superseded` or
`superseded -> anything`. Both `complete` and `superseded` are
terminal; once real work has landed (`complete`) it is never
retroactively relabeled as if it hadn't happened, and a `superseded`
plan's disposition is not reopened — a change of mind about a
superseded plan requires a *new* plan, not resurrecting the old one's
status field, consistent with this project's append-only coordination
discipline applied to plan lifecycle.

## Queue-consumer behavior

`67kq`: "Queue consumers must classify both as terminal while
displaying the replacement link for `superseded`." Concretely:
`agov session resume` and any other tooling that lists
pending/actionable plans (filtering on `status` not in
`{complete, superseded}` for "still open") already implicitly treats
any unrecognized/unhandled status as blocking (schema validation
rejects it before it reaches that logic) — the schema change alone
is what's needed here, since there is currently no separate Rust
enum/match statement over plan status literals in the reviewed
`agov` surfaces (plans are read for validation, not for programmatic
status-based filtering elsewhere in `main.rs` at this scope). If a
future queue feature (e.g. a `agov plan list --status` filter) is
added, it must treat `superseded` as terminal and surface
`superseded_by` — flagging this as a note for that future work, not a
requirement this plan itself must implement, since no such command
exists today to modify.

## Not in this slice

- **The twelve-file migration itself (Slice B)** — a separate plan,
  immediately following this one once Gate 1 approved here. Migrating
  the four frontmatter-less historical files in particular requires
  reconstructing their full required envelope from commit/coordination
  evidence, which `67kq` explicitly warned is real editorial work
  ("do not promote body-text labels into authority merely because they
  say 'approved'") — out of scope for a schema/validator change.
- Any new `agov plan` subcommand (e.g. a status-filtered list command)
  — noted above as a possible future consumer, not built here.
- Any change to `agf/schemas/coordination-message.schema.json` or its
  own, unrelated `supersedes`/`superseded_by` field pair (Row 5 S2) —
  that is a coordination-*message* lifecycle concept, textually similar
  but a fully separate schema and separate slice; this plan does not
  touch it, it only clears S2's *predecessor* requirement by freezing
  this plan-status shape for S2 to reuse its vocabulary from, per the
  parent work order's own instruction ("S2 ... must reuse its
  vocabulary and provenance fields verbatim").

## Test strategy

Per `67kq`'s explicit requirement ("validator tests rejecting missing
or empty provenance") and this project's own
`always/gate-self-test-rule.md` (a schema/validator change is
gate-adjacent):

- **Positive**: a plan with `status: superseded`,
  `superseded_by: <id>`, `superseded_decision: <filename>`,
  `superseded_at: <RFC3339>` all present and non-empty validates OK.
- **Negative (the gate-self-test)**: a plan with `status: superseded`
  and any one of the three provenance fields missing, or present but
  empty string, is rejected by `agov plan validate` with a clear
  schema-path error naming the missing/empty field — demonstrating the
  gate actually fires on the known-bad input it exists to catch. Three
  separate fixtures (one per missing field) plus one for all three
  missing.
- **Regression**: all four existing statuses (`pending-gate1`,
  `approved`, `active`, `complete`) continue to validate exactly as
  before — no behavior change for the 210 currently-valid plans.
- **Corpus check**: re-run `agov plan validate` across all
  `docs/plans/*.md` after this change lands; the same 12 files remain
  invalid (expected — Slice B fixes them, not this plan), and no
  previously-valid file becomes invalid.

## Migration / impact

Per `always/downstream-impact-rule.md`: this touches
`agf/schemas/*.json`, a trigger path. Classification: **ADDITIVE** — a
new optional enum value plus three new optional (conditionally
required only when that new value is used) fields; no existing status
value's meaning or requirements change, no existing valid plan becomes
invalid. No migration needed for the 210 already-valid plans.

## Rollback

Purely additive schema change. Reverting drops the `superseded` enum
value and its three fields; any plan already using `status: superseded`
by that point would need Slice B-style remediation again — acceptable
since rollback of an accepted, in-use terminal status is not expected
to be a normal operation, same risk profile as reverting any other
additive schema field in this project's other recent plans (CP-B-2,
Row 5 S3).
