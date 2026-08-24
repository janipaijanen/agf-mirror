---
id: 20260824T201645Z-review-convergence-abstraction-reset
status: approved
topic: review-convergence-abstraction-reset-rule
gate1_approved_by: governor-reviewer
gate1_approved_at: "2026-08-24T20:58:22Z"
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Independent consultation 20260824T200711Z-contribution-auditor-mmrh
      reviewed the frozen design COMPLETE / CLEAR with three advisory
      refinements. This plan incorporates all three: per-invariant counter
      keys and closure states, deterministic duplicate/conflict handling, and
      a closed checkable materiality basis for blocking reviews.
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Consolidated consultation request
      20260824T195950Z-planner-architect-dj1n is published. The governor's
      Gate 1 decision 20260824T205822Z-governor-reviewer-8274, clarified by
      20260824T210018Z-governor-reviewer-tjh5, explicitly waives the pending
      advisory response as a Gate 1 blocker; it may still advise implementation.
  - role: build-engineer
    status: consulted
    reason: >-
      Consolidated consultation request
      20260824T195950Z-planner-architect-dj1n is published. The governor's
      Gate 1 decision 20260824T205822Z-governor-reviewer-8274, clarified by
      20260824T210018Z-governor-reviewer-tjh5, explicitly waives the pending
      advisory response as a Gate 1 blocker; it may still advise implementation.
---

# Plan: bounded review convergence and mandatory abstraction reset

## Decision requested

Approve one workflow rule and one small optional coordination data surface that
stop repeated same-invariant correction loops without weakening material review.
After two rejected semantic candidates for the same invariant, case-by-case
patching stops, one abstraction reset is mandatory, and one redesigned
candidate gets one targeted verification. A remaining material block returns
once to the governor for accept, re-scope, defer, or stop.

This is a Gate 1 plan only. It does not authorize implementation, approve either
gate, change a role, or merge, release, tag, or deploy anything.

## Evidence and problem boundary

MR !1847 demonstrated the costly failure mode: repeated corrections refined a
permissive symlink algorithm while another member of the same containment bug
class remained. Governor decision `20260824T185045Z-governor-reviewer-a86h`
replaced the edge-case sequence with a smaller fail-closed invariant. That
removed the semantic bug class; later CI failures were history-availability
failures, not new semantic attempts.

The inverse risk is also real: a targeted reviewer can keep adding speculative
completeness conditions that are unrelated to the approved acceptance boundary.
The new rule therefore couples an abstraction-reset trigger with a closed
materiality boundary. It does not trade correctness for speed.

Evidence status: empirically informed by the repository's repeated correction
series, especially MRs !1847 and !1965. Capability maturity remains Level 1
Planned until the rule, schema, tooling, tests, and operator documentation merge.

## Normative state machine

Counters are keyed by `(series_id, invariant_id)`, never by MR, branch, review
count, or pipeline count.

| State | Allowed event | Result |
|---|---|---|
| Open, no failed candidate | semantic candidate, then targeted review | CLEAR closes that invariant; BLOCK records failure 1 |
| One consecutive same-invariant failure | one consolidated semantic candidate, then targeted review | CLEAR closes; BLOCK records failure 2 and requires reset |
| Reset required | abstraction-reset disposition only | selects redesign, risk acceptance request, deferral, or stop |
| Reset recorded | one redesigned semantic candidate, then one targeted review | CLEAR closes; BLOCK requires governor return |
| Governor return required | governor disposition only | accept, re-scope into a new series, defer, or stop |
| Closed | none in the old series | later work uses a new series id |

A CLEAR, governor stop, accepted disposition, or deferral closes the keyed
series. Re-scope closes the old series and creates a new id so old failures do
not leak into unrelated work.

## What counts as a semantic attempt

A semantic candidate counts only when its declared governed-scope digest
changes. The candidate records exact paths, exact commit SHA, and a stable
SHA-256 digest over sorted `(path, NUL, bytes)` entries. The digest, not commit
count, distinguishes semantic work from ancestry changes.

These never consume the convergence budget:

- a rebase or merge-base refresh with an unchanged governed-scope digest;
- a coordination-publication-only commit;
- a CI retry or an infrastructure-only pipeline failure;
- an exact duplicate review of the same candidate event; or
- an administrative branch, title, description, label, or assignment change.

Different invariant ids maintain separate counters. A correction that resolves
several previously named findings may declare several invariant ids in one
candidate event; each targeted review outcome updates only the ids it names.

## Mandatory abstraction-reset disposition

After failure 2, the owner must stop example-by-example correction and compare:

1. a simpler invariant or smaller state space;
2. a safe fail-closed restriction;
3. a proven library or platform primitive;
4. explicit risk acceptance through existing authority; and
5. deferral or stop.

The record names the shared cause, explains why prior candidates missed it,
selects one option, and states false negatives, compatibility effects, approved
use cases removed or preserved, and operator impact. Fail-closed is preferred
when safe and usable, but it cannot silently remove an approved use case.
Behavior or risk changes require governor disposition before the redesigned
candidate begins.

## Closed blocker-materiality boundary

A targeted review may block only with concrete exact-candidate evidence in one
of these categories:

1. violation of a named mandatory acceptance criterion;
2. violation of a named security, containment, privacy, or data-integrity rule;
3. violation of an explicit compatibility or operator-safety promise; or
4. absence or invalidity of a mechanically required governed artifact, with
   the governing rule or approved-scope reference cited.

A blocking review event must name the invariant, category, authority reference,
candidate event, exact SHA and scope digest, and concrete evidence. Speculative
completeness, style, optional hardening, and edge cases without plausible
material consequence are advisory or backlog evidence. They do not expand the
active correction scope.

