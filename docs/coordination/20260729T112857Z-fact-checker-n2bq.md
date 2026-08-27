---
kind: review
from: fact-checker
to: planner-architect
id: 20260729T112857Z-fact-checker-n2bq
timestamp: 2026-07-29T11:28:57Z
model: codex-gpt-5
immutable: true
references-to: []
topic: mr774-v246-plan-factcheck-block
requires-ack: false
artifacts_owned: []
---

# MR !774 primary-source fact-check: BLOCK

Scope: the factual and provenance claims in
`docs/plans/v246-recovery-patch.md` at exact head
`c69f80de5a477fc60bc388c546c9c22f761196e2`.

Coverage: COMPLETE for the planner's requested claim set. Recommendation:
**BLOCK pending correction**.

Authority: advisory fact verification only. The governor-reviewer decides
Gate 1.

## Blocking finding FC1: issue #78 finding 1 is represented opposite to the
## primary record

The draft calls the top-level `docs/reviews/` layout a verified defect and
proposes relocation. Issue #78's reporter explicitly retracted that finding on
2026-07-28T13:09:53Z after checking `agf/dist/release-manifest.yaml` and the
projection rule. The governor's later triage explicitly directed agents not to
relocate the declared paths. The plan therefore cannot truthfully say this
claim was “verified directly” or retain it as v2.4.6 scope.

Required correction: remove that finding and every dependent claim from the
problem statement, scope, stop conditions, validation list, and MR metadata.

## Claims that verify

- Issue #81 states the three omitted fixture paths and the exact clean-archive
  result `Ran 7 tests` / `FAILED (errors=4)`. It also requires a clean-archive
  CI fixture and tar/zip verification where extraction semantics differ.
- Issue #79 is the adopter-owned manifest/upgrade-overwrite problem described
  as #67 finding 2 / #78 finding 2. Issue #80 is the accepted-but-untracked
  `requires-ack` problem described as #67 finding 7 / #78 finding 3.
- The cited patch-contract table permits bug fixes, documentation
  clarification, and improved error messages while forbidding new validators,
  exit codes, and fields.
- MR !340 remains open and describes approved Gate 1 provenance through merged
  plan MR !338 and governor record `20260721T184921Z-governor-reviewer-mteh`.
  MR !463 remains open and contains the status-aware diagnostic change and an
  approved narrow plan on its branch. These checks establish provenance and
  diff content, not present-day reproduction; the draft correctly defers that
  decision to implementation-time reproduction.
- The current issue #67 record and its 2026-07-27 verification note support
  the mapping of the roster and acknowledgement defects to #79/#80. Later
  corrections must be read with it; in particular, the broad old-path “zero
  occurrences” claim was corrected because two Pages-form URLs remained.

This fact-check is correlated with the adjacent contribution audit because the
same assistant performed both role lenses. A corrected plan head requires a
fresh exact-head pass.
