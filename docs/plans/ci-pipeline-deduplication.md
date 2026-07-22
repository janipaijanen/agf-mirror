---
id: ci-pipeline-deduplication
status: pending-gate1
topic: ci-pipeline-deduplication
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >
      Contribution-auditor identified the duplicate-pipeline defect on
      commit `dd2e5e6`, filed the learning proposal
      (`20260722T145245Z-contribution-auditor-rsam` and
      `20260722T151014Z-contribution-auditor-r3hj`), and requested this
      exact plan with the recommended mechanism and required acceptance
      matrix in `20260722T151632Z-contribution-auditor-r5mm`. This plan
      implements that request; the candidate mechanism below matches
      `r5mm`'s recommendation, independently re-derived by build-engineer
      from the live `.gitlab-ci.yml` state before comparison.
  - role: build-engineer
    status: consulted
    reason: >
      Pipeline-source acceptance matrix and candidate mechanism supplied
      in `20260722T163041Z-build-engineer-gbbm`, per `r5mm`'s required
      routing (build-engineer consults; planner-architect drafts the
      plan; governor-reviewer approves Gate 1 before implementation).
  - role: infosec-sentinel
    status: not-applicable
    reason: >
      No secrets, permissions, or trust-boundary surface touched. The
      change is a top-level `workflow.rules` predicate over GitLab
      predefined CI variables; it does not add, remove, or reorder any
      job, gate, or check — it only decides which pipeline instance a
      given commit's checks run in.
---

# Plan: deduplicate GitLab branch-push and merge-request pipelines

## Intent

`.gitlab-ci.yml`'s `workflow.rules` currently has exactly one rule (skip
coordination-only commits) before an unconditional `when: always`. Every
other event source runs the full pipeline — including a branch push on a
branch that already has an open merge request, which also triggers a
separate `merge_request_event` pipeline for the same commit. Both graphs
run the same substantive jobs (Rust build/test/lint, dependency-policy,
governance validation) for one SHA.

This was observed directly: MR !398 commit `dd2e5e6` produced branch
pipeline `2697575704` and merge-request pipeline `2697576236`
concurrently. The human governor reports direct, avoidable runner-cost
impact from this pattern and asked that it be fixed as soon as the
governed process permits — routed via `r3hj` (learning proposal),
`r5mm` (urgent Gate 1 plan request), `r8tg` (build-engineer activation
decision), and `1611`/`ek13` (first-assignment and priority handoffs).

Intent is the smallest `workflow.rules` change that suppresses exactly
the redundant case (branch push, open MR exists, plain push event) while
leaving every other pipeline source's behavior byte-for-byte unchanged.

## Affected files

- `.gitlab-ci.yml` — one new `workflow.rules` entry. No job, stage, or
  path-filter change.

No other file changes. No schema, no code, no migration, no dependency
change.

## Change

Insert one rule between the existing coordination-only skip and the
final `when: always`:

```yaml
workflow:
  rules:
    - if: '$CI_COMMIT_TITLE =~ /^coordination:/'
      when: never
    - if: '$CI_COMMIT_BRANCH && $CI_COMMIT_BRANCH != $CI_DEFAULT_BRANCH && $CI_OPEN_MERGE_REQUESTS && $CI_PIPELINE_SOURCE == "push"'
      when: never
    - when: always
```

All four conditions must hold simultaneously for suppression:

1. **`$CI_COMMIT_BRANCH`** — the pipeline is for a branch (this excludes
   tag pipelines, which set `$CI_COMMIT_TAG` instead and leave
   `$CI_COMMIT_BRANCH` unset).
2. **`$CI_COMMIT_BRANCH != $CI_DEFAULT_BRANCH`** — default-branch
   preservation is enforced structurally by this comparison, not inferred
   from the project convention that `main` is never an MR source branch.
   Gate 1 review `r2hr` BLOCK 1 correctly found the earlier three-condition
   form relied on that convention holding rather than the predicate itself
   guaranteeing it; this condition makes it a property of the rule.
