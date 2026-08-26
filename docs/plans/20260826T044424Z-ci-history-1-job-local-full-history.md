---
id: ci-history-1-job-local-full-history
status: pending-gate1
topic: ci-history-1-shallow-history-correction
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: contribution-auditor
    status: waived
    reason: >-
      Single-agent Gate 1 planning session; no contribution-auditor
      available to consult directly at plan-filing time. Review is
      explicitly requested via a coordination handoff filed alongside
      this plan. Main risk to check: that the job-local `GIT_DEPTH: "0"`
      scope stays bounded to exactly the commissioned job and does not
      silently widen fetch cost for unrelated jobs.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      infosec-sentinel has no configured human/AI player (`played_by:
      "ci (agov validate infosec, on commit)"` per
      `agf/agents/MANIFEST.yaml`). This plan changes CI checkout depth
      for one existing job; it adds no new external input-parsing
      surface, credential, or network-facing endpoint. The correction
      makes more git history available to a job that already runs
      inside the project's own trusted CI, not to less-trusted or
      external code.
  - role: build-engineer
    status: consulted
    reason: >-
      This plan changes `.gitlab-ci.yml` job configuration (checkout
      depth), which `agf/agents/MANIFEST.yaml` names as build-engineer's
      jurisdiction ("owns CI configuration, build automation, ...
      pipeline efficiency"). build-engineer is `played_by: "claude (AI,
      CI/build and release-build work)"` — the same underlying model as
      planner-architect in this session, so this consultation is
      self-review by role, not independent verification; requesting an
      independent read from contribution-auditor and/or coder-tester
      covers the same ground this plan's own author cannot certify
      alone.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: CI-HISTORY-1 — job-local complete history for `rust:tooling:test`

## Decision requested

Approve implementation of the technical direction the governor already
selected in `20260826T043649Z-governor-reviewer-3j3p`: add `GIT_DEPTH: "0"`
to the `rust:tooling:test` job only, so its checkout has complete git
history rather than the project's default depth-20 shallow clone. This
plan freezes the exact scope, evidence, and acceptance criteria `3j3p`
requires before implementation may proceed.

This is a Gate 1 plan only. It does not authorize Gate 2, merge, tag,
release, or retrying an unchanged failed job for cosmetic green status.

## Problem and evidence boundary

`rust:tooling:test`'s `cargo test --workspace` runs
`real_mr1712_generation_acceptance_is_142_included_plus_50_excluded`
(`tooling/rust/crates/agov/src/commercial_corpus.rs:1127`), which calls
`build_manifest` with a pinned `corpus_content_revision` of
`48049759dfd9c681863d510ac6f7971235c5b0fb`. That function resolves file
contents at that exact commit via `git show <rev>:<path>` (through
`sha256_git_path`/`candidate_paths_at_revision`). Under the project's
default depth-20 shallow CI checkout, that commit is not reachable —
`git show 48049759...:.gitlab-ci.yml` fails with "path exists on disk, but
not in <rev>" — so the test panics on `unwrap()` at
`commercial_corpus.rs:1146`.

Confirmed independently on two unrelated MRs (!2041 — the review-convergence
work; !806 — this MCP transport/parity work) at three separate exact heads:
identical failure, identical panic message, 139 other tests in the same job
pass, every other CI job in both pipelines passes. Confirmed locally: the
same test passes from a full (non-shallow) local clone. This is a CI
checkout-depth defect, not a defect in either MR's own changes.

The second constant in the same test, `INVENTORY_REVISION =
"d13bb9084fce9a9b1c7c08bb735e0fe6d08d7e98"`, is recorded only as string
provenance metadata (`manifest.provenance.source_inventory_commit`) — it is
never passed to a git-resolving function. Only `CORPUS_REVISION` requires
git history to be present; the fix's scope is bounded accordingly.

## Selected design (governor-approved in `3j3p`)

Add `GIT_DEPTH: "0"` to `rust:tooling:test`'s own `variables:` block in
`.gitlab-ci.yml`, disabling shallow clone for that job only. GitLab CI
applies `GIT_DEPTH`/`GIT_STRATEGY` per job; setting it in one job's
`variables:` does not change any other job's checkout behavior, including
other jobs that extend `.rust-base`.

**Alternatives considered and rejected, per `3j3p`'s explicit direction:**

- **Explicit pinned-revision fetch** (`git fetch --depth 1 origin
  48049759...`) before the test runs. Rejected as the *default*: adds a
  bespoke pre-test step outside Cargo's own test harness, must be kept in
  sync by hand if the pinned revision ever changes, and only solves this
  one currently-known pinned commit — a future commercial-corpus revision
  bump would silently reintroduce the same failure mode with no fetch step
  updated to match. Recorded as the fallback if job-local full history
  proves too costly (see "Fetch cost measurement" below).
- **Self-contained fixture** (bundle the exact historical file content
  needed instead of resolving it from git at test time). Rejected: this is
  explicitly the "real historical-evidence path" `2rje` requires preserved
  — the test's entire purpose is proving the manifest generator resolves
  real pinned git history correctly, not a canned fixture. Replacing the
  git resolution with a bundled fixture would silently stop exercising the
  behavior the test exists to catch regressions in.
- **A fixed larger shallow depth** (e.g. `GIT_DEPTH: "200"`). Rejected as
  not durable: `48049759...` is already unreachable at depth 20 from
  current `main`; repository growth (this project accumulates commits
  quickly — `main` was 2381 commits ahead of one MR's stale base within
  weeks) will eventually push any fixed depth past the pinned commit again,
  reintroducing CI-HISTORY-1 with no code change to explain why.

## Affected files

- `.gitlab-ci.yml` — one job's `variables:` block
  (`rust:tooling:test`, currently at line 985). Add `GIT_DEPTH: "0"`
  alongside the existing `RUST_CACHE_ROLE: "writer"` override. No other
  job's configuration changes.

No supporting CI-contract test or operator documentation is added. Per
`3j3p`'s instruction to name and justify any additional scope: the
regression this fix prevents is self-verifying — if `GIT_DEPTH: "0"` is
ever accidentally removed from this job, the exact same real
`real_mr1712_generation_acceptance_is_142_included_plus_50_excluded` test
that currently fails without it will simply start failing again,
immediately and loudly, in the same job, with the same diagnostic message.
A separate meta-test asserting the YAML contains `GIT_DEPTH: "0"` would
duplicate that existing signal without adding new protection, at the cost
of a new validator this project would need to maintain — rejected under
`simplicity-rule.md`.

## Migration needed

No. CI configuration only; no schema, code, or data change.

## Fetch cost measurement (required before Gate 2, per `3j3p`/`2rje`)

A full clone is unavoidably slower than a depth-20 shallow clone; the
magnitude is not yet measured. Gate 2 evidence must include the actual
`rust:tooling:test` job duration before and after this change (visible
directly in the corrected exact-head pipeline's job trace, compared
against a recent unmodified-job baseline such as pipeline `2791114960`'s
`rust:tooling:test` job `16107811743`, which ran in the shared runner
environment before this fix). If the measured cost is large enough to be
a real operational concern (this plan does not pre-judge a threshold —
that is a Gate 2 judgment call against the actual number), the fallback
explicit pinned-revision fetch design above is the recorded next
candidate, not a fixed larger shallow depth.

## Fork / MR behavior

`GIT_DEPTH: "0"` is a per-job GitLab CI variable applied identically
regardless of pipeline source (merge request, push, tag, schedule, fork).
It does not depend on `CI_PIPELINE_SOURCE`, `CI_MERGE_REQUEST_*`, or
protected-ref status, so no fork-specific or MR-specific branching logic
is needed in the job definition. A fork's own CI runs under the fork's own
project settings; this change only affects jobs running under this
project's own `.gitlab-ci.yml` on this project's runners.

## Test strategy (Gate 2 evidence)

1. **Pre-fix reproduction** (already gathered, cited above): shallow-clone
   failure confirmed across three exact heads on two unrelated MRs.
2. **Post-fix exact-head pipeline**: a fresh pipeline on the corrected
   `.gitlab-ci.yml`, run against an exact head that includes this change,
   with `rust:tooling:test` green, including
   `real_mr1712_generation_acceptance_is_142_included_plus_50_excluded`
   passing for real (not skipped, not an unrelated-job retry).
3. **Both pinned revisions preserved**: `CORPUS_REVISION` remains
   git-resolvable (proven by the passing test itself);
   `INVENTORY_REVISION`'s provenance-metadata role is unchanged (this plan
   does not touch `commercial_corpus.rs`).
