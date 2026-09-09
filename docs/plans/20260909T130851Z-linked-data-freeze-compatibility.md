---
id: 20260909T130851Z-linked-data-freeze-compatibility
status: pending-gate1
topic: linked-data-evidence-model-assessment-before-lifecycle-freeze
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: waived
    reason: >-
      Temporary schema-constrained draft encoding only: issue #130 confirms
      that the plan schema has no pending/requested state. This is not a
      substantive waiver. An independent exact-head InfoSec review of the
      identity, public/restricted projection, minimization, correlation, and
      URI-persistence boundaries is mandatory before Gate 1 eligibility; this
      row must then be replaced with consulted evidence.
  - role: contribution-auditor
    status: waived
    reason: >-
      Temporary schema-constrained draft encoding only: issue #130 confirms
      that the plan schema has no pending/requested state. This is not a
      substantive waiver. A non-author exact-head contribution audit is
      mandatory before Gate 1 eligibility; this row must then be replaced
      with consulted evidence.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Issue #129: linked-data evidence-model assessment before lifecycle freeze

## Decision requested at Gate 1

Approve a bounded, assessment-only work package that tests whether linked-data
standards can represent AGF governance evidence faithfully enough to justify a
later implementation plan. The output is a reviewed evidence packet and an
explicit governor disposition: **adopt, narrow, defer, or reject** each of
PROV-O/JSON-LD, SKOS vocabularies, and the adjacent in-toto/SLSA/CycloneDX
bridges.

This Gate 1 does **not** authorize an adopter-facing export, vocabulary,
namespace, schema field, CLI/MCP command, site claim, release artifact, gate,
validator, or release. Valid JSON-LD or a successful query is not completion.
The plan exists to prevent an attractive serialization from freezing incorrect
provenance semantics into the v2.11 lifecycle design.

## Authority, provenance, and release relationship

- Issue #129 is the authoritative problem statement. Proposal MR !2460 and
  handoff `20260906T202341Z-contribution-auditor-ggss` are inputs, not approved
  design or release scope.
- Governor decision `20260908T185629Z-governor-reviewer-j2gs` adopts the
  `ds4k` sequencing: this assessment completes and is reviewed before the
  `qbka` lifecycle work order freezes Slice A provenance/supersession fields.
- Governor decision `20260909T133626Z-governor-reviewer-3hdk`, published by MR
  !2609, temporarily allocates Codex the planner-architect role for this plan
  only. Claude acknowledged the non-overlap in
  `20260909T132128Z-planner-architect-aars` and owns CP-B plus the coordination
  semantics lane, not this artifact.
- The assessment is a v2.11 compatibility input. It is not itself authority to
  add linked-data implementation to v2.11 or v2.13. Any implementation follows
  only from a later plan whose scope matches the governor's disposition.
- Temporary planner authority ends when the exact reviewed plan head is handed
  to the governor, withdrawn, or superseded. The author then resumes only the
  coder-tester role.

## Problem and hypotheses under test

The source proposal is directionally useful but treats several contested
mappings as nearly one-to-one. The assessment starts with stricter hypotheses:

1. A coordination Markdown file is a `prov:Entity`; the event or activity it
   documents is a separate candidate `prov:Activity`.
2. An actual human, organization, or identifiable software/session actor can
   be a `prov:Agent`. An AGF role is a `prov:Role` on a qualified association,
   not the acting agent. A model/runtime identifier is provenance about an
   actor or execution context, not proof of actor identity.
3. PROV relations are causal or responsibility-bearing assertions. A generic
   `references-to` edge is not automatically `prov:used`,
   `prov:wasDerivedFrom`, `prov:wasInformedBy`, or delegation.
4. AGF-native predicates remain necessary for binding approval, exact-head
   scope, message kind, acknowledgement, resolution, supersession, authority,
   issue/MR state, release ancestry, and privacy projection.
5. JSON-LD can serialize a derived graph but supplies no truth, signature,
   authorization, access-control, closed-world completeness, or gate validity.
6. SKOS can describe controlled concepts. `skos:broader`/`narrower` describe
   conceptual generality, not maturity order, gate sequence, authority, or
   workflow direction; `skos:related` is symmetric and cannot encode handoff.
