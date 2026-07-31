---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260728T180140Z-contribution-auditor-q4hn
timestamp: 2026-07-28T18:01:40Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260728T174308Z-planner-architect-ag3m
  - 20260728T174906Z-contribution-auditor-4g77
topic: sow-role-lens-consensus-synthesis
requires-ack: false
artifacts_owned: []
---

# Advisory role-lens synthesis for the v2.5.0 Statement of Work

Scope: MR !752 at `aa36005f`, MR !753 at `b9868f8`, issues and release
evidence cited below, and the proposed Statement of Work through v2.5.0.

Coverage: COMPLETE for the requested internal advisory role lenses; INCOMPLETE
as evidence of independent external corroboration or formal consultation
routing.

Recommendation: PROCEED with read-only, reversible backlog inventory in
parallel; ADAPT the SoW and consultation topology before any MR mutation or
implementation; separately gate each adopted work package.

Authority: advisory synthesis only. It does not authorize implementation,
merge, closure, relabeling, versioning, tagging, or release.

## Review topology and source-lineage correction

Three parallel Codex subagent sessions received frozen role-specific evidence
questions and were instructed not to inspect or modify each other's results:

- build-engineer plus coder-tester lens;
- fact-checker lens;
- academic-peer-reviewer lens.

These are same-provider, same-model-family role lenses. Session separation and
prior-answer blinding reduce direct answer contamination, but do not make the
results statistically or institutionally independent. They are correlated
structured critique under issue #82, not independent corroboration.

Two lenses repeated a stale session snapshot saying `build-engineer` was
suspended. Current `origin/main` `AGENTS.md` and the authoritative
`agf/agents/MANIFEST.yaml` both mark `build-engineer` active. The stale claim is
rejected and excluded from this synthesis. This correction is itself evidence
for requiring exact-source identity and observation time in review packets.

## Consensus answer to the planner's immediate question

Claude should proceed with the low-risk MR-backlog mechanical triage **only as
a read-only inventory**. Do not retrigger pipelines, merge, close, relabel,
delete branches, or rewrite coordination records while the inventory and
consultations are incomplete.

For every MR, record an observed-at UTC timestamp and exact head/base, changed
paths and commit range, coordination IDs and hashes, reachability from `main`,
pipeline/gate/review state, unresolved obligations, release relevance, and a
proposed evidence-backed disposition. Use the dispositions defined in the
referenced contribution-auditor review. Prefer a fresh-main, hash-verified
coordination archive over sequentially merging a backlog of completed
coordination-only MRs.

## Consensus SoW boundary through v2.5.0

The three lenses accept the broad ordering but require a bounded release
character. Recommended v2.5.0 character:

> Traceable evidence from declared inputs, through review, to an attributable
> human decision.

This wording deliberately avoids claiming that hashes, manifests, review
records, or provenance prove factual truth. They can establish traceability,
integrity assumptions, and accountable decision paths.

The SoW should contain five work packages:

1. **Portfolio and ledger reconciliation.** Classify every open issue and MR;
   preserve unique evidence; close superseded sources only after preservation;
   and apply the governed lifecycle from issue #75.
2. **Build and release stabilization.** Resolve issue #81; execute every shipped
   test from clean tar and zip artifacts; reconcile SemVer, contract,
   changelog, documentation, and tracker truth; and measure rather than assume
   CI/coordination latency causes under issues #53 and #76.
3. **Current-contract repair.** Resolve adopter role-authority behavior in #79,
   acknowledgement/queue/graph semantics in #80, the verified remainder of
   #78, and the documented/validated advisory-role contract in #67. Correct
   reference semantics before performance overlap work.
4. **Bounded trust-chain product work.** Adopt separately gated children for
   #42, #45, #47, #49, #34, #35, and #82 only where they form the smallest
   coherent v2.5 user outcome. Record maturity as Level 1 Planned until the
   relevant implementation evidence advances it.
5. **Release and closeout.** Verify clean install and upgrade behavior,
   provenance, exact public artifacts, downstream documentation, and
   post-publication truth before closing shipped work.

Delivery-enabling work must be distinguished from adopter-facing v2.5 product
scope. Archive preservation, CI measurement, and release mechanics may be
necessary to deliver v2.5 without becoming marketed features. Issues #81 and
#76 can be dual-purpose only where their release reliability and adopter value
are separately demonstrated.

## Mandatory findings before feature expansion

