<!-- Radar carrier. Retained verbatim from MR !967 (source branch docs/ai-regulation-gap-analysis, head ae29daf8), file docs/plans/ai-regulation-requirements-gap-analysis.md. The 26-line docs/sources.md change of that MR is not carried; it stays on the retained branch.
     Non-authoritative: parked on the radar, grants no Gate 1, rule, template, validator or implementation authority. -->
---
id: ai-regulation-requirements-gap-analysis
status: pending-gate1
topic: ai-regulation-and-governance-requirements-gap-analysis
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Bounded mechanical review (`20260803T073120Z-infosec-sentinel-nqh5.md`)
      at exact head `a78dd266`: CLEAR mechanically, no blocking findings.
      `agov validate infosec` and `agov validate learnings` both OK. This
      establishes only the configured credential/cross-record scan is
      clean — it does not semantically clear the CRA/GDPR prose, which
      remains fact-checker and governor-reviewer scope.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Five review rounds. Round 1 at exact head `8c92c68a`
      (`20260802T215208Z-contribution-auditor-ph5j.md`): BLOCK, six
      findings (B-REG-1 through B-REG-6). Round 2 at exact head
      `bdfc4b02` (`20260802T222149Z-contribution-auditor-b72h.md`):
      B-REG-2 through B-REG-6 confirmed resolved; one renewed finding
      (B-REG-1-R2, the Capability/Mapping coupling was not fully
      corrected in round 1). Round 3 (bounded) at exact head `284c621c`
      (`20260802T224132Z-contribution-auditor-krj5.md`): BLOCK, three
      findings (B-REG-1-R3, B-REG-7, B-REG-3-R2). Round 4 (bounded) at
      exact head `61ea4c74` (`20260802T230052Z-contribution-auditor-r7mg.md`):
      B-REG-7 and B-REG-3-R2 resolved; one residual finding (B-REG-1-R4,
      AI-7/AI-8 grounded the learning loop on unspecified rule files
      that don't define it). Round 5 (bounded) at exact head `84b90a89`
      (`20260802T231559Z-contribution-auditor-h7ha.md`): B-REG-1-R4's own
      fix cited a nonexistent `agf/learnings/_template.yaml` (finding
      B-REG-1-R5); corrected to the real `agf/templates/learning-record.yaml`.
      Final bounded review at exact head `a78dd266`
      (`20260803T233331Z-contribution-auditor-rbdm.md`): B-REG-1-R5 and the
      complete contribution-auditor chain CLEAR. No further contribution-
      auditor action pending unless this revision's factual corrections
      (below) surface a new contract-shape finding.
  - role: fact-checker
    status: consulted
    reason: >-
      Bounded primary-source fact-check dispatched by the governor
      (`20260803T070701Z-governor-reviewer-m31s.md`), completed
      (`20260803T073119Z-fact-checker-aman.md`) at exact head `a78dd266`:
      BLOCK pending four corrections (F-REG-1 through F-REG-4), all
      addressed in this revision. F-REG-1: AI Act high-risk dates are now
      enacted law (Regulation (EU) 2026/1744), not provisional. F-REG-2:
      NIS2 Art. 21 does not itself state a contractual flow-down duty.
      F-REG-3: PLD Art. 9 is a court-ordered, bounded disclosure
      mechanism, not a standing mandatory-disclosure obligation.
      F-REG-4: the MR !962 dependency was stale (!962 is merged).
      Renewed fact-check (`20260803T094116Z-fact-checker-mghg.md`) at
      exact head `4eb464ae`: F-REG-1 and F-REG-4 fully resolved;
      F-REG-2/F-REG-3's headline replacement paragraphs were correct but
      two residual locations still carried the original overclaim —
      `docs/sources.md`'s NIS2 note ("flowed down to suppliers by
      contract", F-REG-2-R2) and both `docs/sources.md`'s PLD note and
      the plan's own AI-4 applicability paragraph ("mandatory evidence
      disclosure" as a standing obligation, F-REG-3-R2). Both corrected
      in this revision — see AI-3/AI-4 sections and `docs/sources.md`
      lines ~260, ~277. A bounded renewed fact-check of this revision is
      requested before Gate 1 approval.
---

# Plan: AI regulation & AI-governance-requirement gap analysis (living document)

**Date:** 2026-08-02T20:56:16Z (drafted); revised 2026-08-02T22:11:25Z per
contribution-auditor review `ph5j`; revised again 2026-08-02T22:33:42Z per
contribution-auditor review `b72h` (finding B-REG-1-R2); revised again
2026-08-03T08:00:00Z per fact-checker review `aman` (F-REG-1 through
F-REG-4).
**Status:** PENDING GATE 1 APPROVAL — contribution-auditor chain is
CLEAR and infosec-sentinel is mechanically CLEAR; this revision addresses
the fact-checker's four factual corrections. A bounded renewed
fact-checker pass is requested before Gate 1 approval.

## Why this document exists

`docs/sources.md` (source registry, batches 1–9 as of this revision —
Batch 9 merged via MR !962, corrected below per finding F-REG-4) and
`docs/radar.md` (active watchlist) name the
regulations and standards this document analyzes. **Source *discovery* is
substantially done for the items listed below; source *verification*
(exact dates, article/clause numbers, current applicability) is explicitly
**not** complete** — several rows below carry an inherited "[dates: verify
before citing]" flag from the registry, and this document does not resolve
those flags itself. That verification is fact-checker's separately declared
scope (see `advisory_consultation` above), not asserted as done here.

**Scope of coverage, stated precisely (per contribution-auditor finding
B-REG-4):** this document covers the regulation/standard/framework-typed
entries in `docs/sources.md` that name a concrete legal, regulatory, or
formal-standards obligation with plausible bearing on AI-assisted software
governance. It does **not** claim to cover every entry in the registry —
tooling/competitor/practice-typed entries (the majority of the registry) are
out of scope by construction, this document is not a full registry
crosswalk. One known omission from this revision: `docs/sources.md`'s
Finland AI Act decentralised-supervision entry (`valtioneuvosto.fi`, lines
~263-266) is jurisdiction/enforcement-authority context for AI-1 rather than
an independent requirement, and is now folded into AI-1's detail section
below rather than left unaddressed.

What this document answers, per requirement: what exactly does it require,
does it apply to AGF-the-project or to AGF's adopters (or both), what does
the current AGF + `agov` combination actually do about it today, and —
stated plainly, not softened — what it does **not** yet do.

This document is a **gap analysis**, not a compliance claim. Per
`agf/rules/docs/implementation-honesty-rule.md`, every row below carries
maturity classification on **two separate axes** (see Legend) rather than
one blended number — collapsing "a generic AGF capability exists" and "a
requirement-specific interface to that capability exists" into a single
maturity level was this document's original, and contribution-auditor
correctly identified, structural flaw (finding B-REG-1). Per
`always/verifiable-claims-rule.md`, every regulatory claim below is either
cited to a specific source in `docs/sources.md` or hedged, and every
capability-to-requirement mapping below carries an explicit evidence-status
label (finding B-REG-5).

**This document does not itself constitute legal advice, a compliance
determination, or a certification.** Every "meets" or "gap" statement below
is an engineering self-assessment, to be verified by a human governor, legal
counsel, or auditor before anyone relies on it. This is the same
artifact-vs-determination boundary `docs/plans/
compliance-export-buyer-side-agf.md` already draws for the (not yet built)
evidence-export tooling — this document draws it for the underlying
requirements themselves.

## How to update this document

