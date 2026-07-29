---
id: first-push-ci-changes-fallback
status: approved
topic: first-push-ci-changes-fallback
gate1_approved_by: "governor-reviewer decision 20260729T175843Z-governor-reviewer-m15q at reviewed head 57e932b6627c382859cfbd008cbc038d84a5fe96"
gate1_approved_at: "2026-07-29T17:58:43Z"
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
    status: consulted
    reason: >
      Per `agf/agents/MANIFEST.yaml`, this project's infosec-sentinel is
      "ci (agov validate infosec, on commit)" — a mechanical check, not a
      separate human/AI review step. Ran `agov validate infosec` at exact
      head `945bfa25` (2026-07-29T17:39Z): `OK agov.validate.infosec`, no
      findings. No secrets, permissions, or trust-boundary surface is
      touched — the change is a `changes:`/`compare_to` predicate over
      existing job `rules:`; it does not add, remove, or reorder any job,
      gate, or check, only which first-push pipeline instance's existing
      job set matches, matching `ci-pipeline-deduplication.md`'s own
      infosec disposition for the same reason.
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

### Candidate A (revised after `8n86` B1 BLOCK): `rules:changes:compare_to`
### with an explicit terminal guard, not a two-rule fallthrough

The original two-rule draft was wrong: GitLab evaluates `rules:` in order,
and a rule's `changes:` predicate being false makes the *whole rule*
non-matching, not "matching with zero jobs." On a first-push pipeline where
the branch's real diff against `main` does **not** touch a given job's
paths, the first rule (with `compare_to`) correctly fails to match — but
evaluation then falls through to the second, unmodified rule, whose
`changes:` has no `compare_to` and therefore re-triggers the exact
`before_sha`-zero fallback this plan exists to fix. Two rules are not
enough; a terminal guard is required between them:

```yaml
- if: '$CI_PIPELINE_SOURCE == "push" && $CI_COMMIT_BRANCH && $CI_COMMIT_BRANCH != $CI_DEFAULT_BRANCH && $CI_COMMIT_BEFORE_SHA == "0000000000000000000000000000000000000000"'
  changes:
    compare_to: 'refs/heads/$CI_DEFAULT_BRANCH'
    paths: *job_paths
- if: '$CI_PIPELINE_SOURCE == "push" && $CI_COMMIT_BRANCH && $CI_COMMIT_BRANCH != $CI_DEFAULT_BRANCH && $CI_COMMIT_BEFORE_SHA == "0000000000000000000000000000000000000000"'
  when: never
- if: '<job's own exact existing branch-push condition — see two variants below>'
  changes: *job_paths
```

Rule 1 includes the job when the branch's real first-push diff against
`$CI_DEFAULT_BRANCH` matches its paths. Rule 2 repeats rule 1's exact `if:`
condition with no `changes:` clause and `when: never` — since GitLab stops
at the first matching rule, rule 2 can only be reached when rule 1's
`changes:` predicate was false (a genuine path miss on first push), and it
unconditionally excludes the job in that case, closing the fallthrough.
Rule 3 is only reached when the `before_sha`-zero condition is false — i.e.
every push after a branch's first — and must keep today's already-correct
incremental diff behavior **exactly** as-is.

`$CI_PIPELINE_SOURCE == "push"` is added to both new rules (`8n86` B2): the
zero-`before_sha` condition alone is not source-specific — a scheduled,
API, web, or trigger pipeline against a freshly-created ref could also
present `before_sha` zero, and this plan's scope is the confirmed
first-*push* defect only. Without this predicate the new rules would
silently broaden scope to sources the assessment never measured.

**Rule 3's exact condition is not uniform across the 12 jobs (`8n86` R2,
verified directly against current `.gitlab-ci.yml`) — two variants, not
one:**

- **`site-html-build`** (1 of 12): its existing condition is
  `$CI_COMMIT_BRANCH && $CI_COMMIT_BRANCH != $CI_DEFAULT_BRANCH` — it
  already excludes `main`. Rule 3 for this job stays exactly
  `$CI_COMMIT_BRANCH && $CI_COMMIT_BRANCH != $CI_DEFAULT_BRANCH`, unchanged.