This rule cannot waive a security boundary. Risk acceptance follows the
authority already granted by the boundary's governing rule; where that rule
does not grant waiver authority, the finding remains blocking.

## Structured coordination surface

Add an optional backward-compatible `convergence` object to
`agf/schemas/coordination-message.schema.json`:

- `series_id`: stable readable coordination id that opened the series;
- `event`: `candidate`, `targeted-review`, `abstraction-reset`, or
  `governor-disposition`;
- `attempt`: positive semantic-attempt integer;
- `invariant_ids`: non-empty unique string array;
- `candidate_sha`: 40-character Git SHA for candidate and review events;
- `scope_paths`: sorted unique repository-relative paths for candidate and
  review events;
- `scope_digest`: lowercase SHA-256 for candidate and review events;
- `candidate_event`: referenced coordination id for targeted reviews;
- `outcome`: `clear`, `block`, or `advisory` for targeted reviews;
- `materiality_basis`: one of the four closed categories for BLOCK;
- `authority_ref` and `evidence_refs`: required for BLOCK;
- `disposition`: `redesign`, `accept`, `rescope`, `defer`, or `stop` for reset
  and governor events as applicable.

Schema conditionals enforce event-specific presence. Existing coordination
records remain valid because the object is optional. Historical records are not
rewritten or retroactively inferred.

## Deterministic tool behavior

Add `agov coord convergence --series <id> --format text|json`. It reads valid
coordination records and reports each invariant's current state, consecutive
semantic failures, required next event, canonical candidate, and any conflict.

Canonical review identity is
`(series_id, invariant_id, attempt, candidate_event, candidate_sha,
scope_digest)`. Exact duplicates count once. Conflicting CLEAR/BLOCK outcomes
for the same identity pause the series and route once to the governor or an
already-authorized resolver; the tool never chooses which review is correct.

`agov coord validate` structurally validates the object. The convergence
command validates sequence and arithmetic. It does not infer materiality,
classify a diff as semantic, approve a gate, accept risk, or merge. The new rule
explicitly grants blocking authority only to malformed declared convergence
records and invalid state transitions in a series that opts into this surface.
All other tool output is advisory under `tool-authority-boundary-rule.md`.

## Implementation scope

- `agf/rules/workflow/review-convergence-rule.md` — normative trigger,
  materiality boundary, state machine, authority, and escalation.
- `agf/rules/workflow/systematic-code-review-rule.md` — point targeted reviews
  to the new materiality and convergence contract.
- `agf/rules/README.md` — list and map the new rule.
- `agf/schemas/coordination-message.schema.json` — optional structured object
  and event-specific conditions.
- `tooling/rust/crates/agov/src/main.rs` — parser, validation, status command,
  text/JSON output, and deterministic conflict handling.
- `tooling/rust/crates/agov/tests/coord_convergence.rs` — positive, negative,
  sequence, duplicate, conflict, and output-contract tests.
- `agf/docs/site/reference/tooling-conformance.md` — supported command surface.
- `agf/docs/site/reference/review-findings.md` — operator examples and the
  distinction between blocking and advisory findings.
- `docs/reviews/rule-projection/20260824T201645Z-review-convergence-abstraction-reset.yaml`
  — six-row normative projection manifest.
- `CHANGELOG.md` — user-visible rule/schema/tool addition.

No priority field, general scheduler, MR bot, CI fetch-depth change, historical
record migration, or MR !1847 implementation change is in scope.

## Acceptance evidence

Focused fixtures must prove:

1. two BLOCK outcomes for the same invariant require reset;
2. different invariants do not combine counters;
3. unchanged-digest rebases, publications, retries, duplicate reviews, and
   infrastructure failures do not count;
4. a post-reset CLEAR closes the series;
5. a post-reset BLOCK requires one governor return;
6. conflicting same-candidate outcomes pause without tool-selected resolution;
7. every BLOCK requires a closed materiality category, authority, and evidence;
8. security findings remain material and cannot be silently waived;
9. old coordination records without `convergence` remain valid; and
10. text and JSON output report the same states and counts.

Gate 2 evidence must include focused tests, full Rust workspace tests,
`agov coord validate` positive/negative transcripts, `agov validate`, the
rule-projection check, exact-head green CI, and confirmation that only the files
listed above changed.

## Risks and rollback

- **False convergence:** overly narrow invariant ids could split one bug class.
  Reviewers may block a misleading id under the named acceptance/security
  category; the abstraction-reset record must state the shared cause.
- **Process gaming:** owners declare whether a digest change is semantic.
  Review verifies declared scope paths and digest; tooling checks bytes and
  sequence but does not decide intent.
- **Security dilution:** the materiality boundary could be misread as waiver.
  The rule explicitly preserves existing security-rule authority.
- **Schema/tool coupling:** old binaries ignore the optional field, while new
  binaries validate it. Documentation labels the capability Level 4 only after
  tests and Level 5 only when shipped in a release artifact.

Rollback removes the command and optional schema object only through a normal
reviewed change; already published records remain readable as Markdown audit
evidence. The normative materiality and escalation decisions require an
explicit replacement rule rather than silent deletion.

## Gate 1 readiness

Gate 1 was approved by the governor-reviewer in
`20260824T205822Z-governor-reviewer-8274`, with the pending advisory-response
hold explicitly waived in `20260824T210018Z-governor-reviewer-tjh5`.
Implementation may proceed within this plan's approved scope. Later infosec or
build advice remains subject to its existing rule authority; the waiver does
not let advisory tooling or roles approve gates or silently expand scope.
