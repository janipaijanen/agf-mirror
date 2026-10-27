<!-- Radar carrier. Retained verbatim from MR !459 (source branch plan/issue53-blinded-ci-followup, head 09b5147f), file docs/plans/issue-53-feedback-latency-measurement.md. Front-matter status was pending-gate1 and is not an approval.
     Non-authoritative: parked on the radar, grants no Gate 1, rule, template, validator or implementation authority. -->
---
id: issue-53-feedback-latency-measurement
status: approved
topic: issue-53-feedback-latency-measurement
gate1_approved_by: "governor-reviewer decision 20260723T232534Z-governor-reviewer-b1ds"
gate1_approved_at: "2026-07-23T23:25:34Z"
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: Review 20260723T231213Z-infosec-sentinel-4erq required fail-closed telemetry retention, cache/artifact trust separation, context-based cancellation boundaries, exact-SHA identity, and supply-chain/negative tests; all blocks are resolved in this revision.
  - role: contribution-auditor
    status: consulted
    reason: Review 20260723T231212Z-contribution-auditor-q454 required observable boundaries, read-only WP6 scope, durable evidence, non-circular impact classification, falsifiable coverage, and bounded human cost; all blocks are resolved in this revision.
---

# Plan: measure and shorten issue #53 release feedback latency

## Status and authority boundary

Gate 1 approves the measurement and procedural-trial scope with the resource
guardrails below. Execution begins only after the final exact-head validation
pipeline succeeds and contribution-auditor clears its CI-evidence block. This
plan authorizes no CI modification, job skipping, artifact reuse, release
action, or change to Gate 2. The assessment and blinded scoring artifacts are
advisory evidence, not approval.

The previously approved `docs/plans/ci-pipeline-deduplication.md` and the
existing `workflow.rules` suppression of duplicate feature-branch push
pipelines are baseline behavior, not new scope. This plan will measure that
behavior and consider only bounded extensions proven necessary by new data.

## Approved resource guardrails and Goodhart safeguard

The governor-reviewer approves these ceilings:

| Resource | Ceiling |
|---|---:|
| Planner/analyst time | 6 hours total |
| Human reviewer time | 60 minutes total |
| GitLab API requests | 300 |
| Experiment-only pipelines | 0 |
| Measurement/experiment-only additional runner time | 0 seconds |
| Procedural-trial cohort | 1 release/MR cycle |

Allocate at most 2 hours to WP1, 1.5 hours to WP2, 1.5 hours to WP4,
and 1 hour to read-only WP6. WP3 shares the total and reviewer-minute ceilings;
it receives no separate free budget.

These numbers are stop-loss boundaries on resource use, not optimization
targets, productivity quotas, comparative scores, or definitions of success.
Finishing below a ceiling is not a positive finding; reaching one pauses work
for re-scope. Nobody is evaluated or rewarded against these values.

Pipeline wall time remains a diagnostic and harm guardrail. Establish a matched
current baseline before the procedural trial. Pause if a comparable successful
pipeline exceeds baseline by the greater of 60 seconds or 5%; stop the trial
after two consecutive comparable breaches. A faster pipeline is not successful
if it weakens assurance coverage, exact-SHA evidence, accessibility, provenance,
review quality, failure timing, or merely transfers cost to humans or another
pipeline.

Success is conjunctive: earlier trustworthy feedback, assurance-obligation
equivalence, no displaced human/CI cost, and preserved release controls. Report
the complete metric vector and adverse effects rather than collapsing results
into a single score or percentage. Apply the project Goodhart design question:
if direct optimization of a metric would make its protected outcome worse, the
metric remains diagnostic and cannot become a gate target.

The exact ignored local retention set and controls are enumerated in governor
decision `20260723T232534Z-governor-reviewer-b1ds`. Files are owned for this
purpose by governor-reviewer, restricted to mode `0600` under mode `0700`
directories, and may not be committed, published, uploaded, or transferred to
an external service. Delete them by 2026-08-31 unless a new governor decision
extends retention. The plan and guardrails are reviewed at the same date or
earlier if a ceiling is reached, evidence is displaced, or gaming appears.

## Objective

Shorten observable time from pipeline creation to trustworthy feedback, and
separately describe durable human-review intervals where timestamps exist,
while preserving:

