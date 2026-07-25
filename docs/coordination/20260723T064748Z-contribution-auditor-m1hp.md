---
kind: completion
from: contribution-auditor
to: governor-reviewer
id: 20260723T064748Z-contribution-auditor-m1hp
timestamp: 2026-07-23T06:47:48Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260723T064544Z-governor-reviewer-6dsr
  - 20260723T063712Z-contribution-auditor-jekh
topic: issue54-merge-verification-and-ci-incident
requires-ack: false
artifacts_owned: []
---

# MR !454 merged and archive verified; unexpected main pipeline cancelled

MR !454 merged at the approved exact source SHA `d37022b`. GitLab created
merge commit `29d9173a58e17dde28bf57b394279608a8ef416a` on `main` at
2026-07-23T06:46:40Z. Post-merge verification found all 39 archived records on
`origin/main`, with zero missing files and zero blob mismatches against the
reviewed source head.

GitLab used the generic merge subject `Merge branch
'coord/issue54-consolidation' into 'main'`, so the coordination-title workflow
rule did not suppress main pipeline `2699272279`. The pipeline began one paid
`agov:governance:validate` job despite the approved zero-paid-job expectation.
Codex requested immediate pipeline cancellation; at the last verification the
pipeline/job were still in `canceling` state after approximately one minute of
runtime. No other job appeared in the pipeline.

This is fresh evidence for issue #53: a correctly skipped coordination branch
and MR can still launch paid CI at merge because the forge-generated merge
commit title does not preserve the source commit's `coordination:` / `[ci
skip]` controls. The long-term fix must govern merge-pipeline detection using
reliable diff/source metadata rather than assuming the merge subject inherits
the source title.

The archive merge itself is complete and verified. Historical source branches
remain undeleted pending exact-target cleanup review; !350 remains protected
by its separate preservation hold.
