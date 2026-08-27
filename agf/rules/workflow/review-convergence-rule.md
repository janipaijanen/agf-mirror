<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Review Convergence and Abstraction Reset Rule

Repeated review must converge on an approved invariant rather than grow an
unbounded sequence of case-specific patches. This rule applies only when a
series opts into the `convergence` coordination surface. It does not weaken any
security boundary or grant tooling, reviewers, or handoffs new gate authority.

## Series and invariant identity

Convergence counters MUST be keyed by `(series_id, invariant_id)`. They MUST
NOT be keyed by merge request, branch, review count, pipeline count, reviewer,
or model. A misleadingly narrow invariant identifier that splits one shared
failure cause is a material review finding under the applicable acceptance or
security boundary.

A series closes for an invariant after CLEAR, an authorized acceptance,
deferral, stop, or re-scope. Re-scope closes the old series and creates a new
series identifier. Old failures MUST NOT leak into unrelated work.

## Bounded state machine

The following transition contract is blocking for every opted-in series:

| Current state | Allowed event | Result |
|---|---|---|
| Open, no failed candidate | semantic candidate followed by targeted review | CLEAR closes; BLOCK records failure 1 |
| One consecutive same-invariant failure | one consolidated semantic candidate followed by targeted review | CLEAR closes; BLOCK records failure 2 and requires reset |
| Reset required | abstraction-reset disposition only | choose redesign, authorized acceptance, deferral, or stop |
| Reset recorded with redesign | one redesigned semantic candidate followed by one targeted review | CLEAR closes; BLOCK requires governor return |
| Governor return required | governor disposition only | accept, re-scope, defer, or stop |
| Closed | no event in the old series | later work uses a new series identifier |

After failure 2, the owner MUST stop example-by-example correction. The reset
MUST compare:

1. a simpler invariant or smaller state space;
2. a safe fail-closed restriction;
3. a proven library or platform primitive;
4. explicit risk acceptance through authority already granted by the
   governing boundary; and
5. deferral or stop.

The reset record MUST name the shared cause, explain why the earlier candidates
missed it, select a disposition, and state false negatives, compatibility
effects, approved use cases removed or preserved, and operator impact.
Fail-closed behavior is preferred when safe and usable, but it MUST NOT silently
remove an approved use case. A behavior or risk change requires the applicable
governor disposition before the redesigned candidate begins.

## Semantic-attempt boundary

A candidate counts as semantic only when its declared governed-scope digest
changes. The candidate MUST record sorted unique repository-relative paths, an
exact 40-character Git commit SHA, and lowercase SHA-256 over each sorted
`(path, NUL, bytes)` entry.

These events MUST NOT consume a semantic attempt:

- an unchanged-digest rebase or merge-base refresh;
- a coordination-publication-only commit;
- a CI retry or infrastructure-only failure;
- an exact duplicate review of the same candidate identity; or
- an administrative branch, title, description, label, or assignment change.

Different invariants retain independent counters. One candidate may name
several previously declared invariants; each review updates only the invariants
it names.

## Closed blocker-materiality boundary

A targeted review may block only when exact-candidate evidence establishes one
of these categories:

1. violation of a named mandatory acceptance criterion;
2. violation of a named security, containment, privacy, or data-integrity rule;
3. violation of an explicit compatibility or operator-safety promise; or
4. absence or invalidity of a mechanically required governed artifact, with
   the governing rule or approved-scope reference cited.

Every BLOCK event MUST name the invariant, category, authority reference,
candidate event, exact SHA, scope digest, and concrete evidence. Speculative
completeness, style, optional hardening, and edge cases without plausible
material consequence are advisory or backlog evidence and MUST NOT expand the
active correction scope.

This boundary cannot waive security. Risk acceptance follows only authority
already granted by the rule that owns the boundary. If that rule grants no
waiver authority, the finding remains blocking.

## Duplicate and conflict handling

Canonical review identity is `(series_id, invariant_id, attempt,
candidate_event, candidate_sha, scope_digest)`. Exact duplicate outcomes count
once. Conflicting CLEAR and BLOCK outcomes for one canonical identity pause the
series. The conflict MUST route once to the governor or another resolver
already authorized by the governing rule; tooling MUST NOT choose a winner.

## Tool authority

`agov coord validate` may block malformed declared `convergence` objects.
`agov coord convergence` may block invalid transitions or arithmetic in a
series that opted into this surface. The command verifies declared commit
bytes, ordering, identity, duplicates, conflicts, and state transitions.

The command does not determine whether a diff is semantic, infer materiality,
classify a security boundary, accept risk, approve Gate 1 or Gate 2, authorize
scope, or merge. Output outside the two structural checks above is advisory
under `always/tool-authority-boundary-rule.md`.

## Required delivery evidence

Gate 2 evidence for an implementation of this rule MUST cover the positive,
negative, duplicate, conflict, unchanged-digest, multi-invariant, reset,
governor-return, schema, text-output, and JSON-output cases. Historical
coordination records remain immutable and MUST NOT be migrated or inferred.