- Gate 2 human approval;
- exact-revision evidence;
- semantic, visual, and accessibility review;
- a stable-SHA comprehensive final release bake;
- post-publication release verification;
- reproducibility and explicit provenance;
- rollback and observable failure behavior.

## Evidence baseline

The frozen v2.4.0 packet establishes only two final successful pipelines at
SHA `0d3e18eae8c0fcf3cc4362bfa4a974675a5c5318`:

- main pipeline `2698842259`: approximately 1,031 seconds wall clock;
- tag pipeline `2698842544`: approximately 1,511 seconds wall clock.

Observed job queue times are small relative to long build, test, distribution,
and release work. Default stage ordering and explicit `needs` relationships,
not runner queueing, explain the principal observed waits. These two snapshots
do not establish correction-cycle count, failure distribution, cache hit rate,
billed cost, obsolete-pipeline frequency, or achievable savings.

Two independent local scorers and a criterion-level adjudication produced
response-level hypotheses, not provider-level claims. Those local `tmp/`
artifacts are advisory and are not durable decision evidence. Mixed
ZIP/plaintext and truncation conditions prevent a provider ranking. Mechanism
selection must trace to pinned source and CI evidence, not response totals;
unsupported percentages and inferred dependencies are excluded.

## Work packages

### WP1 — reconstruct the correction-cycle timeline

The historical correction window must be pinned before collection to named UTC
start/end events and inclusive commits or pipeline IDs. Selection criteria and
excluded events must be recorded before interpreting results. Collect every
pipeline inside that frozen window:

- product SHA, ref, source, creation/start/finish timestamps, status;
- job duration and queued duration;
- retries, cancellations, and supersession reason;
- the reason each new product SHA was required;
- review/approval timestamps when available without collecting private prompt,
  conversation, session, or user telemetry.

Before querying or processing pipeline data, define a field allowlist and a
sanitized output schema. Persist only full commit SHA; ref class or an approved
public ref; pipeline/job IDs; pipeline source; job name, stage, status, and
attempt; timestamps and durations; cancellation classification; cache outcome;
artifact digest; and provenance identifiers required by this plan. Do not
persist raw API responses, raw job logs, notes/comments, commit bodies,
free-form review text, actor/user objects, author or committer emails, avatars,
runner descriptions or manager objects, IP addresses, system IDs, contact
timestamps, project membership, HTTP headers, token values, CI variable values,
or arbitrary environment names/values. Represent “reason for new SHA” with a
fixed categorical taxonomy plus a repository-visible source pointer, not
copied free text. Unknown output fields are rejected, not automatically kept.

Raw responses may be handled only transiently for transformation and must not
become the telemetry dataset. Runtime output remains ignored and user-readable
only. Promotion requires a dated redacted fixture, secret/PII/infrastructure-
metadata scan, and explicit governor-reviewer confirmation. The current frozen
source packet also requires governor-reviewer confirmation before retention or
publication beyond ignored local storage.

This measurement-only phase creates no repository-tracked extractor. An
explicit allowlist transformation and its redacted fixtures may run locally
under the canonical primary-checkout `tmp/`; the durable evidence summary
records the exact query/transformation, tool versions, and normalized output
hash. If existing tools cannot satisfy the contract, collection stops. A
repository-tracked extractor or fixture suite requires a child Gate 1 plan that
names affected files, tests, Gate 2 evidence, and downstream-impact treatment.

Derive separately:

- wall-clock pipeline latency;
- runner queue time;
- stage or `needs` dependency waiting;
- job execution time;
- time to first actionable failure;
- human correction and re-review delay.

Do not infer a missing event. Mark unavailable fields explicitly.

For this plan:

- a **correction cycle** begins with the earliest durable timestamped event
  identifying a product-affecting defect and ends with the first successful
  required-check completion for the corrected product SHA;
- a **product-affecting SHA** changes any source, configuration, documentation,
  asset, schema, lockfile, toolchain, CI, packaging, or provenance input capable
  of altering a shipped artifact, deployed site, release behavior, or required
  assurance. Classification comes from the source-grounded input map, not from
  whether current CI rules happen to select a job. Existing path rules are
  hypotheses to validate, not authority. Unknown impact fails closed as
  product-affecting;
- a **coordination-only commit** has a diff containing only governed
  coordination records, every record validates, its title satisfies the
  existing skip rule, and no product, generated, distribution, or assurance
  input changes. It is recorded separately and never counted as a product
  correction;
