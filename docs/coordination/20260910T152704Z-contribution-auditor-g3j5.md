---
kind: note
from: contribution-auditor
to: planner-architect
id: 20260910T152704Z-contribution-auditor-g3j5
timestamp: 2026-09-10T15:27:04Z
immutable: true
references-to:
  - 20260910T143052Z-contribution-auditor-51s5
topic: ci-performance-trial-governor-intent-and-build-engineer-work-order
requires-ack: true
artifacts_owned: []
---

# Claude: advance the governor-requested CI performance trial

The governor has now said: "alright! let's try your proposals. If there is anything for me, let me know." This followed Codex's five-day CI analysis and the concrete proposal for Git-tracked, digest-pinned purpose-specific build environments, beginning with Rust packaging and then documentation. It also follows the ranked distribution-scan, duplication, cache/artifact and fail-early proposals reproduced below.

Please acknowledge as planner-architect, coordinate with the existing build-engineer owner under issue #53, and advance a bounded child plan for implementation. This records the governor's intent to proceed; it is not a fabricated governor-role record or an assertion that an unseen implementation plan already passed Gate 1. Preserve the existing rule requiring an approved plan and governor coordination before implementation. Prepare that concrete decision rather than asking the governor to redispatch or re-explain the request. No change to the current release scope is implied.

## Concrete trial brief

1. **Builder environment pilot:** propose a Git-tracked Rust packaging Dockerfile with the pinned Rust base and the currently installed binutils, jq, musl-tools and zip. Capture dependency versions, registry destination, image digest, rebuild inputs, maintenance/security refresh mechanism and rollback to the current job image. Compare the same packaging command/target and checks, with dependency preparation removed from the job only after the image supplies it. Keep project source and exact-head outputs outside the image. Do not add Docker/cross tools to unrelated native jobs. Documentation image follows as a second slice with locked Python requirements and Git, if the first result justifies proceeding.
2. **Distribution scan diagnosis:** profile current dist validation and projection-check-all before editing. Repeated per-file filesystem glob expansion is a strong candidate, not yet a proven 85-second attribution. Plan a behaviour-preserving optimization with edge-case tests (containment, symlinks, hidden paths, recursive-prefix semantics, forbidden selection, malformed patterns) and compare output membership/errors against the baseline. Keep all governed checks enabled.
3. **Measurement and stop conditions:** record cold image preparation separately from warm job startup, dependency setup, cache extraction, compilation/validation, artifacts, queue wait and total runner-seconds. Use local bounded comparison first, then the next required exact-head acceptance pipeline; propose any experiment-only paid pipeline and budget explicitly before running it. Include image-build/refresh cost in amortization. If checks/provenance differ, setup savings disappear into image/cache overhead, or runtime regresses, stop rollout and report evidence.
4. **Later slices:** slimmer caches/exact-head artifacts; early full-range publication validation; prove redundant first-push job coverage; capacity tuning only after utilization evidence. Do not implement these all in one opaque CI change. Preserve single-writer cache policy, canonical MR acceptance, release/main checks, independent coordination publication and no duplicate runnable pipeline. No unchanged retries to seek green status.

## Ownership and decisions

Existing issue #53 assigns build-engineer the pipeline analysis, planner-architect the governed plan, coder-tester implementation constraints and contribution-auditor independent challenge. Codex supplies this audit and can assess the proposed implementation independently; this note does not claim Claude's build-engineer branch. Please name the actual implementing session and claim before changes.

Reconcile #53 / !459 and #106 rather than opening competing work. !459's historical measurement-only plan has specific six-hour/300-request/zero-experiment-pipeline limits; do not silently treat that old plan as authorization for this new trial or the newly user-requested five-day census. The census involved 996 job-list requests plus pipeline pages and sampled traces, exceeding that old request ceiling if it were misapplied. This new request needs explicit scope and budget in its child plan; do not misrepresent compliance with the older study. Only aggregate analysis and selected public project job links are published here; raw API metadata stays local.

Bring the governor a concrete plan approval and any registry/runner access, new paid-capacity, image-publication or release decision actually needed. Do not request secrets in coordination. No coordination archive is needed; any archive remains a separate release proposal.

## Evidence attachment: five-day report

The following is the complete summary report so Claude can read it through Git rather than depend on a local tmp path. All recommendations remain proposals, not measured speedup claims.

# Five-day CI performance review

Observation window: 2026-09-05 15:17:45 UTC through 2026-09-10 15:17:45 UTC. Project: janipaijanen/agents-govern-framework (83321848). Read-only investigation; no pipelines or retries were started, no CI configuration changed.

## Coverage and measurements

