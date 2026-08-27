---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260729T170955Z-contribution-auditor-8n86
timestamp: 2026-07-29T17:09:55Z
model: codex
immutable: true
references-to:
  - 20260729T165348Z-contribution-auditor-268m
  - 20260729T165814Z-planner-architect-4qsk
topic: first-push-ci-fallback-plan-review
requires-ack: true
artifacts_owned: []
---

# BLOCK: selected rule structure falls through to the original defect

Scope: MR !790 at exact head `26a39beff74d016b5a410a9e5199dfee7f0e5718`
Coverage: COMPLETE
Recommendation: BLOCK
Blocking findings: 3
Advisory findings: 3
Open findings with accepted risk: 0
Authority: advisory recommendation; governor-reviewer decides Gate 1

## B1 — Candidate A does not stop fallthrough after a path miss

GitLab evaluates rules in order until one matches. On a new branch where the
job's paths do not differ from the default branch, Candidate A's first rule
does **not** match because its `compare_to` path predicate is false. Evaluation
then continues to the existing second branch rule. That rule has no
`compare_to`, so GitLab's documented new-branch fallback makes its `changes`
predicate true and the job runs anyway. Rule ordering alone does not fix this.

The plan needs a terminal guard between the first-push comparison and existing
incremental rule, conceptually:

```yaml
- if: '$CI_PIPELINE_SOURCE == "push" && <non-default-branch> && <before-sha-zero>'
  changes:
    compare_to: 'refs/heads/$CI_DEFAULT_BRANCH'
    paths: [<existing paths>]
- if: '$CI_PIPELINE_SOURCE == "push" && <non-default-branch> && <before-sha-zero>'
  when: never
- if: '<existing non-default branch condition>'
  changes: [<existing paths>]
```

The first rule includes the job when its real default-branch diff matches. The
second terminates the zero-SHA case when it does not. Only non-zero later pushes
reach the existing incremental rule. CI Lint alone cannot prove runtime path
selection; the disposable positive/negative cases remain mandatory.

Resolution condition: update the selected mechanism, matrix, rollback, static
checks, and evidence to include and verify an explicit no-fallthrough guard for
all 12 jobs.

## B2 — source boundary is broader than the stated “first push” scope

The proposed condition checks branch and zero `before_sha` but not
`$CI_PIPELINE_SOURCE == "push"`. Scheduled, API, web, or other branch-ref
pipelines can also expose an all-zero before SHA. The plan says only first
branch pushes change and labels schedule/API behavior unchanged, so the rule
must enforce that boundary or explicitly broaden and validate scope.

Resolution condition: add the push-source predicate to both first-push rules
and correct the acceptance matrix, including explicit handling of web/manual
pipelines.

## B3 — the Rust negative test expects two jobs that should not run

A trivial `.rs` comment under `tooling/rust/**` should select
`agov:governance:validate`, `rust:tooling:deny`, `rust:tooling:test`, and
`rust:tooling:clippy`. It should **not** select `rust:agov:validate`, whose
paths are only `agf/**` and `.gitlab-ci.yml`. It should not select
`dependency:cve-scan` unless the fixture changes a matched TOML/lock/config
path. The plan currently lists `rust:agov:validate` among expected Rust-touch
jobs, which would turn a remaining false positive into purported success.

Resolution condition: define exact fixture paths and derive the expected job
set literally from each job's current rules; include explicit should-run and
must-not-run assertions.

## A1 — implementation role is inactive

The plan assigns implementation to `build-engineer`, but the authoritative
manifest and AGENTS.md mark that role suspended. Assign the active
`coder-tester`, or obtain a separate governor-approved role reactivation before
relying on build-engineer.

## A2 — rollback is not 12 single-line changes

The selected implementation adds a comparison rule plus a terminal guard per
job and duplicates each path list. Describe rollback as reverting the governed
CI commit/MR, not “12 single-line-per-job reverts.” Partial manual rollback
across duplicated rule blocks is more error-prone than the current text admits.

## A3 — byte-identical path-list proof needs a structural check

`grep` is not a reliable proof that two nested YAML path lists are identical.
Require a small parser-based/static fixture or a closed expected-rule snapshot
that fails on missing, extra, reordered where meaningful, or mismatched paths.
The implementation should minimize duplication with YAML anchors/references if
GitLab's supported syntax permits it and CI Lint confirms the expanded rules.

## Disposition

All eight requested planning areas are present, and the evidence basis is
strong, but the selected mechanism as written reproduces the defect after a
first-rule path miss. Revise B1–B3 before Gate 1; address A1–A3 in the same
pass, then request an exact-head re-review and the promised infosec-sentinel
pass.
