# Proposal: deterministic same-SHA pipeline deduplication

Status: consensus draft; not Gate 1 authority and not implementation authority.
Owner: planner-architect. Requested reviewer: contribution-auditor, played by
Claude for this consultation. Tracking parent: GitLab issue #53.

## Decision to reach

Agree on a fail-closed design that stops avoidable repeated CI work for the
same commit without treating every same-SHA pipeline as equivalent. The formal
Gate 1 plan will be written only after planner-architect and Claude record
agreement—or clearly record the remaining disagreement for the
governor-reviewer to decide.

This proposal does not alter v2.4.9. That release should proceed under its own
approved sequence, and its main/tag pipelines provide observation evidence for
the release slice below.

## Verified current behavior

Three cases must not be conflated:

1. **Feature push followed by MR creation.** The first branch push commonly
   starts a `push` pipeline before GitLab registers the MR; MR creation then
   starts a `merge_request_event` pipeline for the same SHA. Examples from the
   latest 100-pipeline sample include:
   - `a70584e6...`: pipelines `2721394467` and `2721396892`, each running only
     `agov:governance:validate`;
   - `a80ce00f...`: pipelines `2721329654` and `2721331117`, with the same
     one-job repetition; and
   - `c58757f6...`: push pipeline `2721503479` repeated seven jobs present in
     MR pipeline `2721504986`; the MR pipeline also ran its legitimate preview
     and release-path jobs.
2. **Main followed by a release tag.** Both pipelines have the same SHA, but
   the tag pipeline has unique publication duties. The v2.4.7 observation on
   issue #53 measured 2,146.5 runner-seconds repeated across eleven shared job
   names. That is a lower bound on repeated compute, not proof that tag
   assurance may be skipped. Release build/publication remains unique work.
3. **Manual/API retries and skipped records.** These are explicit operator
   actions or recovery attempts. They must remain distinguishable in reports
   and are not evidence that automatic push/MR dedup failed.

Existing mitigations are real but partial:

- MR !410 suppresses a branch-push pipeline when an MR is already visible;
- MR !792 corrects first-push `rules:changes` fallback so a surviving first
  branch pipeline selects only path-relevant jobs;
- MR !804 makes eligible same-ref superseded jobs interruptible; and
- the corrected learning in MR !829 classifies push-time MR creation as a
  best-effort exposure reduction, not a deterministic guarantee.

The residual first-push race is therefore a verified defect, not a missing
deployment of those earlier controls.

## Desired invariants

### Feature and planning branches

- A non-default branch intended for merge has exactly one authoritative
  validation pipeline per SHA: the MR pipeline.
- The initial push may create no server pipeline until the MR is opened. MR
  creation immediately validates the current head.
- Later pushes with an open MR create one MR pipeline and zero push pipelines.
- Branch-only work that intentionally has no MR receives no automatic paid CI;
  an explicitly requested API/web pipeline remains available and visible as
  such.

### Main and releases

- Every `main` merge still receives its ordinary default-branch validation.
- Every immutable release tag still receives a release pipeline and all unique
  build, provenance, package, OCI, and publication work.
- Shared assurance may be reused only from a successful `main` pipeline for
  the exact tag SHA, with the required job set and configuration identity
  mechanically verified. Absence or ambiguity fails closed.
- No proposal may substitute branch-name recency, "latest successful", or
  mutable artifact location for exact commit identity.

### Other sources

Scheduled, API, web, trigger, downstream, and fork pipeline behavior remains
unchanged unless separately analyzed and approved. Reports distinguish
intentional same-SHA sources from avoidable duplicates.

## Slice A — deterministic MR-only validation for non-default pushes

### Recommended mechanism

At `workflow.rules` level, preserve MR, tag, and default-branch sources first;
then suppress every ordinary push pipeline for a non-default branch regardless
of `$CI_OPEN_MERGE_REQUESTS` timing:

```yaml
workflow:
  rules:
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
    - if: '$CI_COMMIT_TAG'
    - if: '$CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH'
    - if: '$CI_PIPELINE_SOURCE == "push" && $CI_COMMIT_BRANCH && $CI_COMMIT_BRANCH != $CI_DEFAULT_BRANCH'
      when: never
    - when: always
```

The exact implementation may preserve equivalent existing rule ordering, but
the semantic requirements above are fixed. The current
`$CI_OPEN_MERGE_REQUESTS` suppression becomes redundant and should not remain
as misleading dead policy. Job-level first-push protections from MR !792 stay
unchanged in the first implementation; removing now-unreachable defense would
unnecessarily enlarge rollback scope.

### Why this is preferred

- It removes the timing race rather than trying to win it.
- It requires no mutating bot token and no after-the-fact cancellation race.
- It makes the MR pipeline the single exact-head Gate 2 evidence path already
  used by governance.
- Push-time MR creation remains a convenience, not a correctness dependency.

### Explicit trade-off

A branch without an MR has no automatic branch pipeline. Contributors must
open a draft/regular MR for server validation or deliberately trigger an
API/web pipeline. The formal plan must update contributor and agent guidance
so absence of an initial branch pipeline is expected rather than mistaken for
CI failure.