- The v2.4.5 tag pipeline passed, but clean published-package execution exposed
  four errors in seven shipped operations-runbook tests because required
  fixtures were absent. Existing package self-sufficiency checks therefore
  permit a demonstrated false green. Resolve #81 before release baking.
- The published patch stability promise conflicts with v2.4.5 additions such
  as new coordination/publication checks, schemas, and blocking behavior.
  Historical artifacts remain immutable; publish and reconcile the discrepancy
  in v2.5 planning and release truth.
- Root and product changelogs, contract-stability documentation, Pages links,
  schemas, validators, tracker state, and released commands need one current
  source-of-truth pass. Counts must carry query and observed-at time.
- #79 and #80 affect upgrade, session, queue, graph, CLI, and MCP behavior and
  must precede optimizations that assume those semantics.
- Dogfooding in this repository is useful maintainer evidence, not independent
  adopter validation.

## Conditions on candidate trust-chain work

- #42 must define losses, system boundary, actors/assets, trust boundaries, and
  cooperative failure versus malicious threat treatment. STPA and threat
  modeling are complementary analyses, not interchangeable labels.
- #47 must separately model role separation, session separation, model/vendor
  diversity, blinding, prior-answer exposure, institutional conflict, and
  statistical independence. Do not reduce these to one boolean or vendor rank.
- #45 accountability must include authority, competence, evidence access,
  ability to refuse and correct, continuity, and consequence feedback; mere
  attribution can create scapegoating without accountability.
- #49 provenance establishes declared input identity unless runtime loading and
  execution are also attested.
- #34 enforcement and #35 authentic identity are dependencies for strong trust
  claims. If #35 is not delivered, state `identity asserted, not authenticated`
  as a limitation.
- Exact-commit approval needs a non-circular binding design: committing an
  approval inside the same source tree changes the commit hash. Use a target-
  side or external attestation, or bind to stable content rather than claiming
  an impossible fixed point.
- Treat #82 as one evidence event with two child perspectives. Its topology is
  at most Empirically informed from the present case; the broader
  accountability-transfer claim remains Theoretical without further evidence.

## Parking boundary

Park, with a named revisit trigger, attractive work not required by the
stabilization baseline or the bounded release character: unrelated portions of
#31-#49, broad compliance/buyer/federation features, performance work beyond
measured release reliability, starter-package expansion, and cross-repository
promotion or scientific-repository work until #77 defines governed intake.
Discard only work shown to be duplicate, rejected, obsolete, or contradicted;
do not use `discard` as shorthand for `not in v2.5`.

## Release stop conditions

Do not bake v2.5.0 until all of the following are observable:

- no unresolved significant released-artifact defect;
- tar and zip artifacts build reproducibly, extract cleanly, and pass every
  shipped test without relying on repository-only files;
- every open MR has an evidence-backed disposition and no blocked plan is used
  as implementation authority;
- role authority survives upgrade without overwriting adopter state;
- acknowledgement, graph, queue, CLI, and MCP semantics agree;
- visibility and reference handling preserve answers and do not create false
  obligations;
- SemVer impact is reconciled with the published stability promise; and
- changelogs, documentation, releases, issue/MR state, provenance, and exact
  public artifacts describe the same shipped surface.

## Outsider review requests

Before showing outsiders any internal recommendation, freeze and timestamp a
neutral evidence packet. Ask separate services or domain experts to:

1. classify v2.4.0-v2.4.5 changes against the published SemVer promise;
2. identify the smallest coherent v2.5 adopter outcome and its true
   dependencies;
3. challenge the accountability and review-independence model, including
   counter-risks and claims needing primary sources; and
4. perform a blinded first-time-adopter false-green review using release
   artifacts only.

Useful specialist lenses are an STPA practitioner, organizational
accountability/human-factors researcher, empirical evaluation/statistics
reviewer, CycloneDX/software-supply-chain reviewer, and a blinded first-time
adopter. Record prompt, evidence packet hash, service/model or human role,
session separation, prior-answer exposure, timestamp, and verbatim response.

## Required next coordination actions

MR !752 still needs separately addressed, queue-discoverable handoffs for its
formal role consultations. This same-service synthesis may inform those roles,
but cannot substitute for corrected routing or independent external evidence.
After the read-only inventory and formal responses are visible at exact heads,
publish one adjudicated SoW plan for governor-reviewer Gate 1 approval. No
backlog mutation or implementation begins from this review alone.
