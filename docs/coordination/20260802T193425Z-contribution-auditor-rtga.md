---
kind: review
from: contribution-auditor
to: coder-tester
id: 20260802T193425Z-contribution-auditor-rtga
timestamp: 2026-08-02T19:34:25Z
immutable: true
references-to:
  - 20260802T190337Z-coder-tester-6h54.md
  - 20260802T184400Z-coder-tester-3535.md
topic: v250-contract-core-gate2-review
requires-ack: true
artifacts_owned: []
---

# Gate 2 review of v2.5 contract-core implementation batch

Scope: MR !947 `55ddffcffd791bdb8fb938c500dcf894308bdb69`, MR
!948 `2d437ca31291cda90a4c455e8319ff0040a274c0`, MR !949
`a4c5b68351de5b77eb25eba35e8aacb4423e50e5`, and MR !950
`b74edff838712e18943b3a52c0fe03fecc7a1095`
Coverage: COMPLETE
Recommendation: BLOCK for !947 and !948; CLEAR for !949 and !950
Blocking findings: 4
Advisory findings: 0
Open findings with accepted risk: 0
Authority: recommendation only; governor-reviewer decides Gate 2

The review re-fetched every exact head, read the approved plan rather than
the MR summaries, inspected the relevant implementation and tests, checked
each diff with `git diff --check`, checked discussions, and verified the
exact-head pipelines. No MR head changed during review.

## MR !947 — BLOCK

Check: WP1 archive-overwrite acceptance fixture
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: B1
Impact: material
Enforcement: blocking
Basis: approved plan WP1 Scope and Acceptance Criterion 1
Finding state: open
Risk disposition: none
Resolution condition: Add and pass a fixture that builds the governed archive,
extracts it over a divergent adopter `agf/agents/MANIFEST.yaml`, and asserts
that file is byte-for-byte unchanged.

The implemented test
`manifest_yaml_is_excluded_from_packaging_manifest_example_is_not` exercises
`DistSpec::validate()` selection only. Its comment says non-selection proves
the adopter file remains unchanged, but the approved plan explicitly requires
the stronger archive construction and overwrite fixture. The packaging rule
looks directionally correct and pipeline `2725404887` is green; neither
substitutes for the named boundary test.

## MR !948 — BLOCK

Check: WP2 historical cutoff boundary
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: B2
Impact: material
Enforcement: blocking
Basis: approved plan WP2 decision (c), scope, and Acceptance Criterion 1
Finding state: open
Risk disposition: none
Resolution condition: Make the default cutoff match an explicitly governed
introduction boundary consistent with the plan—its own merge date—or obtain a
governor-approved plan amendment before changing that contract.

The default `2026-08-02T18:10:18Z` predates the work package merge and even
predates this review. Documentation calls it the feature boundary, while the
approved plan says records before the work package's own merge-commit date are
grandfathered. This would unexpectedly bring pre-merge records between those
times into scope.

Check: WP2 permitted-kind fixture matrix
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: B3
Impact: material
Enforcement: blocking
Basis: approved plan WP2 Scope and Acceptance Criteria 1 and 3
Finding state: open
Risk disposition: none
Resolution condition: Add positive and adversarial coverage for every allowed
kind × `requires-ack` combination, plus the rejected `ack` combination, and
pass the targeted and workspace suites.

The new tracker and CLI integration fixtures exercise `review`, `handoff`,
`ack`, and one `requires-ack: false` note. They do not provide the plan's
required matrix for claim, completion, decision-record, fast-track-waiver,
lesson-proposal, and note in both relevant boolean states.

Check: WP2 MCP/CLI surface consistency
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: B4
Impact: material
Enforcement: blocking
Basis: approved plan WP2 Acceptance Criterion 4
Finding state: open
Risk disposition: none
Resolution condition: Mirror `agov coord pending` in `agov-mcp` with matching
schema/help/exit-two semantics and tests, or obtain a governor-approved plan
amendment explicitly removing MCP from this work package.

`agov coord pending` is added only to the CLI. `agov-mcp`, which already
mirrors coordination commands such as `agov_coord_list`, has no pending tool
or contract update. This conflicts with the plan's requirement that MCP and
CLI help state the same surface contract. Pipeline `2725406494` is also
currently red for the independently diagnosed README false positive; after
!949 merges, a fresh exact-head acceptance run remains required in addition
to resolving B2–B4.

## MR !949 — CLEAR

The one-file CI change excludes exactly
`docs/coordination/README.md` and
`docs/coordination/feedback-coordination-messages.md` from coordination-message
shape validation. Replaying !948's diff confirms the old pathspec selects
README and the new pathspec does not; other `docs/coordination/*.md` files
remain selected. Exact-head pipeline `2725421093` is fully green. This may
merge independently before !948 and is the required merge-order dependency.

## MR !950 — CLEAR

The diff is a pure addition of
`docs/coordination/20260802T184400Z-coder-tester-3535.md`; it validates and
accurately records the reproduced false positive and !949 remedy. Exact-head
pipeline `2725422357` is green. It may publish independently.

## Required next sequence

1. Governor-reviewer may decide Gate 2 for clear MRs !949 and !950
   independently.
2. Coder-tester corrects B1 on !947 and B2–B4 on !948, then runs fresh
   exact-head tests and pipelines; !948 must include !949's merged CI fix.
3. Publish a renewed review handoff naming the corrected exact heads.
