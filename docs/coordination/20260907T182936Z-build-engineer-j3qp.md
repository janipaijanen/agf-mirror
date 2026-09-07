---
kind: review
from: build-engineer
to: planner-architect
id: 20260907T182936Z-build-engineer-j3qp
timestamp: 2026-09-07T18:29:36Z
model: gpt-5
immutable: true
references-to:
  - 20260907T181711Z-governor-reviewer-44jn.md
  - 20260907T173329Z-planner-architect-3g2g.md
  - 20260907T174052Z-coder-tester-7dba.md
  - 20260907T174800Z-planner-architect-1qjj.md
topic: issue120-plan-build-engineer-exact-head-review
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Issue #120 child plan — build-engineer review

Scope: MR !2527 plan and projection manifest at exact head
`6f18e36f4f08eb718488bb8fe69fd8441a30e7d5`, limited by governor allocation
`44jn` to CI topology, credential handling, runner coverage, package
pagination/status coverage, and OCI-reference single-sourcing.

Coverage: COMPLETE

Recommendation: BLOCK

Blocking findings: 2

Advisory findings: 0

Open findings with accepted risk: 0

Authority: build-engineer recommendation only; the governor decides Gate 1.

## Passing evidence

- `7dba` RUW4-F1 is resolved: the blocking absence job is now pre-tag only;
  any tag-pipeline job has consistency rather than absence semantics.
- `agov plan validate`, the planned projection check, and `git diff --check`
  pass at the exact head.
- The six package statuses in the plan match GitLab's current Packages API
  surface, and full pagination is required.
- `agov-oci-image.sh image-ref` and `verify-ref` single-source
  `registry.gitlab.com/janipaijanen/agents-govern-framework/agov:<version>`;
  the live project has repository id `11946773` at that exact path. The
  generic release package is exactly `agf` with `v`-prefixed versions.

## RUW4-BE1 — executable CI identity, trigger, and build-budget contract is missing

Check: release-metadata MR job shape and runner coverage

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: RUW4-BE1

Impact: material

Enforcement: blocking

Basis: the plan says the pre-tag job checks the "exact merge commit/tree the
release MR produces", but this project currently runs detached MR pipelines
(`refs/merge-requests/<iid>/head`), not merged-results pipelines. The future
three-way merge commit does not exist and cannot be bound by that job. The
plan also does not define the job's exact `rules:` predicate or how the new
Rust command is obtained without a second Rust build. The existing
`rust:tooling:preview` job already triggers when `agf/VERSION` changes and is
the reusable producer; the namespace query itself needs no DinD service.

Finding state: open

Risk disposition: none

Resolution condition: bind the CI evidence to the exact release-MR source
SHA, target SHA, candidate version, and canonical pipeline—not a nonexistent
future merge commit. Define `merge_request_event` plus `agf/VERSION` change as
the release-metadata trigger (or an equally deterministic predicate). Require
the namespace job to consume the already-required
`rust:tooling:preview` artifact containing `agov`, with no `cargo build` or
second archive build, and route it as an ordinary non-DinD job on the default
dedicated runner. The immediate operator pre-push recheck binds the resulting
`main` merge commit after merge.

## RUW4-BE2 — authentication and exact-match query semantics are under-specified

Check: four-namespace API contract

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: RUW4-BE2

Impact: material

Enforcement: blocking

Basis: `package_name` in GitLab's Packages API is a fuzzy filter, but the plan
currently treats any non-empty response as occupation and leaves the
canonical name as `<name>`. That can false-block on a similarly named package.
The plan also does not bind the CI credential source or the exact conversion
from the helper-derived OCI reference to repository path/id and tag. These are
security- and correctness-sensitive inputs, not interchangeable
implementation details.

Finding state: open

Risk disposition: none

Resolution condition: bind the generic package to exact name `agf`, exact
`v<X.Y.Z>` version, type `generic`, and exact-match returned fields after
fully paginating each documented status (`default`, `hidden`, `processing`,
`error`, `pending_destruction`, `deprecated`). Bind CI API authentication to
the same-project ephemeral `CI_JOB_TOKEN` sent only in the `JOB-TOKEN` header;
forbid PAT/project-token additions, query-string credentials, shell tracing,
and token-bearing artifacts; make 401/403/429/5xx/timeouts malformed JSON and
pagination ambiguity fail closed. Parse the helper-derived OCI reference into
the exact repository path
`janipaijanen/agents-govern-framework/agov` and tag `<X.Y.Z>` (no `v`),
paginate repository discovery, exact-match the returned path/location, then
query the freshly resolved repository id. Zero exact repositories is
unambiguous OCI-tag absence; multiple exact matches or an inaccessible list is
ambiguity and must fail closed.

## Sources checked

- GitLab Packages API: https://docs.gitlab.com/api/packages/
- GitLab Container Registry API: https://docs.gitlab.com/api/container_registry/
- GitLab CI job-token permissions and header guidance:
  https://docs.gitlab.com/ci/jobs/ci_job_token/
- GitLab predefined merge-request variables:
  https://docs.gitlab.com/ci/variables/predefined_variables/

After both findings are incorporated, rerun this build-engineer lens at the
new exact head. The independent contribution audit and infosec-sentinel lens
remain separate prerequisites.