This file is meant to be edited by many contributors over time, not
authored once and frozen:

- **Editing a row's status, gap, or "last reviewed" date** — direct edit,
  fast-track eligible per `agf/rules/workflow/simplicity-rule.md`
  ("single-file... change with obvious cause"). State in the commit
  message what changed and why (a source date moved, a capability
  shipped, a regulation's applicability was clarified).
- **Adding a newly discovered regulation or standard** — add the source
  to `docs/sources.md` first (with its `type`/`relation`/`action`
  classification), then add a row here referencing it. Do not add a row
  here for a source that is not also in the registry — this document is
  a derived view over that registry, not a parallel one (marketing-
  tone-rule's message-integrity principle: state a claim once, in its
  canonical sourcing location).
- **Changing a maturity-level assignment** — requires pointing at the
  artifact that justifies the new level (a shipped command, a merged
  validator, a written schema doc), per implementation-honesty-rule.
  "We started working on it" does not move a Level 1 to a Level 2. A
  source-registry note, an issue number, or a named-but-unbuilt adapter
  is, by itself, **Mapping Level 1** — see Legend.
- **Marking a gap closed** — requires a Gate-2-merged artifact
  reference (MR number, file path), not just a plan or intent.
- **Relying on an unmerged branch/MR's content** — must name that
  dependency explicitly in the citing row (per finding B-REG-4); never
  cite unmerged content as if it were already part of the registry.
- **Structural changes to this document itself** (adding a new section,
  changing the requirement-ID scheme, changing the two-axis model) — go
  through Gate 1 like any other plan-file change, since those decisions
  affect how every future contributor uses the file.

Review cadence: **quarterly, or at every minor release, whichever comes
first** — same cadence as `docs/radar.md`, since most rows here trace
back to a radar item. Next review: 2026-11.

## Legend

**Two maturity axes, one canonical ladder** (correction, finding
B-REG-1-R2: an earlier revision of this document defined Level 4 and
Level 2 in terms of mapping-existence, which silently re-coupled the two
axes this document exists to separate. The ladder below is generic and
identical for both axes, matching `agf/docs/site/reference/
maturity-vocabulary.md`'s canonical Level 4/5 distinction — "Level 4
Validated means implemented and tested in a branch or checkout; Level 5
Deployable means the supported operator path or release artifact
includes it." Each axis rates a *different named artifact* against this
same ladder; neither axis's score is adjusted because of the other's.
Second correction, finding B-REG-1-R3: the Level 5 row below previously
added a "this repository's own operating practice" alternative not
present in the canonical definition, which could promote unreleased,
self-used-only work. That alternative is removed; Level 5 requires the
named release or supported operator path, named or verified per row):

| Level | Name | What exists |
|---|---|---|
| 5 | Deployable | The named artifact (for Capability: the generic AGF/`agov` capability itself; for Mapping: the requirement-specific interface itself) is shipped, tested, documented, and present in the supported distribution or release artifact (e.g. the `agf-{version}.tar.gz`/`.zip` archive per `agf/dist/release-manifest.yaml`, or the `agov`/`agov-mcp` release binaries) |
| 4 | Validated | Implemented and tested in a branch or checkout; not yet in the supported distribution / not yet fully operating |
| 3 | Instantiated | Partial or ad hoc — real code/records exist but are not systematic or comprehensive |
| 2 | Surface-defined | An actual interface has been declared — a written schema, a design doc, a scoped plan — but no working code |
| 1 | Planned | Named in `docs/sources.md`/`docs/radar.md` only; OR the only "support" is a source-registry note, an issue number, or a named-but-unbuilt adapter mentioned in passing — these do **not** count as Level 2 |
| — | Out of scope by design | Not a gap to close — a declared boundary (Layer 4 runtime behaviour; see "Structural non-goals" below) |

- **Capability** — how mature the underlying, generic AGF/`agov` capability
  is, rated **solely on that capability's own lifecycle** (e.g. "the
  append-only coordination trail" as a general artifact: is it shipped,
  tested, and actually operating in this project today?). **The absence
  of a regulation-specific mapping must never lower this score** — that
  absence belongs exclusively on the Mapping axis. A capability earns
  Level 5 only when the named artifact (a rule file, schema, or binary)
  is actually present in a shipped release archive or release binary —
  named or verified per row below. This project's own use of that
  capability (Foundry self-application, e.g. `docs/coordination/*.md`
  as a running instance of the coordination-trail tooling) may
  *demonstrate* the capability is operating correctly, but self-use
  alone is not a substitute for the distribution boundary and does not
  by itself earn Level 5.
- **Mapping** — how mature the *requirement-specific* interface is: a
  document, schema, or command whose actual subject is this exact
  regulation/standard, not just a capability that could plausibly serve it.
  **Conservative rule:** Mapping is capped at Level 1 unless a real,
  requirement-specific artifact exists — a passing mention in a source note
  does not raise it, and the underlying capability's own maturity does not
  raise it either.

**Evidence status** (new field, finding B-REG-5) — separate from maturity;
answers "how validated is the claimed relationship between capability and
requirement," not "does the capability exist":

- **Theoretical** — an engineering judgment connecting capability to
  requirement, not independently validated. Default for every row below
  unless stated otherwise.
- **Empirically informed** — named foundry or adjacent-domain evidence
  supports the specific claim (a real incident, a real audit, a real usage
  pattern), short of independent replication.
- **Empirically validated** — the claim has been independently confirmed
  across multiple independent adopting projects. **Not used anywhere in
  this document** — per `agf/learnings/foundry.yaml`'s customer-state
  field, AGF has no confirmed external paying/production adopters as of
  this revision, so no row can honestly claim this status yet.

**Applies-to tags:**

- **AGF-the-project** — obligations that could fall on this repository /
  its maintainer(s) directly (e.g. as an open-source producer or CRA
  steward).
- **AGF-the-process** — obligations about *how software gets built*,
  which AGF's Gate 1/Gate 2/coordination-trail model addresses
  structurally, regardless of who runs it.
- **AGF-adopters** — obligations that fall on organizations that adopt
  AGF to govern *their own* AI-assisted development, where AGF's job is
  to produce the evidence *they* need — not to be compliant on their
  behalf.

A single requirement can carry more than one tag; where it does, the
capability and gap may differ per tag and both are stated.

---

## Summary table

