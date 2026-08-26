---
id: ci-first-push-expectation-guidance
status: approved
topic: ci-first-push-expectation-guidance
gate1_approved_by: governor-reviewer
gate1_approved_at: "2026-08-26T15:17:49Z"
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >
      The contribution-auditor compared the live workflow rule, AGENTS.md,
      internal operator guidance, README, and public Pages and found that the
      race is evidenced in plans/proposals but not explained at the points
      where assistants and human contributors encounter it. The proposed
      wording preserves the MR pipeline as canonical evidence and forbids
      cancellation based on SHA alone.
  - role: infosec-sentinel
    status: not-applicable
    reason: >
      Documentation and adjacent CI comments only. No secrets, credentials,
      permissions, runner tokens, API mutation, or trust boundary changes.
---

# Plan: make the first-push duplicate-pipeline race unsurprising

**Tracking issue:** GitLab #53

**Gate 1 correction (2026-08-26).** Coder-tester's exact-head review
`20260823T181554Z-coder-tester-d14n.md` (finding `CFG-G1-AUTH-1`)
correctly found no coordination record backing this plan's original
`gate1_approved_at: "2026-08-02T08:11:41Z"` claim. A fresh binding Gate 1
decision, `20260826T151749Z-governor-reviewer-g1cf.md`, supersedes it —
the timestamp above now reflects that fresh decision, not the original
unevidenced one. Independent review `gmn7`'s two implementation findings
(`CFG-R1` PREVENTIVE classification + three-circle impact statement,
`CFG-R2` reconciliation with current `main`) remain open follow-up work,
tracked separately from Gate 1 authority.

## Intent

Document the current GitLab first-push/MR-registration race where both a
branch-push pipeline and a merge-request pipeline can be created for the same
SHA. Put the explanation where AI assistants and humans see it before or while
pushing, not only in historical plan/proposal evidence.

The guidance must distinguish the current state from the separately proposed
deterministic MR-only Slice A. This plan changes no pipeline behavior. When
Slice A later changes that behavior, its implementation must update all
current-state guidance named here in the same MR.

## Verified current-state boundary

The live `.gitlab-ci.yml` suppresses a non-default `push` pipeline only when
`$CI_OPEN_MERGE_REQUESTS` is already populated. The initial branch push can
start before the MR exists or before GitLab exposes it to the push pipeline;
opening the MR then creates a `merge_request_event` pipeline for the same SHA.
The existing guard therefore reduces duplicates on later pushes but cannot
deterministically prevent the first pair.

This is directly evidenced in
`docs/proposals/same-sha-ci-pipeline-dedup-consensus.md` and the independent
review `docs/coordination/20260731T114916Z-contribution-auditor-h65h.md`.
Neither push options nor after-the-fact cancellation are represented as a
deterministic prevention mechanism.

## Affected files and required content

### 1. `AGENTS.md` — mandatory assistant-facing instruction

Extend the existing same-SHA cancellation paragraph. It must say:

- predict that the initial branch push followed by MR creation may create a
  push/MR pair for one SHA;
- the MR pipeline is the canonical Gate 2 acceptance source in that pair;
- do not cancel merely because SHAs match: first compare pipeline source,
  ref, status, and job-name/required-job coverage;
- cancel only the demonstrably redundant push path, recording pipeline/job
  IDs and consumed runner time; and
- a duplicate first pair is a known current limitation, not evidence that the
  contributor pushed twice or that GitLab ignored the later-push guard.

### 2. `docs/reference/agent-cold-start-habits.md` — detailed operator procedure

Add a short "Predict first-push CI" subsection before the autonomous-action
list. Give the current event sequence and the evidence checklist above. State
that API/web, scheduled, default-branch, and tag pipelines are not presumed
redundant merely because they share a SHA.

Although the page is assistant-oriented, write the subsection in plain
operator language so a human using the checklist receives the same warning.

### 3. `README.md` — human point of entry

Add a concise "CI expectation for the first merge-request push" callout under
"Start development in this checkout." It must explain:

- the first push plus MR creation can currently create both pipeline sources;
- later pushes normally retain only the MR pipeline once the MR is visible;
- the MR pipeline is canonical acceptance evidence; and
- contributors should not retry, close/reopen the MR, or assume failure solely
  because two same-SHA pipelines appear.

Link to `docs/reference/agent-cold-start-habits.md` for the detailed response.
Do not place foundry-specific CI behavior in the adopter installation path.

### 4. `.gitlab-ci.yml` — point-of-change maintenance comment

