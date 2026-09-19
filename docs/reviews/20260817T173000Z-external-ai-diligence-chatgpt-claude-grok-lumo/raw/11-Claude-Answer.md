# Open-Source Due Diligence Assessment — agents-govern (AGF)

**Report identifier:** AGF-OSDD-2026-08-17-001

---

## 1. Report Metadata, Assumptions, and Constraints

| Field | Value |
| --- | --- |
| Project | agents-govern (AGF) |
| Assessment date | 2026-08-17 |
| Protocol | Open-Source Project Due Diligence Assessment Protocol, v2.1 |
| Assessment purpose | Enterprise adoption |
| Assessment mode | Reassessment (prior: 2026-08-10, simulated Grok/Copilot reviews of v2.5.0, not under Protocol 2.1) |
| Version examined | Docs build `fb6bee8e3912`, framework content version 2.6.1, build timestamp 2026-08-17T05:43:31Z [S003] |
| Evidence basis | Published documentation site and repository landing page only |
| Analyst independence | This assessment was produced by an AI assistant at the request of the project's sole maintainer. It is not an independent third-party audit, and it should not be presented as one. |

### Licensing classification (Pass 1 screening, retained in Pass 2)

**Dual licensed, with one non-OSI component.** Tooling code is MIT (OSI-approved). Framework *content* — rules, roles, schemas, templates, documentation — is CC BY-SA 4.0, which is **not an OSI-approved software licence** and is not designed for software or for configuration-as-code artifacts. A separate, non-public commercial licence is offered for alternative terms [S002, S003, S010]. Terminology throughout this report reflects that split: "open source" applies accurately to `agov`/`agov-mcp`; "openly licensed content under a copyleft content licence" applies to the framework library.

### Assumptions and access limitations (material)

1. **GitLab's dynamic pages were not retrievable by the assessment tooling.** Releases, issues, merge requests, commit history, file blobs, and CI configuration all returned client-rendered shells with no content. Consequently: **no code was inspected, no test or coverage evidence was inspected, no dependency manifest or lockfile was inspected, no contributor statistics were computed, no issue/MR response times were computed, and `governance.md` and any `SECURITY.md` could not be read.**
2. All architecture, tooling, CI, and provenance findings below are **documentation-derived project claims**, not verified observations of the artifacts themselves.
3. The repository landing page reports creation on **2026-06-14** [S001]. The full history available is therefore ~64 days. Standard 12/24/36-month observation windows are **not applicable**. The existence of an "Upgrade from v1" page [S002] indicates a predecessor line whose history is not in this repository — possible history truncation or migration; unverified.
4. No search-visible third-party discussion, packaging, or adoption reference was located [S011]. This is weak evidence and is not treated as proof of zero adoption.

### Weighting Applied

| Dimension | Weight | Priority | Contextual rationale | Evidence status |
| --- | --- | --- | --- | --- |
| Governance Maturity | 13% | Critical | Framework asserts authority over engineering decisions; control concentration is the dominant adoption question | Partial |
| Operational Sustainability | 12% | Critical | Adopters embed files in their own repos and depend on continuity of rules and tooling | Partial |
| Release Maturity | 12% | Critical | Adopters pin to tags; publication reliability is directly observable and directly consumed | Good |
| Engineering Maturity | 11% | High | `agov` runs on developer workstations and in CI | **None (NE)** |
| Security Posture | 9% | High | Tooling executes locally and in CI; no runtime, but a compromised binary has developer-machine reach | Partial |
| Supply-Chain Maturity | 9% | High | Binary distribution to developer machines and CI | Good |
| Legal Confidence | 8% | High | Non-OSI content licence with ShareAlike, embedded in adopter repositories | Partial |
| Community Health | 8% | High | Evidence base generalisability depends on contributor plurality | Partial |
| Strategic Strength | 7% | Medium | Fit and displacement risk affect a multi-year commitment | Good |
| AI-Era Readiness | 5% | Medium | Most of §18 is N/A — AGF ships no model, dataset, or inference path | Partial |
| Ecosystem Strength | 3% | Low | Early-stage project; low decision weight | Weak |
| Financial & Org Sustainability | 3% | Low | Solo, pre-commercial; little public data expected | **None (NE)** |
| **Total** | **100%** | | | |

No weights were changed between Pass 1 and Pass 2.

---

## 2. Context-Driven Triage and Assessment Plan

The deployment context is repository-local process scaffolding: no hosted service, no runtime interception, no new data flows beyond files the adopter's own git already carries. That materially lowers the blast radius relative to a typical enterprise software dependency, and it raises the weight of the *non-technical* dimensions — governance continuity, licence clarity, and evidence generalisability — because the technical failure mode is "the framework stops being maintained" rather than "the framework breaks production."

Two facts dominate the triage. First, the repository is **~9 weeks old** with a single named authority [S001, S003], so every continuity, succession, and bus-factor question is answered by inspection rather than by analysis. Second, **GitLab's dynamic surfaces were unreadable**, so the entire code-level half of the protocol cannot be executed; Engineering Maturity is unscoreable and several security controls can only be assessed as claims.

The five most decision-relevant questions:

1. What happens to an adopter's embedded governance library if the sole maintainer stops? Is there any succession, co-maintainer, or foundation path?
2. Does CC BY-SA 4.0 on adopter-embedded, adopter-modified content create redistribution obligations for an enterprise that ships software containing derived rule files?
3. Is there a private vulnerability reporting channel and a patch commitment for `agov`/`agov-mcp`?
4. Are release artifacts verifiable by a downstream consumer without trusting the same host that serves the checksums?
5. Is the framework's evidence base generalisable beyond its author, given that the project itself reports zero "overkill" findings and calls that suspicious [S005]?

Potential adoption blockers identified in Pass 1 and confirmed in Pass 2: bus factor of 1; absent security disclosure channel; absent artifact signing and SBOM; non-OSI content licence with unpublished commercial terms; no independently verifiable adopter.