- the primary observable CI metric starts at `pipeline.created_at` and ends at
  the earliest required actionable failure, or, on success, completion of all
  required checks for that pipeline;
- authoring, review, and human-correction latency is reported only when durable
  timestamped events establish both endpoints; otherwise it is `unavailable`.

For each pipeline, derive “required checks” from the expanded CI configuration
at that exact SHA, source, and ref, including `rules`, `needs`, `allow_failure`,
and blocking/manual state. If that set cannot be established, do not classify
the cycle as successfully complete.

The before/after cohort, pipeline sources, runner class, CI SHA, and material
configuration changes must be declared. A single natural trial is a descriptive
case study only: it supports no causal or percentage improvement claim. Every
mechanism report must conclude one of: proceed to a child plan, continue
measurement, or reject/defer.

### WP2 — model historical and current DAGs and duplicate computation

Build separate source-grounded job graphs for:

1. v2.4.0 at `0d3e18eae8c0fcf3cc4362bfa4a974675a5c5318`;
2. the current baseline at a named main SHA captured at measurement start;
3. the delta between those graphs.

Refresh the current baseline before any child plan if main advances. For each
job appearing in both main and tag pipelines, compare:

- exact source SHA and affected inputs;
- rules, variables, cache keys, image, and environment;
- command and produced artifacts;
- purpose and required assurance;
- provenance fields and trust boundary;
- consumers and release obligations.

Classify duplicate-looking work as one of:

1. identical computation with reusable evidence proven safe;
2. similar computation with different purpose or provenance;
3. intentionally repeated final-release assurance;
4. unknown pending logs or telemetry.

Same SHA or job name alone is never sufficient for reuse.

Caches are untrusted mutable performance inputs, never evidence. A cache hit
must still rebuild or validate the exact output claimed. Record the expanded
key and fallback keys, pull/push policy, protected/unprotected scope, allowed
writers/readers, mutability and expiry, full input set, CI-config revision,
lockfiles, resolved image/toolchain, architecture, flags/features, and build-
provenance environment. Any unknown or mismatch classifies work as non-reusable.

Artifact reuse is a separate category requiring immutable content addressing,
digest verification, successful producer pipeline/job identity, full source
SHA, trust context, purpose, and preserved producer provenance. Provenance must
never be rewritten to make an older artifact appear newly produced.

### WP3 — procedural hypotheses and bounded trial

Document and trial, without changing the CI graph:

1. Run already-available cheap local checks before consuming human review time.
2. Complete semantic, visual, and accessibility review before the paid
   acceptance push.
3. Keep coordination-only handoffs separate from product changes and use the
   existing governed skip semantics only when their exact preconditions hold.
4. Freeze a release candidate SHA before the final comprehensive bake.
5. Require a new comprehensive bake whenever the product SHA changes.
6. Keep tag packaging, OCI checks, release creation, downloaded-byte
   verification, and provenance checks comprehensive.

Measure reviewer minutes, re-review cycles, defects first found after review,
correction cycles, and observable elapsed time. Stop the trial if reviewer
effort breaches the pre-approved reviewer-minute threshold or machine-
detectable failures are discovered later in the sequence. Do not describe the
procedure as safe or beneficial until
comparable observations support that conclusion.

### WP4 — local acceptance parity inventory

Map each change-relevant CI check to a locally reproducible command, version,
configuration, required service, and expected output. Identify gaps such as
containers, networking, registry access, browser rendering, cross compilation,
and CI-only variables.

The first deliverable is a static inventory and parity report, not a replacement
for CI. It does not execute registry, cross-compile, browser, network-dependent,
or other costly checks merely to complete the map. Any later one-command
acceptance bundle requires a child Gate 1 plan and must remain advisory before
the exact-SHA CI run.

Resolve and record image digests and exact tool versions actually used.
Floating images, tools, or bootstrap channels—including the pinned v2.4.0
configuration's `release-cli:latest`, broad `rust:1` and `python:3.14-slim`
tags, and unpinned `stable` Rust bootstrap—are existing reproducibility gaps.
They make environment equivalence unknown and block cache/artifact reuse claims.
Remediation requires a separate governed change and cannot be hidden inside a
latency experiment.

### WP5 — bounded fail-fast experiment

Before modifying `.gitlab-ci.yml`, use the reconstructed graph and historical
failure data to choose one cheap, high-yield check whose dependencies are
fully established. Create a child Gate 1 plan for the smallest DAG change.

