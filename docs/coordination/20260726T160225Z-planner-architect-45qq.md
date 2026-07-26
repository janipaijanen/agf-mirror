---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260726T160225Z-planner-architect-45qq
timestamp: 2026-07-26T16:02:25Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260726T140511Z-planner-architect-6q4k
  - 20260726T144924Z-planner-architect-p6kk
  - 20260726T154831Z-fact-checker-7pd8
  - 20260726T155220Z-planner-architect-e3k6
topic: v245-delivery-plan-reconciliation-and-execution
requires-ack: true
artifacts_owned:
  - docs/plans/v2.4.5-next-unplanned-release-assessment.md
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Reconcile MR !589 onto current main and replace its obsolete v2.4.3 review baseline with immutable v2.4.4 at commit 103930dbcdb9127cea65f1ca387ef13ed3ee7c10.
    - Finish MR !599 through exact green CI, accurate MR evidence, final fact-check, and governor Gate 2 without expanding its one-file implementation scope.
    - Prepare the separate standardized multi-LLM documentation-review Gate 1 plan and external web-LLM prompt packet as the required v2.4.5 critical path; do not begin review execution before Gate 1 approval.
    - Refresh MR !590 for current-main evidence and preserve the three MR !591 slices as independently reviewed, schedule-elastic candidates.
    - Return an acknowledgement and updated exact-SHA handoff; do not merge, cut a release branch, bump a version, or tag.
  must_read:
    - docs/plans/v2.4.5-next-unplanned-release-assessment.md
    - docs/plans/ai-tool-guidance-coherence.md
    - docs/coordination/20260726T154831Z-fact-checker-7pd8.md
    - docs/coordination/20260726T155220Z-planner-architect-e3k6.md
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/589
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/599
  do_not_touch:
    - agf/VERSION
    - CHANGELOG.md
    - agf/CHANGELOG.md
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Reconcile and deliver the v2.4.5 plan from the released v2.4.4 baseline

The governor asked Codex to summarize the v2.4.5 plate and then explicitly
directed that delivery plan be coordinated with Claude. This record is the
canonical handoff; chat context is not required.

## Release identity and required outcome

v2.4.5 is a documentation-credibility patch. Its required outcome is an
auditable standardized multi-LLM critical review of the current released
documentation, followed only by bounded remediations that independently clear
their gates.

MR !589 still describes v2.4.3 as the frozen “current-state” review snapshot.
That became obsolete when v2.4.4 was released. The review baseline must now be:

- annotated tag `v2.4.4`;
- release/main merge commit
  `103930dbcdb9127cea65f1ca387ef13ed3ee7c10`;
- pinned tagged documentation tree under `v2.4.4/agf/docs/site/docs`; and
- live Pages only as corroborating rendered evidence, not the normative source.

Reviewing v2.4.3 would omit the documentation-provenance and release-hygiene
baseline that v2.4.4 deliberately shipped and would no longer satisfy the
plan's “current-state” claim.

## Recommended v2.4.5 content lock

Required:

1. standardized multi-LLM review evidence against exact v2.4.4; and
2. only bounded, fact-checked documentation remediations accepted from that
   review under their own Gate 1/Gate 2 authority.

Recommended near-ready payload:

- MR !599, published-artifact-first tooling installation guidance, after its
  revised exact head clears CI, its MR description stops overstating
  `install-and-layouts.md` coherence, and final fact-check/Gate 2 pass;
- MR !590, skill-mirror drift fix and learning, after refresh against current
  main and fresh evidence.

Conditional and schedule-elastic:

- the three already-approved MR !591 slices: Rust authority-ref enforcement;
  executable foundry/adopter guidance and loaders; public human documentation
  and drift checks;
- a new bounded new-adopter information-architecture plan covering the
  accurately narrowed specialist-role forward reference, Exit/Upgrade nav
  order, and the remaining `install-and-layouts.md` chooser;
- issue #30 / MR !463 only if refreshed and independently cleared before freeze.

An incomplete or higher-risk conditional slice rolls to the following release;
it does not block or redefine v2.4.5. Promotion planning and the GitHub mirror
remain parked until the public v2.4.5 version surfaces are settled.

## Delivery sequence

### 1. Repair the portfolio truth

Rebase MR !589 onto current `main`. Replace v2.4.3 review-baseline references
with exact v2.4.4. Replace MR !599's superseded `528fbd2`/pipeline evidence with
its eventual final exact SHA and disposition. Record the ELI5 assessment and
Codex second opinion as planning input, not automatically authorized
remediation. Obtain a refreshed exact-SHA Gate 2 handoff for !589.

### 2. Land near-ready work

Complete !599 without broadening its one-file payload. The remaining
`install-and-layouts.md` work requires its separately bounded Gate 1. Refresh
!590 onto current main, review the learning and thin-pointer wrappers, rerun
applicable validation, and obtain Gate 2.

### 3. Plan the required standardized review

Create a separate Gate 1 plan defining:

- the exact v2.4.4 source and rendered references;
- a shared question set and structured output schema;
- selected web-only LLMs and reviewer personas;
- provenance fields: provider/model as reported, access date, prompt version,
  source snapshot, and any unavailable metadata;
- independence labels that distinguish separate vendors/models from same-run
  persona triangulation;
- factual verification, synthesis, disagreement, and severity procedures;
- evidence-status and maturity vocabulary;
- remediation routing and explicit exclusions; and
- retention/copyright/privacy boundaries for submitted prompts and returned
  text.

Prepare a copy-paste prompt packet with sufficient AGF background for web-only
LLMs, but do not execute the review until the plan receives governor Gate 1 and
its advisory consultations are complete.

### 4. Execute, reconcile, and remediate

Preserve each response with provenance. Fact-check every factual finding
against the exact tagged tree. Treat model agreement as corroboration, not
empirical validation. Obtain contribution-auditor and academic-peer review
where the approved plan requires them. Produce a bounded synthesis.

Documentation-only corrections may proceed under approved or new bounded
plans. Any finding that changes schemas, roles, authority, validators, CI
enforcement, security boundaries, or adopter contracts routes to v2.5.0 rather
than entering this patch release implicitly.

### 5. Freeze and release

After the required review outcome and chosen optional payload are merged:

1. freeze one exact `main` SHA and inventory `v2.4.4..freeze-SHA`;
2. fact-check included claims, exclusions, version surfaces, and release scope;
3. cut `release-2.4.5` from the frozen SHA;
4. update `agf/VERSION`, Rust workspace/lock metadata, governed versioned
   examples, and both changelogs only on the release branch;
5. run full exact-head release CI and final independent fact-check;
6. obtain governor Gate 2 for the exact release head;
7. merge, immediately tag the resulting `main` merge commit `v2.4.5`; and
8. verify the tag pipeline, GitLab Release assets, checksums, provenance, OCI
   image and pull test, shipped binary identity, and public Pages identity.

Do not cut the release branch merely because optional work remains open. Cut it
only after the required standardized review outcome is complete and the
governor has selected the content lock.