| Dimension | Priority | Depth | Key questions | Known limitations |
| --- | --- | --- | --- | --- |
| Strategic Position | Medium | Standard | Scope clarity; displacement by assistant-native governance | No competitor evidence beyond project's own comparisons |
| Governance | Critical | Deepest available | Succession; release authority; relicensing risk | `governance.md` unreadable |
| Community | High | Detailed | Independent contributors; evidence plurality | No contributor statistics available |
| Engineering | High | Screening only | Test strategy; API stability; error handling | No code, tests, or coverage inspected |
| Release | Critical | Deepest available | Publication reliability; support/LTS policy | Release pages unreadable; roadmap is the substitute source |
| Security & Supply Chain | High | Detailed | Disclosure channel; signing; SBOM; dependency hygiene | No manifests, no CI file, no advisories inspected |
| Operational Sustainability | Critical | Deepest available | Maintainer workload; funding; CI capacity | No financial data |
| Legal & Licensing | High | Detailed | ShareAlike reach; CLA/DCO; commercial terms | Licence files not read directly; commercial terms non-public |
| Adoption & Ecosystem | Low | Screening | Verifiable third-party use | Search-based only |
| Financial & Org | Low | Screening | Funding, sponsorship | Essentially none available |
| AI-Era | Medium | Standard | Governance-of-AI controls; MCP tool boundaries | Most subsections N/A |

---

## 3. Executive Summary

AGF is a repository-local governance framework for AI-assisted software development. It ships role definitions, a rule library, JSON schemas, an append-only coordination-record protocol, two human review gates, and an optional Rust CLI (`agov`) with a local stdio MCP server (`agov-mcp`) [S002, S009]. It adds no hosted service and no runtime interception, and it documents a one-commit exit path [S002, S008].

**Maturity and lifecycle stage: early, high-velocity, single-author.** The repository is ~9 weeks old with 4,194 commits, 548 branches, and 27 releases [S001]. The framework content version is 2.6.1; the last *fully published* stable release is v2.5.0 (2026-08-10) [S003, S007].

**Principal strengths.** The project's documentation discipline is unusual and materially valuable: it separates capability maturity (Level 1–5) from evidence status (validated / informed / theoretical), labels single-adopter rules as Provisional, publishes an explicit reviewer protocol to prevent reviewers fabricating claims, and states its own weaknesses in its own marketing surface [S002, S005, S006]. Release provenance is well specified — a closed JSON schema with one record per artifact, SHA-256 sidecars, a tag/version guard, and a post-publication check that re-downloads published bytes and executes the packaged binary's self-report [S004]. The cost, privacy, labour, and exit sections are more candid than most commercial products manage.

**Principal concerns.** All project authority — merge, release, gate approval, licensing, security response — rests with one natural person, with no documented succession [S003, S005]. No public security policy or private vulnerability-reporting channel was identified. Artifact **signing and SBOM generation are explicitly deferred work** [S004]. Four release publications in six weeks were incomplete or broken (v2.4.5, v2.4.6, v2.4.7, v2.6.0) [S004, S007]. The evidence base is self-referential: most adopter projects involve the framework's own designer [S005]. The framework content sits under a non-OSI copyleft *content* licence that adopters copy into and modify inside their own repositories.

**Overall enterprise suitability:** suitable for a bounded, reversible pilot under a named internal owner; not suitable as an organisational standard.

**Overall risk level:** High for standardisation; Medium for a scoped pilot, principally because the exit path is short and no runtime or data flow is added.

**Due diligence rating:** **C** — Acceptable, usable with explicit conditions and mitigations.
**Recommendation:** **Recommend with Reservations**, pilot-scoped.
**Confidence in the overall conclusion:** Medium-Low. The unscored 14% of contextual weight, and the fact that no code was inspected, are the limiting factors.

**Five most decision-relevant findings:** bus factor of 1 [F1]; no security disclosure channel [F2]; no signing/SBOM despite otherwise strong provenance [F3]; repeated release publication failures [F4]; non-OSI ShareAlike content licence embedded in adopter repositories with no public commercial terms and no located CLA/DCO [F5].

---

## 4. Overall Rating and Recommendation

**Overall rating: C.** **Recommendation: Recommend with Reservations.** **Overall confidence: Medium-Low.**

**Decision rationale.** The partial weighted result across scored dimensions is **1.71 / 5** over 86% of contextual weight — arithmetically in D territory. The rating is set at C rather than D as an explicit professional judgment, resting on three specific mitigating facts and nothing else: (a) AGF adds no runtime, no daemon, and no new data flow, so a failure mode is "stale files in your repo," not an outage [S002, S008]; (b) removal is documented as a single atomic cleanup commit [S002, S008]; (c) the project's own documentation states its limitations accurately enough that an adopter can make an informed decision without discovering them later [S005, S006]. **If the assessment purpose were organisation-wide standardisation or regulated development rather than adoption generally, the rating would be D and the recommendation High Risk.**

**Adoption conditions (all required):**
1. Pilot confined to one repository with one named internal governor and a defined evaluation end date.
2. Vendor the framework content at a pinned, verified tag; do not track `main`.
3. Independent legal review of CC BY-SA 4.0 obligations before any modified framework content is distributed outside the organisation.
4. Build `agov`/`agov-mcp` from source, or verify checksums out-of-band, until signing exists.
5. Do not rely on `agov-mcp`'s `dir` argument until the fail-open path-contract defect ships fixed [S009].
6. Do not represent AGF records as evidence of regulatory compliance [S003, S010].

**Blocking issues for business-critical, regulated, or multi-repository use:** bus factor 1; no security disclosure or patch commitment; no signing or SBOM; no independent adopter evidence; no documented LTS or supported-version policy.

**Circumstances that would change the recommendation upward:** a second maintainer with merge and release authority; a published security policy with a private reporting channel; release signing plus SBOM; two or more independent adopters contributing learning records including at least one "overkill" finding.

---

## 5. Decision-Relevant Findings

