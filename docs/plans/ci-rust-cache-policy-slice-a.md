---
id: ci-rust-cache-policy-slice-a
status: pending-gate1
topic: ci-rust-cache-policy-slice-a
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >
      MR !920 independently reproduced MR !910's cache-cost and concurrent-
      writer diagnosis from current GitLab jobs and traces. It recommended
      this measured pull-only Slice A, a deliberate default-branch writer,
      and a separately justified one-writer Slice B rather than automatic
      per-job cache splitting. A renewed exact-plan review is requested before
      Gate 1.
  - role: infosec-sentinel
    status: not-applicable
    reason: >
      Slice A removes non-default cache writes and retains GitLab's existing
      protected/non-protected cache separation. It adds no credentials,
      permissions, external inputs, cross-project cache sharing, or artifact
      authority. The change reduces the cache-poisoning surface; compiled
      cache contents remain non-authoritative build acceleration only.
---

# Plan: make non-default Rust CI cache consumers pull-only

**Tracking issue:** GitLab #101, related child scope of #53  
**Evidence:** MRs !910 and !920  
**Impact classification:** PREVENTIVE — cache write behavior changes, while
job selection, test coverage, and acceptance authority remain unchanged.

## Decision requested

Authorize only Slice A: make every Rust-cache consumer pull-only except jobs in
an ordinary default-branch push pipeline, while preserving one explicit
default-branch write *context*. Add a mechanically exercised policy contract
and measure the result.

This plan does **not** authorize Slice B's final writer topology. Default-branch
jobs will still race to upload the same cache until post-Slice-A evidence is
used in a separate Gate 1 plan choosing a controlled writer. Slice A remains
independently valuable because it removes uploads from feature pushes, merge
requests, tags, schedules, API/web pipelines, triggers, and downstream
pipelines—including the waste repeatedly observed during review work.

## Verified baseline

Current `.rust-base` defines:

```yaml
cache:
  key: rust-$CI_COMMIT_REF_SLUG
  fallback_keys:
    - rust-main
  paths:
    - .cargo/registry
    - .cargo/bin
    - tooling/rust/target
```

No `policy:` is present, so every inheriting job uses GitLab's default
`pull-push` behavior.

MR !920 independently reproduced:

- 438 successful `agov:governance:validate` jobs averaging 261.08 seconds;
  205/256 refs (80.08%) appeared once;
- 134 successful `rust:agov:validate` jobs averaging 231.73 seconds;
  76/92 refs (82.61%) appeared once;
- feature job `15663090789`: about 20 seconds of script followed by about
  125 seconds creating/uploading a branch cache;
- main job `15663334506`: about 18 seconds of script and about 147 seconds
  archiving the cache; and
- pipeline `2724116160`: five `.rust-base` jobs started within 242 ms and
  uploaded different archives to the same cache-object suffix.

Fresh evidence from MR !925 is consistent: the Rust commands finished while
multiple jobs remained occupied by concurrent cache uploads. This plan does
not overclaim that every restore is waste or that runner-seconds translate
one-for-one into human waiting time.

The five-job collision is a measured sample, not the full consumer inventory.
Current source inspection finds 12 concrete shared-cache inheritors. Nine are
direct: `agf:dist:validate`, `agf:dist:preview`, `agf:dist:release`,
`agov:governance:validate`, `rust:agov:validate`, `rust:tooling:deny`,
`rust:tooling:test`, `rust:tooling:clippy`, and
`rust:tooling:build:windows:preview`. Three inherit indirectly through
`.rust-tooling-package`: `rust:tooling:preview`,
`rust:tooling:preview:windows-package`, and `rust:tooling:release`.
`rust:tooling:oci-pull-test` has a separate cache key and is outside this
shared-key topology. CI Lint dry runs for a `.gitlab-ci.yml` change currently
select 10 shared-key jobs in MR/main contexts (8 automatic, 2 manual), 6 in a
feature push, and 7 in a tag pipeline. Acceptance must use the resolved job
graph, not a hard-coded assumption that only five writers exist.

## Slice A design

### 1. Explicit default policy

Add a top-level variable:

```yaml
variables:
  RUST_CACHE_POLICY: "pull"
```

Every context is therefore fail-safe pull-only unless an explicit workflow rule
selects the sole write context.

### 2. Explicit writer context

Add one `workflow:rules` entry after all `when: never` suppression rules and
before the unconditional catch-all:

```yaml
- if: '$CI_PIPELINE_SOURCE == "push" && $CI_COMMIT_BRANCH && $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH'
  variables:
    RUST_CACHE_POLICY: "pull-push"
```

The conjunction is intentional:

- `push` excludes schedules, API, web, trigger, and downstream sources;
- a truthy branch excludes tags, where `CI_COMMIT_BRANCH` is absent; and
- equality selects only the default branch.