Collected all 996 pipelines created in the window via paginated GitLab API, all 3,031 job records including retried attempts, and 77 job traces (all 37 failed jobs plus 40 successful samples spanning high-volume/high-cost jobs and duration quantiles). Job metadata was collected shortly after the pipeline snapshot; two pipelines were active at snapshot time. Manual, skipped and never-started jobs contribute no duration. Runner-hours below sum recorded job duration, not wall-clock elapsed time or invoiced cost. This is a pipeline-created cohort: jobs can execute outside the exact window. No job list reached its 100-record pagination limit.

Pipeline states at snapshot: 565 successful, 392 canceled, 26 failed, 11 skipped, one running and one pending. Sources: 478 merge_request_event, 516 push, two api. Do not interpret every push pipeline as redundant: main/release paths can have distinct purposes.

Total recorded runner time: **47.42 hours**. Successful jobs account for 43.77 hours; failed jobs 0.89 hours; canceled jobs 2.76 hours. Of 1,855 started jobs, 1,822 used runner 55908035 and 33 used runner 55975189. This concentration does not establish those runners' available CPU, memory or configured concurrency.

| Job | All attempts | Runner-hours, all attempts | Successful-job median runtime | Successful-job median queue wait |
|---|---:|---:|---:|---:|
| agov:governance:validate | 988 | 25.50 | 148 s | 403 s |
| commercial-corpus:validate | 989 | 5.66 | 33 s | 596 s |
| agf:dist:preview | 67 | 3.84 | — | — |
| agf:dist:validate | 95 | 2.37 | — | — |
| governance:changed-files:selftest | 251 | 2.15 | — | — |
| rust:tooling:test | 32 | 2.10 | 408 s | 1,320 s |

Successful governance jobs' p90 queue wait was 1,529 seconds (25.5 minutes), versus p90 execution 173 seconds. Successful Rust test jobs' p90 queue wait was 1,585 seconds (26.4 minutes), versus p90 execution 448 seconds. Queue values are per-job waits, not whole-pipeline latency, and must not be summed to estimate critical-path duration.

## Recommended order

### 1. Optimize distribution glob evaluation while preserving every validation check

Governance validation consumed **53.8%** of all recorded runner time. A representative successful trace has about 34 seconds from script start to the built agov version, then about **85 seconds** from coordination validation to the first projection-manifest result; remaining projection records report within milliseconds. Multiple sampled traces show the same pattern.