7. in-toto and SLSA are strong bridges for authenticated artifact/build
   evidence, not replacements for governance decisions. CycloneDX can inventory
   components and reference evidence, while Agent BOM remains emerging rather
   than a normative CycloneDX 1.7 capability.

Each hypothesis may be rejected by the evidence. The assessment must state the
counterexample or cost that changed the disposition.

## Primary standards baseline

The assessment must cite exact sections from official primary sources rather
than relying on this plan's paraphrase:

| Surface | Primary source | Question it constrains |
|---|---|---|
| PROV data model | <https://www.w3.org/TR/prov-dm/> | Entity/activity/agent separation; generation, usage, derivation, communication, association, delegation, revision |
| PROV ontology | <https://www.w3.org/TR/prov-o/> | RDF/OWL classes, qualified relations, `prov:Role`, `prov:Plan`, and domain/range constraints |
| PROV constraints | <https://www.w3.org/TR/prov-constraints/> | Temporal and ordering implications that an AGF mapping must not assert accidentally |
| JSON-LD 1.1 | <https://www.w3.org/TR/json-ld11/> | Serialization, contexts, IRIs, node identity, graph form, and remote-context security considerations |
| SKOS reference | <https://www.w3.org/TR/skos-reference/> | Concept schemes, semantic relations, mappings, and ordered collections |
| in-toto attestation | <https://github.com/in-toto/attestation/blob/main/spec/v1/statement.md> | Subject/predicate binding and the difference between metadata and authenticated attestation |
| SLSA Build Provenance | <https://slsa.dev/spec/v1.2/build-provenance> | Build definition, run details, resolved dependencies, byproducts, and builder trust boundary |
| CycloneDX current specification | <https://cyclonedx.org/specification/overview/> | Current inventory/attestation capabilities |
| CycloneDX Agent BOM proposal | <https://github.com/CycloneDX/specification/issues/895> | Emerging proposal only; its closed-duplicate status and non-normative maturity must be stated |
| CycloneDX behavior-evidence gap | <https://github.com/CycloneDX/specification/issues/1016> | Difference between inventory of what an agent is and evidence of what it did |

Repository prior art must also be reconciled: `docs/sources.md`,
`docs/radar.md`, the two !2460 proposals, issue #129, the `ds4k`/`pe1k`
sequencing records, and the current `agov coord graph-index` output contract.
Sources whose current status cannot be verified are labelled unverified and do
not decide the recommendation.

## Work package A — semantic mapping and negative map

Create a standards-to-AGF matrix with one row per claimed relation or class.
Every row records:

- AGF source field/artifact and its actual governed meaning;
- candidate standard term and exact primary citation;
- cardinality/domain/range/causal or responsibility implication;
- lossiness and omitted AGF semantics;
- confidence (`faithful`, `conditional`, or `reject`);
- stable AGF-native fallback term when the standard mapping is conditional or
  rejected.

The matrix must distinguish at least: record entity, documented event,
human/software session, role, model/runtime, plan, artifact, project,
organization, review, approval, handoff, supersession, generation, use,
derivation, delegation, acknowledgement, exact-head binding, commit/tag
ancestry, and publication verification.

Mandatory negative cases:

- role is not `prov:Agent`; model string is not an acting session;
- record is not the activity it documents;
- `references-to` is not use, derivation, communication, or delegation without
  supporting semantics;
- approval is not inferred from association, authorship, or `prov:hadPlan`;
- handoff is not delegation unless authority/responsibility truly transfers;
- supersession is not `prov:wasRevisionOf` unless it is materially a revision;
- `requires-ack` is not proof that information was received or acted upon;
- chronological/ordinal sequence is not a SKOS hierarchy;
- tag/merge presence is not release authority or provenance completeness;
- valid RDF/JSON-LD is not an authentic or authorized AGF record;
- public-key/login identity is not event authorship without a verifiable
  signature-to-event binding;
- record, role, edge, or model counts are not effectiveness, independence, or
  governance quality.

## Work package B — two fixture-derived candidate graphs

Build two deliberately small, deterministic assessment fixtures from synthetic
coordination records that conform to the current coordination schema. Do not
copy live human names, note bodies, credentials, or sensitive findings.

1. **Ordinary gate chain:** plan entity; plan-authoring event; coder session;
   planner/coder role association; non-author review event; human Gate 1
   decision event; exact plan-head entity; downstream implementation handoff.