3. **`$CI_OPEN_MERGE_REQUESTS`** — GitLab populates this predefined
   variable, in branch-push pipeline context only, with a comma-separated
   list of up to four open MRs sourced from this branch. Empty/unset when
   no MR exists yet.
4. **`$CI_PIPELINE_SOURCE == "push"`** — restricts the rule to a plain
   git push event. Excludes `merge_request_event` (the MR pipeline itself,
   which must always run), `schedule`, `api`, `trigger`, `pipeline`
   (downstream/multi-project), and `web` (manual UI-triggered runs).

## Pipeline-source acceptance matrix

| Event | Open MR on this branch? | Current behavior | Behavior after this change |
|---|---:|---|---|
| Feature-branch push | yes | Branch pipeline runs (duplicate of the MR pipeline) | **No branch pipeline; MR pipeline is the sole evidence path** |
| Feature-branch push | no | Branch pipeline runs | Unchanged — branch pipeline runs |
| Merge-request event | yes | MR pipeline runs | Unchanged — MR pipeline runs |
| Default-branch (`main`) push/merge | n/a | Pipeline runs | Unchanged — structurally guaranteed by `$CI_COMMIT_BRANCH != $CI_DEFAULT_BRANCH`, not inferred from branch-naming convention |
| Tag push | n/a | Tag pipeline runs (`agf:dist:release` etc.) | Unchanged |
| Scheduled pipeline | n/a | Runs | Unchanged |
| API-triggered pipeline | n/a | Runs | Unchanged |
| External/web trigger | n/a | Runs | Unchanged |
| Downstream (multi-project) pipeline | n/a | Runs | Unchanged |
| Coordination-only commit (`^coordination:/` title) | any | Skipped (existing rule, unmodified, evaluated first) | Unchanged |

## Test strategy / acceptance evidence

**Publication sequence (Gate 1 review `r2hr` BLOCK 2 correction):** `ci.skip`
must not be used once the implementation MR exists and the new rule is
present in that branch's `.gitlab-ci.yml`. Exact evidence that `ci.skip`
after an MR exists skips *both* the push and MR pipeline records (not just
the push): MR !401 head `3a9fd1b` (push pipeline `2697834986` skipped, MR
pipeline `2697835033` skipped) and MR !397 head `5b7c66f` (push pipeline
`2697802812` skipped, MR pipeline `2697802875` skipped). A `ci.skip`-based
"zero pipelines" result would look identical to a working guard and
identical to a broken one — it cannot be used as acceptance evidence. The
corrected sequence:

1. The initial implementation-branch push may use `ci.skip` (opening the
   MR then supplies its first ordinary MR pipeline — MR !404 itself
   demonstrated this exact pattern).
2. Once the MR exists and the new workflow guard is present on that
   branch, every subsequent acceptance push must **not** use `ci.skip`.
   The new rule — not a push option — must be what suppresses the branch
   pipeline while one ordinary MR pipeline runs for that exact SHA.

### Gate 2 (pre-merge) evidence

1. **GitLab CI Lint** (`glab ci lint .gitlab-ci.yml` or the project's CI
   Lint page) confirms the modified `workflow.rules` block parses and the
   rule ordering is syntactically and semantically valid (rules evaluate
   top-to-bottom, first match wins — the coordination-skip rule must stay
   first since it is a `when: never` independent of MR state).
2. **Live reproduction on the implementation branch itself, without
   `ci.skip`** (per the corrected sequence above): once this plan's own
   implementation branch has an open MR, a subsequent ordinary push to
   that branch must produce **zero** new branch pipelines and **one**
   ordinary (non-skipped) MR pipeline — the exact condition this plan
   exists to fix, observed on the fix's own branch, with the rule itself
   (not a push option) doing the suppressing.
3. **Negative-path check, per `gate-self-test-rule`**: MR !398's `dd2e5e6`
   is the on-record negative case predating this fix (duplicate pipelines
   `2697575704` + `2697576236`). Item 2 above is the positive-path
   reproduction of the same scenario post-fix; both must be cited
   together as Gate 2 evidence, not a green pipeline alone.
