---
kind: decision-record
from: governor-reviewer
to: all
id: 20260826T043649Z-governor-reviewer-3j3p
timestamp: 2026-08-26T04:36:49Z
model: human
immutable: true
references-to:
  - 20260825T052315Z-coder-tester-6djh.md
  - 20260826T042010Z-governor-reviewer-4ssr.md
  - 20260826T042014Z-coder-tester-2rje.md
topic: ci-history-job-local-full-history-direction-approved
requires-ack: false
artifacts_owned: []
---

# Job-local complete-history direction approved

This record transcribes Jani's explicit follow-up governor-reviewer decision
after `4ssr` commissioned the CI-HISTORY-1 correction and `2rje` identified
the recommended solution:

> approved as proposed

## Decision

The correction plan shall use job-local complete history for
`rust:tooling:test`, expressed as `GIT_DEPTH: "0"` on that job, as the selected
default design. It shall not enlarge repository history for unrelated jobs,
replace the real pinned-history acceptance test with weaker evidence, or use a
fixed larger shallow depth that will decay as repository history grows.

The expected implementation scope is `.gitlab-ci.yml`. If impact assessment
shows that a narrowly supporting CI-contract test or operator documentation is
necessary for durable enforcement, the Gate 1 plan must name and justify those
additional paths before implementation. Necessary scope must not be hidden,
and unrelated cleanup remains excluded.

## Conditional Gate 1 direction

This decision removes solution-selection uncertainty but does not substitute
an unwritten plan for exact-head Gate 1 evidence. Implementation is
conditionally authorized only after a self-contained activation record names
an exact validated plan head and confirms all of the following before coding:

1. the plan freezes the job-local `GIT_DEPTH: "0"` design and bounded paths;
2. `agov plan validate` and the exact-head plan pipeline succeed;
3. planner-architect and coder-tester independently agree that the plan is
   sufficient and within this correction commission;
4. required contribution-auditor and infosec-sentinel consultation statuses
   are present; and
5. acceptance preserves both pinned revisions and the real
   142-included/50-excluded historical-evidence test, addresses merge-request
   and fork behavior, measures the incremental fetch cost, and requires fresh
   corrected exact-head pipelines rather than unchanged-job retries.

When those conditions are documented for the exact plan head, implementation
may proceed through validation and presentation for Gate 2 without another
solution-selection decision. Any failure of a condition, material scope
expansion, or conflicting evidence returns to the governor-reviewer.

## Boundaries

This decision is not Gate 2 approval, merge or release authority, a failed-job
waiver, or permission to retry an unchanged paid job merely to seek green.
MRs !2041 and !806 remain blocked until the correction is governed, reaches
their acceptance base, and fresh exact-head pipelines provide valid evidence.
