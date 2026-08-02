---
id: ci-rust-cache-policy-slice-b
status: approved
topic: ci-rust-cache-policy-slice-b
gate1_approved_by: "governor-reviewer decision 20260802T143239Z-governor-reviewer-tq8r; MR !936 exact head 4d6baa037225d41585b0ccac1e04f9569e89bd1b"
gate1_approved_at: "2026-08-02T14:32:39Z"
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >
      MR !920 independently reproduced the concurrent-writer diagnosis and MR
      !935 measured all 8 automatic shared-cache consumers writing the same
      protected object. The exact-plan review in MR !937 returned CLEAR with 0
      blocking findings and advisory AG-CACHE-SLICEB-001. This revision adopts
      that advisory by naming the full writer comparison and the loss of
      inconsistent release-profile cache reuse explicitly.
  - role: infosec-sentinel
    status: not-applicable
    reason: >
      Slice B narrows the existing protected default-branch cache from eight
      writers to one and makes every other context pull-only. It adds no
      credentials, permissions, external inputs, cross-project sharing, or
      cache authority. GitLab's protected/non-protected cache separation stays
      unchanged, and all tests continue to rebuild and validate source.
---

# Plan: reduce the default-branch Rust cache to one controlled writer

**Tracking issue:** GitLab #101, child scope of #53

**Predecessor:** `docs/plans/ci-rust-cache-policy-slice-a.md`

**Measured baseline:** MR !935 / pipeline `2725033205`
**Impact classification:** PREVENTIVE — default-branch cache ownership and the
Rust image identity change; job coverage and acceptance authority do not.

## Decision requested

Authorize Slice B as a bounded follow-up to Slice A:

1. pin the shared Rust job image to the immutable Rust 1.97.1 image index;
2. make `rust:tooling:test` the only automatic writer to the shared
   `rust-v3-main` cache in an ordinary default-branch push;
3. make every other automatic, manual, tag, schedule, API, web, trigger,
   downstream, feature-push, and merge-request consumer pull-only; and
4. extend the runtime contract so CI fails if a second shared-key writer is
   introduced or the designated writer resolves write policy in the wrong
   context.

The implementation must preserve all existing jobs and test commands. Cache
content remains acceleration only and is never acceptance evidence.

This plan does not authorize per-job caches, a new warm-up job, stage/`needs`
changes, cache deletion, release publication, or project/runner settings.

## Evidence and selected topology

MR !935 retried the original ordinary `main` push pipeline in place, preserving
`source=push`, `ref=main`. Its live traces showed:

- all 8 automatic shared-cache consumers resolved `pull-push`;
- all 8 created and uploaded `rust-v2-main-protected` independently;
- all 8 overwrote the same last-writer-wins object;
- required job coverage was unchanged; and
- the two manual and two tag-only consumers were correctly absent from that
  ordinary automatic run.

The measured control is therefore eight writers, not the earlier five-job
sample. The smallest corrective topology is one existing writer plus pull-only
consumers. A new seed job would add runner work and a dependency edge before
evidence shows either is beneficial. Per-job caches would preserve multiple
uploads and multiply storage. Both alternatives remain rejected for this
slice.

### Why `rust:tooling:test` is the writer

`rust:tooling:test` is an existing automatic job that compiles and tests the
whole Rust workspace. It is preferable to:

- `agov:governance:validate`, which builds only one release binary and can run
  for coordination-only changes where the expensive Rust graph is suppressed;
- `rust:tooling:deny`, whose principal unique cache addition is an installed
  audit tool rather than workspace compilation;
- packaging jobs, which clean and cross-compile target-specific directories;
  and
- a new dedicated writer, which would add work and ordering before that cost
  is justified.

