---
id: first-push-ci-changes-fallback
status: pending-gate1
topic: first-push-ci-changes-fallback
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >
      contribution-auditor (Codex) drove the entire joint CI-usage
      assessment this plan is scoped from: confirmed the root mechanism
      (`20260729T163058Z-contribution-auditor-njdb`), verified the v2.4.0
      release-cut timeline (`20260729T164440Z-contribution-auditor-7kgt`),
      and issued this plan's drafting handoff with its required scope
      (`20260729T165348Z-contribution-auditor-268m`). Requested for a fresh
      pass on this draft before Gate 1.
  - role: infosec-sentinel
    status: not-applicable
    reason: >
      No secrets, permissions, or trust-boundary surface touched, matching
      `ci-pipeline-deduplication.md`'s own infosec disposition for the same
      reason: the change is a `changes:`/`compare_to` predicate over
      existing job `rules:`, evaluated per-job rather than at the
      top-level `workflow:` block this time, but it still does not add,
      remove, or reorder any job, gate, or check — it only decides
      whether a given first-push pipeline instance's existing job set
      matches. Requesting an infosec-sentinel pass before Gate 1 anyway,
      per `268m`'s explicit routing instruction, since this front-matter
      disposition is a self-assessment, not a substitute for that review.
---

# Plan: fix `rules:changes` first-push fallback on the 12 verified exposed jobs

