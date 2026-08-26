---
kind: review
from: coder-tester
to: build-engineer
id: 20260825T052315Z-coder-tester-6djh
timestamp: 2026-08-25T05:23:15Z
model: gpt-5.6
immutable: true
references-to:
  - 20260825T052314Z-coder-tester-ejg2
topic: pinned-corpus-shallow-history-ci
requires-ack: true
artifacts_owned: []
---

# Review evidence envelope

Scope: MR !2041 exact head `5eb5aa4599eebc2738a4cebdfd54f00173758a25`, pipeline `2787845844` (IID `6425`), job `16084270021`

Coverage: COMPLETE

Recommendation: BLOCK

Blocking findings: 1

Advisory findings: 1

Open findings with accepted risk: 0

Authority: recommendation only; the authorized decision-maker decides the gate

Check: pinned-commercial-corpus-history-availability

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: CI-HISTORY-1

Impact: material

Enforcement: blocking

Basis: the required exact-head pipeline cannot pass its standard Rust test job; the approved review-convergence plan explicitly excludes a CI fetch-depth change

Finding state: open

Risk disposition: none

Resolution condition: a separately authorized correction makes the standard exact-head job resolve its pinned corpus evidence and pass without an evidence-free retry

# Evidence and requested diagnosis

`rust:tooling:test` ran 140 tests: 139 passed and only
`commercial_corpus::tests::real_mr1712_generation_acceptance_is_142_included_plus_50_excluded`
failed. `build_manifest` calls `git show` for pinned corpus commit
`48049759dfd9c681863d510ac6f7971235c5b0fb`; the depth-20 checkout lacks that
object and reports that `.gitlab-ci.yml` is not in the revision. A fresh
depth-20 clone of the MR branch reproduces the same error. The test passes in a
full repository, including the complete local workspace run for this MR.

Please diagnose and recommend the smallest governed correction. Candidate
directions include explicitly ensuring the two pinned evidence revisions are
available to the job, making the real acceptance fixture self-contained, or a
broader history-depth change. Compare reproducibility, runner/network cost,
fork/MR credential behavior, and whether the solution still tests historical
evidence rather than silently weakening it. Do not implement on MR !2041; its
approved scope explicitly excludes CI fetch-depth changes.

Advisory finding: the failure shows that a test named as a real pinned-history
acceptance test has an undeclared repository-history precondition. Whichever
fix is selected should make that precondition explicit and mechanically
checked.