- **The other 11 jobs** (`dependency:cve-scan`, `agf:flyer:verify`,
  `agf:dist:validate`, the three `release:*-selftest` jobs,
  `agov:governance:validate`, `rust:agov:validate`, `rust:tooling:deny`,
  `rust:tooling:test`, `rust:tooling:clippy`): their existing condition is
  the bare `$CI_COMMIT_BRANCH`, which is truthy for **both** non-default
  branch pushes and direct pushes/merges to `main`. Rule 3 for these 11
  jobs stays exactly `$CI_COMMIT_BRANCH`, unchanged — it must **not** be
  narrowed to `$CI_COMMIT_BRANCH != $CI_DEFAULT_BRANCH`. An earlier draft of
  this plan used the narrowed form as a single generic example and labelled
  it "unmodified" for all 12 jobs; applying that literally would have
  silently removed those 11 jobs from running on default-branch pushes,
  contradicting this plan's own "preserving exact-SHA Gate 2 and release/tag
  coverage" claim. Corrected per `8n86` R2.

Rules 1 and 2 (the new first-push logic) exclude the default branch
deliberately and identically for all 12 jobs — `before_sha` is never zero
on `main` (it always has prior pipeline history), so this exclusion has no
observable effect on any of the 12 jobs' default-branch behavior; it exists
only to keep the new rules' scope explicit and self-documenting.

`*job_paths` denotes a YAML anchor holding each job's existing path list.
Per `8n86` R1 and independent verification below, it is defined **inline on
each job's first rule** (not as a separate top-level key) and aliased by
rules 1 and 3 — see "Avoiding path-list duplication" below.

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
fallback if Candidate A's corrected three-rule, `before_sha`-gated form
turns out not to be reviewable in CI Lint testing (see Gate 2 evidence
below).

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

**Candidate A, corrected (three-rule form with terminal guard).** It is the
narrowest change that closes the verified defect without the fallthrough
`8n86` B1 found in the original two-rule draft, preserves all existing
incremental-diff behavior for every push after the first, and keeps the
branch-lifetime hot-job tradeoff scoped to exactly the one pipeline where
it's unavoidable (the first push has no incremental history to diff against
— `compare_to: refs/heads/$CI_DEFAULT_BRANCH` is the only correct
comparison available on that one pipeline, not a chosen tradeoff).

## Avoiding path-list duplication (`8n86` A3, anchor placement corrected
## per `hj7h` R1)

Each job's path list now appears in up to three rules (MR event, new
first-push `compare_to` rule, existing incremental rule). Rather than rely
on a `grep`-based byte-identical check (which cannot reliably verify nested
YAML list equality), each job defines its path list as a YAML anchor once
and every other rule referencing it uses the alias.