The remaining automatic candidates are narrower. `rust:agov:validate` and
`agf:dist:validate` execute validation paths rather than the full workspace test
graph; `agf:dist:preview` builds a bounded distribution path; and
`rust:tooling:clippy` compiles the workspace for linting but not its full
`#[cfg(test)]` code paths. `rust:tooling:test` therefore produces the broadest
debug/test-profile overlap for the parallel consumers without introducing new
work.

This choice has a known acceleration tradeoff. `agov:governance:validate` is
the only current shared-cache consumer that runs `cargo build --release -p
agov`. In the eight-writer last-writer-wins topology it could leave reusable
release-profile objects when it happened to upload last, inconsistently. A
sole `rust:tooling:test` writer publishes debug/test-profile output, so that
inconsistent release-profile benefit becomes absent. This cannot affect
correctness or acceptance, but Gate 2 item 10 and post-merge item 14 must name
and measure `agov:governance:validate` separately so the tradeoff is visible
rather than discovered after the fact.

The writer is not a prerequisite for parallel consumers. Consumers restore the
previous successful `main` cache at pipeline start; the designated writer
publishes the next generation at completion. A failed or canceled writer leaves
the prior cache available and cannot make tests falsely green.

## Immutable Rust image and key boundary

Replace floating `image: rust:1` for `.rust-base` with:

```yaml
image: rust:1.97.1@sha256:1bcff4befb740599103a2c7cb51058e14479b2e35e3a34a3f0dc4ede09927488
```

At planning time, Docker Hub's registry API reported the same OCI image-index
digest for tags `1` and `1.97.1`; its linux/amd64 config declares
`RUST_VERSION=1.97.1`. The implementation handoff must re-resolve the named
`1.97.1` tag and record the result. If it no longer maps to this digest, stop
for plan amendment rather than silently choosing another image.

Advance the shared key to `rust-v3-main` with `rust-v2-main` as the first
fallback and legacy `rust-main` second:

```yaml
key: rust-v3-main
fallback_keys:
  - rust-v2-main
  - rust-main
```

The new namespace prevents compiled output created under the formerly floating
image from being treated as the primary cache for the pinned toolchain. GitLab's
existing protected/non-protected suffix behavior remains unchanged. No old
cache is deleted.

Keep the existing combined paths for this minimal topology change:

- `.cargo/registry`
- `.cargo/bin`
- `tooling/rust/target`

This is not a claim that the combined layout is optimal. Splitting dependency,
installed-tool, and compiled-target caches changes restore/upload count,
storage, and failure behavior. It requires comparative evidence from the
one-writer baseline and therefore is deferred, not silently bundled into Slice
B.

## Policy design

### 1. Fail-safe consumer policy

Replace the context-wide write variable with two explicit variables:

```yaml
variables:
  RUST_CACHE_POLICY: "pull"
  RUST_CACHE_WRITER_POLICY: "pull"
```

`.rust-base.cache.policy` continues to consume `RUST_CACHE_POLICY`; therefore
every consumer is pull-only by default, including every future inheritor.

The ordinary default-branch push workflow rule overrides only
`RUST_CACHE_WRITER_POLICY` to `pull-push`. It must no longer override
`RUST_CACHE_POLICY`:

```yaml
- if: '$CI_PIPELINE_SOURCE == "push" && $CI_COMMIT_BRANCH && $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH'
  variables:
    RUST_CACHE_WRITER_POLICY: "pull-push"
```

### 2. Single designated writer

`rust:tooling:test` overrides only the inherited cache policy:

```yaml
rust:tooling:test:
  variables:
    RUST_CACHE_ROLE: "writer"
  cache:
    policy: $RUST_CACHE_WRITER_POLICY
```

GitLab reverse-deep-merges hash keys for `extends`; the implementation must
verify through CI Lint's merged YAML that this override retains the inherited
key, fallbacks, and all three paths. Do not assume merge behavior from source
appearance alone.

`.rust-base` sets `RUST_CACHE_ROLE: "consumer"`. No other concrete shared-key
job may override that role or policy. Manual shared-key jobs remain pull-only
even on `main`; release/tag jobs remain pull-only.