Do not define the override only as “not a feature branch.” Missing variables
must resolve to pull-only, never accidentally to a writer.

### 3. Versioned shared read key

Replace the branch-specific primary key with a versioned main-fed namespace:

```yaml
cache:
  key: rust-v2-main
  fallback_keys:
    - rust-main
  policy: $RUST_CACHE_POLICY
```

Rationale:

- non-default consumers no longer probe or prefer a stale historical branch
  cache that they cannot refresh;
- the implementation MR can fall back to the existing `rust-main` cache before
  the first `rust-v2-main` writer exists;
- the first post-merge ordinary main pipeline populates `rust-v2-main`; and
- GitLab's existing protected/non-protected suffix behavior remains unchanged.

The version migration does not delete old caches. Lifecycle cleanup is an
operator/storage concern outside this Gate 1 plan.

### 4. Runtime policy contract

Add `.gitlab/scripts/check-rust-cache-policy.sh` with two modes:

- `selftest`: exercise the complete context matrix and negative cases locally;
- `assert-current`: derive the expected policy from the live
  `CI_PIPELINE_SOURCE`, `CI_COMMIT_BRANCH`, and `CI_DEFAULT_BRANCH`, then fail
  if `RUST_CACHE_POLICY` differs.

Invoke `assert-current` from `.rust-base.before_script` before the Rust work.
This avoids adding a separate runner job: every existing Rust-cache consumer
checks the actual resolved workflow variable in its own pipeline context.

The script is a test oracle, not a second policy engine. GitLab configuration
remains authoritative; the script must explain a mismatch and exit non-zero.
Its self-test must include deliberately wrong main and MR policies so a
hard-coded always-pass implementation fails mechanically.

## Context acceptance matrix

| Pipeline context | `CI_PIPELINE_SOURCE` | Branch condition | Required policy |
|---|---|---|---|
| Default-branch merge/push | `push` | branch equals default | `pull-push` |
| Feature push without MR | `push` | branch differs | `pull` |
| Merge request | `merge_request_event` | any | `pull` |
| Tag | `push` | branch absent | `pull` |
| Schedule on default ref | `schedule` | any | `pull` |
| API on default ref | `api` | any | `pull` |
| Web/manual on default ref | `web` | any | `pull` |
| Trigger-token pipeline | `trigger` | any | `pull` |
| Downstream/multi-project | `pipeline` | any | `pull` |
| Unknown/future source | other/unset | any | `pull` |

The `selftest` mode must exercise every row. The MR pipeline proves the live MR
row before Gate 2. Main and tag rows are confirmed by the next naturally
authorized pipelines; do not manufacture a main commit or release tag solely
for this plan.

## Affected files

- `.gitlab-ci.yml` — default variable, one workflow variable override,
  versioned key/fallback, explicit cache policy, and runtime assertion call.
- `.gitlab/scripts/check-rust-cache-policy.sh` — deterministic context matrix,
  negative self-tests, and live assertion.
- `CHANGELOG.md` — repository-level CI optimization and residual Slice B
  limitation. Do not add an adopter-facing AGF capability claim.

No Rust/AGF product source, job rules, stages, `needs`, artifacts, test
commands, release checks, secrets, runner permissions, or project settings
change.

## Gate 2 acceptance evidence

### Static and local

1. `git diff --check` passes.
2. `shellcheck .gitlab/scripts/check-rust-cache-policy.sh` passes when
   `shellcheck` is available; otherwise CI must use a pinned ShellCheck image
   or the handoff must state the unavailable local check explicitly.
3. `.gitlab/scripts/check-rust-cache-policy.sh selftest` passes all positive
   matrix rows and proves the two deliberately wrong-policy cases fail.
4. GitLab CI Lint validates the exact implementation head and returns no YAML
   errors. POST the candidate YAML to the project CI Lint API with
   `dry_run=true` and `include_jobs=true` for the default branch, an existing
   feature ref, and an existing authorized tag ref. Record request parameters,
   resolved job names, merged YAML, and response—not only “lint passed.” CI
   Lint cannot faithfully synthesize an MR source from a ref, so the real
   implementation MR supplies that context. The API job list does not expose
   resolved cache policy; source-contract and live-trace checks below remain
   mandatory.
5. A source inspection confirms the writer override occurs after suppression
   rules and before the catch-all, and that `.rust-base.cache.policy` consumes
   exactly `RUST_CACHE_POLICY`.

### Exact implementation MR

6. Every `.rust-base` job in the canonical MR pipeline passes
   `assert-current` with `RUST_CACHE_POLICY=pull`.
7. Enumerate every shared-key consumer in the resolved MR graph, including
   indirect `.rust-tooling-package` inheritors, then fetch every corresponding
   raw job trace. Automatic jobs contain `restore_cache` as applicable but no
   `archive_cache` section and no cache upload. Manual jobs must be source-
   inspected as pull-only and, if invoked for another authorized reason,
   trace-checked too. `rust:tooling:oci-pull-test` is excluded only because its
   cache key is separate. One sampled trace—or only the original five job
   names—is insufficient.