| ID | Finding | Type | Sources | Confidence |
| --- | --- | --- | --- | --- |
| F1 | All approval, release, licensing, and security authority rests with one named individual; no co-maintainer, succession, or foundation arrangement was located. The project states the maintainer personally screens and integrates every contribution. | Observed (docs) | S003, S005 | High |
| F2 | No public project-specific security policy, private vulnerability-reporting channel, or advisory page was identified. Reported as not provided in the assessment inputs and not located during review. | Absence of evidence, corroborated by input | S011, inputs | Medium |
| F3 | Release signing, SBOM generation, installer scripts, and third-party notices are stated by the project as deferred follow-on work. Checksums and provenance JSON are served from the same host as the artifacts. | Observed — capability evidenced as absent | S004 | High |
| F4 | Four releases in ~6 weeks were incomplete or defective: v2.4.5 shipped an archive whose runbook tests failed from a clean extract; v2.4.6 exhausted runner disk; v2.4.7 published OCI only; v2.6.0 constructed a tag and OCI image but never created the GitLab Release or package. | Observed (project's own record) | S004, S007 | High |
| F5 | Framework content is CC BY-SA 4.0 — a content licence, not OSI-approved for software, with no patent grant — and adopters are expected to copy and modify it inside their own repositories. No CLA or DCO was located; commercial licence terms are not public. Sole copyright holder can relicense unilaterally. | Observed + inference | S002, S003, S010 | Medium |
| F6 | The project states its strongest current weakness is single-developer evidence, that most adopter projects involve the framework's own designer, and that zero "overkill" findings across all projects is suspicious. | Project claim (self-critical) | S005 | High |
| F7 | `agov-mcp` has a documented fail-open defect: passing the repository root to `dir` returns `ok: true` while silently skipping non-coordination files. The fix is planned but not Gate 1 approved. Component is capped at Level 4 Validated. | Observed (docs) | S009 | High |
| F8 | Append-only audit trail carrying author identities is documented as in direct tension with GDPR Art. 17 erasure rights; the project explicitly does not resolve it for adopters. | Project claim | S005 | High |
| F9 | Source conflict on current release state: Release information (built 2026-08-17T05:43Z) lists v2.6.1 as latest release at tag commit `f780ac0ea370`; the Roadmap (observed 2026-08-17T03:47Z) states v2.6.1 is Gate 1 approved with release-MR construction in progress and v2.5.0 is the latest fully published stable. | Conflicting primary sources | S003, S007 | Medium |
| F10 | Only 8 of 58 governed artifacts are classified in the adopter applicability catalog; a reduced starter package is Level 1 Planned. The only maintainer-evidenced configuration is the full, unpruned rule set. | Observed (docs) | S008 | High |
| F11 | Linux arm64 tooling builds were dropped because the shared CI runner tier lacks disk; resumption is stated as contingent on sponsorship. | Observed (docs) | S004 | High |
| F12 | No third-party discussion, packaging, dependent project, or adoption reference was located in public search. | Absence of evidence | S011 | Low |

---

## 6. Weighted Scorecard

| Dimension | Score | Conf. | Weight | Rationale (Source IDs) |
| --- | --- | --- | --- | --- |
| Strategic Strength | 3 | Medium | 7% | Scope is unusually precisely bounded, including explicit statements of what AGF is not [S006]. Differentiation from runtime governance, model governance, and agent SDKs is coherent. Displacement risk from assistant-vendor native rule/hook mechanisms is real and unaddressed. Roadmap is detailed but is self-labelled "informational projection," not authority [S007]. |
| Governance Maturity | 2 | Medium | 13% | Roles, gates, and authority boundaries are well defined *within* the framework, and AI assistants are explicitly denied approval authority [S003]. But project-level governance is one person with unilateral relicensing capability and no succession path [S003, S005]. `governance.md` unreadable — score would move only on evidence of a second authority holder. |
| Community Health | 1 | Medium | 8% | Contribution paths, anonymisation policy, and an evidence-graded promotion model (Provisional → Recommended) are defined [S005]. Against that: no second contributor evidenced, the project itself reports the corpus is dominated by its own designer, and no contributor metrics were obtainable [S005, F6]. Scored 1, not 0: contribution machinery demonstrably exists. |
| Engineering Maturity | **NE** | — | 11% | No code, tests, coverage, static analysis, or dependency manifests were inspectable. Documentation describes validators, schemas, and a Rust CLI, but a documented capability is not an inspected one. **Not converted to a score in either direction.** |
| Release Maturity | 2 | High | 12% | Strong: semver discipline with a single authored VERSION scalar, tag/version guard, per-artifact provenance, changelog, tag-only release pipelines, post-publication verification [S003, S004]. Weak: four incomplete/defective publications in six weeks [F4], a live version-state conflict [F9], no located LTS, supported-version, deprecation, or backport policy. High cadence is not credited as maturity. |
| Security Posture | 1 | Medium | 9% | No disclosure channel or advisory process located [F2]; no threat model, secret-scanning, dependency-scanning, or security-testing evidence located; one documented fail-open contract defect in a shipped component [F7]. Offsetting: the disclaimer correctly states validators are not proof of security [S010]. Public evidence cannot establish the absence of undisclosed vulnerabilities. |
| Supply-Chain Maturity | 2 | Medium | 9% | Above typical for a solo project: closed provenance schema with `additionalProperties: false`, one record per artifact, legacy records never implicitly upgraded, SHA-256 sidecars, digest-pinned OCI verification, packaged-binary self-report execution against downloaded bytes [S004]. Capped by absent signing and absent SBOM — both stated as deferred, so evidenced-absent, not unknown [F3]. |
| Operational Sustainability | 1 | Medium | 12% | Bus factor 1; no funding or sponsorship disclosed; CI capacity is already a binding constraint that has cost a build target [F11]; change velocity (4,194 commits / 548 branches in ~9 weeks) against one human governor raises a review-depth question the project's own gate model depends on [S001]. Documentation continuity is the counterweight and is strong. |
| Legal Confidence | 2 | Medium | 8% | Licensing is stated clearly and consistently on every page footer, licence texts are stated to ship, citation metadata is provided [S003]. But CC BY-SA 4.0 is a content licence applied to configuration-as-code that adopters modify; no CLA/DCO located; commercial terms non-public; single copyright holder; no patent grant. Requires specialist legal review before any external distribution of modified content. |
| Ecosystem Strength | 1 | Low | 3% | No verifiable third-party adopter, package-registry presence, integration, or downstream dependent located [F12]. The project itself is actively soliciting independent adopters [S005]. Low confidence: absence in search is weak evidence. |
| Financial & Org Sustainability | **NE** | — | 3% | No sponsorship, grant, foundation, revenue, or paid-maintainer data available. One indirect signal only: CI capacity described as awaiting sponsorship [F11]. No revenue or runway estimate is offered. |
| AI-Era Readiness | 2 | Medium | 5% | Human-gate design, explicit denial of AI approval authority, AI-assistance disclosure on releases and figures, and an EU AI Act Art. 50-aligned figure-disclosure control are genuine, documented controls [S003]. Against that: no threat model, no prompt-injection or indirect-injection defence documentation, no evaluation framework measuring whether governance improves outcomes, and a fail-open MCP tool boundary [F7]. |

**Treatment of NE.** Two dimensions are unscored, representing **14% of total contextual weight** (Engineering 11%, Financial 3%). Neither is Critical priority, and 14% is below the 20% threshold, so an overall rating is issued rather than deferred. **Partial weighted result across the scored 86%: 1.71 / 5.** The unscored weight could move the result in either direction: strong test and static-analysis evidence in the Rust codebase would raise it by roughly 0.3–0.4; discovery of poor engineering hygiene would lower it similarly. It would not, on its own, change the recommendation, because the binding constraints (governance concentration, security disclosure, licence) are non-engineering.

---

## 7. Prioritized Risk Register

| ID | Cat. | Risk (cause → event → impact) | Ev. | Inh. L / I / Rating | Existing controls | Ctrl. eff. | Residual | Conf. | Mitigation | Post-mit. | Owner | Timeframe | Adoption implication |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| R01 | Governance | Sole maintainer holds all merge, release, licensing, and security authority → illness, loss of interest, or employment change halts the project → adopters hold an unmaintained governance library embedded in active repositories | F1 | H / High / **High** | Public repo; content licence permits forking; documented exit path | Partially Effective | **High** | High | Vendor at a pinned tag; internal fork; name an internal owner for the vendored copy; require a second maintainer before standardisation | Medium | Eng. leadership | Immediate | **Blocking for standardisation** |
| R02 | Security | No private vulnerability-reporting channel → a defect in `agov`/`agov-mcp` is disclosed publicly or not at all → unpatched tooling runs on developer workstations and CI runners | F2 | M / High / **High** | None identified | No Control Identified | **High** | Medium | Publish SECURITY.md with a private channel, triage SLA, and supported-version statement; adopters treat `agov` as untrusted third-party tooling | Medium | Maintainer | Within 30 days | Requires mitigation |
| R03 | Supply chain | No signing and no SBOM → a tampered or substituted artifact cannot be distinguished by a downstream verifier, and no component inventory exists for CVE triage | F3 | M / High / **High** | SHA-256 sidecars; per-artifact provenance; post-publication re-download check | Partially Effective | **Medium-High** | High | Add Sigstore or GPG signing over tags and artifacts; generate CycloneDX or SPDX SBOM per release and link it from the Release page | Medium | Maintainer | Within 90 days | Requires mitigation |
| R04 | Operational | Release construction complexity exceeds one-person CI capacity → tags exist without complete published assets → adopters pin to a version they cannot download or verify | F4, F9, F11 | H / Medium / **High** | Tag/version guard; `release:verify` post-publication check; batching amendment | Partially Effective | **Medium** | High | Make release-asset contract validation a pre-tag gate rather than post-publication; publish a "fully published" badge or machine-readable index | Low-Medium | Maintainer | Within 30 days | Requires mitigation |
| R05 | Legal | CC BY-SA 4.0 content licence + adopter modification + no CLA/DCO + non-public commercial terms → an enterprise distributes a product containing derived framework content → unanticipated ShareAlike obligations or a contested contribution provenance | F5 | M / High / **High** | Clear licence statement per page; separate MIT tooling licence; no-warranty disclaimer | Partially Effective | **High** | Medium | Legal review before external distribution; maintainer to publish commercial terms and adopt a DCO; consider relicensing framework content under a software-appropriate licence | Medium | Legal counsel | Within 30 days | **Blocking for distribution of modified content** |
| R06 | Strategic / evidence | Rules derived predominantly from one developer's practice, with zero overkill findings recorded → the library encodes maintainer preference as general practice → adopters carry ceremony that does not pay for itself | F6, F10 | H / Medium / **High** | Provisional labelling; maturity/evidence vocabulary; applicability catalog (8 of 58) | Partially Effective | **Medium** | High | Pilot with explicit overkill logging; complete the applicability catalog; require independent corroboration before promoting rules | Medium | Pilot governor | Within 90 days | Requires mitigation |
| R07 | AI security | `agov-mcp` returns success on a misused `dir` argument while silently skipping records → an assistant or CI step reports validated governance that was never checked → false assurance in the audit trail | F7 | M / High / **High** | Documented limitation; planned fix (not yet Gate 1 approved) | Ineffective (documentation only) | **High** | High | Ship the `coordination_dir` rename with a hard error on misuse; until then, forbid explicit `dir` in adopter configuration | Low | Maintainer | Immediate | Requires mitigation |
| R08 | Privacy | Append-only trail carrying personal identifiers → an erasure request arrives → either the audit trail is destroyed or the request cannot be honoured | F8 | M / High / **High** | Documented tension; identity-minimisation guidance; anonymisation policy for upstream submissions | Partially Effective | **Medium-High** | High | Adopt role-based or pseudonymous attribution from day one; document lawful basis and retention before the pilot starts | Medium | DPO / privacy owner | Immediate | Requires mitigation |
| R09 | Operational | Very high change velocity against a single human governor → gate review becomes formality → the framework's core control is weaker in practice than in specification | S001, F1 | M / Medium / **Medium** | Two-gate model; validators; append-only records | Not Verified | **Medium** | Low | Independent sampling audit of the project's own coordination records against its gate rules | Low-Medium | External reviewer | Within 90 days | Informational |
| R10 | Vendor concentration | Sole dependency on GitLab.com shared runners → capacity limits recur → build targets are dropped or releases fail | F4, F11 | M / Medium / **Medium** | Optional/mandatory target split; source-build fallback documented | Partially Effective | **Medium** | High | Document a supported-platform matrix with explicit non-support statements; consider self-hosted runner | Low | Maintainer | Within 6 months | Acceptable with monitoring |
| R11 | Strategic | AI assistant vendors ship native rules, hooks, and governance-event mechanisms → repository-local scaffolding is subsumed → sunk commissioning cost | S006 | M / Medium / **Medium** | Explicit positioning as complementary to runtimes; low lock-in; documented exit | Partially Effective | **Medium** | Medium | Reassess annually; keep the exit path tested | Medium | Eng. leadership | Longer term | Acceptable with monitoring |
| R12 | Governance | Documentation status and release state can diverge (docs build ahead of published release) → adopters cite or install a version whose assets do not exist | F9 | M / Medium / **Medium** | Explicit "documentation status" and "live docs build" rows | Partially Effective | **Medium** | Medium | Machine-readable published-release index consumed by the docs build | Low | Maintainer | Within 90 days | Informational |
| R13 | Engineering | Engineering quality is unassessed → latent defects in tooling that adopters run in CI | — | Not assessable | Unknown | Not Verified | **Not assessable** | — | Obtain code, test, and coverage evidence; run SAST and dependency scan on a checkout | — | Adopter security | Within 30 days | Requires evidence |

No inherent risk was reduced on the basis of documentation alone.

---

## 8. Methodology, Sampling, and Limitations

**Sampling basis:** This assessment is based on sampled inspection of **eight published documentation pages and one repository landing page**. No source files, commits, merge requests, issues, CI configuration, dependency manifests, test suites, or release assets were inspected.

**Confidence modifier:** Confidence is **Low to Medium** because the published documentation corpus and repository metadata were inspected, while the entire code, test, dependency, CI, issue, merge-request, and release-asset surface was not.

Pages inspected deeply: home, release information, release artifacts, contribute, evaluate, roadmap, should-you-adopt, MCP server, disclaimer. Inspected superficially: repository landing page metadata. Not inspected: everything else, including all `agf/` content, all schemas, `governance.md`, any `SECURITY.md`, `CHANGELOG.md`, the issue tracker, and all releases.

This sample was selected because it is the corpus the project itself designates as canonical for external review [S006], and because it was the only corpus technically retrievable. Findings derived from it describe **what the project documents about itself**, not what the artifacts do. They are not generalised to the codebase.

This assessment is not, and does not substitute for: a source-code audit, penetration testing, malware analysis, formal legal advice, licence scanning, financial audit, maintainer interviews, production threat modelling, or regulatory certification. Conclusions specifically requiring those activities: all Engineering findings (code audit); R02 and R13 (SAST, dependency scan, licence scan); R05 (legal counsel); R08 (privacy counsel); R09 (records audit and maintainer interview).

**Deviations from the Assessment Plan.** Contributor and control-concentration metrics (§11) could not be computed at all — no commit, MR, or review data was retrievable. Public vulnerability baseline (§13) could not be executed — no dependency manifest or lockfile was retrievable, and no ecosystem advisory lookup was possible against an unknown dependency set. Both are recorded as unscored gaps rather than negative findings.

---

## 9. Detailed Dimension Assessments

**9.1 Strategic Position (Score 3, Medium).** Purpose and scope are stated with unusual precision, including an explicit non-identity list distinguishing AGF from Microsoft's Agent Governance Toolkit, agent SDKs, ML governance tools, and runtime monitoring platforms [S006]. The value proposition — reconstructibility of AI-assisted decisions without new infrastructure — is coherent and addresses a real and growing gap. Interoperability is good by construction: files in git, loaded by whatever assistant the adopter already runs. The roadmap is detailed and honestly labelled non-authoritative [S007]. Weaknesses: differentiation rests on a positioning argument rather than on demonstrated outcomes, and no comparison against credible alternatives is attempted here because insufficient evidence was available to do so responsibly. Displacement risk is the main strategic exposure — the same governance primitives are candidates for native implementation by assistant vendors.

**9.2 Governance (Score 2, Medium).** Within the framework, the authority model is a strength: roles carry written jurisdictions, an implementing agent cannot approve its own work, AI assistants are explicitly denied gate and release authority, and the human governor is named and accountable [S002, S003]. At project level, the same person holds every role: approval, publication, licensing, contribution screening, and by default security response [S003, S005]. No contributor promotion path to merge rights, no conflict-resolution procedure, no succession arrangement, and no foundation or corporate affiliation were located. Risk of unilateral relicensing is structural, not hypothetical: a single copyright holder with an existing commercial-licence track. Organisational concentration and individual concentration are the same thing here. `governance.md` was not readable; a documented succession clause inside it would raise this score.

**9.3 Community Health (Score 1, Medium).** Three graded contribution paths exist with clear evidence requirements and an explicit promotion bar (Provisional → Recommended requires independent corroboration) [S005]. An external-review issue template and a reviewer protocol exist [S006]. No trend data is available and no second contributor is evidenced. The project's own statement — that most adopter projects involve the same person who designed the framework, and that zero overkill findings is suspicious — is the strongest available evidence on this dimension and it is negative [F6]. No code of conduct was located. No demographic inference was attempted.

**9.4 Engineering Quality and Architecture (NE).** Architectural intent is clear from documentation: file-based protocol as source of truth, Rust CLI as a deterministic accessor, MCP server as a pure transport adapter over the CLI with no added authority [S009]. Portability is addressed via platform-specific archives and an OCI image. API stability is addressed via a versioned, closed provenance schema with explicit legacy classification [S004]. None of this was verified against code. No test strategy, coverage figure, static-analysis configuration, or lint policy was inspectable. One concrete quality signal is available and it is negative: the v2.4.5 archive's own runbook test suite failed from a clean extract because fixtures were missing from the release manifest [S007]. That is a packaging-manifest defect rather than a code defect, but it indicates the packaged artifact was not being tested as shipped at that point.

**9.5 Release Management (Score 2, High).** Cadence over the observed window (2026-06-14 to 2026-08-17): 27 releases, 39 tags [S001]. Version discipline is well specified — one authored VERSION scalar, a CI guard that fails the job before any archive is written on tag/VERSION mismatch, tag-only release pipelines, preview artifacts explicitly labelled as not releases [S004]. Against that, the observed publication record is poor: v2.4.5 shipped defective, v2.4.6 failed on runner disk, v2.4.7 published OCI only, v2.6.0 produced a tag and image but no Release or package [S004, S007]. That is four defective or partial publications out of a small recent set. Documented support policy: none located — no LTS, no supported-version list, no backport policy, no deprecation policy. Upgrade tooling: an "Upgrade from v1" guide exists; upgrade realism beyond that is unassessed. Frequent releases are not credited as maturity here; the relevant signal is the failure rate, not the count.

**9.6 Security and Software Supply Chain (Scores 1 and 2).** Covered above at F2, F3, F7 and R02, R03, R07. Two further observations. First, the supply-chain design shows real sophistication in one specific respect: `post-publication-check` re-fetches from the Release API, pulls the OCI image by immutable digest, extracts and executes the packaged binary, compares its self-report, and emits explicit SKIP lines for targets the verifier cannot run rather than passing silently [S004]. That is a stronger verification posture than most projects of any size. Second, that strength is undercut by the absence of a trust root: without signing, every check verifies internal consistency of artifacts served by the same authority. Branch protection, CI job permissions, third-party action pinning, secret handling, and release credential management could not be observed. **Public evidence cannot establish the absence of undisclosed vulnerabilities in this project, and none of the above should be read as a statement that the code is secure.**

**9.7 Operational Sustainability (Score 1, Medium).** Documentation continuity is strong — runbooks, session continuity, day-to-day operation, and exit procedures are all published. Automation is substantial. Everything else on this dimension is constrained by one person: maintenance velocity, backlog disposition (issue and MR ledgers are maintained, which is a positive control), release continuity, and security response all share the same single point of failure. CI capacity is already binding [F11]. Likelihood of sustained operation over one to three years is not estimable from public evidence; the honest statement is that it is a function of one individual's continued interest and availability, with no structural buffer.

**9.8 Legal, Licensing, IP, and Compliance (Score 2, Medium — non-legal-advice).** Licence statements are clear, consistent, and repeated on every page footer, with distinct terms for content and tooling and complete licence texts stated to ship in the distribution [S003]. The disclaimer correctly disclaims professional advice, warranty, and vendor endorsement, and correctly states that validators are not proof of compliance [S010]. Matters requiring specialist legal review: (i) ShareAlike scope when adopters modify framework content and distribute software containing it; (ii) absence of a patent grant in CC BY-SA 4.0 as applied to functional artifacts; (iii) absence of a located CLA or DCO combined with contributions produced with AI assistance — contribution provenance for upstream-integrated learning records and rule proposals; (iv) undisclosed commercial licence terms, including whether they cover the same version and edition under assessment; (v) the GDPR Art. 17 tension at F8. The project's EU AI Act Art. 50 reference is correctly framed as supporting transparency objectives rather than as certification [S003] — that framing should be preserved in any procurement conversation.

**9.9 Adoption and Ecosystem (Score 1, Low).** No verifiable third-party deployment, package-registry listing, downstream dependent, integration, or commercial support option was located. The project does not claim otherwise and is openly recruiting independent adopters [S005]. Popularity metrics were not used and would not have changed this score.

**9.10 Financial and Organisational Sustainability (NE).** No public funding, sponsorship, grant, foundation participation, or revenue data. A commercial licensing track is stated to exist but its terms and status are not public. No revenue, runway, or valuation estimate is offered.

**9.11 AI-Era Assessment (Score 2, Medium).** Applicability: AGF ships no model, no dataset, no training pipeline, no retrieval system, and no inference path. Subsections **18.2 (model/provider dependency), 18.3 (evaluation/benchmarking of models), and 18.4 (dataset transparency) are Not Applicable** as written — AGF depends on whichever assistant the adopter already uses and makes no model-level claims. The applicable subsections:

- *18.1 AI Governance and Accountability:* documented and genuinely operationalised — named accountable human, defined role authority, explicit denial of AI approval rights, AI-assistance disclosure on releases and figures [S003]. **Score 3.**
- *18.5 Prompt, Agent, and Workflow Management:* rules ship as versioned files under git; coordination records are schema-validated and append-only; handoff, claim, and acknowledgement semantics are defined; human approval gates are explicit [S002, S009]. No sandboxing, cost/token control, or termination-behaviour controls are AGF's responsibility. **Score 3.**
- *18.6 AI Security and Abuse Resistance:* no prompt-injection or indirect-injection defence documentation located, and rule files loaded by assistants are themselves an injection surface if an attacker can commit to the repository. One fail-open tool-boundary defect documented [F7]. **Score 1.** No claim is made here that any AI system is safe or injection-proof.
- *18.7 Explainability, Observability, Reproducibility:* the append-only coordination channel is a genuine audit and incident-reconstruction capability, with plans, decisions, and gate records tied to commits and releases [S002]. **Score 3.**
- *18.8 Responsible AI and Human Oversight:* human-in-the-loop is the design centre; AI-generation disclosure and alt-text are enforced on published figures via a strict build check [S007]. No fairness, bias, or harm-monitoring surface — largely N/A for this artifact class. **Score 3.**
- *18.9 AI Legal and Regulatory Readiness:* transparency controls documented and correctly framed as non-certifying [S003]; record-keeping is a strength; risk-management documentation for adopters is guidance, not a management system. **Score 2.**
- *Evaluation maturity of the framework's own claims:* no evaluation framework, baseline, or measurement of whether governed projects produce better outcomes than ungoverned ones. Token-ROI is explicitly reported as local telemetry rather than a measured claim [S002] — honest, but it means the central efficiency claim is unevaluated. **Score 1.**

---

## 10. Enterprise-Readiness Matrix

| Scenario | Verdict | Conditions / blockers | Confidence |
| --- | --- | --- | --- |
| Experimentation / PoC | **Ready with conditions** | Pinned tag; build tooling from source or verify checksums out-of-band; avoid explicit `dir` in MCP config | Medium |
| Internal non-critical use | **Ready with conditions** | All of the above, plus named internal owner, vendored copy, identity minimisation in records from day one | Medium |
| Governing customer-facing product development | **Ready with conditions** | All of the above, plus legal review of ShareAlike before shipping anything containing derived content; do not depend on AGF records for external assurance | Low-Medium |
| Business-critical development | **Not ready** | Blockers: bus factor 1; no security disclosure or patch commitment; no supported-version policy | Medium |
| Regulated / safety-sensitive development | **Not ready** | Above, plus no SBOM, no signing, unresolved GDPR Art. 17 tension, no independent validation of the framework's own claims | Medium |
| Large-scale multi-repository production | **Not ready** | The project states federation and org-wide enforcement are hypotheses, not shipped capability [S008] — a project-declared non-fit, not an assessment inference | High |

---

## 11. SWOT Analysis

**Strengths.** Epistemic discipline is the differentiator: capability maturity and evidence status are separated, single-adopter rules are labelled Provisional in the rule text itself, and the marketing surface states the project's weakest point [S002, S005, S006] — this is rare and it materially reduces adopter surprise risk. Release provenance design is genuinely strong for the project's size: closed schema, per-artifact cardinality, no implicit legacy upgrade, real post-publication re-download and binary-execution verification [S004]. Low lock-in is designed in and documented, with a one-commit exit [S002, S008]. Cost, privacy, labour, and provider-boundary implications are itemised rather than elided [S008].

**Weaknesses.** Single point of control and failure across every project function [F1]. Absent security disclosure channel [F2]. Absent signing and SBOM [F3]. Repeated publication failures [F4]. Self-referential evidence base with zero recorded overkill findings [F6]. Only 8 of 58 governed artifacts classified for applicability, while the only evidenced configuration is the full unpruned set — a large default surface with an incomplete map [F10]. No evaluation of whether the framework achieves its stated benefit.

**Opportunities.** The governance-for-AI-assisted-development gap is real and under-served for teams that do not want a hosted platform. The Provisional/Recommended promotion model would convert quickly into credible differentiation with even two or three independent adopters. Completing the applicability catalog and shipping the Level 1 Planned reduced starter package would directly address the largest adoption-friction objection [S008]. Signing and SBOM are bounded, well-understood work that would move two dimensions at once.

**Threats.** Assistant vendors absorbing governance primitives natively [R11]. Maintainer discontinuity [R01]. A licence-driven procurement rejection at enterprises that filter on OSI approval [F5]. Evidence-base credibility: if independent adopters do not appear, the "empirically informed" framing weakens over time rather than strengthening.

---

## 12. One-to-Three-Year Outlook

*Forecast, not fact. Confidence: Low-Medium.*

**Most likely trajectory.** Continued rapid solo development with strong documentation and continuing publication-reliability friction; a small number of independent adopters; incremental supply-chain hardening; the commercial licensing track remaining nascent. The framework stays useful to its author and to a handful of aligned solo maintainers and small teams, without reaching the plurality of evidence needed for enterprise standardisation.

**Upside.** A second maintainer with merge and release authority joins; three or more independent adopters contribute learning records including at least one overkill finding, which would validate the promotion model empirically; signing and SBOM ship; a reduced starter package lowers onboarding cost. That combination would plausibly move the rating to B within 12–18 months.

**Downside.** Maintainer availability drops; releases stall mid-publication as they have four times already; the applicability catalog stays at 8 of 58; assistant vendors ship native equivalents. Adopters are left maintaining a vendored fork of a governance library they did not design.

**Leading indicators to monitor.** Appearance of a second person with merge rights; publication of SECURITY.md; ratio of fully published to partially published releases over the next 90 days; number of `learning`-labelled issues from non-maintainer accounts; whether an overkill finding is ever recorded; completion rate of the applicability catalog.

**Trigger events that would materially change this assessment.** Any of: a co-maintainer with release authority; a licence change on framework content; a published security incident; a first verifiable independent enterprise adopter; 30 consecutive days without a commit.

---

## 13. Remediation Roadmap

| Priority | Action | Risk reduction | Effort | Owner | Evidence of completion | Dependencies |
| --- | --- | --- | --- | --- | --- | --- |
| **Immediate** | Publish SECURITY.md: private reporting channel, triage commitment, supported-version statement | R02 High → Medium | Low | Maintainer | File present and linked from repo and docs | None |
| **Immediate** | Ship the `agov-mcp` `coordination_dir` fix with a hard error on repository-root misuse | R07 High → Low | Low-Med | Maintainer | Fix in a fully published release; docs limitation removed | Gate 1 approval |
| **Immediate (adopter)** | Pilot scoping: one repo, named governor, pinned vendored tag, identity minimisation, end date | R01, R08 | Low | Eng. leadership | Written pilot charter | None |
| **First 30 days** | Move release-asset contract validation to a pre-tag gate | R04 High → Medium | Medium | Maintainer | Two consecutive tags fully published first time | Release pipeline work |
| **First 30 days** | Legal review of ShareAlike scope, patent-grant absence, and contribution provenance; adopt a DCO | R05 High → Medium | Medium | Legal counsel + maintainer | Written opinion; DCO in CONTRIBUTING | External counsel |
| **First 30 days (adopter)** | SAST, dependency scan, and licence scan on an `agov` checkout | R13 unknown → known | Medium | Adopter security | Scan reports | Source access |
| **31–90 days** | Release signing (Sigstore or GPG) over tags and artifacts, with verification instructions | R03 High → Medium | Medium | Maintainer | Signed release + published verification steps | Key management |
| **31–90 days** | SBOM (CycloneDX or SPDX) generated per release and linked from the Release page | R03; enables CVE triage | Medium | Maintainer | SBOM asset on a release, matched to artifact digest | Build integration |
| **31–90 days** | Complete the adopter applicability catalog beyond 8 of 58 artifacts | R06 High → Medium | High | Maintainer | Catalog covers full inventory | None |
| **3–6 months** | Publish a supported-version and deprecation policy | R04, release maturity | Low | Maintainer | Policy page + observed backport practice | None |
| **3–6 months** | Recruit and publicly record two independent adopters; require one overkill finding before promoting further rules | R06 High → Medium | High | Maintainer | Learning records from non-maintainer accounts | External participation |
| **Longer term** | Establish a second maintainer with merge and release authority; document succession | R01 High → Medium | High | Maintainer | Named co-maintainer in governance.md with observed merges | Trust-building |
| **Longer term** | Design an evaluation of whether governed projects produce measurably better outcomes | AI-Era evaluation 1 → 3 | High | Maintainer + adopters | Published methodology and results | Multi-adopter data |

---

## 14. Open Questions and Evidence Requests

Ranked by decision importance:

1. **Is there any succession or co-maintainer arrangement?** Evidence: `governance.md` contents; project member list with access levels. *Would change: Governance, Operational Sustainability, overall rating.*
2. **Does a SECURITY.md or private disclosure channel exist that this review could not see?** Evidence: repository root listing. *Would change: Security Posture, R02.*
3. **What are the engineering-quality facts?** Evidence: test suite, coverage report, CI configuration, `Cargo.lock`, clippy/lint configuration. *Would change: Engineering NE → score; resolves 11% of unscored weight.*
4. **What are the commercial licence terms, and do they cover the same edition and version under assessment?** *Would change: Legal Confidence, R05.*
5. **Is v2.6.1 fully published, or is the Release information page ahead of reality?** Evidence: the GitLab Releases page asset inventory for v2.6.1. *Resolves F9.*
6. **Contributor and review concentration figures** — commits, merged MRs, and reviews by top contributor over 90 days. *Would change: Community Health confidence, R09.*
7. **Are there any independent adopters?** Evidence: `learning`-labelled issues authored by non-maintainer accounts. *Would change: Ecosystem, R06.*
8. **Does the project's own coordination record set actually satisfy its own gate rules?** Evidence: sampled audit of `docs/coordination/` against the rule library. *Would change: R09, and the credibility of the self-governance claim generally.*
9. **What dependencies does `agov` carry, and what is their advisory status?** *Would change: Supply-Chain, R13.*

---

## 15. Change Log from Prior Assessment

| Field | Previous | Current |
| --- | --- | --- |
| Date | 2026-08-10 | 2026-08-17 |
| Protocol | Not Protocol 2.1 (simulated Grok/Copilot reviews) | Protocol 2.1 |
| Version assessed | v2.5.0 series | v2.6.1 documentation build; v2.5.0 last fully published stable |

The content of the prior assessments was not available to this review, so **no score-by-score or risk-by-risk delta can be produced**. Distinguishing the four required change categories:

- **Actual project change since 2026-08-10:** v2.6.0 was constructed and incompletely published (OCI only, `release:create` failure); a fix was merged to `main` on 2026-08-16; v2.6.1 was Gate 1 approved on 2026-08-17T03:47Z [S007]. Framework content version advanced 2.5.0 → 2.6.1.
- **Newly discovered evidence:** the release-artifacts page's explicit deferral of signing, SBOM, installers, and third-party notices [S004]; the `agov-mcp` fail-open limitation [S009]; the applicability-catalog coverage figure of 8 of 58 [S008].
- **Changed assessment context:** the purpose is now explicitly enterprise adoption, which raises the weight of governance concentration, licence clarity, and supported-version policy relative to a general-purpose review.
- **Changed methodology:** Protocol 2.1 introduces explicit weighting, NE handling, and sampling annotation — the numeric result is not comparable to any prior assessment.
- **Changed confidence without changed capability:** the inability to retrieve GitLab's dynamic pages lowers confidence on several dimensions relative to what an equivalent review with repository access would produce; this reflects the assessment tooling, not a change in the project.

---

## 16. Appendix — Radar Chart Data

Readable list (NE excluded; not plotted as zero): Strategic Strength 3; Governance Maturity 2; Community Health 1; Release Maturity 2; Security Posture 1; Supply-Chain Maturity 2; Operational Sustainability 1; Legal Confidence 2; Ecosystem Strength 1; AI-Era Readiness 2. Not scored: Engineering Maturity (NE), Financial & Organisational Sustainability (NE).

```json
{
  "Strategic Strength": 3,
  "Governance Maturity": 2,
  "Community Health": 1,
  "Engineering Maturity": null,
  "Release Maturity": 2,
  "Security Posture": 1,
  "Software Supply-Chain Maturity": 2,
  "Operational Sustainability": 1,
  "Legal Confidence": 2,
  "Ecosystem Strength": 1,
  "Financial and Organizational Sustainability": null,
  "AI-Era Readiness": 2,
  "_meta": {
    "scale": "0-5",
    "null_means": "NE - not enough evidence; not zero",
    "partial_weighted_result": 1.71,
    "scored_weight_percent": 86,
    "unscored_weight_percent": 14,
    "assessment_date": "2026-08-17"
  }
}
```

---

## 17. Source Register

| ID | Title | Type | Publisher | Date | URL | Retrieved | Used in |
| --- | --- | --- | --- | --- | --- | --- | --- |
| S001 | Repository landing page (commits, branches, tags, releases, creation date, root licence) | Repository metadata | GitLab / J. Päijänen | Created 2026-06-14 | https://gitlab.com/janipaijanen/agents-govern-framework | 2026-08-17 | Metadata, §9.5, §9.7, F-table |
| S002 | Home — value proposition, gates, tooling, cost, evidence statement | Project documentation | GitLab Pages | Build 2026-08-17 | https://janipaijanen.gitlab.io/agents-govern-framework/ | 2026-08-17 | §3, §9.1, §9.4, §11 |
| S003 | Release information — version, source commit, AI-assistance acknowledgement, licences, citation | Project documentation | GitLab Pages | 2026-08-17T05:43:31Z | https://janipaijanen.gitlab.io/agents-govern-framework/reference/release-info/ | 2026-08-17 | §1, §9.2, §9.8, §9.11, F9 |
| S004 | Release artifacts — provenance contract, checksums, CI wiring, deferred work | Project documentation | GitLab Pages | Build 2026-08-17 | https://janipaijanen.gitlab.io/agents-govern-framework/reference/release-artifacts/ | 2026-08-17 | F3, F4, F11, §9.5, §9.6, R03, R04 |
| S005 | Contribute — contribution paths, append-only/GDPR tension, anonymisation, independent-adopter appeal | Project documentation | GitLab Pages | Build 2026-08-17 | https://janipaijanen.gitlab.io/agents-govern-framework/contribute/ | 2026-08-17 | F1, F6, F8, §9.3, R08 |
| S006 | Evaluate — reviewer protocol, identity check, maturity vocabulary | Project documentation | GitLab Pages | Build 2026-08-17 | https://janipaijanen.gitlab.io/agents-govern-framework/evaluate/ | 2026-08-17 | §9.1, §11, methodology |
| S007 | Roadmap — release train, publication failures, portfolio routing | Project documentation | GitLab Pages | Observed 2026-08-17T03:47:48Z | https://janipaijanen.gitlab.io/agents-govern-framework/roadmap/ | 2026-08-17 | F4, F9, §9.5, §15 |
| S008 | Should you adopt? — commitments, wait/skip conditions, privacy and labour checklist | Project documentation | GitLab Pages | Build 2026-08-17 | https://janipaijanen.gitlab.io/agents-govern-framework/docs/start/should-you-adopt/ | 2026-08-17 | F10, §10, §9.8 |
| S009 | agov MCP server — status, known limitation, tool surface | Project documentation | GitLab Pages | Build 2026-08-17 | https://janipaijanen.gitlab.io/agents-govern-framework/docs/adopt/mcp-server/ | 2026-08-17 | F7, §9.11, R07 |
| S010 | Disclaimer — no professional advice, responsibility, tooling limits, no warranty | Project documentation | GitLab Pages | Build 2026-08-17 | https://janipaijanen.gitlab.io/agents-govern-framework/reference/disclaimer/ | 2026-08-17 | §1, §9.8 |
| S011 | Public web search for third-party references to the project | Search result set | Multiple | 2026-08-17 | — | 2026-08-17 | F12, §9.9 |

Sources not retrievable during this assessment, and therefore not assigned IDs: the GitLab Releases page, issue tracker, merge requests, commit history, `README.md`, `CHANGELOG.md`, `agf/docs/governance/governance.md`, and all repository file blobs — all returned client-rendered pages without content.

---

Two closing notes, outside the protocol. First, the biggest single lever on this rating is not technical: a second person with merge and release authority would move Governance, Operational Sustainability, and Community simultaneously, and no amount of documentation work substitutes for it. Second, the C rather than D rests entirely on reversibility — if the exit path is ever compromised (for example, by adopters accumulating governance records they cannot cleanly detach from), that judgment uplift no longer holds.