4. **139-other-tests-in-job regression guard**: the same job's full
   `cargo test --workspace` run must show no new failures beyond the
   fixed test resolving — a full local run already confirms this (140/140
   passing from a complete local clone, per MR !806's own verification
   evidence).
5. **Fresh corrected exact-head pipelines for the affected work**: MR
   !2041 and MR !806 each need a new pipeline run against a head that
   includes this correction, per `2rje`'s explicit requirement — not a
   retry of their existing failed jobs.
6. **Fetch-cost evidence**: the before/after job-duration comparison
   described above.

## Downstream impact (`downstream-impact-rule.md`, `tooling/**`-adjacent trigger)

This plan touches `.gitlab-ci.yml`, one of `always/downstream-impact-
rule.md`'s trigger paths.

```
Impact:
  Foundry:          ADDITIVE. Removes a standing, already-diagnosed CI
                     false-failure affecting any MR whose pipeline
                     reaches `rust:tooling:test` after the pinned
                     commercial-corpus revision falls outside the
                     default shallow-clone depth. No existing passing
                     behavior is removed.
  Direct adopters:  Not applicable — CI pipeline configuration internal
                     to this project's own GitLab instance; adopters
                     consuming released `agov`/AGF artifacts are
                     unaffected. No schema, CLI, or artifact-shape
                     change.
  Downstream:       No change to any released artifact, schema, or
                     tool behavior. The only effect is CI checkout
                     depth for one job.
  Migration:        None required.
  Evidence:          `20260825T052314Z-coder-tester-ejg2.md`,
                     `20260825T052315Z-coder-tester-6djh.md` (original
                     MR !2041 diagnosis); `20260825T203938Z-planner-
                     architect-j3rm.md` (MR !806 hitting the identical
                     failure); `20260826T042010Z-governor-reviewer-
                     4ssr.md` (commission);
                     `20260826T043649Z-governor-reviewer-3j3p.md`
                     (selected design and conditional Gate 1
                     direction).
```

