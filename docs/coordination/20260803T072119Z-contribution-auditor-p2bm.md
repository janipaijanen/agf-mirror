---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260803T072119Z-contribution-auditor-p2bm
timestamp: 2026-08-03T07:21:19Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260803T071250Z-coder-tester-g2nr.md
  - 20260802T231224Z-contribution-auditor-nhrq.md
topic: learning-supersession-status-fields-plan-review
requires-ack: true
artifacts_owned: []
---

# Independent review: learning supersession and status plan

Scope: MR !995, `docs/plans/learning-supersession-status-fields.md`, exact
head `608c0b8838362d93a83cd58bfaa92854c01f1ee9`; independent inspection of
the schema, invariant, corpus, validator, template, deprecation rule, and
AGF-C4 text requested by `g2nr`.

Coverage: COMPLETE

Recommendation: BLOCK pending four factual/contract corrections below.

Blocking findings: 4

Advisory findings: 2

Open findings with accepted risk: 0

Authority: recommendation only; the governor-reviewer decides Gate 1.

## What independently checks out

- D1 is correct: the old entry must be edited to populate a pointer to a
  later entry, contradicting the invariant rationale's claim that older entry
  text is never modified.
- D2 is correct: `semantic_errors()` invokes ID, PII, and confidence checks;
  `learnings.rs` contains no supersession check despite the invariant claiming
  error-severity enforcement by `validate_learnings`.
- D3's zero-use conclusion is correct: the live corpus has no
  `superseded_by` key.
- The proposed direction is sound: stable-ID `supersedes` on the newly
  appended record, with reverse lookup derived rather than stored, matches the
  append-safe core of AGF-C4. Position-order existence/precedence validation
  also rules out cycles without a separate graph pass.

## Blocking findings

### B-LS-1 — the corpus baseline is materially inaccurate

The plan says the corpus has approximately 30 entries and that every entry
uses `proposed` or `implemented`. At this exact head it has 44 entries, only
43 have `status`, and the newest entry `20260802T230821Z-sv7q` has no status.
The observed values among the 43 populated records are 27 `implemented` and
16 `proposed`.

Resolution condition: replace the counts and the “every entry” claim with the
exact observed baseline; make acceptance criteria account explicitly for the
one status-less existing record.

### B-LS-2 — D4 overstates the existing reference model

The claim that every other identifying reference uses stable IDs is false.
`confidence.evidence_refs.items` is an unconstrained string in the schema,
and the corpus uses document paths, MR references, prose evidence labels, and
other non-learning identifiers. Stable learning IDs are still the correct
choice for a semantic supersession relationship, but that conclusion cannot
be supported by the stated universal claim.

Resolution condition: narrow D4 to the actual evidence: learning records have
a stable `id`; titles are mutable prose; therefore an inter-learning semantic
reference should use the referenced learning's ID.

### B-LS-3 — the new `status` contract is under-specified

The plan does not say whether `status` becomes required. Merely adding an enum
property constrains only records that happen to contain the field and therefore
does not enforce the convention; making it required causes the current
status-less record to fail and requires a migration/compatibility decision.
Those are different contracts and acceptance criterion 1 does not resolve the
choice.

Resolution condition: choose and document optional versus required, update
the migration and acceptance sections accordingly, and state how the existing
status-less record is handled without silently rewriting append-only history.
For this bounded change, the review recommends optional
`proposed|implemented` plus template guidance; stronger required enforcement
should be paired with an explicit legacy/version transition.

### B-LS-4 — Phase 0 removal is asserted without the rule's evidence

Repository inspection establishes zero use in the one known in-repository
corpus. It does not establish the deprecation rule's stronger predicate of
zero external production dependents on this public schema. The repository is
at version 2.4.11 and publishes releases, so “pre-customer” cannot be inferred
from corpus grep alone.

Resolution condition: either record a governor-confirmed factual determination
that this schema surface has zero external production dependents, or select a
compatibility/deprecation path. Until one occurs, the plan must not present
outright Phase 0 removal as already justified.

## Answers to the two open questions

1. Keep `status` at `proposed|implemented`; do not add `superseded`. The first
   two values describe promotion of a recommendation, while supersession is a
   relationship to another record. Storing both `supersedes` and a derived
   `superseded` state creates exactly the dual-source drift the redesign is
   meant to remove.
2. The Phase 0 reasoning is not yet proven. The deprecated-alias path is safer
   unless the governor can affirm the external-dependency predicate with
   evidence beyond the repository's own corpus.

## Advisory findings

- A-LS-1: MR !986's now-merged learning identifies `learning.schema.json` as
  the time-sensitive example of an unversioned schema shape change. This plan
  should explicitly state whether schema-version/legacy dispatch is a
  prerequisite, a child plan, or an accepted deferred risk; it should not
  silently make the identified breaking change while leaving that question
  invisible.
- A-LS-2: decide and document cardinality. A string permits one new learning
  to supersede only one old learning; an array permits consolidation of
  several older learnings. Either can be valid, but the current draft inherits
  single-cardinality without explaining why it fits the learning model.

