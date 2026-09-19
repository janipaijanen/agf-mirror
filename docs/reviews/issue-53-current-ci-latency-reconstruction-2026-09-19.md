<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Issue #53 CI latency evidence reconstruction

**Reconstruction date:** 2026-09-19  
**Evidence status:** Empirically informed; Foundry dogfooding only  
**Authority boundary:** Measurement and evidence preservation only. This record
does not authorize a CI change, cache or artifact reuse, job removal, release
action, or a weaker acceptance path.

## Why this carrier exists

MR !810 contains useful longitudinal evidence but cannot merge as-is. Its
history includes coordination commit `74723dca` with a forbidden commit-message
`[ci skip]` marker. The independent review in
`20260730T084357Z-planner-architect-2arh` therefore returned `PARTIAL` with a
blocking publication-history finding while independently corroborating the
report's main measurements.

This file reconstructs the durable evidence from MR !810 at exact source head
`15c06c859ef64d3e7d7b35833f430137348609cb`, then refreshes it against the
September five-day census and the current CI configuration. It carries the
four preservation requirements recorded on issue #53 on 2026-09-19. It does
not import MR !810's unsafe history.

## Preserved July evidence

The original assessment covered 3,154 pipelines, 10,933 jobs, 804 merge
requests, and 90 issues from 2026-06-14 through 2026-07-30. Its cache finding
used a stratified 60-trace sample across four Rust jobs from weeks 25, 29, and
the partial week 30.

| Cohort | Samples | Mean restore + archive | Share of sampled job duration |
|---|---:|---:|---:|
| Week 25 | 12 | 86 s | 45.5% |
| Week 29 | 12 | 101 s | 44.3% |
| Week 30 partial | 12 | 179 s | 56.3% |

In the week-30 sample, `agov:governance:validate` averaged 174 seconds of cache
handling in a 243-second job, or 71.7%. `rust:tooling:test` averaged 188 seconds
in a 262-second job, or 71.8%. The result supports a cache-transport problem in
that sample. It does not establish a universal percentage, invoice cost, or a
safe implementation choice.

### Independent out-of-sample corroboration

The planner-architect review
`20260730T084357Z-planner-architect-2arh` inspected four fresh jobs from MR
!804, outside the report's extraction. It measured cache shares of 73.7% for
governance validation, 71.4% for Rust tests, 46.0% for `rust:tooling:deny`, and
38.9% for distribution validation. Three results were within about two
percentage points of the report's corresponding week-30 mean; distribution
validation was about seven points lower. This corroborates the mechanism and
rough magnitude while retaining the small-sample limitation.

### Queue-metric counterexample

The same independent review found four jobs with GitLab `queued_duration`
below two seconds but `created_at` to `started_at` gaps of about 726 to 728
seconds:

| Job | `queued_duration` | Creation-to-start gap |
|---|---:|---:|
| `rust:tooling:deny` | 0.75 s | 727.1 s |
| `rust:tooling:test` | 1.71 s | 728.3 s |
| `agov:governance:validate` | 0.71 s | 727.1 s |
| `agf:dist:validate` | 0.15 s | 726.2 s |

`queued_duration` therefore cannot support a broad claim that waiting is
negligible. Future baselines must report both GitLab's queue field and the
creation-to-start interval. The counterexample does not identify whether the
unmeasured delay came from runner scarcity, GitLab pipeline processing, stage
dependencies, or another mechanism.

## September refresh

The five-day census embedded in
`20260910T152704Z-contribution-auditor-g3j5` covered 2026-09-05 15:17:45 UTC
through 2026-09-10 15:17:45 UTC. It collected 996 pipelines, 3,031 job records,
and 77 traces. Recorded runner time was 47.42 hours. Governance validation used
25.50 hours, or 53.8% of the total. Successful governance jobs had 148-second
median execution and 403-second median queue wait; successful Rust tests had
408-second median execution and 1,320-second median queue wait. These are
per-job values and must not be summed into a pipeline critical path.

The September trace review identified two distinct cost candidates:

- distribution validation repeatedly expands filesystem globs over a checkout
  that can contain a restored Rust target tree; source inspection makes this a
  strong profiling candidate, not a proven 85-second causal attribution;
- small validator jobs still pay cache transport and compilation overhead. In
  sampled commercial-corpus jobs, 19 to 23 seconds were cache restoration while
  the actual validation script took 3 to 9 seconds.

The current `.gitlab-ci.yml` is materially different from July. Cache-policy
Slice B, merged in commit `7f95807c`, introduced a versioned `rust-v3-main`
namespace, pull-only consumers, and a controlled default-branch writer. That
reduces branch-cache publication and stale-branch amplification. The cache
still includes `.cargo/registry`, `.cargo/bin`, and the complete
`tooling/rust/target`, so the July result remains historical evidence rather
than a current effect-size estimate. A current experiment must measure bytes,
restore/archive time, compilation, cold and warm branches, and the added
producer or artifact path before selecting a change.

## Duplicate first-publication refresh

The September census found 265 same-SHA feature-push/MR candidate pairs whose
push job names were covered by their MR job names. Of those push pipelines, 125
consumed 5.86 runner-hours. This is an upper bound: equal SHA and job names do
not prove equivalent purpose, configuration, required coverage, or artifact
provenance.

The current workflow suppresses push pipelines only after GitLab can observe
an open merge request. A brand-new branch can therefore race ahead of MR
registration. The supported publication path now prevents runnable duplication
at the push handshake: push the new branch with the `ci.skip` push option,
immediately register the MR, and retain the canonical MR-event pipeline. This
is a push option, not a forbidden commit-message marker.

The latest local use provides a bounded operational example at commit
`1ef77ca01056e43eba0941899aca4e705552351e`: pipeline `2863789511` is a skipped
push record and pipeline `2863789593` is the successful MR-event acceptance
path for MR !3012. The example proves the procedure worked once; it does not
establish a sustained savings rate. Paths that cannot use the prevention step
still require source/ref/status/job-coverage comparison before canceling a
demonstrably redundant push pipeline.

## Current conclusions

1. Cache transport was a large measured share of the July sample and was
   independently corroborated. Cache-policy Slice B changed the mechanism, so
   July percentages must not be presented as current performance.
2. September evidence keeps cache size, distribution traversal, and queue wait
   as live measurement targets. It supports profiling and a bounded child
   experiment, not a speedup claim.
3. GitLab `queued_duration` and creation-to-start latency are separate metrics.
   Both are required for a trustworthy feedback-latency baseline.
4. Publication-time duplicate prevention is the current operator control. One
   successful example validates the path at Level 4 for that case; continued
   monitoring is required before a broader empirical claim.
5. MR !810 should close as superseded only after this carrier passes review and
   merges. MR !459 remains a separate historical plan/evidence source until its
   unique plan and normalized-data content receive an explicit disposition.

## Provenance and limitations

- July report source: MR !810, exact head
  `15c06c859ef64d3e7d7b35833f430137348609cb`.
- Independent July challenge:
  `20260730T084357Z-planner-architect-2arh` on current `main`.
- September census and recommendations:
  `20260910T152704Z-contribution-auditor-g3j5` on current `main`.
- Cache-policy implementation: commit `7f95807c` and
  `docs/plans/ci-rust-cache-policy-slice-b.md`.
- First-publication example: MR !3012, pipelines `2863789511` and `2863789593`.

The July and September datasets use different windows, pipeline populations,
job mixes, and CI configurations. Their values are not a before/after causal
comparison. Raw API extracts and traces are not republished here. All cost
figures are recorded job duration rather than billed currency.
