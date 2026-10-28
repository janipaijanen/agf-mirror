<!-- Radar carrier. Retained verbatim from MR !459 (source branch plan/issue53-blinded-ci-followup, head 09b5147f), file docs/proposals/issue-53-feedback-latency-evidence-2026-07-24.md.
     Non-authoritative: parked on the radar, grants no Gate 1, rule, template, validator or implementation authority. -->
# Issue #53 feedback-latency evidence — 2026-07-24

## Outcome

The v2.4.0 correction and release window is reconstructed, but it does not
justify a CI change yet. The strongest observed mechanism is stage
serialization, not runner queueing. The same evidence also shows why the final
main and tag pipelines must not be called waste merely because they share a
SHA: their trust contexts and release obligations differ, and exact execution-
environment and artifact-equivalence evidence is missing.

The appropriate next step is one natural stable-SHA procedural trial. Cache or
artifact reuse, auto-cancellation, job skipping, and a fail-fast DAG change are
deferred. No child implementation plan is proposed from this single release
case: its seven primary product/release pipelines succeeded, but an ancillary
MR !442 review pipeline was canceled after consuming runner work.

Evidence status is **Empirically informed** for the observed timing and graph
findings. The expected benefit of the procedural checklist remains
**Theoretical** until the natural trial is observed. The checklist is AGF
capability maturity **Level 2 — Surface-defined**; no new CI capability was
instantiated.

## Goodhart boundary

