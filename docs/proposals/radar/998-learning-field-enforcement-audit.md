<!-- Radar carrier. Retained verbatim from MR !998 (source branch plan/learning-field-enforcement-audit, head 945c335f), file docs/plans/learning-field-enforcement-audit.md. Front-matter status was pending-gate1 and is not an approval.
     Non-authoritative: parked on the radar, grants no Gate 1, rule, template, validator or implementation authority. -->
---
id: learning-field-enforcement-audit
status: pending-gate1
topic: learning-schema-enum-enforcement-audit
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: contribution-auditor
    status: waived
    reason: >-
      Collaborative first draft. A coordination handoff requesting
      independent review has not been filed yet for this specific plan —
      unlike its sibling `learning-supersession-status-fields.md`
      (MR !995/!996), this one was drafted on explicit request to produce
      the plan first; whether and when to signal Codex is a separate,
      not-yet-taken step. Marking "waived" rather than "consulted" is the
      honest statement that nothing here has been audited yet.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      No PII, secrets, or access-control handling touched. This is
      validation-strictness for classification fields already public
      within the project's own corpus.
  - role: fact-checker
    status: not-applicable
    reason: >-
      Every count and value below is reproducible by running the grep/
      python commands cited against this project's own
      `agf/schemas/learning.schema.json` and `agf/learnings/foundry.yaml`
      — no external or comparative claim requiring independent sourcing.
---

# Plan: enforcement audit for learning-record `category`/`severity`/`scope`/`confidence.level`

**Date:** 2026-08-03T07:21:33Z (drafted).
**Status:** DRAFT — audit findings verified against the live schema and
corpus; the decision (which values to enforce, how, and whether to widen
the documented convention first) is explicitly left open for Gate 1.

## Why this document exists

`docs/plans/nasa-llis-lessons-learned-comparison.md`'s candidate 5 named,
but did not investigate, an enforcement-stance question: AGF's learning
schema documents `category`, `severity`, and `scope` as having
"conventional values" that the validator does not enforce, and
`confidence.level` as an enforced four-tier ladder whose top tier has no
evidence threshold. That candidate explicitly warned: "adding a new field
on top of an unresolved enforcement question compounds it" — this plan is
the audit that question calls for, done before any of the sibling
`learning-supersession-status-fields.md` plan's new fields (`status`,
`supersedes`) are added on top of it.

## Verified current state

All findings below were reproduced against the live repository
immediately before drafting this plan (`agf/schemas/learning.schema.json`,
`agf/schemas/invariants.yaml`, `agf/learnings/foundry.yaml`, 44 entries)
— not carried over from the LLIS comparison's own findings without
re-checking.

**E1 — the schema is more permissive than "some enums are conventional."**
The `learnings` array item schema has no `required` list at all (nothing
is mandatory) and does not set `additionalProperties: false` (any field
name not in the documented list is silently accepted). This is broader
than the enum-enforcement question alone.

**E2 — `category`: 30% of categorized entries use undocumented values.**
Documented convention: `gap | overkill | validation | adaptation |
tension`. Observed across 44 entries: `gap`(22), `process`(10),
`validation`(6), `adaptation`(3), `improvement`(1), `bug`(1),
`boundary`(1). Four of the seven observed values (`process`,
`improvement`, `bug`, `boundary` — 13 entries, ~30%) are not in the
documented list; two documented values (`overkill`, `tension`) are never
used at all.

**E3 — `severity`: one undocumented value, one literal typo.** Documented:
`critical | significant | minor | informational`. Observed: `significant`
(17), `minor`(2), `moderate`(1), `informational.`(1, with a trailing
period). `critical` is never used. `moderate` is not documented anywhere.
The `informational.` occurrence is a data-entry typo, not a new value —
located at `agf/learnings/foundry.yaml:796`, entry
`20260624T065744Z-k7m2`. This typo is concrete evidence of what
"documented but not enforced" costs in practice: it has sat unnoticed in
the corpus.

**E4 — `scope`: only one of two documented values has ever been used.**
Documented: `framework | project`. Observed: `framework`, 25 out of 25
occurrences. `project` has never appeared. Enforcing this specific enum
today would reject nothing in the existing corpus.