| ID | Source | Applies to | Capability | Mapping | Evidence | Gap headline |
|---|---|---|---|---|---|---|
| AI-1 | EU AI Act Art. 26 (deployer duties) | AGF-adopters | 5 (Deployable) | 1 (Planned) | Theoretical | Coverage is analogy, not a validated mapping; "duty to suspend" has no enforced mechanism |
| AI-2 | EU CRA Art. 24 (open-source steward) | AGF-the-project | 1 (vuln-reporting channel stale; policy text separately Level 5) | 1 (Planned) | Theoretical | Entity-structure decision unresolved (issue #13); no current v2 vulnerability-reporting channel |
| AI-3 | NIS2 Art. 21 + Finland Kyberturvallisuuslaki 124/2025 | AGF-adopters | 5 (Deployable) | 1 (Planned) | Theoretical | No NIS2-specific supply-chain evidence mapping exists or is roadmapped |
| AI-4 | EU Product Liability Directive (EU) 2024/2853 | AGF-adopters | 5 (Deployable) | 1 (Planned) | Theoretical | No liability-defense framing written; AGF itself likely outside scope (non-commercial OSS) |
| AI-5 | GDPR Art. 17 vs. append-only coordination trail | AGF-the-project, AGF-the-process | 1 (Planned) | 1 (Planned) | Theoretical | **Unresolved active tension**, not just an unmet requirement — blocks issue #8's exit-guarantee wording |
| AI-6 | EU MCC-AI (procurement contractual clauses) | AGF-adopters | 2 (Surface-defined) | 2 (Surface-defined) | Theoretical | `agov compliance export`/`verify` (#40/#41) schema-designed, not built |
| AI-7 | ISO/IEC 42001 (AI management systems) | AGF-adopters | 5 (Deployable) | 1 (Planned) | Theoretical | No clause-mapped evidence pack; explicitly deferred past MCC-AI first pass |
| AI-8 | NIST AI RMF (Govern/Map/Measure/Manage) | AGF-the-process, AGF-adopters | 5 (Govern, Manage); 3 (Map); — (Measure, out of scope) | 1 (Planned) | Theoretical | No formal crosswalk; "Measure" function structurally out of AGF's scope |
| AI-9 | NIST SP 800-218A (SSDF GenAI profile) | AGF-the-process | 5 (Deployable) | 1 (Planned) | Theoretical | No practice-ID-mapped self-assessment; rule-title-to-practice inference is not a crosswalk |
| AI-10 | SLSA / in-toto attestations | AGF-the-project | 5 (Deployable) | 1 (Planned) | Theoretical | No signed/verifiable attestations; no SLSA level self-assessed for `agov` release builds |
| AI-11 | CycloneDX ML-BOM / Agent-BOM (open proposal) | AGF-the-project, AGF-adopters | 5 (Deployable, narrow scope) | 1 (Planned) | Theoretical | Format itself unsettled upstream; explicitly watch-only |
| AI-12 | W3C PROV-DM | AGF-the-project | 5 (Deployable) | 1 (Planned) | Theoretical | Coordination trail not modeled in PROV entity/activity/agent terms |
| AI-13 | OWASP LLM Top 10 / AI Exchange + declared threat model | AGF-the-project | 2 (Surface-defined) | 2 (Surface-defined) | Theoretical | A Gate-1-pending threat-model plan exists (`threat-model-stpa-owasp.md`) but is not approved/shipped |
| AI-14 | OpenSSF Scorecard / Best Practices Badge | AGF-the-project | 1 (Planned) | 1 (Planned) | Theoretical | Not yet run/applied for; low cost, not yet done |
| AI-15a | IETF WIMSE (active WG) | AGF-the-project | 1 (Planned) | 1 (Planned) | Theoretical | Intentionally deferred — active WG, but agent identity today is a plain string |
| AI-15b | IETF AIMS (presentation/proposal, not a WG) | AGF-the-project | 1 (Planned) | 1 (Planned) | Theoretical | Distinct, less mature than WIMSE — not comparable in standing; see AI-15 detail |

---

## A. Binding or near-binding law

### AI-1 — EU AI Act Art. 26 (deployer duties)

**Source:** `docs/sources.md` line ~231, artificialintelligenceact.eu/article/26/.
**What it requires (corrected, fact-checker finding F-REG-1,
`20260803T073119Z-fact-checker-aman.md`):** named-human oversight of a
high-risk AI system; ongoing monitoring with a duty to suspend use if
serious risk is detected; auto-generated logs retained ≥ 6 months
(Art. 26(6)). The high-risk timeline is **enacted law, not provisional**:
Regulation (EU) 2026/1744 was published in the Official Journal on 24
July 2026 and entered into force in July 2026, setting **2 December
2027** for Annex III/stand-alone high-risk systems and **2 August 2028**
for systems under Article 6(1)/Annex I. The earlier "Omnibus VII agreed
but not yet law" framing is stale and is corrected here, not merely
softened. Primary source:
https://eur-lex.europa.eu/legal-content/EN/ALL/?uri=CELEX:32026R1744 .
Article 26's substantive summary otherwise checks out against Regulation
(EU) 2024/1689 itself: deployers assign competent natural persons to
oversight, monitor operation, suspend use where the system may present
Article 79 risk, and keep automatically generated logs under their
control for at least six months, subject to other law.

**Jurisdiction/enforcement context (folded in per contribution-auditor
finding B-REG-4):** `docs/sources.md`'s `valtioneuvosto.fi` entry
(lines ~263-266) describes Finland's decentralised AI Act supervision —
sectoral authorities, a Traficom single point of contact and sandbox,
powers from 2026-01-01 (HE 46/2025). This is enforcement-authority
context, not an independent requirement distinct from Art. 26 itself; it
does not get its own row.

**Applicability caveat:** Art. 26 governs *deployers of high-risk AI
systems* — a runtime obligation on whoever operates an AI system serving
end-users or decisions. AGF governs *how a codebase gets built* (a
development-process framework), not a deployed AI system's runtime
behavior. Whether Art. 26 applies to "an AGF-governed AI coding agent"
at all is not established here and should not be assumed — this row
exists because the *pattern* AGF already implements (named human
oversight, logged decisions) is a close structural match to what an
adopter who *does* operate an in-scope high-risk system would need to
demonstrate, not because AGF has confirmed its own applicability.

**Current AGF+`agov` capability (Capability: 5 — Deployable; Mapping: 1 —
Planned; Evidence: Theoretical):**
- Named human oversight → the governor-reviewer role and Gate 1/Gate 2
  approval sequence is defined by rule files
  (`agf/rules/workflow/development-workflow-rule.md`,
  `always/high-risk-operations-rule.md`) that ship in the release
  archive (`agf/dist/release-manifest.yaml`'s `agf/rules/**` glob), and
  is additionally exercised on this project's own history — genuinely
  Level 5 as a *generic capability*, rated solely on its own lifecycle
  (per B-REG-1-R2's correction, the absence of an Art.-26-specific
  mapping does not lower this score). Calling this "exactly" Art. 26's
  named-human-
  oversight process (as an earlier revision of this document did) is a
  mapping claim this document cannot make: Art. 26's oversight process
  has specific content requirements (competence, authority, instructions
  to the deployer) that a code-review gate has not been checked against.
  The **mapping** — a document establishing that Gate 1/Gate 2
  specifically satisfies Art. 26's oversight-process requirements — does
  not exist. Mapping Level 1.
- Logged decisions ≥ 6 months → `docs/coordination/*.md` is append-only
  (`coordination-publication-rule.md`) and lives in git history, which
  persists indefinitely by default. This is **not** the same claim as
  "satisfies Art. 26(6)'s logging floor": Art. 26 describes *automatically
  generated* logs of the *deployed system's operation*, under the
  deployer's retention control; AGF's coordination trail evidences
  *development decisions*, generated by human/agent action, not runtime
  telemetry. Indefinite git persistence is a fact about this repository's
  history, not a declared, lawful *retention policy* an adopter could
  point to. Both distinctions matter and neither is resolved here.
- Monitoring with duty to suspend → **no enforced mechanism.** This is
  Layer 4 (runtime behaviour) territory, explicitly out of scope for
  every AGF compliance tier per `agf/docs/adopt/compliance-profile.md`.
  AGF can show that a human *approved* a change; it has no kill-switch
  or automated-suspend capability for an AI agent already acting.

**Gap:** (a) no Art. 26-specific mapping document exists connecting Gate
1/Gate 2 to Art. 26's actual oversight-process requirements — what exists
today is a structural analogy, not a validated mapping; (b) no evidence
export exists — an adopter today would assemble evidence manually from
raw coordination files, and that evidence is development-process
evidence, not deployed-system logs; (c) the "duty to suspend" clause has
no AGF-side answer at all, by design — adopters needing that must compose
AGF with a runtime-enforcement layer (the same boundary the framework
already states for Layer 4 generally). An adopter-specific mapping must
identify the actual deployed-system logs, the actual oversight
instructions/competence/authority documentation, and the actual retention
controls before any coverage level above Theoretical can be claimed.

---

### AI-2 — EU Cyber Resilience Act (CRA), Art. 24 "open-source steward"

**Source:** `docs/sources.md` line ~244, digital-strategy.ec.europa.eu/en/policies/cra-open-source; `docs/plans/cra-entity-structure-decision.md`; `docs/radar.md` "CRA steward structure."

**What it requires:** an open-source steward must maintain a
cybersecurity policy and a vulnerability-reporting/handling process. No
fines attach to the steward role itself. Reporting duties begin
2026-09-11 per the registry's cited date; full applicability 2027-12-11.
**Critical gate:** Art. 3(14) requires a *legal person* — a solo
natural-person maintainer cannot be a CRA steward at all, so the
question of whether this row applies to AGF-the-project is contingent
on an entity-structure decision that is explicitly unresolved (radar.md:
"decision pending (issue #13)").

**Current AGF+`agov` capability (Capability: 1 as the vulnerability-
reporting channel — the operationally missing half; Mapping: 1; Evidence:
Theoretical):**
- Cybersecurity policy → `agf/rules/always/security-hardening-rule.md`
  is real, shipped, applied normative guidance — genuinely **Level 5
  Deployable as a shipped rule artifact.** That is a claim about the
  artifact's existence and application, not about CRA conformance: the
  rule contains generic adopter placeholders and does not itself
  constitute, and must not be equated with, a CRA-shaped public
  cybersecurity policy document. Several of its items are agent
  instructions rather than mechanically configured checks (e.g. no
  automated security-test command is wired to it). The rule's
  Capability=5 as guidance; its Mapping to "CRA Art. 24 cybersecurity
  policy" is Level 1 — no CRA-shaped document exists.
- Vulnerability-reporting process → **stale.** A `SECURITY.md` exists
  at `agents-govern/SECURITY.md`, but that file describes the
  *predecessor* v1.x project ("does not contain executable code that
  runs in production environments except a bash validation script") —
  no longer accurate for AGF-NG v2, which ships a real Rust `agov`
  binary, an MCP server, and CI tooling. **No current `SECURITY.md`
  exists for the present project at the repository root or under
  `agf/`.** This is the row's binding capability number: whatever the
  policy-text artifact's own maturity, there is currently no live,
  accurate vulnerability-reporting artifact at all.

**Gap:** two independent blockers, not one — (1) the entity-structure
decision (`docs/plans/cra-entity-structure-decision.md`) must resolve
before AGF-the-project's CRA applicability is even determined; (2)
regardless of that decision, the vulnerability-reporting-process
artifact is out of date for what the project actually ships today and
should be refreshed on its own merits (this is a security-hygiene gap
independent of CRA).

---

### AI-3 — NIS2 Directive Art. 21 + Finland Kyberturvallisuuslaki 124/2025

**Source:** `docs/sources.md` lines ~252, ~258; digital-strategy.ec.europa.eu, traficom.fi.

**What it requires (corrected, fact-checker finding F-REG-2,
`20260803T073119Z-fact-checker-aman.md`):** Art. 21(2)(d) requires
in-scope entities to manage supply-chain cybersecurity risk, including
security aspects of their relationships with direct suppliers/service
providers; Art. 21(3) requires them to take into account those
suppliers' vulnerabilities and practices. **Art. 21 does not itself
state a contractual flow-down duty** — the earlier framing ("flowing
security and compliance-documentation requirements down to suppliers by
contract") overstated the Article's actual text; contractual flow-down
is a reasonable *implementation measure* an entity might choose, not the
quoted requirement itself, and is now labelled as such rather than
presented as Art. 21's content. Primary source:
https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32022L2555 .
Finland's Kyberturvallisuuslaki 124/2025 (in force 2025-04-08) is the
national transposition; Traficom/NCSC-FI act as single point of contact
and CSIRT — this part of the citation was already accurate and is
unchanged.

**Applicability caveat:** AGF-the-project is not itself an "essential or
important entity" under NIS2 — it is a development-process framework,
not critical infrastructure. This row's relevance is entirely
**AGF-adopters**: an in-scope NIS2 entity that adopts AGF can point
in-scope suppliers' AGF-produced coordination trails at its own Art. 21
supplier-documentation obligation.

**Current AGF+`agov` capability (Capability: 5 — Deployable; Mapping: 1 —
Planned; Evidence: Theoretical):** same coordination-trail tooling as
AI-1 — the schema (`agf/schemas/coordination-message.schema.json`), the
`agov coord` command surface, and the governing rule
(`agf/rules/workflow/coordination-publication-rule.md`) all ship in the
release archive; this project's own trail additionally demonstrates it
operating. Rated on its own lifecycle regardless of NIS2 mapping status,
per B-REG-1-R2. No NIS2-specific
mapping/export exists (`docs/plans/compliance-export-buyer-side-agf.md`
names ISO 42001/Art. 26/SSDF as its own explicit follow-on scope and does
not mention NIS2 at all — this is a genuinely open gap even in the
roadmap, not merely unimplemented).

**Gap:** no NIS2-specific evidence mapping exists or is currently
planned in any Gate-1 document. Add as follow-on scope to the
compliance-export cluster, or a new issue, when prioritized.

---

### AI-4 — EU Product Liability Directive (EU) 2024/2853

**Source:** `docs/sources.md` line ~268, freshfields.com summary.

**What it requires (corrected, fact-checker finding F-REG-3,
`20260803T073119Z-fact-checker-aman.md`):** treats software/AI as
products for strict-liability purposes and defines rebuttable
presumptions of defectiveness (Article 10). **Article 9 does not impose
a standing "mandatory evidence disclosure" obligation on every
commercial software supplier** — the earlier framing overgeneralized it.
A national court *may* order a defendant to disclose relevant evidence,
but only after a claimant presents facts and evidence supporting the
plausibility of the claim; the order is limited to what is necessary and
proportionate, and protects confidential information. AGF's own
argument — that a process trail could help satisfy such a court-ordered
disclosure — remains explicitly Theoretical and may stand, but must not
be presented as something the Directive automatically requires of every
supplier. Primary source:
https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32024L2853 .
Transposition due by 2026-12-09; applies to products placed on the
market or put into service after 2026-12-08; Article 2(2) excludes
free/open-source software developed or supplied outside commercial
activity — these facts were already accurate and are unchanged.

**Applicability caveat:** AGF-the-project is currently non-commercial
open-source software (no evidence in this repo of monetization) —
**on the registry's own reading, AGF itself is likely outside this
directive's scope today.** This row's practical value is as a
**demand-driver for AGF-adopters**: an organization shipping a
commercial AI-containing product could point to its own AGF-produced
process trail if a national court orders relevant, necessary, and
proportionate disclosure under Art. 9 in litigation following a
plausible claim — not as a standing disclosure obligation the trail
pre-emptively satisfies — and as a rebuttal aid against the
defectiveness presumption (evidence of a disciplined development/review
process). [corrected 2026-08-03, F-REG-3-R2]

**Current AGF+`agov` capability (Capability: 5 — Deployable, the generic
coordination trail; Mapping: 1 — Planned; Evidence: Theoretical):** the
same shipped coordination-trail tooling as AI-1/AI-3 (schema, `agov
coord` commands, governing rule, all present in the release archive)
is the raw material; **no framing document
exists** that presents it explicitly as "litigation-evidence" or maps it
to PLD's disclosure categories — nothing above a source-registry citation
supports any Mapping level higher than 1, per this document's own
conservative rule.

**Gap:** entirely unbuilt — no plan, no issue filed specifically for
this framing (distinct from the general compliance-export cluster,
which targets MCC-AI/42001/SSDF, not PLD). This is the lowest-mapping-
maturity row among the binding-law group precisely because no prior work
(radar, plan, or issue) has picked it up yet — flagging it here is this
document's first concrete step toward that.

---

### AI-5 — GDPR Art. 17 (right to erasure) vs. append-only coordination trail

**Source:** `docs/sources.md` line ~275, hoop.dev/blog/why-gdpr-compliance-lives-in-your-git-history (secondary source). A primary-source entry
(official EUR-Lex text, Regulation (EU) 2016/679, CELEX 32016R0679) was
added in Batch 9. **Corrected (fact-checker finding F-REG-4,
`20260803T073119Z-fact-checker-aman.md`): MR !962 is merged on `main`**,
not unmerged as an earlier revision of this document stated — the
primary-source citation is live in the registry today, not
forward-dependent. `docs/radar.md` "GDPR erasure vs append-only trail."

**What it requires:** GDPR Art. 17 gives data subjects a right to
erasure of personal data, subject to exceptions. A git-stored audit
trail recording named human authors/approvers is personal-data
processing; an append-only design (which `coordination-publication-
rule.md` and the sibling immutability principle for tags both actively
enforce) structurally resists erasure by construction.

**This row is not a "gap" in the ordinary sense — it is an unresolved
internal design tension**, already flagged in `docs/radar.md` as
blocking issue #8's exit-guarantee wording ("must resolve").

**Current AGF+`agov` capability (Capability: 1 — Planned; Mapping: 1 —
Planned; Evidence: Theoretical):** none yet. No lawful-basis statement
for processing contributor names/emails in the coordination trail, no
identity-minimisation design, no documented erasure story (e.g.,
pseudonymous contributor handles with a separate, erasable identity-
mapping table; a stated legitimate-interest basis; a policy that only
the human governor's own repository, not AGF's design, controls
retention). Both axes are Level 1 because nothing beyond problem
identification exists yet.

**Gap:** genuinely open, and higher priority than most rows here because
it is *actively blocking* other work (issue #8) rather than sitting
passively unaddressed. Per the "how to update" section above, this is
exactly the kind of row where infosec-sentinel input is needed before
any resolution lands — flagged again under "Before Gate 1 approval."

---

## B. Procurement / contractual requirement

### AI-6 — EU Model Contractual Clauses for AI Procurement (MCC-AI)

**Source:** `docs/sources.md` line ~237; `docs/plans/compliance-export-buyer-side-agf.md` (issues #40/#41, status: pending-gate1).

**What it requires:** public buyers get contractual rights to technical
documentation, logs, record-keeping, and audits from AI vendors (updated
2025-03-05).

**Current AGF+`agov` capability (Capability: 2 — Surface-defined; Mapping:
2 — Surface-defined; Evidence: Theoretical):** this is the best-planned
row in this document — `docs/plans/compliance-export-buyer-side-agf.md`
already defines the artifact-to-evidence-category mapping, the shared
evidence-pack JSON schema, the non-claim disclaimer, and both the export
(`agov compliance export --target mcc-ai`) and verify (`agov compliance
verify`) command surfaces, scoped to MCC-AI first. Capability and Mapping
are the same number here because, unlike the other rows, this plan *is*
already requirement-specific design work, not a generic capability
retrofitted onto the requirement. **None of it is implemented yet** —
that plan is itself still `pending-gate1`.

**Gap:** implementation, not design. The schema exists as a design
document; no code exists. This is the single highest-leverage row to
close given how much of the rest of this document's "no mapping exists"
gaps (AI-1, AI-3, partially AI-7) would be answered by the same
underlying tooling once MCC-AI ships and the other adapters follow —
though each of those adapters would still start at Mapping Level 1 until
their own requirement-specific work is actually done, per this
document's conservative rule.

---

## C. Management-system / voluntary standards used as compliance baselines

### AI-7 — ISO/IEC 42001 (AI management systems)

**Source:** `docs/sources.md` lines ~284, ~360.

**What it requires:** establish, implement, maintain, and continually
improve an AI management system (AIMS) — a certification-oriented
standard describing a PDCA-style management-system structure (policy,
risk assessment, resourcing, monitoring, continual improvement), not a
line-by-line technical checklist.

**Current AGF+`agov` capability (Capability: 5 — Deployable; Mapping: 1 —
Planned; Evidence: Theoretical):** the Gate 1/Gate 2 pipeline plus the
learnings loop (governed work → evidence → learning record → analysis →
proposal → approval → implementation, closing back to observed use) is
defined by `agf/docs/site/docs/concepts/evidence-and-learnings.md` and
`agf/docs/governance/learning-system.md`, with the loop pattern also
stated in `agf/framework.md`, and the record surface supplied by
`agf/schemas/learning.schema.json` and `agf/templates/learning-record.yaml`
(correction, finding B-REG-1-R4: an earlier revision cited unspecified
`agf/rules/workflow/**` rule files as defining this loop; no rule file
actually does — some rules ask adopters to file learning records, which
is not the same as defining the loop itself. **Further correction,
finding B-REG-1-R5:** the B-REG-1-R4 fix itself cited a nonexistent
`agf/learnings/_template.yaml` — the similarly-named
`agents-govern/learnings/_template.yaml` is bootstrap-era v1.x reference
material, not part of the AGF v2 product; the actual adopter-facing
template is `agf/templates/learning-record.yaml`, confirmed present by
direct `ls`.). All of these are shipped
in the release archive per `agf/dist/release-manifest.yaml`'s
`agf/docs/site/**`, `agf/docs/governance/**`, `agf/framework.md`,
`agf/schemas/**`, and `agf/templates/**` entries. `agf/learnings/*.yaml` is this project's own
(unshipped, adopter-populated-in-practice) instance of that loop's
schema. The shipped documentation/schema/template's presence, not the
specific learning records in this repository, is what grounds
Capability 5. Structurally a
continual-improvement management-system loop. No formal clause-by-clause mapping exists: 42001 is named as
*out of scope* for the first pass of the compliance-export plan (AI-6),
a deliberate, documented deferral, not an oversight, but a deferred
adapter is Mapping Level 1, not Level 3 as an earlier revision of this
document claimed.

**Gap:** the loop exists and runs (Capability 5), but nothing maps it
to 42001's clause structure (Mapping 1), and the standard text itself is
paywalled (per `compliance-export-buyer-side-agf.md`'s own citation
caveat: "cite only the publicly documented clause structure... until a
primary copy is reviewed"). No certification is claimed or implied.

### AI-8 — NIST AI Risk Management Framework (Govern / Map / Measure / Manage)

**Source:** `docs/sources.md` line ~294.

**What it requires:** a voluntary four-function framework (Govern, Map,
Measure, Manage) widely referenced in RFPs and emerging regulation as a
baseline governance vocabulary.

**Current AGF+`agov` capability, function by function (per-function
nuance retained; each function's Capability is rated solely on that
function's own underlying artifact, per B-REG-1-R2 — RMF-mapping
absence never lowers it):**
- **Govern** — strong structural match: roles, rules, gates
  (`agf/rules/`, `agf/agents/`) are defined by files that ship in the
  release archive (`agf/dist/release-manifest.yaml`'s `agf/rules/**`
  and `agf/agents/**` globs), and are additionally exercised on this
  project's own history — Capability 5 for this function specifically.
- **Map** (context and risk identification) — individual plans name
  risks ad hoc (see the "Risks" tables in `docs/plans/*.md`), but no
  consolidated, standing risk register exists as its own artifact —
  Capability 3 (Instantiated): this is a fact about what exists, not
  about RMF mapping — a systematic risk register genuinely does not
  exist yet, regardless of any framework it might eventually map to.
- **Measure** (metrics on AI-system trustworthiness/performance) —
  **structurally out of scope.** AGF does not measure a deployed AI
  system's runtime performance or trustworthiness; that is Layer 4
  territory by the framework's own declared boundary. This is not a
  todo item — it is a permanent, honest exclusion that should be stated
  explicitly wherever this framework is compared against the RMF, not
  silently omitted.
- **Manage** (risk response) — the learnings-record corrective-loop
  *mechanism* is the same shipped artifact cited for AI-7 (correction,
  finding B-REG-1-R4: `agf/docs/site/docs/concepts/evidence-and-learnings.md`,
  `agf/docs/governance/learning-system.md`, `agf/framework.md`'s loop
  pattern, and `agf/schemas/learning.schema.json` — not unspecified
  `agf/rules/workflow/**` rule files, which do not define this loop) —
  Capability 5. `agf/learnings/*.yaml` itself is this project's own
  unshipped instance of that loop and does not by itself ground the
  score. That the mechanism is not framed in RMF vocabulary is a Mapping
  fact, not a Capability one.

**Gap:** no formal RMF crosswalk document exists at all (Mapping 1 across
every function, including Govern and Manage despite their Capability 5).
Given the Measure gap is structural rather than a build item, a future
crosswalk should say so explicitly rather than implying partial coverage
will eventually reach full coverage.

### AI-9 — NIST SP 800-218A (SSDF GenAI profile)

**Source:** `docs/sources.md` line ~289.

**What it requires:** a GenAI-specific profile of the Secure Software
Development Framework, addressing producer, integrator, and acquirer
roles explicitly.

**Current AGF+`agov` capability (producer role; Capability: 5 —
Deployable; Mapping: 1 — Planned; Evidence: Theoretical):**
`agf/rules/workflow/tdd-rule.md`, `always/security-hardening-rule.md`,
`workflow/systematic-code-review-rule.md`, `workflow/
migration-generation-rule.md`, and `workflow/boundary-shape-testing-
rule.md` are real, shipped **active normative rules** (they ship in the
release archive and are the stated instructions an agent must follow)
covering secure-development-adjacent practice areas for a producer.
This is a claim about the rules' existence and normative status, not
about uniform mechanical enforcement: as AI-2 already notes for
`security-hardening-rule.md`, several items in this rule set are agent
instructions rather than mechanically configured checks — e.g. no
project-agnostic, automated security-test command is wired to the
generic `security-hardening-rule.md` for every adopter (CI-level
mechanical enforcement, where it exists, is evidenced per-rule, not
assumed uniformly here). **Acquirer role:** answered by the
not-yet-built buyer-side verification tooling (AI-6/#41).

**Gap:** no self-assessment against named SSDF practice IDs (PO, PS,
PW, RV groups) has been written. The coverage claim above is this
document's author reading rule titles against practice descriptions —
an inference, not a formal crosswalk, and therefore Mapping Level 1, not
a level implying systematic coverage.

---

## D. Supply-chain / provenance standards

### AI-10 — SLSA / in-toto attestations

**Source:** `docs/sources.md` line ~299.

**What it requires:** cryptographically-verifiable attestations
describing what process produced a software artifact and how (in-toto
predicate types; SLSA maturity levels for build integrity).

**Current AGF+`agov` capability (Capability: 5 — Deployable; Mapping:
1 — Planned; Evidence: Theoretical):** `agov`'s own release-build
tooling (`tooling/rust/crates/agov/src/dist.rs`, exercised via the
shipped `agov dist build`/`agov dist validate` commands and this
project's own release pipeline) produces checksums and provenance
records for distribution archives — real, shipped, tested, and in active
use in the supported distribution, hence Capability 5 (correction,
B-REG-1-R2: an earlier revision rated this Capability 3 *because* SLSA/
in-toto alignment was absent — that absence is a Mapping fact, not a
reason to lower the underlying checksum/provenance tooling's own
lifecycle score). Gate 2 approval records and the coordination trail are
separately, conceptually attestation-like (who approved what, when,
referencing which artifact) — also real and shipped. Neither is
schema-aligned to in-toto predicate types, neither is cryptographically
signed, and no SLSA level has been self-assessed.

**Gap:** `docs/sources.md`'s own note names the target action ("align
schema to in-toto predicate types instead of inventing a format") — that
note is a source-registry mention, not a declared interface, so Mapping
is Level 1.

### AI-11 — CycloneDX ML-BOM / open "Agent-BOM" proposal

**Source:** `docs/sources.md` line ~310; `docs/radar.md` "Agent-BOM /
input provenance."

**What it requires:** an SBOM-style inventory of ML components, or (per
the open GitHub issue this tracks) of agent tools/MCP servers and their
authorizations.

**Current AGF+`agov` capability (Capability: 5 — Deployable, narrow
scope; Mapping: 1 — Planned; Evidence: Theoretical):**
`agov-core::roi::MCP_TOOL_ALLOWLIST` (the synchronized MCP-tool allowlist
described in `agf/docs/site/docs/adopt/mcp-server.md`) is real, tested
code compiled into the `agov-mcp` binary, which ships alongside `agov`
in the per-platform release archive (`dist/agov-{version}-{platform}.tar.gz`/`.zip`,
built by this project's own tag pipeline) — it enumerates which MCP
tools are permitted. Its narrowness relative to a full Agent-BOM is a scope/fit
question for the Mapping axis (correction, B-REG-1-R2: an earlier
revision rated this Capability 3 because the allowlist is "narrower than
Agent-BOM" — that narrowness describes how well it maps onto this
specific requirement, not the allowlist's own lifecycle, which is fully
shipped). It is not shaped as, and does not export, a CycloneDX-
conformant Agent-BOM.

**Gap:** Mapping Level 1 by design — the upstream format itself is an
open, unsettled proposal (radar.md: "format unsettled... review-by
2027-01"). Correctly watch-only for now; building against an unsettled
upstream schema would be premature.

### AI-12 — W3C PROV-DM

**Source:** `docs/sources.md` line ~305.

**What it requires:** not a compliance obligation by itself, but the
mature semantic model (entities / activities / agents) that auditors
and downstream provenance tooling increasingly expect provenance claims
to be expressible in.

**Current AGF+`agov` capability (Capability: 5 — Deployable; Mapping: 1 —
Planned; Evidence: Theoretical):** the coordination trail (same
Capability-5 artifact as AI-1/AI-3/AI-4) captures who (agent role /
human), what (action, referenced artifact), and when (timestamp) —
semantically close to PROV's model — but the schema
(`agf/schemas/coordination-message.schema.json`)
does not use PROV's vocabulary or distinguish entities from activities
from agents explicitly.

**Gap:** `docs/sources.md`'s own note ("Describe the trail in PROV
terms") is a source-registry mention, not a declared interface — Mapping
Level 1, not Level 2 as an earlier revision of this document claimed.
Low cost to describe the existing trail *in* PROV terms in documentation
even before any schema change.

---

## E. Threat vocabulary / voluntary legibility signals

### AI-13 — OWASP LLM Top 10 / AI Exchange + declared threat model

**Source:** `docs/sources.md` line ~326; `docs/sources.md` concept #11
("declared threat model," tracked as issue #11);
`docs/plans/threat-model-stpa-owasp.md` (status: `pending-gate1`, id
`threat-model-stpa-owasp-42`).

**What it requires:** not a regulation — a widely-cited threat
vocabulary increasingly referenced *by* regulation and procurement
requirements (including, plausibly, future MCC-AI or Art. 26 risk-
assessment expectations) as the baseline taxonomy for LLM/agent threats.

**Correction (contribution-auditor finding B-REG-6):** an earlier
revision of this document said "no declared threat model exists yet at
all." That is inaccurate — `docs/plans/threat-model-stpa-owasp.md` is a
real, pending-Gate-1 plan proposing an STPA+OWASP-derived threat model
for AGF. The accurate claim is narrower: **no approved or shipped
threat model exists yet**; a scoped plan exists and has not cleared
Gate 1.

**Current AGF+`agov` capability (Capability: 2 — Surface-defined;
Mapping: 2 — Surface-defined; Evidence: Theoretical):**
`always/security-hardening-rule.md` and `always/high-risk-operations-
rule.md` cover overlapping concerns (SSRF guards, secrets, CSRF, tenant
isolation) but were not written *against* the OWASP LLM Top 10 taxonomy
specifically. `docs/plans/threat-model-stpa-owasp.md` is the actual,
requirement-specific artifact — a real design document whose subject is
exactly this gap — so both Capability and Mapping are Level 2, not
Level 1: a declared interface exists, even though it has not shipped or
been approved.

**Gap:** the threat-model plan is pending Gate 1 approval, not yet
approved or implemented; prompt-injection-style risks are not named
anywhere in currently-shipped rules. This gap blocks honest claims about
AI-1's and AI-9's risk-management coverage — both ultimately ask "did
you think about what could go wrong," and AGF cannot yet answer that in
a named, *shipped* vocabulary, even though a proposal is in flight.

### AI-14 — OpenSSF Scorecard / Best Practices Badge

**Source:** `docs/sources.md` line ~315.

**What it requires:** voluntary, automated or self-attested repository
legibility signals (dependency pinning, branch protection, fuzzing,
vulnerability disclosure, etc.), increasingly cited in CRA
implementation guidance as evidence of baseline hygiene.

**Current AGF+`agov` capability (Capability: 1 — Planned; Mapping: 1 —
Planned; Evidence: Theoretical):** unknown/unverified — this document
does not assert whether a Scorecard run or Best Practices Badge
application currently exists for this repository (would require a live
check against the repository's actual state, not a claim from memory).

**Gap:** cheap to obtain (an OpenSSF Scorecard run requires no code
change), not yet done. Good first concrete action item from this
document.

---

## F. Emerging / explicitly deferred

### AI-15 — IETF agent-identity protocols: WIMSE (active WG) vs. AIMS (presentation/proposal)

**Source:** `docs/sources.md` line ~320 (original aggregate note);
`docs/radar.md` "Agent identity standards race." Precision split below
sourced from Batch 9's `datatracker.ietf.org/wg/wimse/about/` and
`datatracker.ietf.org/doc/slides-126-wimse-aims/` entries — **merged via
MR !962** (corrected per finding F-REG-4, which found the earlier
"unmerged" framing stale elsewhere in this document; the same correction
applies here). The distinction described below was independently
confirmed via direct IETF Datatracker fetches during this document's
revision, not solely relayed from the source entries.

**Correction (contribution-auditor finding B-REG-6):** an earlier
revision of this document treated "AIMS/WIMSE" as one immature effort of
equal standing. They are not comparable: **WIMSE (Workload Identity in
Multi-System Environments) is an active IETF working group** (chairs
Justin Richer, Pieter Kasselman; Applications and Real-Time Area),
chartered on workload identity/access-management propagation and a
JOSE-based token solution. **AIMS ("AI Agent Authentication and
Authorization") currently exists only as an IETF 126 presentation/
proposal** — the IETF Datatracker has no working group with acronym
AIMS. Treating them as one effort overstates AIMS's standing.

**What this requires:** not yet a requirement for either — WIMSE is an
active standards effort with no ratified RFC yet; AIMS is earlier-stage
than that.

**Current AGF+`agov` capability (both AI-15a/WIMSE and AI-15b/AIMS —
Capability: 1 — Planned; Mapping: 1 — Planned; Evidence: Theoretical):**
agent identity today is a string role name in coordination-message
frontmatter plus the git commit author's identity — no cryptographic
binding, no verifiable credential, regardless of which standard is
eventually targeted.

**Gap:** intentionally not a gap to close yet, for either sub-item.
`docs/radar.md` already states the correct posture: "keep identity
abstract, bind later... Do not integrate yet" — pending a stable draft
with a real security section (WIMSE is closer to that bar than AIMS) or
an adopter requiring signed provenance. Listed here for completeness and
precision, not as an action item.

---

## Cross-cutting gaps

These block more than one row above and are worth prioritizing as such
rather than fixing per-row:

1. **No compliance-export tooling shipped.** Blocks the requirement-
   specific-mapping half of AI-1, AI-3, AI-4, AI-6, AI-7, AI-9. The
   MCC-AI-scoped plan (`compliance-export-buyer-side-agf.md`) is the
   highest-leverage single piece of work in this document — closing it
   raises AI-6's Mapping toward 4/5 and gives the other rows a concrete
   template to follow, though each still starts its own Mapping at
   Level 1 until its requirement-specific adapter is actually built.
2. **GDPR erasure tension unresolved (AI-5).** Actively blocks issue #8;
   also means any future retention-policy claim made for AI-1/AI-6 must
   be written carefully — "the trail persists indefinitely" is currently
   a fact, not yet a *lawful, deliberately-designed* retention policy.
3. **Threat model pending, not shipped (AI-13).** Blocks honest claims
   about AI-1's and AI-9's risk-management coverage — both ultimately
   ask "did you think about what could go wrong," and AGF has a proposal
   (`threat-model-stpa-owasp.md`) but not yet a shipped answer.
4. **CRA entity-structure decision unresolved (AI-2).** Blocks
   determining whether AGF-the-project has a CRA steward obligation at
   all.
5. **`agents-govern/SECURITY.md` is stale relative to AGF-NG v2.**
   Independent of CRA, this is a plain security-hygiene gap: the current
   project ships executable code (the `agov` Rust binary, an MCP server)
   that its own security policy document does not acknowledge exists.
6. **No provenance/attestation format alignment (AI-10, AI-12).** Both
   name the same fix ("align to an existing schema instead of inventing
   one") — worth doing together if either is picked up.
7. **Every row's Evidence status is Theoretical.** Not a per-row gap —
   a structural fact about AGF's current adoption state (no confirmed
   external paying/production adopters per `foundry.yaml`). Worth
   restating here so a reader scanning only the summary table does not
   need to notice this independently: nothing in this document should be
   read as externally validated.
8. **The Capability/Mapping split (finding B-REG-1-R2's fix) produces a
   pattern worth naming precisely, not impressionistically (finding
   B-REG-7: the prior wording here overstated it)** — counting all 16
   rows (AI-15a and AI-15b separately): **8 rows are unambiguously
   Capability 5** (AI-1, AI-3, AI-4, AI-7, AI-9, AI-10, AI-11, AI-12 —
   AGF's underlying tooling for these is genuinely shipped and mature:
   the coordination trail's schema/commands/rule, the Gate 1/Gate 2 and
   security/TDD rules, the release-provenance tooling, the MCP
   allowlist); **AI-8 is mixed** (Govern and Manage are Capability 5,
   Map is 3, Measure is structurally out of scope); **the remaining 7
   rows (AI-2, AI-5, AI-6, AI-13, AI-14, AI-15a, AI-15b) stay at
   Capability 1 or 2** — real capability gaps, not merely unmapped
   maturity. Mapping is Level 1 in 14 of the 16 rows regardless of
   Capability. Read precisely, scoped to the 8-plus-partial-AI-8 subset
   with mature underlying capability: **for those specific rows, AGF's
   tooling is not the bottleneck; requirement-specific documentation and
   adapters are.** That conclusion does not extend to the other 7 rows,
   where the underlying capability itself is still the open item.

## Structural non-goals (not gaps — declared boundaries)

Per `agf/docs/adopt/compliance-profile.md`, **Layer 4 (runtime
behaviour) is out of scope for every AGF compliance tier, by design.**
This directly caps what several rows above can ever claim:

- AI-1's "duty to suspend" clause.
- AI-8's "Measure" function (runtime trustworthiness/performance).
- Any requirement that asks "did the AI system behave correctly while
  running," as opposed to "was the change that shipped it reviewed,
  approved, and logged."

Restating this here (rather than letting each row imply it might
eventually close) is itself the honesty requirement — a reader should
not need to reverse-engineer AGF's Layer 4 boundary from silence.

## Before Gate 1 approval

Open items that should be resolved, or explicitly accepted as still
open, before this document is Gate 1 approved:

- [x] contribution-auditor review round 1 (`8c92c68a`, `ph5j`): BLOCK,
      six findings, all addressed.
- [x] contribution-auditor review round 2 (`bdfc4b02`, `b72h`): B-REG-2
      through B-REG-6 confirmed resolved; B-REG-1-R2 (Capability/Mapping
      coupling not fully corrected in round 1) addressed in this
      revision.
- [x] Bounded renewed review of B-REG-1-R2 (`284c621c`, `krj5`): found
      three further findings — B-REG-1-R3 (Level 5's self-use
      alternative), B-REG-7 (cross-cutting gap 8's unquantified
      aggregate claim), B-REG-3-R2 (AI-9's "actively enforced"
      overclaim). All three addressed in this revision: the Legend's
      Level 5 row now requires a named release/distribution path (no
      self-use alternative), every Capability-5 row now names or
      verifies its shipped path, cross-cutting gap 8 is quantified
      (8 unambiguous Capability-5 rows, AI-8 mixed, 7 rows at
      Capability 1-2) and its conclusion scoped accordingly, and AI-9
      now says "active normative rules" with the mechanical-enforcement
      caveat already used at AI-2.
- [x] Bounded renewed review of B-REG-1-R3, B-REG-7, and B-REG-3-R2
      (`61ea4c74`, `r7mg`): B-REG-7 and B-REG-3-R2 resolved; B-REG-1-R3
      mostly resolved (legend and distribution-path verification
      correct) but one residual finding, B-REG-1-R4, remained: AI-7 and
      AI-8 Manage grounded the learning loop on unspecified
      `agf/rules/workflow/**` rule files that do not actually define it.
      Fixed in this revision: both rows now cite the actual shipped
      artifacts — `agf/docs/site/docs/concepts/evidence-and-learnings.md`,
      `agf/docs/governance/learning-system.md`, `agf/framework.md`'s
      loop pattern, and `agf/schemas/learning.schema.json` — all
      confirmed present in `agf/dist/release-manifest.yaml`'s shipped
      globs before citing them.
- [x] Bounded renewed review of B-REG-1-R4 (`84b90a89`, `h7ha`): the
      cited learning-loop artifacts were correct except one path —
      `agf/learnings/_template.yaml` does not exist (a similarly-named
      file lives only in the frozen `agents-govern/` v1.x bootstrap
      reference, not the AGF v2 product). Fixed in this revision: cites
      `agf/templates/learning-record.yaml`, the actual shipped
      adopter-facing template, confirmed present by direct `ls`.
- [x] Bounded renewed review of B-REG-1-R5 (`a78dd266`, `rbdm`): CLEAR.
      The complete contribution-auditor chain is resolved.
- [x] infosec-sentinel review, bounded to AI-2 and AI-5
      (`20260803T073120Z-infosec-sentinel-nqh5.md`): CLEAR mechanically
      at exact head `a78dd266`. No blocking findings; establishes only
      that the configured credential/cross-record scan is clean, not
      semantic clearance of the CRA/GDPR prose (governor-reviewer scope).
- [x] fact-checker verification against primary sources
      (`20260803T073119Z-fact-checker-aman.md`, dispatched via
      `20260803T070701Z-governor-reviewer-m31s.md`): BLOCK pending
      F-REG-1 through F-REG-4, all four addressed in this revision (see
      AI-1, AI-3, AI-4, AI-5 above and the `docs/sources.md` correction
      below).
- [ ] Bounded renewed fact-checker review of F-REG-1 through F-REG-4 at
      this revision's new exact head, requested via coordination
      handoff.
- [x] Confirm MR !962 (source registry Batch 9, primary GDPR and
      WIMSE/AIMS sources) has merged — confirmed merged on `main`
      (finding F-REG-4); forward-dependency notes in AI-5 and AI-15
      corrected to reflect the merged state.
- [ ] Decide whether AI-4 (PLD) and AI-11 (Agent-BOM) warrant their own
      tracked issues, or stay as radar-adjacent watch items inside this
      document until a concrete trigger (per `docs/radar.md`'s own
      review-by discipline).
- [ ] Update `docs/sources.md`'s AI Act entry in the same correction
      series (fact-checker advisory A-FACT-1) — otherwise this plan will
      immediately inherit the stale provisional-date status again next
      time it's read.

## Related documents

- `docs/sources.md` — the classified source registry every row above
  cites into; the canonical home for source discovery and verification
  status.
- `docs/radar.md` — several rows above (AI-2, AI-5, AI-11, AI-15) are
  direct restatements of existing radar items; this document does not
  duplicate their controversy notes, only their regulatory-requirement
  framing.
- `docs/plans/compliance-export-buyer-side-agf.md` — the one piece of
  concrete implementation planning this document's gaps most directly
  motivate (AI-6, and named follow-on scope for AI-1/AI-7/AI-9).
- `docs/plans/cra-entity-structure-decision.md` — resolves the
  applicability gate for AI-2.
- `docs/plans/threat-model-stpa-owasp.md` — the pending-Gate-1 threat
  model plan that is AI-13's actual requirement-specific artifact.
- `agf/docs/adopt/compliance-profile.md` — the five-layer verifiability
  model and three-tier vocabulary this document's maturity claims are
  written to stay consistent with (a different vocabulary from the
  implementation-honesty five-level scale used per-row above; the two
  answer different questions and are not meant to be merged).
- `agf/rules/docs/implementation-honesty-rule.md` — source of the
  five-level maturity scale used throughout.
- `always/verifiable-claims-rule.md` — governs the hedging language and
  citation discipline used throughout.