**Adopter docs parity: not applicable.** No adopter-facing Pages page,
runbook, or AI-loader documents this project's own internal CI checkout
depth. No `documentation-rule-projection-rule.md` update is required.

## Risks

- **Risk:** full-history clone cost turns out unexpectedly large (e.g. if
  a runner-side network or storage constraint makes a full clone
  materially slower than budgeted). **Mitigation:** the fetch-cost
  measurement step above is a required Gate 2 evidence item specifically
  so this is discovered before merge, not after; the explicit
  pinned-revision-fetch alternative is already recorded as the fallback
  design if this risk materializes.
- **Risk:** scope creep — widening `GIT_DEPTH: "0"` to other jobs "while
  we're in the file." **Mitigation:** the affected-files section names
  exactly one job's `variables:` block; any broader depth change is
  explicitly out of scope and would need its own Gate 1 plan and cost
  justification, per `3j3p`'s own scope boundary.
- **Risk:** this fix alone does not itself unblock MR !2041 or MR !806 —
  both still need a fresh corrected exact-head pipeline after this change
  lands on their base, which is separate follow-up work, not automatic.
  **Mitigation:** named explicitly in the test strategy above so it is
  not silently assumed.

## Rollback

Revert the single `GIT_DEPTH: "0"` line. `rust:tooling:test` returns to
the project's default shallow-clone depth; CI-HISTORY-1 recurs under the
same conditions as today. No data, schema, or artifact state to unwind —
this is CI configuration only.