**E5 — `confidence.level`: the enforced ladder's upper tiers are
untested by real data.** All 27 entries carrying a `confidence` block use
`single-adopter`. `multi-adopter`, `longitudinal`, and
`industry-corroborated` have never been used. The
`learnings.confidence_corroboration` invariant (evidence_count >= 2
required for `multi-adopter`/`longitudinal`) is schema-enforced but has
never actually been exercised against a real entry in this project's own
corpus.

**E6 — the corpus contains two disjoint entry shapes, not one schema with
optional fields.** Cross-tabulated field presence across all 44 entries:
22 entries have `title` and not `summary`; 21 entries have `summary` and
not `title`; exactly 1 entry
(`20260625T063214Z-dpd3`) has both. The `title`-shape entries pair with
`description`/`business_impact` (the schema-documented shape); the
`summary`-shape entries pair with `detail`/`framework_impact` (a
different, schema-undocumented shape). Since E1 means nothing is
required and extra properties are silently accepted, both shapes
validate today with zero warning that they disagree — this is a larger
finding than the enum question alone, and blocks any straightforward
"just add `enum`" fix to E2–E4 without first deciding what to do about
the ~48% of the corpus using the undocumented shape.

**E7 — `status` is out of scope here.** Covered by the sibling
`learning-supersession-status-fields.md` plan (MR !995); not duplicated
in this plan's scope.

## Decision requested

Not "should these become enforced enums" as a yes/no — the audit above
shows that question can't be answered field-by-field in isolation. Three
separable decisions, in dependency order:

1. **E6 first: what is the status of the `summary`/`detail`/
   `framework_impact` shape?** Is it a superseded historical shape (in
   which case: was there ever a migration plan for the ~21 entries still
   using it, and did it silently not happen — an instance of the ripple-
   check gap `migration-generation-rule.md` warns about for schema
   changes generally, applied here to a schema-shape change instead of a
   database migration)? Or is it a still-valid alternate shape that the
   schema should formally document alongside the `title`-shape? Enforcing
   `severity`/`category` as required fields without resolving this would
   immediately fail roughly half the corpus.
2. **Per-field enum decision, after E6:** for `category` and `severity`,
   classify each currently-observed non-documented value as (a) a
   legitimate addition to the convention (used more than once, distinct
   meaning — `process` at 10 uses is the clearest candidate) versus (b) a
   one-off or typo to fix in the data rather than encode in the schema
   (`moderate`, `informational.`, `improvement`, `bug`, `boundary` at 1
   use each need individual judgment, not a blanket rule). `scope` is the
   simplest case (E4): enforcing `framework | project` today changes
   nothing about the existing corpus.
3. **Enforcement mechanism, after 1–2:** hard `enum` rejection (matching
   `confidence.level`'s existing pattern) versus an advisory/warning-level
   check (matching `learnings.confidence_corroboration`'s severity split
   between the enum itself, which is `error`, and the evidence-count
   threshold, which only applies to two of four tiers) versus leaving
   specific fields as documented-only convention if the audit concludes
   enforcement isn't worth the cost for that field.

## Scope

- `agf/schemas/learning.schema.json` — resolve E6 (document or deprecate
  the second shape), then apply whatever enum decision Gate 1 makes for
  `category`/`severity`/`scope`.
- `agf/schemas/invariants.yaml` — add invariant entries for any field
  that gains enforcement, mirroring the existing
  `learnings.confidence_level_enum` shape.
- `tooling/rust/crates/agov-validators/src/learnings.rs` — implement
  whatever enforcement Gate 1 decides; add positive/negative tests per
  `always/gate-self-test-rule.md` for any new enforced check.
- `agf/learnings/foundry.yaml` — the `informational.` typo (E3) is a
  one-character data fix, called out separately below since it touches
  existing entries' content, not additive schema surface.

**Explicitly not in scope:** `status`, `supersedes`, `superseded_by` —
owned by the sibling `learning-supersession-status-fields.md` plan
(MR !995). The two plans are deliberately kept separate so each can be
reviewed against its own yes/no question, per
`workflow/branch-and-worktree-rule.md`'s sizing test.

## The `informational.` typo — a fix, not a finding

Correcting `agf/learnings/foundry.yaml:796` (`informational.` →
`informational`) is trivial in size but is a change to an existing
entry's recorded content within an adopter-owned governance file (per
`always/governance-file-layout-rule.md`, `agf/learnings/` is
adopter-owned). Unlike coordination records, learning entries are not
individually immutable-by-design — but a silent drive-by edit to
historical content is still worth flagging explicitly rather than folded
into an unrelated commit. If Gate 1 approves this plan, the typo fix
should land as its own clearly-labeled one-line change, not bundled
invisibly into a larger enforcement commit.

