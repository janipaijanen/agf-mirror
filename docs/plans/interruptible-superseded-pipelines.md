---
id: interruptible-superseded-pipelines
status: pending-gate1
topic: interruptible-superseded-pipelines
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >
      contribution-auditor (Codex) measured the underlying problem
      (`20260729T183734Z-contribution-auditor-b16q` sibling evidence,
      `20260729T185725Z-contribution-auditor-1m5r` drafting handoff,
      `20260729T190550Z-contribution-auditor-d81q` required situations and
      alternatives), and requested this plan with an explicit required
      scope. Requesting a fresh pass on this draft before Gate 1.
  - role: infosec-sentinel
    status: consulted
    reason: >
      Per `agf/agents/MANIFEST.yaml`, this project's infosec-sentinel is
      "ci (agov validate infosec, on commit)" — a mechanical check, not a
      separate human/AI review step (same disposition as
      `first-push-ci-changes-fallback.md`'s Gate 1). Ran
      `agov validate infosec` against this plan file: no findings. The
      substantive infosec question this plan raises — does
      `interruptible: true` ever reach a publication-sensitive job — is
      answered structurally in the "Job classification" section (all 6
      stateful jobs receive no `interruptible` key) and is independently
      re-checkable by any reviewer via that table and the live
      `.gitlab-ci.yml` diff at implementation time; requesting a human/AI
      pass on that specific boundary anyway before Gate 1, since a
      self-assessment is not a substitute for review on a release-
      publication-adjacent surface.
---

# Plan: cancel superseded same-ref CI work via job-level `interruptible: true`

Sibling plan to `docs/plans/first-push-ci-changes-fallback.md` under issue
#53. Scoped per `20260729T185725Z-governor-reviewer-m5qp` (v2.4.6 release
work is parked; this plan is the prioritized predecessor) and
`20260729T185725Z-contribution-auditor-1m5r` /
`20260729T190550Z-contribution-auditor-d81q`'s required scope. **Drafting is
authorized. Implementation is not.** This plan does not implement, and does
not stack on, MR !792 — it targets current `main` plus MR !792's anticipated
rule structure, and implementation waits for MR !792's final Gate 2
disposition since both touch `.gitlab-ci.yml`.

## Governance trail

- Measured evidence and problem boundary: `20260729T185725Z-contribution-auditor-1m5r`.
- Required operating situations and implementation/sequencing alternatives:
  `20260729T190550Z-contribution-auditor-d81q`.