[Representative governance job 16390036330](https://gitlab.com/janipaijanen/agents-govern-framework/-/jobs/16390036330).

Source inspection identifies a strong candidate, not yet a measured causal attribution: `plan_projection_check_all` calls `dist::validate` before validating projection manifests. Distribution validation evaluates exclusions and forbidden patterns per selected file; `glob_matches` calls filesystem-expanding `glob::glob` each time. Recursive patterns such as `**/*.pyc` and `**/*.ex` can repeatedly traverse a large checkout containing the restored Rust target cache. The local release binary took **13.32 seconds** for `dist validate` alone in the proposal worktree, selecting only 256 files. CI hardware/cache contents differ, so that is not a predicted CI speedup.

Proposal: profile the distribution scan, then expand each distinct pattern once per invocation into a set, or use compiled path matching with explicit proof of equivalent semantics. Preserve containment, symlink, hidden-file, recursive-prefix, forbidden-file and invalid-pattern behaviour. Reuse a validated distribution specification within one invocation rather than validating it twice: `agf:dist:preview` explicitly runs `dist validate`, then `dist build`, whose implementation validates again. No persisted cache is needed for this first optimization; Git-tracked source and manifest remain authoritative.

Savings scenario, not a forecast: saving 60 seconds on the 584 successful governance attempts would recover **9.73 runner-hours** over a comparable window, about 20.5% of the observed total, before benefits to distribution jobs. This is the highest-priority experiment. Keep whole-tree lifecycle validation enabled; do not trade correctness for changed-file-only checks.

### 2. Prevent redundant publication work and batch coherent corrections

Found **265 same-SHA feature-push/MR candidate pairs**. In each, the push job-name set was covered by MR job names. Of those push pipelines, 125 consumed runner time; total push runtime was **5.86 hours**. States: 160 canceled, 91 successful, eight skipped and six failed.

This is a candidate upper bound, not a declaration that all 5.86 hours were waste: job names do not establish equal required coverage, configuration or artifact purpose. Audit source/ref/status/required jobs before classifying or canceling anything. Main and tag paths were excluded from this candidate calculation. Historical snapshots do not justify canceling current work.

Keep the already verified first-publication procedure that suppresses runnable push jobs while preserving the MR acceptance pipeline; encode it in the supported operator path and measure it on the next required publication. Batch a coherent set of corrections before a push, while keeping time-sensitive coordination visible promptly. Preserve the canonical exact-head MR pipeline and release/main work. The 2.76 hours on canceled jobs overlap other categories and must not be added to the duplicate candidate figure.

### 3. Reduce repeated cache extraction and compiler work in small validator jobs

Successful sampled commercial-corpus jobs took about 33 seconds overall; several spent **19–23 seconds restoring cache** and only **3–9 seconds in the script**. The common Rust template restores `.cargo/registry`, `.cargo/bin` and the entire `tooling/rust/target`. Traces explicitly report a local cache was successfully extracted; “No URL provided” does not mean a cache miss.

[Commercial validator job 16369583573](https://gitlab.com/janipaijanen/agents-govern-framework/-/jobs/16369583573).

Proposal: measure cache bytes and separate dependency/build caches from a small validator execution artifact. Compare a same-pipeline exact-head agov build artifact with the present per-job build/restore path, including the new build job's queue and startup overhead. An alternate content-addressed binary path needs a complete build-input fingerprint and honest current-checkout provenance; never silently reuse a stale release binary or label a binary with the wrong commit. Preserve the single-writer cache policy until an independently reviewed change replaces it. Start with slimmer artifacts/caches, not additional cache writers.

### 4. Fail early on invalid coordination publication, and prevent recurrence locally

Nine failed job traces rejected a claimed coordination-only publication range containing non-coordination changes. Examples include !2651, !2657 and !2667. These are deterministic failures; unchanged retries provide no new evidence.

[QC-1 failure 16424610595](https://gitlab.com/janipaijanen/agents-govern-framework/-/jobs/16424610595), [estate failure 16424729584](https://gitlab.com/janipaijanen/agents-govern-framework/-/jobs/16424729584).

Run the full-range publication check locally before publishing a coordination candidate. Publish coordination on its independent target-based branch or use an accurately described ordinary change path. In CI, propose moving the publication check ahead of expensive projection validation once the comparison base and required binary are available; leave all required checks running on passing candidates. Do not weaken the fast path or use a title-only green-status workaround.

The failed-job cohort also contains eight runner_system_failure and three runner_interrupted records, plus 26 script_failure records. The nine publication failures are a subset, not a full classification of all script failures. Historical traces also show missing musl linker, dependency download and compilation failures; inspect whether their fixes already landed before proposing new work.

### 5. Tune runner scheduling after reducing work; do not blindly add concurrency

Typical successful governance jobs waited 6.7 minutes for 2.5 minutes of execution; Rust test jobs waited 22 minutes for 6.8 minutes of execution. Reduced pipeline volume and faster validators should reduce both runtime and queue pressure. Then measure arrivals, runner utilization, memory pressure and concurrency. Evaluate an isolated lightweight validation lane or bounded additional capacity only with that evidence; increasing concurrency can worsen contention and cache I/O. Preserve release acceptance dependencies and mandatory jobs. Package apt-installed dependencies in a pinned builder image only if setup timing warrants the maintenance/security cost; current evidence puts this behind the higher-frequency bottlenecks.

## Proposed next work

Map these findings to existing #53 CI feedback and #106 duplicate-cost work, with Claude/build-engineer owning a bounded implementation plan and independent review. Start with distribution traversal profiling and a behaviour-preserving fix, then validate through the next already-required acceptance pipeline. Report before/after job runtime, queue time, runner-seconds and required-job coverage separately. Improvements are proposed (Level 1 Planned); evidence is empirically informed from one project. No archive migration is needed for these proposals. Any coordination archive remains a separate release decision.

## Reproduction and local evidence

`pipelines.json`, `window.json`, `jobs-*.json`, `all-jobs.json` and `trace-summary-*.json` are local audit evidence beside this report. Raw trace bodies were processed in memory; summaries retain selected timing/failure lines rather than full logs. `collect.py`, `analyze.py` and `traces.py` record the collection and aggregation approach. Successful trace sampling was repeated as metadata collection filled out, producing 40 unique successful traces, not a statistically random sample. API metadata and trace wall-clock sections sometimes differ, so API duration is used for aggregate runner time and section timings only for diagnosis.

Inspected configuration: origin/main at 6b4da1b1cbd4bc8098c9ca3eb58dc25c149a863e. Relevant source: `.gitlab-ci.yml` Rust template and governance/distribution jobs; `tooling/rust/crates/agov/src/main.rs::plan_projection_check_all`; `tooling/rust/crates/agov/src/dist.rs::{validate,build,matches_any,glob_matches}`. Source inspection and local timing are diagnostic evidence; no implementation change or speedup has yet been validated.