The numeric fields in this report are observations or stop-losses, not targets.
This follows the project [Goodhart lens](../../agf/docs/site/docs/concepts/philosophical-lenses.md#goodharts-law):

- pipeline wall time, reviewer minutes, API calls, and runner seconds may expose
  harm or locate delay, but no person or mechanism is ranked by them;
- the approved ceilings bound resource consumption; finishing below a ceiling
  is not evidence of success, while reaching one pauses work;
- a faster result fails if assurance coverage, exact-SHA identity,
  accessibility, provenance, review quality, failure timing, or displaced cost
  worsens;
- findings retain the complete metric vector, missing fields, and adverse
  effects instead of collapsing them into a score or percentage;
- if observation changes behavior—for example by encouraging under-reporting
  reviewer time or avoiding a required comprehensive bake—the affected metric
  is withdrawn from gating and the plan is reviewed.

Success therefore remains conjunctive: earlier trustworthy feedback,
equivalent assurance obligations, no displaced human or CI cost, and preserved
release controls.

## Dataset and provenance

The normalized machine-readable dataset is
[`data/issue-53-feedback-latency-normalized-2026-07-24.json`](data/issue-53-feedback-latency-normalized-2026-07-24.json),
SHA-256
`3b21a45e3e61f3229bf3189b80d2ab4e30d5f4e0864f9c12f5c92f861497af19`.
It contains only the approved allowlisted fields.

Collection used GitLab API v4 through `glab 1.103.0`. The frozen query window
was `2026-07-23T00:35:00Z` through `2026-07-23T02:30:00Z`; its single page
contained 54 candidates. The published ledger classifies all 54: seven primary
product/release pipelines, canceled review pipeline `2698764772` as ancillary
evidence, and 46 exclusions. Fourteen exclusions fall outside the frozen
`created_at` boundary, 30 are skipped commits not on the product/release chain,
and two are the empty non-product ancestor `33b189d8...`.
`git merge-base --is-ancestor` confirms that none of the 30 skipped exclusions
is an ancestor of final release SHA `0d3e18e...`; `git show --name-only`
confirms that `33b189d8...` changes no files.

The dataset contains all 69 returned job rows for the eight included pipelines,
queried with `include_retried=true`. Attempt numbers are derived by grouping on
pipeline ID and job name and sorting by job ID. This makes every job sum,
queue sum, retry identity, and successful critical-chain timestamp auditable
from the durable artifact.

At the `2026-07-24T00:10:23Z` dataset freeze, collection had used 43 API
requests and operational review 12, for 55 total against the 300-request
stop-loss. The operational count includes the contribution-auditor's ten
post-freeze requests that exposed the canceled pipeline omission. Later
review/validation traffic still consumes the plan ceiling but does not mutate
the frozen dataset.

The historical CI file at
`0d3e18eae8c0fcf3cc4362bfa4a974675a5c5318` and the measurement-start main CI
file at `29d9173a58e17dde28bf57b394279608a8ef416a` are byte-identical, with
SHA-256 `95312dd57637e1001b80ea48625b7fdaf9bda5a971e5cc2face28ce614e2da14`.
This establishes a matched structural baseline; it does not establish matched
runner images or cache state.

The retained local raw snapshots are separately controlled by governor
decision `20260723T232534Z-governor-reviewer-b1ds` and are due for deletion on
2026-08-31 unless extended. Their hashes are supplementary provenance; this
report and normalized dataset do not depend on access to private assessment
content.

Promotion remains draft pending explicit governor-reviewer confirmation of the
final report and dataset hashes. Presence on the MR is not itself that approval.

## Allowlist transformation and privacy validation

The transformation used `glab 1.103.0` and `jq 1.7`. Pipeline responses were
projected explicitly as:

```jq
{
  pipeline_id: .id, sha, ref, source, status, created_at, started_at,
  finished_at, api_duration_seconds: .duration,
  pipeline_queued_duration: .queued_duration
}
```

Job responses from
`GET /pipelines/{id}/jobs?include_retried=true&per_page=100` were projected as:

```jq
{
  pipeline_id: $pid, job_id: .id, name, stage, status, allow_failure,
  created_at, started_at, finished_at, duration, queued_duration
}
```

Derived pipeline rows may contain only: `pipeline_id`, `sha`, `ref_class`,
`source`, `status`, three timestamps, wall/API duration, executed/manual job
counts, execution/queue sums, critical-path IDs/execution/transition gap,
fixed-taxonomy reason, source pointer, and selection role. Job rows may contain
only the projected fields plus one-based `attempt`, `attempt_count`, and
`superseded_by_retry`. Candidate rows may contain only pipeline ID, SHA, ref
class, source, status, creation timestamp, selection, fixed-taxonomy reason,
and source pointer. The local validator declares exact keys for every top-level
and nested output object; an injected unknown output field is rejected.

The dated local validation assets are redacted or synthetic, mode `0600` in the
canonical primary-checkout `tmp/`, and share the 2026-08-31 cleanup date:

| Asset | SHA-256 | Purpose |
|---|---|---|
| real-shape positive | `f931d0b09be8726cdb9a670b5ed1bd6b3837ffc57727bfc5c90aa3339de5fe0c` | GitLab pipeline shape retains every allowed projection field |
| sensitive negative | `f408876693df4c6ebd8e44a31299cd63a1901e234d5151aa62e261126765f03f` | Synthetic user/email/avatar, runner/IP/system ID, token, log, comment, variable, and environment values do not reach output |
| pagination cases | `f48d7c9c0aa4bcd672957682ebc1ea8398dc2e1e2b6e2e1ac14dfbee6bac2e03` | Two real-shape pages plus malformed, truncated, repeated, missing, loop, and duplicate-ID cases |
| allowlist validator | `c88bb91d3a7ca914ef49723ea3575ca96c9ce7d51033798bf986abbd0007f954` | Exact-key output schema and pagination contract |

The positive real-shape projection and two-page traversal passed with two
unique item IDs matching provider `X-Total: 2` and `X-Total-Pages: 2` metadata.
Malformed metadata, truncation, repeated page, missing page, pagination loop,
duplicate item ID, and an injected unknown output field each returned nonzero.
The sensitive negative projection passed this output scan:

```jq
([.. | objects | keys_unsorted[] |
  select(test("^(user|email|avatar|avatar_url|runner|runner_manager|ip|ip_address|system_id|private_token|token|trace|log|comment|variables|environment_value)$"; "i"))] | length == 0)
and
([.. | strings |
  select(test("glpat-[A-Za-z0-9_-]{8,}|[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,}|(?:[0-9]{1,3}\\.){3}[0-9]{1,3}"))] | length == 0)
```

The same key/value scan and the exact-key schema passed on the final durable
dataset. Provider responses used for the final 54-candidate and 69-job
collection were transformed in pipes and were not written to disk. This does
not describe the two earlier raw timing snapshots: those are deliberately
retained under governor decision `20260723T232534Z-governor-reviewer-b1ds`,
hash-listed above, local-only, and due for deletion on 2026-08-31.

## Correction and release timeline

The inclusive window starts when the first product pipeline was created and
ends when the immutable tag pipeline finished:

- start: `2026-07-23T00:42:48.819Z`, pipeline `2698747650`;
- end: `2026-07-23T02:23:29.806Z`, pipeline `2698842544`;
- elapsed wall time: **6,040.987 seconds** (1 h 40 min 40.987 s).

The MR !442 product/review subwindow spans 2,407.054 seconds (40 min 7.054 s).
Its two durable inter-pipeline correction/re-review intervals total 639.324
seconds. Those intervals cannot be subdivided into authoring, reviewer, or
waiting time from available durable evidence, so no human-speed claim is made.

| Pipeline | Purpose | Created-to-finished | GitLab `duration` | Job execution sum | Job queue sum | Observed critical chain |
|---|---|---:|---:|---:|---:|---|
| 2698747650 | Initial MR !442 product SHA | 604.068 s | 590 s | 780.197 s | 0.370 s | dist preview → dist validate |
| 2698764772 | Ancillary MR !442 review pipeline, canceled | 105.383 s | 101 s | 530.702 s | 2.938 s | Not derived; cancellation cause unknown |
| 2698769464 | Semantic review correction | 580.480 s | 576 s | 963.314 s | 11.412 s | dist preview → dist validate |
| 2698792960 | Definition review correction | 583.182 s | 579 s | 941.271 s | 0.582 s | dist preview → dist validate |
| 2698804817 | Merge to main | 657.101 s | 617 s | 946.180 s | 0.422 s | dist preview → dist validate → Pages |
| 2698823782 | Release truth surfaces | 980.803 s | 976 s | 2,580.691 s | 5.304 s | Rust preview → cargo deny |
| 2698842259 | v2.4.0 merge on main | 1,039.771 s | 1,031 s | 2,451.733 s | 7.882 s | Rust preview → cargo deny → Pages |
| 2698842544 | Immutable v2.4.0 release | 1,521.234 s | 1,511 s | 2,696.392 s | 6.612 s | Rust release → cargo deny → dist release → create → verify |

Across the seven primary pipelines, 55 jobs started and four manual jobs did not
run; job execution totals 11,359.777 seconds and queueing 32.584 seconds. Adding
the ancillary canceled review pipeline yields 65 started jobs, 11,890.479 job
seconds, and 35.522 queue seconds. These sums are runner-work and queue
indicators, not elapsed time: jobs run concurrently and pipelines overlap.

The critical-chain transition gaps were small. Most successful-pipeline
latency followed the sequential build, test, and deploy/release structure.
All seven primary pipelines succeeded. The ancillary pipeline had four
successful and six canceled jobs, but cancellation is not an actionable test
failure and its cause/initiator is unavailable. Time to first actionable
failure and historical failure yield therefore remain unavailable. That
missing negative evidence blocks selection of a fail-fast job under the
approved plan.

## Historical and current DAG finding

The historical and current `.gitlab-ci.yml` files are identical, so the graph
delta is empty. The relevant default-stage structure is:

```text
build jobs in parallel
  → test jobs in parallel
    → default-branch Pages, or tag release/deploy dependency chains
```

Explicit tag `needs` edges then preserve artifact and publication order:

```text
rust:tooling:release
  → agov:oci:release ─┐
agf:dist:release ─────┼→ release:create → release:verify
rust:tooling:release ─┘
```

Starting independent tests earlier could theoretically shorten successful
wall time, but the current sample contains no failures with which to establish
a cheap, high-yield fail-fast candidate. Graph modification therefore remains
outside this report.

## Main/tag computation-equivalence ledger

Both final pipelines used
`0d3e18eae8c0fcf3cc4362bfa4a974675a5c5318`, but SHA equality alone is not
reuse evidence.

| Work | Observation | Classification | Reuse decision |
|---|---|---|---|
| Seven same-named test jobs: Clippy, Rust tests, cargo-deny, `rust:agov:validate`, dist validate, flyer verify, CVE scan | Same source SHA and CI definition; tag reruns final-release assurance. Resolved images, tools, cache state, and artifact digests are incomplete. | 3 — intentionally repeated final-release assurance, with environment equivalence unknown | Prohibited |
| `agf:dist:preview` on main vs `agf:dist:release` on tag | Similar build core, but release adds tag/VERSION guard, release naming, longer retention, and release provenance purpose. | 2 — similar computation with different purpose/provenance | Prohibited |
| `rust:tooling:preview` on main vs `rust:tooling:release` on tag | Shared packaging template, but tag builds the full release set, including release-specific identity and Windows output. | 2 and 3 — different artifact set plus intentional final assurance | Prohibited |
| Pages on main vs tag deploy jobs | Pages publishes documentation; tag jobs build, publish, pull, and verify release assets and OCI evidence. | Different computation and trust boundary | Not applicable |

Caches remain mutable performance inputs. No cache hit/miss outcome, expanded
key, writer trust context, resolved image digest, or immutable artifact digest
was available. No cache or artifact may satisfy an assurance obligation from
this evidence.

## Local-versus-CI parity inventory

| CI checks | Local command surface | Important parity gaps |
|---|---|---|
| `agf:dist:validate`, `agf:dist:preview`, `rust:agov:validate` | Local `agov dist validate`, `agov dist build`, and `agov validate` through the built Rust binary | CI uses broad `rust:1`, ref-scoped mutable caches, and CI provenance variables; resolved image/toolchain identity is unavailable |
| `agov:governance:validate` | Build `agov`, then validate changed coordination messages and plans | Correct comparison base, MR variables, fetched target history, and exact CI diff context must be reproduced |
| `rust:tooling:test`, Clippy, cargo-deny | `cargo test --workspace`, `cargo clippy --workspace -- -D warnings`, `cargo deny check` | Rust/tool versions and cached target contents can differ; cargo-deny is installed from a semver range |
| `site-html-build`, Pages | Palette checks and strict MkDocs builds from `.gitlab-ci.yml` | `python:3.14-slim` is not digest-pinned; pip ranges and Pages publication context differ |
| `agf:flyer:verify` | Python generator plus headless Chromium comparison | Moving apt Chromium and rendering environment prevent exact visual equivalence |
| `dependency:cve-scan` | Two `trivy fs` invocations with the CI arguments | Vulnerability database is network-fetched and mutable; cache writer state and resolved image digest are unknown |
| Rust preview/release and Windows package jobs | Docker service, `cross`, native Rust build, archive/checksum/provenance scripts | Docker-in-Docker, architecture, `cross` semver range, caches, CI identity, and Windows output make this costly and non-equivalent on an ordinary workstation |
| OCI pull/release and post-publication verification | Podman, registry pull/push, packaged-binary execution, downloaded-byte checks | Registry/network access, CI credentials, publication side effects, and immutable producer identity are CI/release-only controls |
| `release:create`, `release:verify`, `agf:dist:release`, `agov:oci:release` | Individual scripts can be inspected locally | Full local execution would publish or depend on public release state and is not authorized by this measurement plan |

The static map supports cheap local checks as advisory preflight. It does not
support replacing an exact-SHA CI run.

## Stable-SHA procedural checklist and trial state

For the next natural release/MR cohort:

1. Run the already-supported, change-relevant cheap local checks before asking
   for human review; record commands and outcomes without claiming CI parity.
2. Complete semantic, visual, and accessibility review before the paid
   acceptance push.
3. Keep validated coordination-only records separate from product changes and
   use existing skip semantics only when every precondition holds.
4. Freeze and record the full candidate SHA before the comprehensive bake.
5. If any product-affecting input changes, invalidate prior review/CI evidence
   and run a new comprehensive bake.
6. Preserve the full immutable-tag packaging, OCI, release creation,
   downloaded-byte, provenance, and post-publication verification path.
7. Record wall time, runner work, queueing, reviewer minutes, re-review cycles,
   defect-discovery stage, assurance coverage, and adverse effects separately.

No valid trial result is claimed yet. This planning/evidence MR is not a matched
product-release cohort, and reviewer minutes were not durably instrumented. The
checklist therefore remains surface-defined for the next natural cycle.

## WP6 read-only findings

For the four MR product/release-preparation SHAs, the frozen query found one MR
pipeline each and no same-SHA feature-branch push pipeline. This is consistent
with the existing duplicate-pipeline suppression rule. It is a scoped
observation, not proof for every pipeline source.

The product correction pipelines completed before their replacement product
SHA pipelines were created. However, review-branch pipeline `2698764772` was
canceled after 10 jobs had started: four succeeded, six were canceled, and
530.702 runner seconds had already been consumed. Its cause and initiator are
unknown, so this report neither attributes it to GitLab auto-cancel nor claims
how much work cancellation saved. Main and tag pipelines intentionally
overlapped at the same SHA, but their default-branch and immutable-release
purposes differ; they are not a safe cancellation pair.

Cache outcomes, artifact digests, and billed cost are unavailable. Read-only
WP6 therefore yields no reuse or cancellation proposal.

## Recommendation and resource ledger

| Mechanism | Benefit evidence | Main risk/cost | Reversibility | Decision |
|---|---|---|---|---|
| Stable-SHA review order | Theoretical; two review corrections and durable gaps are observed, but causality is not | Human cost displacement or delayed machine failure | Procedural and immediately reversible | Proceed to one natural descriptive trial |
| Stage/`needs` fail-fast change | Structural opportunity is observed; failure yield is missing | False green, missing obligations, concurrency/cost displacement | Requires governed CI revert and exact-SHA proof | Continue measurement; no child plan yet |
| Cache/artifact reuse | No equivalence evidence | Stale/cross-trust evidence and provenance corruption | Potentially difficult after release consumption | Reject/defer |
| Auto-cancellation | One canceled review pipeline is observed, but cause and avoided work are unknown | Loss of final/release evidence | Revertible config, but unsafe boundary risk is high | Continue measurement; no child plan |
| Skip or merge main/tag assurance | Same-SHA work is observed, not equivalent purpose | Weakens final release and post-publication controls | Release harm may be irreversible | Reject |

Measurement used no experiment-only pipelines and no additional experiment
runner seconds. At dataset freeze, 43 collection plus 12 operational-review API
requests used 55 of the 300-request ceiling; later operational traffic is
tracked separately. One analyst hour is conservatively charged for this
measurement pass against the six-hour ceiling; elapsed analyst time was not
independently instrumented. No separate human-review session was measured;
reviewer minutes are therefore `unavailable`, not zero. The local Cargo wrapper
misfire consumed filesystem space but no CI runner time; measurement paused and
493.9 MiB of generated worktree build output was removed before work resumed.

The report should return for advisory review before any child plan or CI
modification. Main-branch advancement requires refreshing the named baseline.
