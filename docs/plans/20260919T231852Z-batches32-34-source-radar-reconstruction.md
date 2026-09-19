---
id: 20260919T231852Z-batches32-34-source-radar-reconstruction
status: pending-gate1
topic: batches32-34-source-radar-reconstruction
advisory_consultation:
  - role: contribution-auditor
    status: pending
    reason: >
      Requested before Gate 1 because the recovered text makes comparative
      governance, maturity, performance, and competitive-positioning claims
      across three stale source batches.
  - role: infosec-sentinel
    status: pending
    reason: >
      Requested before Gate 1 because Batch 33 discusses MCP admission,
      authentication, credential handling, sandboxing, and revocation, while
      Batch 34 discusses runtime approval and policy enforcement. Review must
      ensure that descriptive source material cannot be read as AGF security
      guidance or authorization.
affected_paths:
  - docs/sources.md
  - docs/radar.md
depends_on:
  - "!2306"
  - "!2390"
  - "!2408"
---

# Plan: reconstruct source and radar Batches 32–34 on current main

## Problem and decision boundary

MRs !2306, !2390, and !2408 contain three source-classification batches and
five related radar edits, but each branch is more than 1,500 mainline commits
behind and conflicts in the two shared registry files. Their unique research
should not be lost, and their stale histories should not be merged.

The normal sources/radar maintenance exception is unavailable. Batches 33 and
34 include security, authentication, licensing, governance-framework,
competitive-positioning, performance, and maturity material, all of which are
explicit fast-path disqualifiers. This plan therefore uses the full Gate 1 and
Gate 2 workflow.

The work classifies public material and preserves non-binding questions. It
does not install, execute, trial, integrate, depend on, endorse, prioritize, or
assign implementation of any source. External tools, protocols, scores,
identity systems, policies, approvals, and verdicts acquire no AGF authority.

## Planned reconstruction

1. Recover the unique Batch 32 AgentConnect classification from !2306. Keep
   the repository link pinned to reviewed commit
   `4eff3b12cf296cd4bde216dae6155f9352fa814e`, recheck its primary architecture
   source, and retain the external operator-plane question without replacing
   the repository trail as canonical authority.
2. Recover the eleven classified Batch 33 entries plus its explicit dropped
   item from !2390. Recheck canonical identities and named primary pages,
   distinguish source-authored figures from AGF evidence, preserve licensing
   caveats, and retain only the two non-binding radar questions about adjacent
   MCP governance and local Git-resident advisory memory.
3. Recover the twelve Batch 34 entries from !2408. Recheck canonical
   identities, evidence classes, licences where asserted, and claim limits.
   Retain only the two non-binding radar questions about claim-by-pattern A2A
   coordination and advisory cost/spend governance.
4. Insert Batches 32–34 into `docs/sources.md` after Batch 31 so the recovered
   allocation is unambiguous despite later historical batch-number ordering.
5. Reconcile the five radar edits against current `docs/radar.md`: amend the
   existing Piyaz operator-plane item for AgentConnect rather than duplicating
   it; add the four still-distinct open questions; preserve controversy,
   explicit non-adoption language, source cross-references, and `review-by`
   dates.
6. Publish one implementation MR from current `main`. After its exact-head
   review and Gate 2 merge, close !2306, !2390, and !2408 with notes pointing
   to the current carrier, exact pipeline, review record, and merge commit.

## Evidence and claim controls

- Primary repository or project documentation is the default evidence source;
  secondary posts remain labelled as discussion or experience reports.
- Stars, commit counts, versions, operational figures, benchmark results,
  adoption counts, security findings, and performance claims remain
  source-authored unless independently reproduced. Volatile counts may be
  removed instead of refreshed when they add no classification value.
- Licence and project-status statements must be supported by the reviewed
  primary surface or explicitly labelled unverified. No source text is copied
  beyond short identifiers or phrases necessary for accurate classification.
- Security and authentication mechanics are descriptive prior art, not
  instructions, recommendations, assurances, or AGF controls.
- Comparative labels describe registry relation only. They do not make public
  claims of AGF superiority, compatibility, conformance, or effectiveness.

## Collision and sequencing controls

Before implementation and again immediately before Gate 2:

- fetch and prune `origin`;
- inventory every open MR targeting `main`, recording IID, head SHA,
  `updated_at`, and changed paths for any MR touching `docs/sources.md` or
  `docs/radar.md`;
- check existing registry URLs, repository identities, batch headings, radar
  titles, and edit regions for duplicates or allocation conflicts;
- stop and refresh the reconstruction if a relevant target or MR changes.

The three stale MRs are acknowledged in `depends_on` and remain open until the
replacement carrier has merged. No other implementation branch may edit the
same registry regions without an explicit sequencing disposition.

## Acceptance and validation

- Only `docs/sources.md` and `docs/radar.md` change in the implementation MR.
- Every recovered source has a named URL, evidence class, access date, material
  limitations, and a canonical-identity decision.
- Every radar entry remains an open, controversial or unresolved question with
  a source cross-reference and `review-by` date.
- Independent contribution and infosec consultations are resolved before this
  plan becomes `approved` or `active`.
- `agov plan validate`, `agov plan collision-check`, targeted duplicate and
  batch scans, `git diff --check`, and the repository documentation validators
  pass.
- Gate 2 uses a fresh target/open-MR collision inventory and exact-head
  pipeline evidence.

The coder-tester may implement only after a binding governor-reviewer Gate 1
record. The governor-reviewer retains the binding Gate 2 merge decision.