### 3. Runtime policy and topology contract

Extend `.gitlab/scripts/check-rust-cache-policy.sh` rather than adding a second
policy script. `assert-current` must accept the live role and require:

| Context | Role | Required policy |
|---|---|---|
| Ordinary default-branch push | designated writer | `pull-push` |
| Ordinary default-branch push | consumer | `pull` |
| Any non-default context | writer-labelled job | `pull` |
| Any context | consumer | `pull` |
| Unknown or missing role | any | fail closed |
| Unknown or future pipeline source | any | `pull` |

The self-test must exercise every Slice A context for both roles where
meaningful and include negative fixtures for:

- a default-branch consumer resolving `pull-push`;
- a merge-request writer-labelled job resolving `pull-push`;
- the designated default-branch writer resolving `pull`;
- an unknown role; and
- an unset policy.

Add a static topology mode that reads the resolved CI configuration supplied
by the test harness and fails unless the entire shared-key graph contains
exactly one concrete automatic writer candidate, named `rust:tooling:test`,
and every other automatic/manual/release shared-key consumer resolves pull-only
for its context. It must dynamically inventory direct and indirect
`.rust-base` inheritors and distinguish `rust:tooling:oci-pull-test`, whose key
is separate. A hard-coded list of eight jobs is insufficient as the sole
oracle.

If robust dynamic inspection cannot be implemented in the existing POSIX shell
contract without creating a fragile YAML parser, use GitLab CI Lint's merged
YAML response as the topology input and a small deterministic repository script
using an already-present runtime. Do not add an unpinned dependency solely to
parse YAML.

## Affected files

- `.gitlab-ci.yml` — immutable Rust image, `rust-v3-main` key/fallbacks,
  fail-safe variables, one writer override, and role declarations.
- `.gitlab/scripts/check-rust-cache-policy.sh` — role-aware live assertion and
  positive/negative self-tests.
- One deterministic topology-check fixture/script only if the existing script
  cannot inspect CI Lint merged YAML safely.
- `CHANGELOG.md` — repository-level CI optimization, immutable image, and the
  remaining measurement boundary. Do not claim an adopter-facing AGF feature.

No Rust/AGF product behavior, job selection rules, stages, `needs`, artifacts,
test commands, secrets, permissions, runner settings, or release paths change.

## Gate 2 acceptance evidence

### Static and resolved configuration

1. `git diff --check` and all normal repository validation pass.
2. The policy script's self-test passes every role/context row and proves every
   negative fixture fails.
3. `shellcheck` passes when available; otherwise the handoff records the gap
   and CI uses a pinned ShellCheck image before Gate 2.
4. Docker Hub re-resolution confirms `rust:1.97.1` still maps to the approved
   digest, and CI's job trace reports Rust 1.97.1. A moved tag or different
   runtime blocks rather than silently amending the pin.
5. GitLab CI Lint validates the exact implementation head with `dry_run=true`
   and `include_jobs=true` for default-branch, feature, and tag refs. Preserve
   the full request parameters, merged YAML, job names, and response.
6. Inspect the merged YAML, not only source YAML, and prove:
   - all shared consumers retain `rust-v3-main`, both fallbacks, and all paths;
   - only `rust:tooling:test` consumes `RUST_CACHE_WRITER_POLICY`;
   - every other shared-key job consumes the fail-safe pull policy;
   - manual and release-only consumers are pull-only; and
   - the separate OCI cache is unchanged.
7. Compare the exact resolved job-name/required-job set with the pre-change
   configuration. No job or required check may disappear.

### Exact implementation MR

8. Enumerate every direct and indirect shared-cache consumer in the canonical
   MR pipeline. Fetch every automatic consumer trace. Each must log exactly one
   successful role-aware assertion with `policy=pull`, no cache archive
   creation/upload, and the runner's harmless policy-suppressed archive wrapper
   may appear. Manual jobs are source/resolved-config inspected and, if invoked
   for another authorized reason, trace-checked too.