The experiment must compare control and candidate pipelines using:

- successful wall-clock duration;
- time to a deliberately seeded representative failure;
- total runner execution time and billed-cost proxy;
- assurance-obligation coverage equivalence;
- artifact/provenance equivalence where applicable.

Success requires earlier actionable failure without coverage loss, false-green
behavior, final-release weakening, or a breach of the governor-approved numeric
successful-wall-time or cost budget. A green pipeline alone is not success
evidence.

Before execution, the child plan must map every assurance obligation to its
required job, command, rules, `allow_failure` state, inputs, artifacts, and
evidence, and establish equality between control and candidate. It must specify
a governor-approved numeric successful-wall-time and cost budget. The seeded
negative case runs only on a disposable non-release branch: no experiment tag,
release, publication, or deployment is permitted.

Failure is any missing required job, command, or evidence; dependency or
artifact error; false green; provenance mismatch; release-path regression; or
budget breach. Rollback requires both reverting the child CI change and proving
the restored graph with an ordinary exact-SHA pipeline. The existing final
release path remains unchanged throughout the experiment.

### WP6 — cache, artifact, and cancellation measurement

WP6 under this plan is limited to read-only extraction from existing API fields,
logs, artifacts, and cache output: cache restore/save outcomes, cache keys,
relevant inputs, artifact digests, and provenance fields. Adding telemetry,
logging, variables, jobs, `interruptible`, auto-cancel, cache behavior, or
artifact behavior requires a separately approved child Gate 1 plan. Reuse
remains prohibited until an exact input/environment/purpose/provenance
equivalence rule and negative tests are approved in such a child plan.

The WP1 data contract applies equally to WP6. Raw API responses, logs, artifact
contents, and cache contents are processed transiently and are not written to
disk by default. If raw retention is technically necessary, stop and obtain
prior governor-reviewer confirmation naming the purpose, fields, path, owner,
access mode, and deletion deadline. Approved raw captures remain user-readable
only and must never be committed, uploaded, or sent to an external service.
Measurement hashes artifact bytes and schema-validates provenance metadata
without executing downloaded binaries, and it does not restore remote caches.
Archive inspection is sandboxed and list-only with traversal and symlink-escape
rejection; executable or otherwise unsafe inspection requires a child plan.

Measure separately: duplicate feature-branch push pipelines prevented by the
approved `workflow.rules` rule; already-created pipelines superseded by newer
SHAs; and pipelines actually canceled. The approved rule establishes only the
first mechanism, not auto-cancellation. Any new `interruptible` or auto-cancel
behavior must exclude
tag, release, publication, deploy, and post-publication verification jobs by
construction unless a later explicit governor decision establishes a safe
boundary. Cancellation saves resources; it is not assumed to shorten feedback
for the surviving SHA.

Cancellation safety is context-based, not job-name-based. A child cancellation
plan must provide a source/ref acceptance matrix covering branch push, MR,
default branch, tag, schedule, API, trigger, web/manual, and downstream
pipelines. Every job in a tag/release graph—including upstream test, build, and
artifact producers—is non-interruptible. The frozen final-candidate bake and
all side-effecting, publication, and post-publication work are non-interruptible.
Cancellation is allowed only after a newer full SHA for the same branch/MR has
created a valid replacement evidence graph. Unknown sources and newly added
jobs fail closed. Compiled-configuration tests must prove tag and final-bake
contexts cannot inherit interruptibility.

## Explicitly rejected shortcuts

- treating a green prior revision as evidence for a new revision;
- path-based skipping based only on a “docs-only” label or description;
- dropping Gate 2, accessibility review, exact-SHA reruns, or final verification;
- reusing cache entries or artifacts solely because the SHA or job name matches;
- making all jobs interruptible, especially tag, deploy, release, and verify jobs;
- replacing CI evidence with local acceptance output;
- reporting theoretical percentages as measured improvement;
- implementing CI changes directly from the blinded assessment.

## Deliverables

1. A machine-readable correction-cycle and job-timing dataset with provenance.
2. An observed critical-path report separating execution, queue, dependency
   waiting, and wall-clock latency.
3. A main/tag computation-equivalence ledger.
4. A local-versus-CI parity inventory.
5. A stable-SHA review-order checklist and trial report.
6. A recommendation update using AGF evidence-status and maturity labels.
7. At most one narrowly scoped child Gate 1 plan for the first CI experiment.