Child plan of the joint GitLab CI usage assessment (issue #53, MR !786),
scoped narrowly per `20260729T165348Z-contribution-auditor-268m`'s explicit
instruction: **this plan does not close issue #53.** The broader v2.4.0
workstream/correction-MR archaeology continues independently. This plan
addresses exactly one verified, measured defect.

## Governance trail

- Assessment handoff: `20260729T143455Z-contribution-auditor-jt4n`.
- Root-mechanism finding + coverage map: `20260729T160152Z-planner-architect-rksg`.
- Correction (12 jobs, not 14; `site-html-build` misclassified):
  `20260729T163058Z-contribution-auditor-njdb`,
  `20260729T163710Z-planner-architect-2rh8`.
- v2.4.0 timeline verification + counterfactual:
  `20260729T164440Z-contribution-auditor-7kgt`,
  `20260729T165104Z-planner-architect-r6qa`.
- This plan's drafting handoff: `20260729T165348Z-contribution-auditor-268m`.

## The defect, precisely

Every job below has a `rules:` entry that runs on `$CI_COMMIT_BRANCH &&
$CI_COMMIT_BRANCH != $CI_DEFAULT_BRANCH` with a `changes:` path filter, with
no `compare_to`. GitLab's `changes:` keyword, absent `compare_to`, diffs
against the pipeline's `before_sha`. On a branch's first-ever pipeline
`before_sha` is the all-zero SHA, and GitLab's documented behavior for that
case is: `changes:` cannot compute a real diff and every such rule matches
unconditionally, regardless of which files the push actually touched.

Verified independently (`rksg`, confirmed by `njdb`): all 26 non-suppressed
push pipelines in a 100-pipeline sample had `before_sha:
0000000000000000000000000000000000000000`. Measured (`r6qa`, confirmed
sufficient by `268m`): a 7-pipeline counterfactual sample found 12,986.1
actual job-seconds against 3,310.3 seconds a correct diff would have
selected — 9,675.8 seconds (74.5%) measured avoidable work in that bounded
sample, concentrated in coordination/plan-only single-commit branches. The
10.4-hour full-population figure remains an explicit extrapolation, not a
measurement — this plan does not depend on that number being exact, only on
the 7-sample measurement being real, which it is.

## Exact scope — the 12 jobs, and only these

Per `njdb`'s corrected coverage-map count. No other job's `rules:` changes.

1. `dependency:cve-scan`
2. `site-html-build`
3. `agf:flyer:verify`
4. `agf:dist:validate`
5. `release:assets-selftest`
6. `release:elixir-exclusion-selftest`
7. `release:self-sufficiency-selftest`
8. `agov:governance:validate`
9. `rust:agov:validate`
10. `rust:tooling:deny`
11. `rust:tooling:test`
12. `rust:tooling:clippy`

Every other job in `.gitlab-ci.yml` (`pages`, `agf:dist:preview`,
`agf:dist:release`, `release:create`, `release:verify`,
`release:paths-validate`, `rust:tooling:oci-pull-test`,
`rust:tooling:preview`, `rust:tooling:preview:windows-package`,
`rust:tooling:release`, `agov:oci:release`,
`rust:tooling:build:windows:preview`) is either MR-pipeline-only,
default-branch-only, or tag-only already, and is unaffected by this defect
(`rksg`'s coverage map, `njdb`-corrected). This plan does not touch their
`rules:`.

## Mechanism comparison

### Candidate A: `rules:changes:compare_to`, applied only to the confirmed
### first-push case

GitLab's `changes:compare_to` (GitLab 15.5+) lets a rule diff against an
explicit ref instead of `before_sha`. Applied narrowly — gated on
`$CI_COMMIT_BEFORE_SHA` being the all-zero SHA, not applied to every
non-default push — this targets exactly the confirmed defect:

```yaml
- if: '$CI_COMMIT_BRANCH && $CI_COMMIT_BRANCH != $CI_DEFAULT_BRANCH && $CI_COMMIT_BEFORE_SHA == "0000000000000000000000000000000000000000"'
  changes:
    compare_to: '$CI_DEFAULT_BRANCH'
    paths:
      - <job's existing path list, unchanged>
- if: '$CI_COMMIT_BRANCH && $CI_COMMIT_BRANCH != $CI_DEFAULT_BRANCH'
  changes:
    - <job's existing path list, unchanged>
```

The first rule only fires on a branch's genuinely first pipeline (`before_sha`
zero); every subsequent push on that branch falls through to the second,
unmodified rule, which keeps today's incremental (previous-commit) diff
behavior. This is deliberately narrower than gating every non-default push
through `compare_to`.

### Candidate B: blanket `compare_to` on every non-default push (no
### `before_sha` gate)

Simpler to write (no first-push condition), but changes behavior for every
push on a branch, not only the first: once any push on a branch touches a
given job's path, `compare_to: $CI_DEFAULT_BRANCH` keeps that job "hot" for
every later push on the same branch for as long as the branch differs from
`main` in that path — even a push that itself touches nothing relevant.
This is the branch-lifetime hot-job tradeoff `268m` item 6 requires measuring
(see below). Rejected as the primary mechanism because it changes more
behavior than the confirmed defect requires; it remains eligible as a
fallback if Candidate A's `before_sha`-gated form turns out not to be
reviewable in CI Lint testing (see Gate 2 evidence below).

### Candidate C: MR-pipelines-only design (branch pipelines suppressed
### entirely once an MR exists, no comprehensive branch evidence pre-MR)

Rejected for this plan, per `268m`'s explicit narrow-scope instruction and
the standing rejection in `qq35`/`njdb`: it removes CI evidence for any push
made before an MR is opened, which this project's workflow does use (a
disposable branch may get several pushes before `glab mr create`). This is
a structurally different, larger design than the confirmed defect requires
and belongs to its own future plan under issue #53's broader scope, not this
child plan.

### Selected mechanism

**Candidate A.** It is the narrowest change that closes the verified defect,
preserves all existing incremental-diff behavior for every push after the
first, and keeps the branch-lifetime hot-job tradeoff scoped to exactly the
one pipeline where it's unavoidable (the first push has no incremental
history to diff against — `compare_to: $CI_DEFAULT_BRANCH` is the only
correct comparison available on that one pipeline, not a chosen tradeoff).

## Branch-lifetime "hot job" tradeoff — measured, not assumed

Per `268m` item 6: Candidate A's `compare_to` only fires on the first
pipeline of a branch. It does not keep any job hot beyond that one pipeline
— the second and later pushes use the unmodified incremental rule. The
tradeoff Candidate B would have introduced (a job staying hot for a branch's
entire life once touched once) does **not** apply to Candidate A by
construction. This is a design property, verifiable by reading the two-rule
structure above (first rule matches `before_sha == 0` only; that condition
cannot be true twice for the same ref). Gate 2 evidence must still
empirically confirm this on a disposable branch (see below) — a design
property is not the same as observed behavior.

## Full pipeline-source acceptance matrix