9. Candidate and pre-change CI Lint configurations resolve the same job set.
10. Report separately for every automatic shared-cache job:
    - restore duration;
    - archive/upload duration;
    - `step_script` duration;
    - total duration;
    - aggregate runner-seconds; and
    - pipeline critical path/wall-clock duration.
    Use observed values; do not promise a percentage from one pipeline.
11. `agov validate`, the exact-head MR pipeline, and all normal tests pass.
    Cache content is never accepted as test evidence.

### Post-merge confirmation

12. The next naturally occurring ordinary full `main` pipeline must show:
    - exactly `rust:tooling:test` resolves `pull-push` and creates/uploads
      `rust-v3-main-protected` once;
    - every other automatic shared-key consumer resolves `pull` and creates no
      cache archive/upload;
    - the full shared-key graph has exactly one automatic writer and zero
      manual/release writers; and
    - required job coverage is unchanged.
13. The next independently authorized tag/release pipeline must retain all
    release-only checks and show zero Rust shared-cache uploads.
14. After either 10 non-default pipelines / 30 successful `.rust-base` jobs or
    14 days, compare the one-writer result with the eight-writer MR !935
    baseline: restore value, archive elimination, storage effects, aggregate
    runner-seconds, and critical path. Report to issue #101 before closing it.

Post-merge observations are follow-up evidence, not retroactive Gate 2
conditions. A mismatch triggers issue escalation and rollback assessment; it
does not authorize an unreviewed topology correction.

## Rollback

Revert the Slice B implementation commit:

- restore `rust:1` and the Slice A context-wide writer variable;
- restore `rust-v2-main` as primary key with `rust-main` fallback;
- remove role-aware writer selection and topology assertions; and
- restore all ordinary default-branch consumers to `pull-push`.

Do not delete `rust-v3-main` or any prior cache object. Reversion changes no
source, artifact, schema, or release data. If only the image pin proves
incompatible, stop and seek a governed plan amendment rather than partially
reverting cache ownership ad hoc.

## Risks and mitigations

- **The selected writer does not run on a docs-only main push.** No Rust input
  changed, so retaining the previous cache is correct. The next tooling change
  runs the writer and refreshes it.
- **A single writer fails or is canceled.** The previous cache remains; all
  consumers still compile and test source. No pipeline acceptance depends on
  upload success.
- **Merged YAML drops inherited cache fields.** CI Lint merged-YAML inspection
  blocks Gate 2 unless key, fallbacks, and all paths remain present.
- **Pinned image breaks an architecture or runner.** Exact-head CI and the
  resolved multi-architecture image-index pin test the supported runner path;
  mismatch blocks rather than falling back to a floating tag.
- **Combined cache restores irrelevant compiled targets.** Measurement is
  mandatory and may motivate a later separately gated split; this slice does
  not multiply uploads speculatively.
- **A new inheritor becomes a writer.** Fail-safe base policy plus dynamic
  topology validation makes new consumers pull-only and detects any second
  override.
- **Runner wrapper text is mistaken for upload.** Acceptance checks actual
  archive creation/upload (`Creating cache`, `Created cache`, `Uploading cache
  to`) and permits the harmless policy-suppressed `archive_cache` wrapper
  already documented by the Slice A Gate 2 decision.

## Gate boundary and next handoff

This pending plan authorizes no CI edit. Contribution-auditor must review the
exact plan head. Only a governor-reviewer coordination decision may change the
status to `approved` and authorize implementation. After approval,
coder-tester implements on a separate feature branch and returns exact-head
evidence for independent review and binding Gate 2.

Issue #101 remains open through Gate 2, post-merge item 12, and the measurement
window in item 14. Closing it earlier would contradict this plan.