`hj7h` R1 flagged the first draft's anchor location — a separate top-level
dot-prefixed key holding a bare sequence — as not valid GitLab hidden-job
syntax. Checked directly rather than accepting or rejecting on assertion:
ran `glab ci lint <file> -R janipaijanen/agents-govern-framework --dry-run
--include-jobs` against a minimal reproduction of exactly that shape (a
top-level `.foo-paths: &foo_paths` holding a bare list, aliased from a real
job's `changes:`) using this project's own GitLab CI Lint API. Result: `✓
CI/CD YAML is valid!`, including under `--dry-run` pipeline simulation, not
only static lint. The specific claim "not a valid GitLab hidden-job
definition" does not hold — GitLab resolves YAML anchors before CI-specific
schema validation, and a hidden key never referenced via `extends:` is not
validated as a job body.

Not disputing the resolution condition, though: `hj7h`'s suggested inline
form (anchor the list on the job's first real rule, alias it from the
others) is adopted anyway, empirical top-level validity notwithstanding — it
is no more complex, avoids introducing a new top-level construct purely to
hold an anchor, and removes any dependency on hidden-key-validation
behavior for correctness, which is one less thing a future reader needs to
know is safe:

```yaml
agov:governance:validate:
  rules:
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
      changes: &agov_governance_validate_paths
        - docs/coordination/*.md
        - docs/plans/*.md
        - docs/reviews/rule-projection/*.yaml
        - .gitlab-ci.yml
        - tooling/agov
        - tooling/rust/**
        - agf/schemas/**
        - agf/agents/MANIFEST.yaml
    - if: '$CI_PIPELINE_SOURCE == "push" && $CI_COMMIT_BRANCH && $CI_COMMIT_BRANCH != $CI_DEFAULT_BRANCH && $CI_COMMIT_BEFORE_SHA == "0000000000000000000000000000000000000000"'
      changes:
        compare_to: 'refs/heads/$CI_DEFAULT_BRANCH'
        paths: *agov_governance_validate_paths
    - if: '$CI_PIPELINE_SOURCE == "push" && $CI_COMMIT_BRANCH && $CI_COMMIT_BRANCH != $CI_DEFAULT_BRANCH && $CI_COMMIT_BEFORE_SHA == "0000000000000000000000000000000000000000"'
      when: never
    - if: '$CI_COMMIT_BRANCH'
      changes: *agov_governance_validate_paths
```

(`agov:governance:validate` is one of the 11 jobs whose rule 3 stays the
bare `$CI_COMMIT_BRANCH`, per the two-variant correction above —
`site-html-build`'s rule 3 instead keeps `$CI_COMMIT_BRANCH &&
$CI_COMMIT_BRANCH != $CI_DEFAULT_BRANCH`.)

The anchor is defined once per job (on its first-occurring rule, the MR
rule), so there is nothing for a manual edit to accidentally desynchronize
across the other two rules. Gate 2 evidence item 6 (below) is `glab ci lint
--include-jobs` (or the CI Lint web UI's expanded-configuration view)
against the fully-resolved rules after anchor expansion — that expanded
output, not the anchor source, is what actually determines runtime
behavior, and is what must be verified.

## Branch-lifetime "hot job" tradeoff — measured, not assumed

Per `268m` item 6: Candidate A's `compare_to` only fires on the first
pipeline of a branch. It does not keep any job hot beyond that one pipeline
— the second and later pushes fall through (rule 1's `if:` is false once
`before_sha` is non-zero) to the unmodified rule 3, using ordinary
incremental diffing. The tradeoff Candidate B would have introduced (a job
staying hot for a branch's entire life once touched once) does **not**
apply to Candidate A by construction. This is a design property, verifiable
by reading the three-rule structure above (rules 1 and 2 both require
`before_sha == 0`, which cannot be true twice for the same ref). Gate 2
evidence item 4 (below) empirically confirms this on a disposable branch —
a design property is not the same as observed behavior.

## Full pipeline-source acceptance matrix

| Event | `before_sha` | Source | MR open? | Current behavior (12 jobs) | Behavior after this change |
|---|---|---|---:|---|---|
| Branch first push, coordination/plan-only commit | `0000...0` | push | no | All 12 jobs run (fallback) | Rule 1 matches only path-matching jobs; rule 2 (`when: never`) excludes the rest |
| Branch first push, real code commit (e.g. touches `tooling/rust/**`) | `0000...0` | push | no | All 12 jobs run (fallback, but many legitimately match anyway) | Rule 1 matches the same jobs correctly, via a real diff instead of coincidence |
| Branch second-or-later push, before MR exists | real SHA | push | no | Incremental diff (already correct) | Unchanged — rules 1/2's `if:` is false (non-zero `before_sha`); falls through to unmodified rule 3 |
| Branch push, MR already open | real SHA | push | yes | Suppressed by existing `ci-pipeline-deduplication.md` workflow rule | Unchanged — that rule is evaluated before job-level `rules:`, untouched by this plan |
| MR pipeline | n/a | merge_request_event | yes | Correct merge-base diff (already correct) | Unchanged — this plan does not touch MR-event rules |
| Default-branch push/merge | real SHA (never zero — `main` always has pipeline history) | push | n/a | Correct incremental diff (already correct) | Unchanged — rules 1/2's `if:` requires `$CI_COMMIT_BRANCH != $CI_DEFAULT_BRANCH`, structurally excluding `main` |
| Tag push | n/a (`$CI_COMMIT_BRANCH` unset) | push (tag) | n/a | Tag-only jobs run; the 12 jobs' branch-push rules don't apply | Unchanged — rules 1/2 require `$CI_COMMIT_BRANCH` truthy, which is unset for a tag pipeline |
| Scheduled pipeline on a non-default ref | may be `0000...0` on a freshly-created ref | schedule | n/a | Falls through to unmodified rule 3 (schedules never matched `$CI_PIPELINE_SOURCE == "push"`, which none of the 12 jobs' existing rules check today either) | Unchanged — rules 1/2 explicitly require `$CI_PIPELINE_SOURCE == "push"` (`8n86` B2), so a scheduled pipeline can never match them regardless of `before_sha`; **verify at Gate 2** since this project has no live scheduled pipeline to observe today |
| API-triggered pipeline | real SHA (typically) | api | n/a | Falls through to unmodified rule 3 | Unchanged, same reasoning as scheduled pipelines — the push-source guard excludes it |
| Web/manual-triggered pipeline | may vary | web | n/a | Falls through to unmodified rule 3 | Unchanged, same reasoning |
| Fork MR (this project has zero forks today) | n/a | merge_request_event | yes | No fork-specific rule exists; treated identically to any `merge_request_event` pipeline | Unchanged — not a present risk (`rksg`), no fork-specific behavior introduced by this plan |

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
   the new three-rule structure (per job) parses, anchors expand correctly,
   and rule ordering is correct (rules 1 and 2 — `before_sha` zero — must be
   evaluated before rule 3, the unmodified incremental fallback) for each of
   the 12 jobs.
2. **Positive case (defect fixed)**: push a disposable branch's first commit
   touching only `docs/coordination/*.md` (mirroring the `b07870ac`/`beeee12c`
   pattern from the assessment). Confirm via `glab api .../pipelines/:id/jobs`
   that only `agov:governance:validate` runs — not all 12, and not zero —
   reproducing the exact positive case the assessment's counterfactual
   predicted, and confirming rule 2's `when: never` guard actually excludes
   the non-matching jobs rather than falling through (the exact bug `8n86`
   B1 found in the original draft).
3. **Negative case (real changes still trigger), fixture-exact per `8n86`
   B3**: on a second disposable branch, push a first commit that touches
   only `tooling/rust/crates/agov/src/main.rs` (a trivial comment). Derived
   directly from each job's current `rules:` path list, the expected result
   is:
   - **must run**: `agov:governance:validate` (path `tooling/rust/**`),
     `rust:tooling:deny`, `rust:tooling:test`, `rust:tooling:clippy` (all
     three: path `tooling/rust/**`).
   - **must not run**: `rust:agov:validate` (paths are `agf/**` and
     `.gitlab-ci.yml` only — does not include `tooling/rust/**`, corrected
     from the first draft's incorrect inclusion), `dependency:cve-scan`
     (paths are `.gitlab-ci.yml`/`mise.toml`/lockfile/`*.toml` only, and this
     fixture touches none of them), `site-html-build`, `agf:flyer:verify`,
     `agf:dist:validate`, and all three `release:*-selftest` jobs (none of
     their paths match `tooling/rust/crates/**`).
   Both assertions (must-run and must-not-run) are required evidence, not
   only the must-run list.
4. **Incremental-push regression check**: on either disposable branch, push
   a second commit. Confirm the second pipeline's `before_sha` is non-zero
   and that rules 1/2's `if:` no longer matches, so it falls through to the
   unmodified rule 3 (only jobs whose paths the second commit actually
   touched run) — proving Candidate A does not alter today's
   already-correct multi-push behavior.
5. **Default-branch and tag regression**: the next ordinary push/merge to
   `main` and the next real release tag must produce unchanged pipelines
   (per the acceptance matrix above) — release-monitoring evidence, observed
   post-merge/post-release, same convention as
   `ci-pipeline-deduplication.md`'s own Gate 2 evidence.
6. **Structural configuration check (`8n86` A3, revised from a `grep`
   proposal)**: `glab ci lint --include-jobs .gitlab-ci.yml` (or the CI Lint
   web UI's "View merged YAML" output) against the fully-expanded
   configuration — not the anchor source — confirming: no job outside the 12
   has any `rules:` change, and each of the 12 jobs' resolved rule 1 and
   rule 3 path lists are identical to each other and to that job's resolved
   `merge_request_event` path list post-anchor-expansion. A textual `grep`
   over the anchor definitions is not accepted as this evidence, since it
   cannot detect anchor-expansion or YAML-merge errors.

Disposable test branches are deleted after evidence capture; their pipeline
IDs and job results are recorded in the implementation MR description as
permanent evidence (GitLab pipeline history remains queryable by ID even
after branch deletion).

## Rollback and observability

**Rollback (`8n86` A2, corrected)**: the implementation adds a `compare_to`
rule and a `when: never` terminal guard per job, and moves each job's
existing MR-rule path list to an inline anchor aliased by the other two
rules — two new `rules:` entries plus one converted anchor per job, not a
single-line change. Rollback is **reverting the governed CI commit/MR as a
whole** (`git revert <implementation-commit>` on a fresh branch, through a
new MR, per `merge-request-workflow-rule.md` — never a direct revert on
`main`), not 12 independent manual single-line edits. Partial, job-by-job
manual rollback across duplicated/anchored rule blocks is more error-prone
than reverting the full commit and is not the recommended path. No schema,
data, or artifact migration either way.

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
- **Rule-ordering / fallthrough mistake**: rules 1 and 2 (the `compare_to`
  match and its `when: never` terminal guard) must both evaluate before rule
  3 (the unmodified incremental fallback) for each job, or the fix does
  nothing — this is exactly the mistake `8n86` B1 caught in this plan's own
  first draft, not a hypothetical risk. Mitigated by CI Lint (evidence #1)
  and, critically, the positive-case disposable-branch test (evidence #2),
  which is the only evidence that actually distinguishes "rule 2 correctly
  excludes a non-matching job" from "job ran anyway via fallthrough" — CI
  Lint alone proves syntax validity, not runtime rule selection.
- **Scheduled-pipeline edge case**: flagged in the acceptance matrix as the
  one cell without a real pipeline to observe today. If this project adds a
  scheduled pipeline on a non-default ref before this plan's Gate 2, that
  case must be added to the disposable-branch test set.
- **Downstream tooling expecting a branch's first pipeline to always run all
  12 jobs**: none found — verified by reading every job's `rules:` in
  `.gitlab-ci.yml`; no job's `needs:` or artifact dependency assumes another
  of the 12 jobs ran on the same pipeline unconditionally.

## Role and gate boundary

`agf/agents/MANIFEST.yaml` marks `build-engineer` `status: active`,
activated by governor-reviewer on 2026-07-22 specifically for "CI
configuration... trigger rules, pipeline efficiency" — this plan's exact
implementation surface. `capability-developer`, not build-engineer, is the
manifest's suspended role. An initial review pass (`8n86` A1) suggested
build-engineer was suspended and implementation should go to
`coder-tester`; that finding was independently retracted in
`20260729T172459Z-contribution-auditor-hj7h` after re-checking the current
manifest and `AGENTS.md` (the same claim, and the same retraction, as an
earlier occurrence: `20260726T120156Z-planner-architect-ahmj` →
`20260726T121630Z-planner-architect-rkmm`). **Implementation assignment is
build-engineer**, per `ci-pipeline-deduplication.md`'s own precedent (the
sibling plan this one is modeled on) and the confirmed active manifest
state. Planner-architect drafted this plan and does not implement it
either way. Implementation does not begin before Gate 1
approval.

## Out of scope

- Closing issue #53 — remains open, per `268m`.
- The remaining 19-of-26 counterfactual pipelines not yet sampled.
- Codex's v2.4.0 workstream/correction-MR archaeology (separate stream, not
  a dependency of this narrow plan).
- Any change to `interruptible`, MR-pipelines-only design, or any of the 12
  jobs' `merge_request_event`, default-branch, or tag rules.
