---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260824T145655Z-contribution-auditor-s6sm
timestamp: 2026-08-24T14:56:55Z
model: codex-gpt-5.6
immutable: true
references-to:
  - 20260824T144725Z-planner-architect-n75b.md
  - 20260824T142545Z-governor-reviewer-kqm3.md
  - 20260824T071103Z-governor-reviewer-ekns.md
topic: mr1965-85a62269-round1-complete-audit
requires-ack: false
artifacts_owned: []
---

# MR !1965 exact-head implementation audit: COMPLETE / BLOCK

Scope: GitLab MR !1965 (`id: 523111786`) at exact source head
`85a62269abe7564a05fe670f50b281a52c6f83bf`, targeting `main` from base
`e1ae69ec40767951a3cb2034cf2bc21ddfb616fe`.

Coverage: COMPLETE
Recommendation: BLOCK
Blocking findings: 4
Advisory findings: 0
Open findings with accepted risk: 0
Authority: recommendation only; the governor-reviewer decides Gate 2.

The review was performed in a detached, read-only worktree by a Codex
contribution-auditor session that did not author the plan, implementation, or
the historical-gap decision. The complete nine-file contribution, approved
plan, published decisions, live MR description, exact commit ancestry, focused
tests, and current pipeline/job state were inspected.

## CA-ADV-R1-1 — future timestamps defeat the promised bounded window

Check: bounded `upgrade_in_progress` suppression lifecycle
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Impact: material
Enforcement: blocking
Basis: approved plan's "Mid-upgrade transient window" and test strategy
Finding state: open
Risk disposition: none

`live_record_findings` parses `upgrade_started_at`, computes whole elapsed
hours, and defines the active window only as `hours < 24`. It never requires
the elapsed value to be non-negative. A future RFC3339 timestamp therefore
produces a negative number and suppresses a real version mismatch until that
future date plus 24 hours. The schema also constrains the value only to a
string, not RFC3339/date-time semantics.

An isolated review probe used `upgrade_started_at:
"2099-01-01T00:00:00Z"`, `upgrade_in_progress: true`, a live record at
`2.0.0-provisional`, and `agf/VERSION` at `2.6.1`; `validate_all` returned OK.
The probe was removed after execution and the candidate worktree was confirmed
clean. This contradicts the approved design's bounded 24-hour suppression.

Resolution condition: require `0 <= elapsed < 24h` for suppression; treat a
future, missing, or invalid timestamp on an in-progress marker as non-
suppressing and visibly malformed/stale, and add boundary/future/invalid tests.

## CA-ADV-R1-2 — a live record without `framework_version` passes

Check: live-record version comparison activation
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Impact: material
Enforcement: blocking
Basis: approved plan's exactly-one-live-record behavior and product claim
Finding state: open
Risk disposition: none

The schema does not require `adoption.framework_version`, including when
`checkout_role` is `live`. `live_record_findings` returns an empty finding set
when that field is absent. Consequently exactly one live record can activate
neither the zero-live advisory nor the drift comparison and still pass.

An isolated review probe created exactly one `checkout_role: live` record with
no `framework_version`; `validate_all` returned OK. The probe was removed after
execution. This makes the advertised opt-in check fail open at its controlling
value.

Resolution condition: make `framework_version` mandatory whenever
`checkout_role: live` (schema conditional or equivalent blocking semantic
finding), add the missing-value test, and fail visibly if `agf/VERSION` cannot
be read rather than silently returning no findings.

## CA-ADV-R1-3 — mandatory Gate self-test evidence is not reproducible

Check: Gate 2 negative CLI acceptance evidence
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Impact: material
Enforcement: blocking
Basis: approved plan lines 842-846 and `gate-self-test-rule.md`
Finding state: open
Risk disposition: none

The approved plan requires an exact reproducible negative command, expected
and actual blocking output, and a link to the CI job log where it ran. The MR
description's command creates only `/tmp/gate2-fixture/agf/learnings` and then
copies the schema into the nonexistent `/tmp/gate2-fixture/agf/schemas/`
directory. It also contains no CI job-log link for this demonstration. The
ordinary unit-test/pipeline statements do not satisfy the plan's explicit
end-to-end evidence contract.

Resolution condition: repair the fixture setup, run the exact negative CLI
demonstration in the exact-head acceptance path, and link the specific passing
CI job log containing its expected blocking output and exit status.

## CA-ADV-R1-4 — exact-head acceptance pipeline is not green

Check: exact-head required CI
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Impact: material
Enforcement: blocking
Basis: Gate 2 acceptance and approved plan test strategy
Finding state: open
Risk disposition: none

Exact-head MR pipeline `2785864447` has a failed required job:
`rust:tooling:test` job `16071039214` reports 139 passed, one failed, two
ignored. The failure is
`commercial_corpus::tests::real_mr1712_generation_acceptance_is_142_included_plus_50_excluded`;
its fixture tries `git show 48049759...:.gitlab-ci.yml`, but that path is not
available at the referenced revision in the CI checkout. This is outside the
new learning-validator logic, but the mandatory exact-head acceptance path is
not green and may not be retried merely to seek green status.

Resolution condition: diagnose and correct the historical-revision/CI-checkout
fixture contract on a required corrected acceptance run, then provide a green
pipeline for the corrected MR head. Preserve the failed job as evidence; do
not retry it unchanged.

## Passing evidence

- Published governor decision `kqm3` precedes all three implementation commits
  and authorizes the disclosed correction without claiming retroactive review.
- The exact candidate ancestry and changed-file scope match the approved plan.
- Focused local tests passed: 22 existing `learnings` tests, including all ten
  new candidate tests. Two additional disposable review probes reproduced
  findings R1-1 and R1-2 and were removed; the worktree is clean.
- Local umbrella validation showed only the documented pre-existing warnings.
- Exact-head pipeline `2785864447` was still completing other jobs when this
  record was authored, but required job `16071039214` had already failed as
  recorded in R1-4.

No candidate file, MR metadata, pipeline, issue, or gate state was modified by
this audit.