## Migration and SemVer classification

Per `always/downstream-impact-rule.md`, this touches `agf/schemas/*.json`
(non-exempt trigger path). The classification depends on the Gate 1
decision:

- If any adopted enum is the **union** of documented + legitimately-
  observed values (e.g. `category` gains `process` but keeps
  `gap|overkill|validation|adaptation|tension`), the change is
  **Additive** — nothing currently valid becomes invalid.
- If any adopted enum **narrows** to only the originally-documented five
  values, rejecting `process`/`improvement`/`bug`/`boundary` (13 entries),
  the change is **Breaking** against this project's own corpus and would
  need the same Phase 0 disclosure treatment
  (`workflow/deprecation-policy-rule.md`) as the sibling plan, or those
  13 entries would need fixing before the enum lands.
- **Recommendation, not yet a decision:** prefer the union approach for
  any field where the non-documented value has more than one occurrence,
  to avoid manufacturing an unforced breaking change against a corpus
  this project itself produced.

## Risks

- **Scope size.** Four fields plus a two-shape corpus question is a lot
  of surface for a single plan/branch. May need splitting into child
  plans per field (E6 resolution first, then category, then severity,
  then scope/confidence follow-ups) once Gate 1 has weighed in on
  sequencing — flagged here rather than pre-emptively split, since the
  right split boundary depends on the E6 decision.
- **E6 may be a bigger finding than an enforcement-audit plan is meant to
  carry.** If it turns out a schema migration was planned for the
  `summary`/`detail` shape and simply never executed, that's closer to a
  learning-record-worthy gap in its own right than a schema tweak — worth
  naming as a possible fork in this plan's own path rather than assuming
  it resolves as a two-line schema edit.

## Acceptance criteria

1. Gate 1 resolves the E6 shape question (documented as approved, or
   deferred to its own explicitly-named follow-up).
2. Gate 1 approves, per field, which observed values join the enforced
   set and which are treated as one-off data fixes.
3. Gate 1 approves the enforcement mechanism (hard enum vs. advisory) per
   field.
4. `cargo test --workspace` passes with any new/updated validator tests,
   including at least one negative test per newly-enforced field per
   `always/gate-self-test-rule.md`.
5. Every existing entry in `agf/learnings/foundry.yaml` validates cleanly
   after the change, or the plan's Breaking-change disclosure (see SemVer
   section) is completed for any entry that does not.

## Stop conditions

- Do not implement any enforcement change before Gate 1 resolves E6 and
  the per-field union/narrow decision — implementing enum enforcement
  first and discovering the corpus shape mismatch second is the exact
  ordering this plan exists to avoid.

## Related documents

- `docs/plans/nasa-llis-lessons-learned-comparison.md` — candidate 5, the
  origin of this audit.
- `docs/plans/learning-supersession-status-fields.md` (MR !995) — sibling
  plan; `status`/`supersedes`/`superseded_by` are explicitly out of this
  plan's scope.
- `agf/schemas/learning.schema.json`, `agf/schemas/invariants.yaml`,
  `tooling/rust/crates/agov-validators/src/learnings.rs`,
  `agf/learnings/foundry.yaml` — the files this audit's findings and any
  resulting implementation would touch.
- `workflow/migration-generation-rule.md` — cited by analogy in E6/Risks
  for the ripple-check discipline a schema-shape change of this kind
  needs, even though that rule is written for database migrations.