8. Compare the exact job-name/required-job set with a representative pre-change
   `.gitlab-ci.yml` MR pipeline. No required job may disappear because of the
   cache policy change.
9. Report, separately:
   - cache restore and archive duration;
   - `step_script` duration;
   - total job duration;
   - aggregate runner-seconds; and
   - pipeline critical-path/wall-clock duration.
   Use observed values without promising an unsupported percentage.
10. `agov validate` and all normal exact-head CI checks pass. Cache content is
    never accepted as test evidence by itself.

### Post-merge monitoring

11. The next ordinary default-branch push must resolve `pull-push`, populate
    `rust-v2-main`, and retain the same required job set. Enumerate the full
    shared-key graph and record exactly how many automatic and manual jobs are
    writers and how many traces actually archive/upload. This is the measured
    baseline for Slice B, not a Slice A failure.
12. The next independently authorized tag/release pipeline must resolve
    `pull`, retain release-only checks, and contain no Rust cache upload.
13. After either 10 non-default pipelines / 30 successful `.rust-base` jobs or
    14 days (whichever supplies evidence first), report repeated-branch compile
    cost, restore value, archive elimination, runner-seconds, and critical path
    to issue #101.

## Slice B boundary and follow-up

Slice B exists to eliminate the remaining default-branch concurrent writers.
After item 11 supplies the post-Slice-A main trace, planner-architect must draft
a separate Gate 1 plan comparing:

1. current multiple main writers (control);
2. one controlled writer plus pull-only consumers (preferred hypothesis); and
3. job/profile-specific caches only if measurements justify their extra
   uploads and storage.

That plan must decide dependency-cache versus compiled-target separation,
input-bound keys (`Cargo.lock`, pinned Rust image/toolchain, build profile),
writer ordering/failure behavior, cold-start recovery, storage lifecycle, and
whether restore cost is actually beneficial. The floating `rust:1` image must
be resolved before compiled-target reuse is treated as stable across pipelines.
Its acceptance matrix must prove exactly one automatic writer across the whole
default-branch shared-key graph and zero other writers; manual shared-key jobs
remain pull-only even if invoked. It must dynamically inventory consumers so a
new indirect inheritor cannot silently become another writer.

No Slice B topology may be implemented under this Slice A approval.

## Risks and mitigations

- **Repeated feature branches can lose useful branch-local warm state.** About
  17–20% of sampled refs ran more than once. Measure their compile/restore
  behavior explicitly; rollback remains available if pull-only costs exceed
  the removed archive work.
- **A workflow-rule ordering error could set the wrong policy.** Fail-safe
  global `pull`, complete self-test matrix, runtime assertions, CI Lint, and
  exact source-order inspection make the error visible.
- **A stale branch cache can shadow main.** The versioned shared primary key
  removes branch-key preference; legacy `rust-main` is fallback only.
- **The new main cache is initially absent.** The legacy fallback avoids a
  forced cold implementation pipeline; the first ordinary main writer warms
  the new namespace.
- **Concurrent uploads remain on main.** This is explicit residual scope with
  a required measured Slice B follow-up, not represented as fixed.
- **A consumer is missed because the original evidence named five jobs.** CI
  Lint job inventories, inheritance/source inspection, and per-trace evidence
  cover all direct and indirect shared-key consumers; the separate OCI cache
  is explicitly distinguished.
- **Cache poisoning or false-green behavior.** Non-default writes are removed,
  protected-cache separation is unchanged, and all tests rebuild/validate
  source normally. Cache remains acceleration, never authority.
- **Floating toolchain/image invalidates compiled-target assumptions.** Slice A
  makes no stable-reuse claim. Slice B must bind the toolchain/image before
  relying on compiled targets across pipelines.
- **Aggregate savings are mistaken for wall-clock savings.** Every report must
  separate runner-seconds from pipeline critical path.

## Rollback

Revert the Slice A commit:

- remove `RUST_CACHE_POLICY` and its writer workflow rule;
- restore the former branch key/fallback and implicit/default policy;
- remove the runtime contract script and call; and
- revert the repository changelog entry.

No cache object must be deleted. `rust-v2-main` can remain inert and expire
under the cache backend's lifecycle policy. Rollback changes no source,
artifact, schema, or release data.

## Gate boundary

This pending plan authorizes no CI edit. After contribution-auditor review, the
governor-reviewer may approve Slice A at an exact plan head. Implementation then
requires a separate feature branch/MR, normal exact-head CI, contribution-
auditor verification of the complete trace set, and governor Gate 2 approval.

Issue #101 remains open through post-merge Slice A measurement and the separate
Slice B decision. Neither the plan MR nor Slice A implementation closes it.