| Event | `before_sha` | MR open? | Current behavior (12 jobs) | Behavior after this change |
|---|---|---:|---|---|
| Branch first push, coordination/plan-only commit | `0000...0` | no | All 12 jobs run (fallback) | Only path-matching jobs run (per `compare_to: main` diff) |
| Branch first push, real code commit (e.g. touches `tooling/rust/**`) | `0000...0` | no | All 12 jobs run (fallback, but many legitimately match anyway) | Only path-matching jobs run — same jobs, correctly attributed rather than coincidentally correct |
| Branch second-or-later push, before MR exists | real SHA | no | Incremental diff (already correct) | Unchanged — falls through to the unmodified second rule |
| Branch push, MR already open | real SHA | yes | Suppressed by existing `ci-pipeline-deduplication.md` workflow rule | Unchanged — that rule is evaluated before job-level `rules:`, untouched by this plan |
| MR pipeline (`merge_request_event`) | n/a | yes | Correct merge-base diff (already correct) | Unchanged — this plan does not touch MR-event rules |
| Default-branch push/merge | real SHA (never zero — `main` always has pipeline history) | n/a | Correct incremental diff (already correct) | Unchanged — the new rule's `if:` requires `$CI_COMMIT_BRANCH != $CI_DEFAULT_BRANCH`, structurally excluding `main` |
| Tag push | n/a (`$CI_COMMIT_BRANCH` unset) | n/a | Tag-only jobs run; the 12 jobs' branch-push rules don't apply | Unchanged |
| Scheduled pipeline on a non-default ref | real SHA (schedules run against existing refs) | n/a | Falls through to the unmodified second rule (source is `schedule`, not `push`, and this project's 12 jobs' rules use `$CI_COMMIT_BRANCH`, not `$CI_PIPELINE_SOURCE == "push"` alone, for the branch case) — **verify at Gate 2**: confirm the new first rule's `$CI_COMMIT_BEFORE_SHA` check does not spuriously match a scheduled pipeline against a freshly-created ref | Same as current if verified; flagging as the one acceptance-matrix cell needing explicit Gate 2 confirmation since this project has no active scheduled pipelines to observe today |
| API-triggered pipeline | real SHA (typically) | n/a | Falls through as above | Unchanged |
| Fork MR (this project has zero forks today) | n/a | yes | No fork-specific rule exists; treated identically to any `merge_request_event` pipeline | Unchanged — not a present risk (`rksg`), no fork-specific behavior introduced by this plan |

## Preserving exact-SHA Gate 2 and release/tag coverage

This plan does not touch: `agf:dist:release`, `release:create`,
`release:verify`, `rust:tooling:release`, `agov:oci:release`, or any
`$CI_COMMIT_TAG`-gated rule. It does not touch the `merge_request_event`
rules on any of the 12 jobs — those already diff correctly and are the basis
of Gate 2 exact-SHA review evidence. The only rows changed are the
non-default-branch-push rows of 12 jobs, and only for the first pipeline on a
given ref. No exact-SHA MR-pipeline evidence path is altered.

## Gate 2 evidence — disposable-branch positive and negative tests

Per `gate-self-test-rule.md` and `268m` item 4, the implementation MR must
include, not merely assert:

1. **CI Lint / static validation**: `glab ci lint .gitlab-ci.yml` confirms
   the new two-rule structure parses and rule ordering is correct (first
   rule — `before_sha` zero — must be evaluated before the fallback
   incremental rule for each of the 12 jobs).
2. **Positive case (defect fixed)**: push a disposable branch's first commit
   touching only `docs/coordination/*.md` (mirroring the `b07870ac`/`beeee12c`
   pattern from the assessment). Confirm via `glab api .../pipelines/:id/jobs`
   that only `agov:governance:validate` runs — not all 12 — reproducing the
   exact positive case the assessment's counterfactual predicted.
3. **Negative case (real changes still trigger)**: on a second disposable
   branch, push a first commit that touches `tooling/rust/**` (e.g. a
   trivial comment change). Confirm the relevant Rust jobs (`rust:tooling:test`,
   `rust:tooling:clippy`, `rust:tooling:deny`, `rust:agov:validate`,
   `agov:governance:validate`) still run on that first pipeline — proving the
   fix narrows false positives without introducing false negatives.