Only normalized, allowlisted runtime datasets remain under `tmp/`. Durable
evidence publishes a dated redacted dataset or summary with retrieval time,
query method, included pipeline IDs, pinned CI SHA, field definitions, missing
fields, confidence limits, and the normalized dataset hash. A raw-snapshot hash
may be included only when the underlying raw snapshot has separately approved
retention controls. A child plan never cites a raw or `tmp/` artifact as its
sole evidence.

## Measurement budget and stop order

Before WP1–WP4 or read-only WP6 execution, the governor approves numeric
person-hour, reviewer-minute, API-call, successful-wall-time, and cost stop
thresholds. The measurement phase creates zero experiment-only paid pipelines.
If the budget is exhausted, stop in this order: retain WP1's frozen timeline,
then WP2's two DAGs/delta, then WP4's static parity inventory; do not start or
continue lower-priority procedural trials. Human-review minutes are tracked as
cost, not treated as free displacement from CI.

## Acceptance criteria

- Every numeric timing traces to a pipeline/job timestamp or duration field.
- Queue time is not conflated with dependency waiting.
- Every recommendation states benefit evidence, implementation and maintenance
  cost, reproducibility risk, false-green risk, reversibility, and missing data.
- The current job graph and every proposed edge are source-grounded.
- The release path remains comprehensive and exact-SHA.
- Exact-SHA evidence records the full 40-character commit SHA, pipeline ID,
  source and ref, CI-configuration revision, job ID and retry attempt, resolved
  execution environment, and, where a job produces an artifact, artifact
  SHA-256 and provenance producer IDs. For a
  release, the immutable tag dereferences to that commit, every provenance
  `source-commit` matches it, and post-publication verification validates
  freshly downloaded public bytes plus OCI pull-by-digest evidence. A prior
  pipeline success cannot be relabelled or substituted.
- No private prompt, response, session, client, request, user, or raw environment
  data is added to telemetry.
- The extractor is field-allowlisted and fails closed on unknown output fields.
- A real-shape redacted GitLab fixture proves allowed fields survive; a negative
  fixture containing user/email/avatar, runner-manager/IP/system-ID,
  token-shaped, log, comment, raw-response, variable, and environment-value
  fields proves none reach output. A real paginated fixture is a positive
  boundary test proving every page is traversed exactly once, item IDs are
  deduplicated, and the final count reconciles with provider pagination
  metadata. Malformed, truncated, repeated-page, missing-page, and pagination-
  loop fixtures fail closed rather than producing a partial dataset.
- The experiment has control, seeded negative case, success, failure, cost,
  observability, and rollback criteria.
- Child-plan negative tests cover cross-SHA, lockfile, CI-config, image digest,
  toolchain, architecture, feature/flag, purpose, protected-ref, corrupt or
  missing digest, stale cached binary, missing/extra provenance, and producer-
  pipeline mismatches. Tag/final-bake/release/publication/verification
  cancellation attempts remain non-interruptible. A seeded fail-fast failure
  blocks downstream packaging/publication while preserving the complete
  required-job set. New gates show visible failure evidence.
- Results support response-level and mechanism-level conclusions only.

## Sequencing

1. Complete advisory reviews and Gate 1 decision on this measurement plan.
2. Execute WP1–WP4 and read-only WP6 as measurement and procedural trials.
3. Report findings and update the ranked mechanisms without unsupported savings.
4. If warranted, submit one child Gate 1 plan for WP5 or a bounded WP6 change.
5. Implement only after that child plan is approved; obtain Gate 2 review before
   merge; confirm merge target and remote with the governor-reviewer.

## Rollback and stopping rules

WP1–WP4 and read-only WP6 do not mutate CI and can stop without rollback. Any
child CI experiment must be independently revertible. Stop and return to
measurement when a governor-approved numeric reviewer-time, successful-wall-
time, or cost threshold is breached; required coverage cannot be mapped;
provenance equivalence is uncertain; or a release-boundary job could be
cancelled, skipped, or satisfied by stale evidence.

## Verdict requested

Approve measurement and procedural trial work only. Do not yet approve a CI
implementation. After observed data and advisory findings are available, choose
whether to authorize a separate fail-fast, cache, reuse, or cancellation plan.