2. **Adverse/superseded chain:** an approval tied to one exact head, a later
   incompatible revision, a blocking review, a governor-authorized
   supersession/cancellation, the replacement identifier, and a release-window
   boundary that excludes at least one post-tag verification record.

For each fixture publish:

- minimal schema-valid input records;
- candidate JSON-LD graph with deterministic local/URN identifiers (no public
  namespace commitment and no remote context fetch required);
- an expected-triples or normalized-expanded representation;
- a plain-language lossiness ledger naming every fact intentionally left as an
  AGF-native predicate or omitted from the public projection.

The fixture artifacts are research evidence, not a prototype export command.
No file under `tooling/rust/`, `agf/schemas/`, or `agf/docs/site/` changes.

## Work package C — executable queries and comparator

Provide executable, version-pinned example queries and expected results for:

1. whether reviewer and author are represented as distinct sessions/actors,
   while explicitly stating this does not prove real-world independence;
2. whether each represented Gate 1/2 approval resolves to a human governor
   decision and the exact artifact head it approved;
3. whether a superseded artifact announces the replacement and authorizing
   decision without abusing `prov:wasRevisionOf`;
4. whether a claimed release evidence window proves commit ancestry, event
   time, decision time, tag boundary, and post-tag verification separately.

Run the same four questions against a normalized view of current
`agov coord graph-index` JSON. The assessment must name the concrete query or
consumer that linked data answers materially better. If no query is clearer,
more portable, or less bespoke after setup/maintenance costs are included, the
correct disposition is defer or reject, not implementation for its own sake.

The runnable evidence may use a documented local SPARQL/JSON-LD tool or a small
locked script confined to the assessment directory. It must run offline after
dependencies are available, never dereference remote contexts during the test,
and must record tool/version and exact commands. Adding a project/runtime
dependency outside the assessment directory is a scope expansion requiring a
new Gate 1 decision.

## Work package D — privacy, positioning, and persistence failure analysis

The report must separately assess:

- **public projection:** codename/role and minimized stable identifiers only;
  no bodies, quotations, names, credentials, hidden issue/MR content, or
  cross-project joins;
- **restricted projection:** explicit audience, retention, access, and
  revocation assumptions; it is not created merely because richer data exists;
- pseudonymous session linkage and model-field trust, including cross-record
  re-identification and participation-pattern inference;
- remote JSON-LD context resolution, URI tracking, content drift, cache and
  availability risks;
- repository/Pages URIs, redirect services such as w3id, and archival
  identifiers, without choosing or reserving a permanent namespace;
- standards theatre, count gaming, surveillance incentives, false precision,
  and the explicit rule that participation volume does not evidence review
  quality, effectiveness, independence, or human accountability;
- AGF positioning: interoperability and independent inspectability may be
  credible; compliance, security, authenticated identity, proof of policy
  validity, or superior outcomes are not established by this assessment.

Sensitive identity/privacy/security judgments are routed to the
infosec-sentinel. The public assessment records the safe conclusion and threat
category, not exploitable detail or protected source content.

## Work package E — freeze contract and disposition

The final report gives one of `adopt`, `narrow`, `defer`, or `reject` for each:

- optional PROV-O/JSON-LD derived view;
- optional SKOS concept schemes;
- in-toto/SLSA release-artifact bridge;
- CycloneDX inventory/evidence reference bridge;
- per-release role/model participation map from !2460.

Before `qbka` Slice A design is finalized, the report must provide a compact
compatibility contract stating whether lifecycle work should reserve stable
identifiers and distinct fields for record Entity, documented Activity,
actual actor/session, role, model/runtime, plan/artifact, project/organization,
replacement identifier, authorizing decision, and disposition timestamp.

That contract may preserve future compatibility. It must not add RDF terms,
URIs, export fields, or an ontology to the lifecycle schema without a separate
approved implementation plan. The governor's explicit disposition, recorded in
coordination, determines which recommendations may become future Gate 1 scope.

## Affected-file estimate

Assessment implementation is limited to this new evidence packet:

| Path | Purpose |
|---|---|
| `docs/reviews/linked-data-evidence-model-assessment.md` | Main matrix, positioning/privacy analysis, comparator, freeze contract, recommendation |
| `docs/reviews/linked-data-evidence-model/README.md` | Reproduction commands, pinned tools, fixture provenance, limitations |
| `docs/reviews/linked-data-evidence-model/fixtures/ordinary/` | Synthetic schema-valid input, candidate graph, expected normalization/results |
| `docs/reviews/linked-data-evidence-model/fixtures/adverse-superseded/` | Synthetic adverse/superseded input, graph, expected normalization/results |
| `docs/reviews/linked-data-evidence-model/queries/` | Four executable queries plus comparator queries/expected results |
| `docs/coordination/*.md` | Claims, independent reviews, disposition, completion and handoffs required by normal governance |

If the assessment shows that a tiny local runner script is necessary, it must
remain under `docs/reviews/linked-data-evidence-model/` and use only an
already-available or explicitly pinned assessment dependency. Any change to
product code, schemas, rules, validators, CI, AGENTS/assistant projections,
public site, changelog, roadmap, release metadata, or URI infrastructure is out
of scope and returns to Gate 1.

## Roles, independence, and review order

1. After Gate 1, a coder-tester or separately allocated research implementer
   produces the packet without editing product/governance surfaces.
2. A non-author contribution-auditor checks source fidelity, hostile mappings,
   negative cases, evidence completeness, standards-theatre risk, and whether
   the recommendation follows from the fixtures rather than preference.
3. The infosec-sentinel reviews both fixtures and conclusions for
   re-identification, cross-record linkage, remote-resolution, stable-URI,
   credential, protected-content, and disclosure risks.
4. A fact-checker or academic-peer-reviewer, if explicitly activated by the
   governor, checks exact standards citations and claim strength. This is
   advisory and does not replace the two mandatory reviews above.
5. The governor makes the binding assessment disposition at Gate 2 (and any
   later implementation Gate 1). The assessment author cannot approve its own
   recommendation.

The current draft is not Gate 1 eligible while either front-matter row uses the
schema-constrained temporary `waived` value. Exact-head review records must be
incorporated and both rows replaced with truthful `consulted` evidence before
the Gate 1 handoff.

## Acceptance evidence

Gate 2 requires all of the following at the exact assessment head:

- every primary-source URL resolves or is recorded as unavailable with no
  unsupported inference; cited sections support the stated semantics;
- the mapping matrix covers every concept and relation named in Work package A
  and explicitly lists unmapped AGF semantics;
- both fixture inputs validate under the current coordination schema;
- graphs normalize deterministically and queries reproduce documented expected
  results offline;
- each negative mapping has a failing/adverse example or explicit reason it
  would overstate semantics;
- the graph-index comparator uses its documented output contract and reports
  invalid/partial input honestly;
- public/restricted projection and re-identification analyses receive an
  exact-head InfoSec disposition;
- claims use both AGF capability maturity and evidence-status vocabulary; no
  prototype or valid serialization is called Deployable or empirically
  validated;
- the freeze contract is concise enough for the `qbka` planner to consume
  without importing an ontology;
- `git diff --check`, SPDX checks, plan/coordination validation, and the
  repository's applicable documentation tests pass;
- independent contribution-auditor review is CLEAR or every finding is
  resolved/explicitly dispositioned by the governor.

## Collision and sequencing controls

- Re-fetch and re-scan issue #129, !2460, open MRs, the lifecycle work-order
  branch/MR, coordination-semantics work, and `docs/reviews/linked-data-*`
  before implementation and again before Gate 2.
- The assessment may proceed in parallel with CP-B and unrelated
  queue-convergence batches because their files and authority are distinct.
- The `qbka` planner may collect evidence in parallel but must not freeze Slice
  A's provenance/supersession contract until this assessment is reviewed and
  the governor records its disposition.
- A concurrent edit to any estimated packet path pauses mutation until the
  sessions publish and acknowledge a temporary allocation.

## Failure, rollback, and closure

The work is reversible: remove the assessment branch/MR and its new packet
files. No adopter data, schema, command, namespace, release artifact, or public
contract is migrated. A negative conclusion is a valid successful outcome.

Issue #129 closes only when the reviewed packet and explicit governor
disposition are visible. MR !2460 is then retained, narrowed, superseded, or
closed according to that disposition; this plan does not pre-decide its forge
state. The assessment handoff must name any later Gate 1 plan required and must
not silently convert a recommendation into implementation authority.
