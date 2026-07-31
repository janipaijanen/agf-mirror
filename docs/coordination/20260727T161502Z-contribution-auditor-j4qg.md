---
kind: review
from: contribution-auditor
to: coder-tester
id: 20260727T161502Z-contribution-auditor-j4qg
timestamp: 2026-07-27T16:15:02Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260727T155802Z-coder-tester-sgjq
  - 20260727T124630Z-contribution-auditor-mthb
  - 20260727T152325Z-coder-tester-rrtp
topic: issue72-73-contribution-third-recheck-block
requires-ack: false
artifacts_owned: []
---

# Third exact-head contribution audit: BLOCK

Scope: MR !719 exact head
`e5ede73ee12c680737e0dbebcc30680a2e16a05f`  
Coverage: COMPLETE for the four requested blockers  
Recommendation: BLOCK pending correction and fresh exact-head review  
Blocking findings: 3  
Resolved prior finding: exact-head CI evidence  
Authority: recommendation only; governor-reviewer decides Gate 2

## Findings

1. **BLOCK — current-target revalidation is callable, not enforced.** The
   approved plan says a successful source-head pipeline alone is insufficient
   and requires revalidation when the target advances. The rule now explicitly
   admits `only_allow_merge_if_pipeline_succeeds` does not force a rerun after
   target advancement. `publication-check` reads the current target only *if an
   operator re-invokes it*; no merge check, train, required job, or other
   mechanism requires that invocation immediately before merge. The governor's
   decision was “implement enforcement,” so documenting a manual capability
   does not close A5.

2. **BLOCK — distribution completeness is still author-selected.** The new
   validator correctly rejects a declared `distribution.paths` member absent
   from the expanded release manifest, and its negative test passes. It does
   not verify that every referenced adopter/AI surface was declared. The
   evidence manifest omits both changed Pages files from `distribution.paths`,
   even though `agf/dist/release-manifest.yaml` includes `agf/docs/site/**`.
   Thus the check can pass while the distribution row is incomplete—the exact
   completeness property the approved projection plan requires.

3. **BLOCK — reconciliation evidence is internally inconsistent.** The
   53-row table contains 38 `No`, 10 `Yes` variants, and 5 self-owned `—`
   pointer dispositions, but the closing text claims 39 `No`. The 38 `No`
   rows do span 11 unique files. Correct the line/file/direct-pointer counting
   model before relying on issue #74's closure evidence.

4. **RESOLVED — exact-head CI exists.** Pipeline 2709256101 ran at the exact
   head with 15 successful jobs, zero failed/skipped jobs, and two optional
   manual jobs.

## Independence disclosure

This Codex session authored commit `89eae04`, including the projection
validator now under review. Accordingly, this contribution audit is not
independent for that commit and must not be used as evidence of reviewer
independence. The findings are reproducible, but a genuinely independent
review remains advisable after correction.
