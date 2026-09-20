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
  - "!967"
---

# Plan: reconstruct source and radar Batches 32–34 on current main

## Problem and decision boundary

MRs !2306, !2390, and !2408 contain three source-classification batches and
eight logical radar edits: five decision questions and three auxiliary
watch-list edits. Each branch is more than 1,500 mainline commits behind and
conflicts in the two shared registry files. Their unique research should not
be lost, and their stale histories should not be merged.

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
   caveats, and evaluate the two non-binding radar questions about adjacent MCP
   governance and local Git-resident advisory memory. Separately disposition
   the proposed OWASP MCP watch-source addition as retain, fold into an
   existing watch source, or drop as redundant, with a reason.
3. Recover the twelve Batch 34 entries from !2408. Recheck canonical
   identities, evidence classes, licences where asserted, and claim limits.
   Evaluate the two non-binding radar questions about claim-by-pattern A2A
   coordination and advisory cost/spend governance. Separately disposition
   the proposed competitor-list extension and Machine Testimony watch-source
   addition as retain, fold, or drop, with reasons.
4. Insert Batches 32–34 into `docs/sources.md` after Batch 31 so the recovered
   allocation is unambiguous despite later historical batch-number ordering.
5. Reconcile all eight logical radar edits against current `docs/radar.md`:
   one AgentConnect/Piyaz amendment, four candidate decision questions, and
   three auxiliary watch-list edits. Do not pre-decide that a candidate is
   still distinct. Map each candidate's underlying decision, evidence, owner,
   and review trigger against the current radar, open issues, and newer source
   batches, then explicitly amend an existing item, merge, add, or drop it.
   Preserve controversy, explicit non-adoption language, source
   cross-references, and `review-by` dates for every retained question.
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

### Per-entry reconstruction matrix

Before writing registry prose, produce a reviewable matrix in the
implementation MR description or another artifact within the approved two-file
scope. For every candidate it records: source MR and head, old URL and material
claim, canonical project name and owner, canonical repository and site/repo
relationship, fresh primary source and exact commit/tag/release where
available, retrieval UTC and access result, identity/redirect/fork/rename
decision, licence and project-status evidence, volatile-field disposition,
claim-level current/historical/unverified classification, and radar-dedup
outcome.

Every old assertion receives one explicit disposition: current as of a named
fresh revision/date, historical source statement as of the old review date, or
not presently verified and therefore omitted or marked as such. A new access
date must never be attached to copied stale prose. Dynamic pages record the
retrieval UTC and, where lawful and available, a content digest or archive
provenance. An inaccessible source records URL, UTC attempt, and failure mode;
it cannot establish current capability, status, or licence. Secondary evidence
remains labelled and cannot establish licence or formal project status.

Repository ownership, formal foundation/program status, development or release
status, repository licence at the exact revision, and hosted-service/site terms
are separate facts. Organisation placement or naming does not establish formal
project status. If no primary licence artifact is verified at the reviewed
revision, say that no licence was verified for that revision and keep the item
citation-only. Volatile counts are removed by default; any retained count must
materially affect classification, be freshly measured, and be labelled with
UTC and source-reported/volatile status under one consistent batch-wide policy.

### Semantic security reconstruction controls

Every authentication, credential, revocation, sandbox, approval, policy,
identity, audit, immutability, default-deny, or supply-chain capability is
subject-attributed (`the source describes/claims`) and pinned to the reviewed
repository/specification revision when possible. Such descriptions are never
AGF controls, assurances, authorizations, recommendations, operational recipes,
or endorsements. In particular:

- do not classify OWASP MCP Governance as a standard or imply formal OWASP
  project endorsement without primary status evidence; verify the exact GPL
  identifier and use citation or short paraphrase only;
- frame the OAuth article's bearer-token, verifier, refresh, HMAC, and
  revocation design only as one author's reported implementation, not
  production validation, reusable guidance, or an endorsed OTGR design;
- attribute absolute capability language for InterMCP, FULCRUMAXE, Radia,
  Bridle, Actradeck, and the local swarm, preserving optional, best-effort, and
  deployment-limit qualifications;
- remove any prediction that AGF would score well in Machine Testimony and do
  not present its census as certification or an established standard;
- say AGF rules *address* review-time concerns only where accurate; never imply
  mechanical enforcement without a named rule granting blocking authority;
- treat interns-review as source-described reviewer separation, not independent
  review under AGF's governed independence criteria; and
- keep claim-by-pattern/A2A material non-binding prior art. It is neither
  load-bearing nor a prerequisite for a future AGF gate.

## Collision and sequencing controls

Before implementation and again immediately before Gate 2:

- fetch and prune `origin`;
- inventory every open MR targeting `main`, recording IID, head SHA,
  `updated_at`, and changed paths for any MR touching `docs/sources.md` or
  `docs/radar.md`;
- check existing registry URLs, repository identities, batch headings, radar
  titles, and edit regions for duplicates or allocation conflicts;
- stop and refresh the reconstruction if a relevant target or MR changes.

The three stale source-batch MRs and MR !967 are acknowledged in `depends_on`.
The direct 2026-09-19 forge scan found that !967 also changes
`docs/sources.md`, although its regulatory-source corrections are in an older
historical batch rather than the Batch 32–34 insertion region. Treat that as a
real path collision: if !967 advances or merges, refresh the target, rebase the
implementation, and verify that both its corrections and the reconstructed
batches survive. The three stale source-batch MRs remain open until the
replacement carrier has merged. No other implementation branch may edit the
same registry regions without an explicit sequencing disposition.

The forge-inclusive collision checker is supporting evidence, not proof of a
clear worktree. At reviewed base `dcc506f5`, the fail-on-unacknowledged command
exits 1 because ten remote-tracking refs still touch one or both registry
paths; four live forge-MR overlaps are acknowledged via `depends_on`, 205
historical plans have no `affected_paths`, and two MR diffs are unavailable.
Some flagged refs belong to already merged or closed work and are not live
implementation claims. The decisive control is therefore the fresh direct
all-open-MR path inventory plus explicit disposition of every flagged remote
ref that remains independently live. The checker output must be recorded
honestly; it must not be described as passing while it exits non-zero.

## Acceptance and validation

- Only `docs/sources.md` and `docs/radar.md` change in the implementation MR.
- The reconstruction matrix covers every recovered or dropped source and
  records identity, exact evidence revision or retrieval, access result,
  licence/status basis, volatile-field handling, claim freshness, and radar
  disposition.
- All eight stale radar edits have an explicit retain, fold, amend, merge, or
  drop disposition. Every retained question remains open, controversial or
  unresolved and has a source cross-reference and `review-by` date.
- Independent contribution and infosec consultations are resolved before this
  plan becomes `approved` or `active`.
- `agov plan validate`, targeted duplicate and batch scans, `git diff --check`,
  and the repository documentation validators pass. The collision checker is
  rerun and its actual exit/result recorded; the direct all-open-MR scan finds
  no unacknowledged live overlap, and every still-live remote-ref collision has
  an explicit sequencing disposition.
- Gate 2 uses a fresh target/open-MR collision inventory and exact-head
  pipeline evidence.

The coder-tester may implement only after a binding governor-reviewer Gate 1
record. The governor-reviewer retains the binding Gate 2 merge decision.