Extend the comment immediately above the `$CI_OPEN_MERGE_REQUESTS` workflow
rule. State explicitly that the rule handles pushes after MR visibility but
cannot suppress a push that starts before the MR is registered. Link to issue
#53 or the consensus proposal for the planned deterministic replacement.

This is a comment-only CI edit, but the implementation MR must still run the
normal exact-head pipeline because the affected path itself is CI-sensitive.

### 5. Public operator guidance

Add a qualified GitLab note to
`agf/docs/site/docs/operate/poll-and-coordinate.md`. It must be framed as an
applicable hosting configuration, not a universal AGF behavior:

- GitLab projects that create both push and MR pipelines and suppress pushes
  through `$CI_OPEN_MERGE_REQUESTS` can see a first-push race;
- operators compare source/ref/job coverage, not SHA alone;
- projects must identify their canonical acceptance source; this foundry uses
  the MR pipeline; and
- intentional default-branch, tag, schedule, API, web, trigger, and downstream
  pipelines can legitimately share a SHA.

Link to GitLab's official predefined-variable and merge-request pipeline
documentation. Do not imply that AGF itself creates or cancels pipelines.

### 6. Changelog truth

Add one concise `agf/CHANGELOG.md` entry describing the new operator guidance.
Use the root changelog's normal documentation/waiver mechanism only if the
repository validator requires it; do not claim a CI fix because behavior is
unchanged.

## Current wording contract

The implementation should use this meaning consistently, with surface-specific
compression permitted:

> Current limitation: the initial branch push can begin before GitLab has
> registered the merge request. Opening the MR may then create a second
> pipeline for the same commit. Preserve the MR pipeline as this project's
> canonical acceptance evidence. Cancel a push pipeline only after comparing
> its source, ref, status, and required-job coverage; matching SHA alone is not
> sufficient proof of redundancy.

## Required future transition

The future deterministic MR-only Slice A must replace, in the same
implementation MR, every current-state statement added by this plan with the
new expectation:

> Non-default branch pushes do not start automatic push pipelines. Open a
> draft or regular merge request for the canonical MR pipeline, or deliberately
> request an API/web pipeline for branch-only validation.

Slice A's Gate 1 plan must list `AGENTS.md`, README development guidance,
`docs/reference/agent-cold-start-habits.md`, public operator guidance, and the
adjacent `.gitlab-ci.yml` comment as acceptance items. Until Slice A is merged,
the current-limitation wording remains accurate.

## Validation and acceptance evidence

1. `git diff --check` passes.
2. `agov validate` passes, including any required downstream-documentation
   classification.
3. `mkdocs build --strict` passes for the public guidance change.
4. A repository-wide search shows no contradictory claim that the current
   `$CI_OPEN_MERGE_REQUESTS` guard guarantees exactly one first-push pipeline.
5. Fact-check the exact implementation head against the live workflow rule and
   at least one recorded push/MR same-SHA pair.
6. Contribution-auditor checks that cancellation requires coverage comparison
   and does not conflate intentional same-SHA sources with duplicates.
7. The implementation MR description states that `.gitlab-ci.yml` behavior is
   unchanged and reports the exact push/MR pipeline graph created by its own
   first push.
8. Governor-reviewer Gate 2 applies to the exact implementation head.

## Risks and mitigations

- **Guidance becomes stale when Slice A lands.** The mandatory future
  transition lists every affected surface and makes atomic replacement a Slice
  A acceptance item.
- **People cancel legitimate same-SHA work.** Every operational surface
  requires source/ref/job-coverage comparison and names intentional sources.
- **Public docs overgeneralize a foundry configuration.** The public note is
  conditional and states that projects define their own canonical source.
- **Documentation implies a shipped CI fix.** Changelog and README explicitly
  classify this as current-limitation guidance; no behavior claim changes.
- **A comment-only CI edit consumes runner time.** Accept the one governed
  exact-head pipeline because adjacency materially reduces future maintainer
  surprise; do not add an extra correction push for prose-only evidence.

## Rollback

Revert the documentation/comment commit. No schema, data, cache, runner, or CI
behavior migration exists. If Slice A lands first, do not implement this
current-state wording; apply the future-state wording directly under Slice A's
approved plan instead.

## Gate boundary

This plan authorizes documentation and comments only after Gate 1. It does not
authorize modification of `workflow.rules`, cancellation automation, pipeline
reuse, cache policy, release-tag deduplication, or any other issue #53
implementation. Those remain separate Gate 1 decisions.
