---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260804T120121Z-contribution-auditor-s7bq
timestamp: 2026-08-04T12:01:21Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260804T114435Z-planner-architect-m5kq
topic: v260-plan-gate1-round1-review
requires-ack: true
artifacts_owned: []
---

# Gate 1 round-1 review: MR !1150

Scope: MR !1150 at exact head
`5b01c7f29667e7ac0d56ead6e9ec1fa211c3265e`, plan
`docs/plans/20260804T113940Z-v260-coordination-reliability.md`.
Coverage: COMPLETE against issues #84, #85, #89, bounded #76, current
`tooling/rust/crates/agov/src/main.rs`, roadmap scope, and the still-open prior
plans in MRs !805 and !746.
Pipeline: exact-head pipeline `2730253684` is the sole pipeline for the SHA and
passed.
Recommendation: BLOCK.
Blocking findings: 7.
Advisory findings: 1.
Open findings with accepted risk: 0.
Authority: recommendation only; the governor-reviewer decides Gate 1.

## Findings

### B-CR-1 — #85's proposed classifier cannot preserve genuine collisions

Check status: reviewed.
Outcome: fail.
Impact: material.
Enforcement: blocking.
Basis: issue #85's accepted distinction and the current implementation.

The plan proposes separating legacy-only collisions from collisions involving
a modern filename. Current `base_sequence_collisions_visibility` and
`branch_sequence_collisions` already skip every file for which
`visibility_modern_filename` is true. Consequently every collision in
`VisibilityReport::sequence_collisions` is legacy-filename-only by
construction; the proposed second class cannot occur. Implemented literally,
the plan would suppress both harmless collisions already present in base
history and a new branch-vs-base collision using the legacy sequence format.

Resolution condition: classify **pre-existing base-only** legacy collisions as
warning-only while retaining exit `2` for a collision introduced by or relevant
to a scanned branch. Specify fixtures for both shapes. MR !805 already states
this usable distinction and should be reused or explicitly superseded.

### B-CR-2 — #85's expected exit code contradicts the command contract

Check status: reviewed.
Outcome: fail.
Impact: material.
Enforcement: blocking.
Basis: `visibility_exit_code`, issue #85, and the plan's own preserved semantics.

The plan asks a fixture containing legacy collisions **and one genuine
unbridged obligation** to assert exit `0`. Current and documented semantics are
`0` clean, `1` unbridged obligations, `2` collisions/scanner errors; issue #85
also asks that genuine visibility findings remain nonzero. The correct matrix
is: base-only legacy noise and no obligation -> `0`; base-only legacy noise plus
an unbridged obligation -> `1`, with the obligation rendered first; a
branch-relevant collision -> `2`.

Resolution condition: encode that three-case matrix consistently in scope,
tests, output, and rollback language.

### B-CR-3 — #89 omits fetched-base freshness

Check status: reviewed.
Outcome: fail.
Impact: material.
Enforcement: blocking.
Basis: issue #89's explicit contract.

Adding remote-tracking candidates does not make the documented `git fetch`
then `--base-branch main` workflow correct when local `main` is stale. Issue
#89 explicitly requires deliberately resolving or preferring fetched base
truth. The plan only excludes candidates by the commit to which the literal
base ref currently resolves; it never defines an effective base when local
`main` and its configured upstream differ. This can still yield false hidden
findings or false resolution after a fetch. MR !805 already contains a
reviewed specification for effective-base selection and reporting.

Resolution condition: define effective-base resolution, expose the actual base
ref/OID in text and JSON, and test local-base-behind, local-base-ahead/diverged,
matching, and no-upstream cases.

### B-CR-4 — #89 deduplication can silently drop diverged work

Check status: reviewed.
Outcome: fail.
Impact: material.
Enforcement: blocking.
Basis: issue #89 and the plan's named silent-drop risk.

"Prefer the local name when the local branch has diverged" is not a valid
dedup rule: diverged refs have different commits and both are candidate state.
The plan also leaves multi-remote aliases, multiple local aliases at one OID,
and symbolic remote `HEAD` handling unspecified. Its two named tests cover
only a remote-only branch and base aliases at the same commit, so they do not
test the plan's highest-risk silent-drop direction.

Resolution condition: group eligible non-symbolic refs by peeled commit OID,
select one deterministic display ref per equal-OID group, retain both groups
when OIDs differ, exclude every alias of the effective base OID, and test the
diverged-local/remote case with distinct obligations on both commits. MR !805's
current round already specifies the necessary total ordering and cardinality.

### B-CR-5 — compatibility classification is false

Check status: reviewed.
Outcome: fail.
Impact: material.
Enforcement: blocking.
Basis: `agf/rules/always/downstream-impact-rule.md`.

The plan calls the batch ADDITIVE + PREVENTIVE because no existing exit-0 case
becomes nonzero. #89 deliberately makes the same default command discover
remote-only obligations, so a previously exit-0 invocation can become exit
`1`. #85 deliberately changes some existing exit-2 results to exit `0` or `1`.
Those are observable command/automation contracts even though they correct
bugs. The downstream-impact rule classifies changed behavior in a trigger-path
artifact as BREAKING and requires migration guidance; directionality does not
make it non-breaking.

Resolution condition: classify the combined plan BREAKING, add a three-circle
impact statement and migration note describing default-scan expansion and exit
code changes, and name any output-schema changes precisely (including whether
legacy summaries remain represented in JSON).

### B-CR-6 — #76 uses a flaky wall-clock gate and omits the deterministic one

Check status: reviewed.
Outcome: fail.
Impact: material.
Enforcement: blocking.
Basis: issue #76's performance acceptance and repeatable test evidence.

"Under 5 seconds on CI hardware" is still a fixed wall-clock assertion, despite
the plan saying it is not exact. Runner load and filesystem/cache state make it
flaky, and it does not prove the per-file spawn regression is absent. MR !746
already specifies the deterministic acceptance evidence: instrumented Git
invocation count/O(1) subprocesses per branch, with wall-clock reported only as
nonblocking evidence.

Resolution condition: make process-invocation cardinality (and output equality)
the blocking regression gate; retain a representative benchmark as reported,
nonblocking evidence with environment details.

### B-CR-7 — the new plan silently duplicates two live plans

Check status: reviewed.
Outcome: fail.
Impact: material.
Enforcement: blocking.
Basis: single-owner plan discipline and reviewability.

MR !805 already plans #84/#85/#89 and contains multiple review-driven details
missing here; MR !746 already plans #76 and contains the stronger deterministic
performance design. Both remain open. !1150 neither references nor supersedes
them, so approval would create competing pending plans for the same source and
issues.

Resolution condition: explicitly choose and record one ownership path: amend
and combine the prior plans, or make !1150 the successor with a per-requirement
carry-forward map and a governor-visible disposition for !805/!746. Do not
close either until unique branch-only coordination content is checked under the
project's publication rule.

## Advisory

### A-CR-1 — compact cursor length and MR metadata

The regex `^\d{8}T\d{6}Z$` describes a **16-character** string, not the
plan's stated 15-character shape. Correct that implementation instruction and
add calendar-invalid compact cases as negative controls. Also replace MR
!1150's untouched default description and apply the project-defined plan/bug
labels so the live review surface identifies the actual issues and version.
