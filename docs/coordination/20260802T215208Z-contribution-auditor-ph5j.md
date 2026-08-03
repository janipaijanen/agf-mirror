---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260802T215208Z-contribution-auditor-ph5j
timestamp: 2026-08-02T21:52:08Z
model: codex-gpt-5
immutable: true
references-to: []
topic: ai-regulation-gap-analysis-gate1
requires-ack: true
artifacts_owned: []
---

# Contribution audit: AI regulation requirements gap analysis

Scope: MR !967 exact head
`8c92c68ae30f3bc0e538c2f391a1e5ec15224c6e`, sole changed file
`docs/plans/ai-regulation-requirements-gap-analysis.md`.

Coverage: COMPLETE for contribution-auditor scope: every maturity assignment,
current-capability claim, gap claim, scope/completeness claim, and the boundary
between capability maturity and evidence strength. Legal dates, clause text,
and applicability remain the separately declared fact-checker review; AI-2 and
AI-5 security/privacy analysis remains the separately declared
infosec-sentinel review.

Recommendation: BLOCK Gate 1 until the six findings below are corrected and
the fact-checker and infosec reviews are completed or explicitly dispositioned
by the governor-reviewer.

Blocking findings: 6

Advisory findings: 0

Authority: recommendation only; governor-reviewer decides Gate 1.

Evidence: exact-head merge-request pipeline `2725561849` succeeded
(`agov:governance:validate`). The plan passes `agov plan validate` and its
range passes `git diff --check`. Structural validity does not resolve the
semantic findings below. Review compared the plan with the canonical maturity
vocabulary, named rules and plans, Rust/MCP surfaces, source registry, radar,
and the newly verified AIMS/WIMSE source distinction.

## B-REG-1 — one maturity number conflates base artifacts with requirement support

Check: each summary maturity follows one reproducible unit of assessment

Outcome: fail

Impact: material

Enforcement: blocking

Basis: `implementation-honesty-rule` and the canonical maturity vocabulary

Finding state: open

The document says maturity measures how well AGF meets each requirement, but
several rows assign a higher level because a generic coordination trail or a
source-registry note exists. This produces contradictions under the document's
own legend:

- AI-4 is Level 3 although its text says no framing, plan, or issue exists and
  that this document is the first concrete step: Level 1 as requirement support.
- AI-10 is Level 2 solely because `docs/sources.md` names a target action; that
  is a design note and therefore Level 1, not an interface.
- AI-12 repeats the same Level-2-for-a-source-note error.
- AI-1, AI-3, AI-7, and AI-9 mix a mature generic process artifact with an
  unimplemented requirement-specific mapping, so their aggregate number cannot
  be reproduced from the stated rubric.

Resolution condition: either give every row separate `underlying AGF
capability` and `requirement-specific support` maturity fields, or define and
apply a conservative aggregation rule consistently. A source note, issue, plan,
or proposed adapter is Level 1 unless an actual interface has been declared.

## B-REG-2 — Art. 26 analogies are presented as operational coverage

Check: AI-1 distinguishes development-governance evidence from high-risk-system
deployer evidence

Outcome: fail

Impact: material

Enforcement: blocking

Basis: verifiable-claims and implementation-honesty rules

Finding state: open

The applicability caveat is good, but the capability section then calls Gate
1/Gate 2 "exactly" named-human oversight and treats coordination records as
Art. 26 auto-generated logs retained for six months. AGF's records evidence
development decisions; they are not automatically generated runtime logs under
the deployer's control, and code-change approval is not by itself the
high-risk-system human-oversight process. Indefinite git persistence also is
not a declared retention policy. These are useful analogies, not validated
Art. 26 coverage.

Resolution condition: label the mapping Theoretical, remove "exactly" and the
claim that coordination records satisfy the logging floor, and state that an
adopter-specific mapping must identify the actual system logs, oversight
instructions/competence/authority, and retention controls before any coverage
level can be assigned.

## B-REG-3 — the generic security rule is called enforced CRA policy