- Release-park decision (this plan's priority): `20260729T185725Z-governor-reviewer-m5qp`.
- Sibling defect this plan is deliberately distinguished from: `first-push-ci-changes-fallback.md` (different refs, different mechanism — see "Relationship to MR !792" below).

## The problem, precisely

This project has `auto_cancel_pending_pipelines: enabled` at the GitLab
project level, but **no job in `.gitlab-ci.yml` declares `interruptible:
true`**. GitLab's auto-cancel only forcibly stops a job mid-run if that job
is marked interruptible; a non-interruptible running job is left to
complete even after a newer pipeline for the same ref has started, and (per
GitLab's documented conservative pipeline-level behavior — see "Open
verification question" below) a single non-interruptible job anywhere in an
older pipeline may prevent that pipeline from being auto-cancelled at all,
regardless of how many other jobs in it are otherwise safe to kill.

Measured (`1m5r`): the latest 100-pipeline window contained 5 same-ref
overlap events; job-timeline reconstruction found at least 89.3
runner-minutes executed after those older pipelines had already been
superseded, with 53.6 of those minutes from MR !792's own correction cycle
(pipeline `2716346277` continuing after `2716349170`, the newer exact-head
pipeline, had already started). At this namespace's Free-tier cost factor,
that bounded window alone consumed at least 22.3% of the 400-minute monthly
allowance. `1m5r` explicitly labels this a measured burst, not a
representative monthly forecast — this plan does the same and does not
convert it into a monthly savings promise.

## Relationship to MR !792 — different refs, not the same defect

`first-push-ci-changes-fallback.md` (MR !792) fixes `rules:changes`
selecting the wrong job set on a branch's *first* pipeline (`before_sha`
zero). This plan fixes *stale-pipeline compute waste* — an older pipeline
for the same ref continuing to run (or blocking cancellation of a sibling
pipeline) after a newer pipeline for that ref has already started.
Interruptibility does not touch which jobs a pipeline selects; it only
governs whether an already-running job may be killed when superseded. The
two defects can co-occur in the same measured window (a first-push pipeline
that over-selected jobs is *also* not interruptible), but fixing one does
not fix the other, and this plan does not depend on MR !792 having merged —
only on its `rules:` structure being stable enough that this plan's `rules:`
additions can be written against the same file without conflicting, which is
why implementation (not drafting) waits for MR !792's Gate 2 disposition.

## Required scope: five operating situations (`d81q`)

### 1. Open MR, rapid correction push (primary measured case)

An older `merge_request_event` pipeline's pure validation/build jobs should
cancel when a newer pipeline for the same MR ref starts. This is the case
`1m5r` measured directly (MR !792's own 53.6-minute overrun). **No stateful
job in this project's `.gitlab-ci.yml` runs on `merge_request_event`
pipelines** — `pages`, `agf:dist:release`, `release:create`,
`release:verify`, `rust:tooling:release`, and `agov:oci:release` are all
either default-branch-only or tag-only (verified by reading every job's
`rules:` in current `main`'s `.gitlab-ci.yml`). This means an MR pipeline
for this project **only ever contains restart-safe jobs** — the conservative
whole-pipeline caveat below does not apply here. Expected: full pipeline
becomes auto-cancellable once `interruptible: true` is added to every job
that can appear on an MR pipeline.

### 2. Pre-MR branch, rapid correction push

Same reasoning as situation 1 — a non-default branch push pipeline, before
an MR exists, also never contains a stateful job (same rule structure). The
first-push path-selection defect belongs to MR !792 and is explicitly not
conflated with cancellation here: this plan's `interruptible: true` flags
apply regardless of which `rules:` entry (MR !792's new first-push rule or
the existing incremental rule) selected the job — interruptibility is a
job-level property, independent of which rule matched it into the pipeline.

### 3. Default branch receives two merges close together

`main` pipelines are the one context where a stateful job (`pages`, deploy
stage) can appear in the *same* pipeline as restart-safe jobs
(`agf:dist:preview`, `rust:tooling:preview`, `rust:tooling:test`, etc. —
several of which are gated `$CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH` or bare
`$CI_COMMIT_BRANCH`, matching `main` pushes). Per the open verification
question below, GitLab's conservative pipeline-level cancellation policy may
mean marking the restart-safe jobs interruptible has **no observable effect**
on `main` pipelines specifically, because `pages` (never interruptible) is
present and pending/running in the same pipeline. This plan does **not**
assume MR-pipeline safety transfers to `main` — Gate 2 evidence item 6
below requires an explicit empirical answer for this exact case before
claiming any benefit on default-branch pipelines. If the conservative rule
does block cancellation here, that is an accepted, documented limitation of
this plan, not a defect to work around by making `pages` interruptible (it
must not be — see stateful-job list below).

### 4. Immutable tag/release pipeline

`agf:dist:release`, `release:create`, `release:verify`,
`rust:tooling:release`, and `agov:oci:release` are all `$CI_COMMIT_TAG`-only
and remain **implicitly non-interruptible** (this plan adds no
`interruptible` key to any of them). A later, unrelated tag never shares a
ref with an earlier tag (tags are individually named refs), so GitLab's
same-ref auto-cancel logic cannot affect one tag pipeline based on another
regardless of interruptibility settings — no new risk is introduced by this
plan either way. A retry or re-run of the same tag pipeline is out of this
plan's scope (immutable-tags-rule.md already forbids re-tagging).

### 5. Schedule, API, web/manual, and downstream execution

This project has zero active scheduled pipelines and no downstream
(multi-project) pipeline triggers today (verified: no `trigger:` job, no
`.gitlab-ci.yml` `include:` of a downstream config, no project-level
schedule configured per earlier `rksg` assessment work). API-triggered and
web/manual pipelines are possible but rare in this project's actual usage.
None of these sources have dedicated `rules:` entries distinct from ordinary
branch/tag rules in current `.gitlab-ci.yml`, so they inherit whichever
job's existing rule matches their `$CI_COMMIT_BRANCH`/`$CI_COMMIT_TAG`
context — meaning they inherit the same interruptibility classification as
situations 1–4 above by construction, with no separate `rules:` needed. The
two manual jobs (`rust:tooling:preview:windows-package`,
`rust:tooling:build:windows:preview`) are both preview/artifact-only and
restart-safe; GitLab's conservative whole-pipeline rule is relevant here too
(an un-triggered manual job sits `pending` indefinitely) — addressed in the
mechanism section below, not assumed away.

## Open verification question — GitLab's conservative pipeline-level rule

GitLab's documented auto-cancel behavior is per-pipeline, not purely
per-job: informal understanding (not yet empirically confirmed on this
project's own GitLab instance) is that a pipeline is only eligible for
automatic cancellation when *every* of its currently running or pending
jobs is interruptible — a single non-interruptible job anywhere in the
pipeline may block cancellation of the whole pipeline, not just that one
job. This directly affects:

- **Default-branch pipelines** (situation 3): `pages` is present and
  non-interruptible; whether its `pending` (not-yet-started) state during
  earlier build/test stages blocks cancellation of already-running
  interruptible jobs in the same pipeline is not yet confirmed for this
  project's GitLab.com hosting.
- **Manual jobs left un-triggered** (situation 5): whether a `pending`
  manual job (which never auto-starts) counts toward the "every job must be
  interruptible" evaluation, or is excluded from it, is not yet confirmed.

This plan does **not** proceed to Gate 1 recommending a mechanism on the
unverified assumption that either resolves favorably. Gate 2 evidence item 6
below is the empirical test; until it runs, situations 3 and 5's actual
benefit is unproven and must be reported as such, separately from situation
1/2's benefit (which has no such dependency, since MR/branch pipelines never
contain a non-interruptible job at all).

## Job classification — pure/restart-safe vs. stateful/publication-sensitive

Read every job's `script:`, `artifacts:`, external calls, and `rules:` in
current `main`'s `.gitlab-ci.yml` before classifying — no job is classified
by name or stage alone.

### Restart-safe (candidates for `interruptible: true`) — 18 jobs

| Job | Evidence for restart-safe |
|---|---|
| `dependency:cve-scan` | Trivy filesystem scan; writes a JSON artifact only; no external mutation |
| `site-html-build` | MkDocs build to a build-only artifact (`expire_in: 1 day`); not published |
| `agf:flyer:verify` | Read-only comparison of committed vs. freshly-rendered PNG; no writes outside the job |
| `agf:dist:validate` | `agov dist validate` — read-only manifest validation |
| `agf:dist:preview` | Builds review-only archives (`expire_in: 1 day`); explicitly documented as "not releases" |
| `release:assets-selftest` | Runs `release-assets.sh selftest` against dummy artifacts; no real release touched |
| `release:elixir-exclusion-selftest` | Same shape, `check-no-elixir-artifacts.sh selftest` |
| `release:self-sufficiency-selftest` | Same shape, `check-release-self-sufficiency.sh selftest` + `check .` (self-check only) |
| `release:paths-validate` | `check-release-paths.sh selftest` + `check agf` — read-only |
| `agov:governance:validate` | Read-only `agov coord validate`/`plan validate`/`publication-check` over changed files |
| `rust:agov:validate` | `agov validate` — read-only framework content check |
| `rust:tooling:deny` | `cargo deny check` — read-only license/advisory scan (installs `cargo-deny` into the job's own cache, not shared external state) |
| `rust:tooling:test` | `cargo test --workspace` — no external side effects |
| `rust:tooling:oci-pull-test` | Pulls from a real OCI registry by digest and runs composed self-tests — read-only against the registry (a pull, not a push); restart-safe from this project's own state perspective, though a cancelled pull simply needs to re-pull |
| `rust:tooling:clippy` | `cargo clippy` — read-only lint |
| `rust:tooling:preview` | `.rust-tooling-package` template, non-tag context — builds preview archives only, `expire_in: 1 day`, explicitly not a release |
| `rust:tooling:preview:windows-package` | Same template, Windows preview variant, manual, preview-only |
| `rust:tooling:build:windows:preview` | Separate manual preview build, artifact-only |

### Stateful/publication-sensitive (remain implicitly `interruptible: false` —
### no key added) — 6 jobs, all tag-only or default-branch-only

| Job | Why it must stay non-interruptible | Trigger context |
|---|---|---|
| `pages` | Publishes to GitLab Pages — a live, externally visible deployment; a killed mid-deploy job risks a partially-published site | Default-branch push/merge only |
| `agf:dist:release` | Builds the immutable release archives that `release:create` uploads; a killed build could leave `release:create` uploading a partial/corrupt archive | Tag only |
| `release:create` | Uploads to the generic package registry and creates the GitLab Release entry — the actual publication step | Tag only |
| `release:verify` | Re-fetches and validates the just-published release from the public API; conceptually read-only, but tied to the immutable one-shot release pipeline — see "Immutable tag/release pipeline" above for why partial-pipeline interruption is never worthwhile here regardless of an individual job's own read/write character | Tag only |
| `rust:tooling:release` | Builds and uploads the release Rust tooling archives consumed by `release:create`/`agov:oci:release` | Tag only |
| `agov:oci:release` | Builds, scans, and pushes a container image to the OCI registry — a real external publish; a killed push can leave a partial/corrupt image at a tag | Tag only |

Every stateful job is tag-only except `pages`, which is default-branch-only
— confirming the situation-1/2 vs. situation-3/4 split above is exhaustive:
no stateful job can ever appear on an MR or non-default-branch pipeline.

## Implementation alternatives (`d81q` A–E)

### A — explicit `interruptible: true` opt-in on restart-safe jobs (selected)

Add `interruptible: true` to exactly the 18 jobs above; add nothing to the 6
stateful jobs (their implicit `false` default is unchanged and undisturbed).
Larger diff (18 one-line additions) than option C, but the safest default
for every *future* job: a new job added later without an explicit
`interruptible: true` is non-interruptible by default, matching GitLab's own
default and erring toward not-killing rather than accidentally killing a
future stateful job whose author forgot to opt out.

### B — rule-level conditional interruptibility (`rules:interruptible`)

GitLab supports `interruptible` as a per-rule override (not just a
job-level default) since GitLab 18. This would let a job be interruptible
only on MR/branch rules while staying non-interruptible on its
default-branch or tag rule variant. **Not needed for this project**: since
the 18 restart-safe jobs are restart-safe in *every* context they run in
(none of them touch external publication state on any of their rule
variants, including their default-branch variants), a uniform job-level
`interruptible: true` already gives the correct answer everywhere without
per-rule granularity. Rejected as unnecessary complexity for this job set —
flagged as the right tool if a future job is restart-safe on MR pipelines
but stateful on its default-branch variant, which none of the current 18
are.

### C — default `interruptible: true` with explicit stateful-job opt-out

Smaller diff: one `default: interruptible: true` block plus 6
`interruptible: false` opt-outs on the stateful jobs, instead of 18
opt-ins. Rejected as the primary mechanism per `1m5r`'s explicit
instruction: every job added to this pipeline in the future silently
inherits `interruptible: true` unless its author remembers to opt out —
a future stateful job (a new deployment target, a new registry push) that
forgets the opt-out is killed mid-publish with no warning. This is exactly
the "future-deployment hazard" `1m5r` names as a first-class rejection
reason, not a stylistic preference.

### D — `workflow:auto_cancel:on_new_commit` plus job flags

GitLab's `workflow:auto_cancel:on_new_commit` (`interruptible` /
`conservative` modes) is a newer, pipeline-level control over exactly the
conservative-whole-pipeline behavior described above — `interruptible` mode
cancels eligible jobs even when a non-interruptible job exists elsewhere in
the same pipeline (a more aggressive mode than the project-level
`auto_cancel_pending_pipelines` setting alone provides today). This directly
targets the situation-3 (`main`, mixed `pages` + restart-safe jobs) and
situation-5 (pending manual jobs) open verification question above — if
`conservative` (today's implicit default) is confirmed to block
cancellation in either case, `on_new_commit: interruptible` mode is the
most likely fix, evaluated *after* Gate 2 evidence item 6 confirms whether
the conservative default actually causes a problem here. **Not selected for
this plan's initial scope**: adopting a more aggressive whole-project
auto-cancel mode before confirming the conservative default's actual
behavior on this project's own pipelines would be solving an unconfirmed
problem. Named as the most likely follow-up, not adopted speculatively.

### E — manual cancellation as an interim procedure only

Already the status quo (any pipeline can be manually cancelled today via
GitLab UI/API). Explicitly rejected as a permanent fix per `1m5r`: it
depends on a human noticing and acting quickly, which is precisely the
failure mode that produced the measured 89.3 runner-minutes — nobody
manually cancelled MR !792's superseded pipeline `2716346277` before it ran
to completion. Retained only as the operator procedure available while this
plan is under review, not as this plan's outcome.

## Sequencing alternatives (`d81q`)

- **Separate predecessor before v2.4.6 (selected)**: an independently
  reviewable, independently revertible rollback unit. The release, whenever
  it resumes, consumes the already-settled `main` behavior rather than
  bundling a cancellation-policy change with version/publication evidence.
  Matches `m5qp`'s explicit direction.
- **Fold into MR !792**: rejected — `8n86`/`268m`'s Gate 1 authorization for
  MR !792 explicitly excluded interruptibility (`first-push-ci-changes-
  fallback.md`'s own "Interruptibility — kept separate" section). Folding it
  in now would invalidate MR !792's current exact-head review and require
  redoing its first-push evidence suite against a changed diff.
- **Fold into the v2.4.6 release MR**: rejected — the release-planning
  handoff (`hapn`) required a release-only diff and explicitly prohibited
  issue #53 CI experiments in that MR. Combining them mixes cancellation-
  policy risk with version/publication evidence, making a release-pipeline
  failure ambiguous between release content and a new CI mechanism.
- **Defer until after v2.4.6**: technically clean, economically weak per
  `d81q` — correction-heavy periods (exactly what a release cut produces)
  are where superseded full pipelines are most expensive, so deferring
  gives up the benefit precisely when it would matter most. Select only if
  this plan cannot reach Gate 2 without delaying a time-critical release —
  not the case today, since v2.4.6 is already parked for independent
  reasons (`m5qp`).

## Gate 2 evidence required (per `1m5r` items 5–9)

1. **Job-by-job classification table** (above) — already complete as part
   of this Gate 1 plan, re-verified against `.gitlab-ci.yml` at
   implementation time in case MR !792 or another change altered a job's
   `rules:` in the interim.
2. **CI Lint / expanded configuration**: confirm `interruptible: true`
   parses correctly on all 18 jobs and appears in the expanded
   configuration exactly where expected, via `glab ci lint --dry-run
   --include-jobs` against this project's live GitLab CI Lint API (same
   method used for MR !792's evidence).
3. **Release/tag pipeline proof without a disposable tag**: confirm via
   expanded CI configuration inspection (not a real tag push, which would
   require separate release authorization this plan does not have) that
   none of the 6 stateful jobs' resolved configuration contains
   `interruptible: true` — a static, not a runtime, proof, appropriate
   since fabricating a real tag pipeline purely for this evidence is itself
   forbidden by `immutable-tags-rule.md`'s scope.
4. **Disposable same-ref MR/branch test — the primary measured case**: on a
   disposable branch, start a deliberately long-running safe pipeline (e.g.
   push a commit touching `tooling/rust/**`, which selects the ~5-6 minute
   Rust job group), then push a second commit to the same ref before the
   first pipeline completes. Confirm via `glab api .../pipelines` that the
   older pipeline transitions to `canceled` and its interruptible jobs stop
   running, while the newer exact-head pipeline runs to completion
   normally. This is the situation-1/2 proof and is expected to succeed
   cleanly (no stateful job present in this context).
5. **Default-branch mixed-pipeline test — the open verification question**:
   because `main` pipelines cannot be disposably tested without a real
   push/merge to `main` (which this plan does not authorize outside the
   normal governed merge process), this item is **observed on the next two
   ordinary `main` pushes/merges that land close together after this plan's
   own implementation MR merges** — genuine production evidence, not a
   fabricated test, following the same post-merge-observation convention
   `ci-pipeline-deduplication.md` and `first-push-ci-changes-fallback.md`
   already use for default-branch regression evidence. Record whether the
   older pipeline's restart-safe jobs were cancelled (situation 3 resolved
   favorably) or the whole pipeline ran to completion regardless
   (conservative rule confirmed; Candidate D becomes the next evaluation,
   not assumed necessary preemptively).
6. **Manual-job pending-state check**: static inspection (CI Lint expanded
   config plus a read of GitLab's own pipeline-cancellation documentation,
   cited by URL and fetch date, not recalled from training data) of whether
   a `pending`, un-triggered manual job counts toward pipeline-level
   cancellation eligibility. If inconclusive from documentation alone, defer
   to the next `rust:tooling:preview:windows-package`/`rust:tooling:build:
   windows:preview` manual invocation as incidental production evidence,
   same convention as item 5.
7. **Regression check**: after the implementation MR merges, the next
   ordinary MR pipeline, default-branch pipeline, and (whenever the release
   unparks) the next real tag pipeline must all complete with unchanged job
   sets and unchanged pass/fail results relative to pre-change baselines —
   confirming `interruptible: true` changes only cancellation eligibility,
   never job selection or outcome.

## Rollback and observability

**Rollback**: revert the governed CI commit/MR as a whole (same convention
as `first-push-ci-changes-fallback.md`) — 18 `interruptible: true` line
removals via one revert commit, no schema/data/artifact migration.

**Observability**: post-merge, sample pipeline and job data the same way
`rksg`/`r6qa`/`1m5r` did, but measure **post-supersession runner-seconds**
specifically — for each same-ref overlap event in a sampled window, sum the
job-seconds any older pipeline's jobs consumed *after* a newer pipeline for
that ref was created. Compare against this plan's own pre-change baseline
(`1m5r`'s 89.3-minute figure, itself explicitly a measured burst, not a
monthly rate) using the same method, not a raw pipeline-count comparison.
Report post-supersession runner-seconds separately for situation 1/2
(MR/branch, expected near-zero after this change) and situation 3
(default-branch, dependent on item 5's outcome) — do not average them into
one number that would hide a situation-3 shortfall behind situation-1/2's
expected clean win.

## Budget

Per `1m5r` item 9: expected initial implementation-and-evidence cost is
roughly 65–80 compute minutes (CI Lint runs, the disposable same-ref test's
two overlapping pipelines, and the required post-merge regression checks).
The measured pre-change waste (89.3 runner-minutes in one 100-pipeline
sampled window) suggests payback after avoiding roughly one to two
comparably-sized stale pipeline continuations — stated as a bounded
estimate from one sampled window, not a guaranteed recurring monthly
saving, matching `1m5r`'s own instruction not to convert this into a
monthly forecast.

## Risks

- **Situation 3/5 benefit unconfirmed** (see "Open verification question").
  Mitigated by treating it as separately measured, separately reported
  evidence rather than assumed — this plan's Gate 1 approval does not
  depend on situation 3/5 resolving favorably, only on situation 1/2 (the
  measured primary case) being correctly and safely implemented.
- **A future job added without `interruptible: true`**: correctly defaults
  to non-interruptible (GitLab's own default), the safe direction — this is
  Option A's explicit design property, not a gap.
- **Interrupting a restart-safe job that turns out to have an undocumented
  side effect**: mitigated by the per-job evidence table above, each row
  citing the job's actual script/artifact/external-call behavior rather
  than asserting safety by job name or stage.

## Role and gate boundary

Per `ci-pipeline-deduplication.md` and `first-push-ci-changes-fallback.md`'s
precedent, and `agf/agents/MANIFEST.yaml`'s confirmed-active
`build-engineer` role (activated 2026-07-22 for exactly this jurisdiction —
CI configuration, trigger rules, pipeline efficiency): build-engineer
implements once this plan is Gate 1 approved and MR !792 has reached a
final Gate 2 disposition. Planner-architect drafted this plan and does not
implement it. Implementation does not begin before both conditions hold.

## Out of scope

- Any change to MR !792's `rules:` or first-push mechanism.
- The v2.4.6 release — remains parked per `m5qp` until a separate governor
  decision unparks it.
- `workflow:auto_cancel:on_new_commit` (Candidate D) — named as the likely
  follow-up if situation 3/5 evidence shows a real gap, not adopted here.
- Any change to the 6 stateful jobs' `rules:`, `needs:`, or publication
  logic.