4. **No MR left un-evaluated**: confirm via the GitLab MR "pipeline"
   widget (or `glab api`) that the MR pipeline itself always still runs
   and is picked up as the MR's status check — i.e. suppressing the
   branch pipeline does not leave the MR at "checking pipeline status"
   with nothing to check.
5. **Static source-boundary check**: confirm via `grep` over
   `.gitlab-ci.yml` that no job's own `rules:` references
   `$CI_OPEN_MERGE_REQUESTS`, and that no job relies on
   `$CI_PIPELINE_SOURCE == "push"` in a way the new top-level suppression
   would break (already checked for the current file as part of this
   plan's risk analysis below).

### Post-merge / post-release monitoring (not Gate 2 evidence)

6. **Default-branch regression**: the next ordinary push/merge to `main`
   after this change merges must still produce exactly the pipeline it
   produces today (confirmed via `glab api`'s `source` field). This can
   only be observed after merge, not before.
7. **Tag regression**: the next *independently authorized* tag push (the
   next actual release) must still produce a tag pipeline unchanged. This
   is release-monitoring evidence, observed whenever the next real release
   happens — it is not something this plan's own Gate 2 can produce, since
   fabricating a tag push purely for this plan's evidence would require
   its own release authorization this plan does not have and should not
   request.

No new CI jobs, no removed jobs, no changed job conditions — the
acceptance evidence is entirely about *which pipeline instance* runs the
existing jobs, never about the jobs' own pass/fail behavior. Per
`downstream-impact-rule`, this is classified **PREVENTIVE/behavioral**, not
editorial: it intentionally changes which pipeline instances exist for a
given commit, even though no job body changes.

## Risks

- **A rule ordering mistake could suppress the wrong pipeline.** Mitigated
  by keeping the new rule as a single, narrowly-conjoined `if:` with three
  required conditions, placed after the existing coordination-only skip
  (which must keep evaluating first) and before the catch-all
  `when: always`. GitLab CI Lint catches syntax errors; the live
  reproduction (acceptance evidence #2) catches semantic errors before
  this plan's own implementation MR merges.
- **`$CI_OPEN_MERGE_REQUESTS` population timing.** GitLab documents this
  variable as available in branch pipelines once an MR exists sourcing
  from that branch; there is a narrow window between opening an MR and
  GitLab's internal state updating where a push might land before the
  variable is populated, in which case this rule simply does not match
  and the branch pipeline runs as it does today — a fail-safe direction
  (worst case reverts to current duplicate behavior for that one push,
  never to zero pipelines for a change).
- **Downstream expectation that a branch always gets its own pipeline.**
  No CI job or external tooling in this repository currently depends on a
  branch pipeline existing independently of an MR pipeline for the same
  commit (verified by reading every job's `rules:`/`only:`/`except:` in
  `.gitlab-ci.yml` — none reference `$CI_PIPELINE_SOURCE == "push"`
  exclusively in a way this rule would break).

## Rollback

Single-line revert: delete the added `workflow.rules` entry. No schema,
data, or artifact migration; no other file depends on this rule's
presence. Fully and independently revertible from this plan's own MR.

## Role and gate boundary

Per `agf/agents/build-engineer.yaml`'s jurisdiction ("cannot_touch: Plan
documents or ADRs — owned by Planner-Architect") and `r5mm`'s explicit
routing, build-engineer supplied the consultation input above (matrix,
mechanism) but did not draft this plan; planner-architect drafted it.
Implementation of `.gitlab-ci.yml` itself is build-engineer's, and does
not begin before this plan's Gate 1 approval, per `r8tg`/`1611`/`ek13`.

## Cost containment while this plan is under review

Per `r5mm`'s interim instruction and `r2hr`'s BLOCK 2 correction: plan-only
and coordination-only pushes (no `.gitlab-ci.yml` change) use `ci.skip`.
Once the implementation branch's MR exists and carries the new rule,
`ci.skip` is no longer used on that branch — the acceptance evidence
requires an ordinary, non-skipped push so the new rule itself is what's
observed suppressing the branch pipeline (see "Test strategy" above).