Check: AI-2 describes the actual enforcement and policy surface

Outcome: fail

Impact: material

Enforcement: blocking

Basis: tool-authority-boundary and verifiable-claims rules

Finding state: open

`security-hardening-rule.md` is an active normative agent rule, but it contains
generic adopter placeholders and an empty security-test command. Several items
are instructions rather than mechanically configured checks. Saying it "is
enforced on every change" and is a real applied cybersecurity policy erases
that distinction and overstates the repository evidence.

Resolution condition: describe the rule as Level 5 shipped normative guidance
whose project-specific/mechanical enforcement is partial and separately
evidenced; do not equate it with a CRA public cybersecurity policy. Retain the
valid finding that the present vulnerability-reporting artifact is stale.

## B-REG-4 — source completeness and primary-source claims are false

Check: declared input corpus matches the actual source registry

Outcome: fail

Impact: material

Enforcement: blocking

Basis: verifiable-claims rule

Finding state: open

The introduction says every regulation and standard in the registry is covered
and that primary-source discovery is done. The same document later admits that
the AI Act dates, MCC-AI clauses, ISO clauses, and SSDF revision need primary
verification; the PLD and GDPR rows rely on secondary sources at this head.
The registry also contains Finland's AI Act supervision entry
(`valtioneuvosto.fi`, lines 263-266) but no corresponding row. A primary GDPR
entry exists only on unmerged MR !962 and cannot be treated as part of this
exact head.

Resolution condition: narrow the scope claim to an explicit enumerated subset
and say primary verification is incomplete, or cover every classified item and
replace secondary support where required. Do not rely on unmerged source-branch
content without naming that dependency.

## B-REG-5 — maturity is used without evidence-status labels

Check: capability existence is separated from strength of the claimed mapping

Outcome: fail

Impact: material

Enforcement: blocking

Basis: `maturity-vocabulary.md` and AGF telemetry-memory guidance

Finding state: open

Rows such as AI-3, AI-4, AI-7, and AI-9 make benefit or standards-mapping
claims—e.g. AGF evidence can answer NIS2 supplier duties, aid a PLD rebuttal,
has the shape ISO 42001 expects, or implements "much" of SSDF—without the
separate evidence-status label required for framework claims. The document
occasionally hedges the prose, but maturity alone answers only what exists, not
whether the claimed relationship is validated.

Resolution condition: add an Evidence status field to every row. Use
Theoretical for unvalidated crosswalks; use Empirically informed only where
foundry or adjacent-domain evidence is named; do not use Empirically validated
without multiple independent projects. Replace the SSDF "implements much"
claim with a proposed crosswalk until practice-ID review exists.

## B-REG-6 — AI-15 collapses distinct AIMS and WIMSE states

Check: emerging standards row preserves the verified identity-source boundary

Outcome: fail

Impact: material

Enforcement: blocking

Basis: verifiable-claims rule and source-registry precision

Finding state: open

AI-15 calls AIMS/WIMSE one immature agent-identity standards effort and cites
the older aggregate registry note. The verified source crosswalk now shows a
material distinction: WIMSE is an active IETF working group for workload
identity, while AIMS is presently an IETF 126 presentation/proposal, not a
working group or established draft series. The document must not imply equal
status or a single protocol lineage.

Resolution condition: split or precisely qualify AIMS and WIMSE, cite their
primary Datatracker pages, and leave any broader identity-protocol conclusion
to the fact-checker. Also distinguish "no deployed threat model" in AI-13 from
the existing pending plan `docs/plans/threat-model-stpa-owasp.md`, which is the
artifact supporting Level 2.

## Required next sequence

1. Planner-architect corrects B-REG-1 through B-REG-6 without marking the plan
   approved.
2. Fact-checker verifies the legal/standards corpus against primary sources;
   infosec-sentinel reviews AI-2 and AI-5.
3. Publish a corrected exact head and green pipeline, then request renewed
   contribution audit.
4. Governor-reviewer makes the binding Gate 1 decision only after all three
   advisory lanes have visible dispositions.