### Rejected primary alternative

An automated watcher could cancel a push pipeline once it detects an MR
pipeline for the same SHA. This is reactive: both pipelines can consume runner
time before cancellation; cross-ref interruptibility does not solve it; job
coverage must be compared before cancellation; and a mutating API credential
adds a trust boundary. Retain this only as a fallback candidate if consensus
rejects MR-only branch CI.

## Slice B — exact-SHA main evidence reuse for release tags

This slice has a higher assurance boundary and must not be implemented in the
same MR as Slice A.

### Recommended staged mechanism

1. **Observe first.** Use v2.4.9 without CI behavior changes to capture the
   exact `main` and tag pipeline job matrices, durations, outputs, and unique
   release duties.
2. **Add an advisory evidence resolver.** Given a candidate tag SHA, locate a
   successful `push` pipeline on `main` for that exact SHA and verify a frozen
   list of required job names, statuses, and CI configuration identity. It
   must reject missing, ambiguous, canceled, skipped, manual/API-only, or
   different-SHA evidence.
3. **Self-test the resolver on historical positive and negative fixtures.** A
   successful same-SHA main pipeline is positive; different SHA, incomplete
   job set, failed main pipeline, and same SHA from the wrong source/ref are
   negative fixtures.
4. **Only after a separate Gate 1/Gate 2 decision**, make the resolver a
   blocking pre-tag and tag-pipeline check, and stop rerunning only the shared
   validation jobs it proves. Release-only archive builds, binary builds,
   provenance, tag/version checks, OCI work, package upload, anonymous
   post-publication verification, and Release creation continue on the tag.
5. Provide an explicit full-validation override for recovery that is visible,
   governor-authorized, and reruns the tag assurance rather than silently
   accepting absent evidence.

Artifact reuse is deferred. First reuse exact-SHA *assurance results* while
still building release artifacts in tag context. Cross-pipeline artifact reuse
requires its own content-addressed provenance contract and is not assumed safe
merely because the git SHA matches.

## Required acceptance matrix

The formal child plans must require live or CI-Lint evidence for every row:

| Case | Required result |
|---|---|
| First push, non-default branch, no MR | zero automatic pipelines |
| Create MR for that unchanged head | exactly one MR pipeline |
| Push new head with MR open | one MR pipeline; zero push pipeline |
| Push-time MR creation option | one MR pipeline; zero push pipeline regardless of registration timing |
| Explicit API/web branch validation | one visibly manual/API pipeline |
| Merge to `main` | one default-branch pipeline |
| Non-release tag if supported | current behavior preserved or explicitly classified |
| Release tag on exact successful `main` SHA, Slice B observe stage | full current tag pipeline; evidence recorded only |
| Release tag after Slice B enforcement | release-only work plus blocking exact-SHA main-evidence check |
| Tag SHA without qualifying main evidence | fail closed before publication |
| Schedule, trigger, downstream, fork | unchanged or stop for separate design |

Gate self-tests must include must-exist and must-not-exist pipeline assertions;
a green pipeline alone is insufficient. Post-change observability groups by
SHA and source, compares job-name intersections, and reports runner-seconds
without labeling unique release work as waste.

## Sequencing and release placement

1. Reach planner-architect/contribution-auditor consensus on this proposal.
2. The governor decides unresolved trade-offs and authorizes separate Gate 1
   plans for Slice A and Slice B.
3. Complete v2.4.9 unchanged; use it as Slice B observation evidence.
4. Implement Slice A as an internal foundry CI/process change after v2.4.9. It
   does not require an adopter version by itself unless adopter-facing AGF
   templates or rules change.
5. Keep Slice B advisory until its resolver and negative fixtures are reviewed;
   enable skipping only in a later separately approved change.
6. Close issue #53 only when its remaining named causal/effectiveness evidence
   is either completed or explicitly routed to a new issue. Do not keep it open
   as an unbounded CI wishlist.

## Consensus questions for Claude

Return `AGREE`, `DISAGREE`, or `NEEDS EVIDENCE` for each item, with blocking
findings separated from advisories:

1. Is unconditional suppression of non-default `push` pipelines the narrowest
   deterministic fix for the first-push/MR race?
2. Is draft/regular MR creation an acceptable replacement for automatic
   branch-only CI, with API/web validation retained for exceptions?
3. Should MR !792's now-unreachable first-push job rules remain for rollback
   safety in Slice A?
4. Is reactive API cancellation correctly rejected as the primary mechanism?
5. Is exact-SHA assurance-result reuse, without artifact reuse, the right first
   release-tag optimization boundary?
6. What immutable identity must bind reusable main evidence besides commit SHA
   (CI config blob, required-job-set version, toolchain/container inputs)?
7. Are the stop conditions and acceptance matrix sufficient to prevent false
   green release tags, fork regressions, and hidden loss of branch coverage?
8. Should either slice affect the public version roadmap, or remain internal
   foundry work tracked under issue #53?

No implementation begins until this consultation is published, the proposal
is revised to reflect consensus, a schema-valid Gate 1 plan exists for the
selected slice, and the governor approves its exact head.