4. **Incremental-push regression check**: on either disposable branch, push
   a second commit. Confirm the second pipeline's `before_sha` is non-zero
   and that it uses the unmodified incremental rule (only jobs whose paths
   the second commit actually touched run) — proving Candidate A does not
   alter today's already-correct multi-push behavior.
5. **Default-branch and tag regression**: the next ordinary push/merge to
   `main` and the next real release tag must produce unchanged pipelines
   (per the acceptance matrix above) — release-monitoring evidence, observed
   post-merge/post-release, same convention as
   `ci-pipeline-deduplication.md`'s own Gate 2 evidence.
6. **Static source-boundary check**: `grep` confirms no other job's `rules:`
   was touched, and that the new first-rule's `changes:` path list for each
   of the 12 jobs is byte-identical to that job's existing path list (only
   `compare_to` and the `before_sha` guard are new).

Disposable test branches are deleted after evidence capture; their pipeline
IDs and job results are recorded in the implementation MR description as
permanent evidence (GitLab pipeline history remains queryable by ID even
after branch deletion).

## Rollback and observability

**Rollback**: revert the two-rule split back to the single unmodified rule
per job — 12 single-line-per-job reverts, no schema, data, or artifact
migration, independently revertible from this plan's own MR, same shape as
`ci-pipeline-deduplication.md`'s rollback.

**Observability** (runner-seconds and pipeline/job counts, no unsupported
percentage claims): after merge, sample the next 100-pipeline window the
same way `rksg`/`r6qa` did (`glab api .../pipelines?per_page=100`, group by
SHA, count `before_sha`-zero push pipelines, sum job durations per pipeline)
and compare against the pre-change baseline recorded in this plan
(12,986.1s actual / 3,310.3s legitimate / 9,675.8s waste across the
7-pipeline pre-change sample). Report the post-change ratio using the same
method — do not report a percentage improvement without a comparable
post-change sample of similar size and commit-type mix.

## Interruptibility — kept separate

Per `268m` item 8 and `njdb`'s C2: `interruptible: true` addresses a
different problem (same-ref superseded pipelines, e.g. the `!762`-style
multi-push correction pattern) and does not address the first-push fallback
this plan fixes. Not included in this plan. It remains a candidate for a
separate, independently reviewable plan unless a future review explicitly
finds combining them reduces total review surface without conflating two
distinct fixes' acceptance evidence — not assumed here.

## Risks

- **`compare_to` availability**: requires GitLab 15.5+. This project is
  hosted on GitLab.com (SaaS, always current), so this is not expected to be
  a real constraint, but `glab ci lint` (Gate 2 evidence #1) is the actual
  verification, not this assumption.
- **Rule-ordering mistake**: the `before_sha`-zero rule must evaluate before
  the fallback incremental rule for each job, or the fix does nothing.
  Mitigated by CI Lint (evidence #1) and the positive-case disposable-branch
  test (evidence #2), matching `ci-pipeline-deduplication.md`'s precedent for
  catching exactly this class of mistake.
- **Scheduled-pipeline edge case**: flagged in the acceptance matrix as the
  one cell without a real pipeline to observe today. If this project adds a
  scheduled pipeline on a non-default ref before this plan's Gate 2, that
  case must be added to the disposable-branch test set.
- **Downstream tooling expecting a branch's first pipeline to always run all
  12 jobs**: none found — verified by reading every job's `rules:` in
  `.gitlab-ci.yml`; no job's `needs:` or artifact dependency assumes another
  of the 12 jobs ran on the same pipeline unconditionally.

## Role and gate boundary

Per `agf/agents/build-engineer.yaml`'s jurisdiction and
`ci-pipeline-deduplication.md`'s precedent: build-engineer implements the
`.gitlab-ci.yml` change once this plan is Gate 1 approved; planner-architect
drafted this plan and does not implement it. Implementation does not begin
before Gate 1 approval.

## Out of scope

- Closing issue #53 — remains open, per `268m`.
- The remaining 19-of-26 counterfactual pipelines not yet sampled.
- Codex's v2.4.0 workstream/correction-MR archaeology (separate stream, not
  a dependency of this narrow plan).
- Any change to `interruptible`, MR-pipelines-only design, or any of the 12
  jobs' `merge_request_event`, default-branch, or tag rules.
