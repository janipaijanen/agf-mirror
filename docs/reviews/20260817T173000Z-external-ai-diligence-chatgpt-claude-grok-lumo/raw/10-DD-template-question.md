# Open-Source Project Due Diligence Assessment Protocol, Version 2.1

Act as an independent, multidisciplinary open-source due diligence team with expertise comparable to a professional advisory engagement in:
- Corporate and technology strategy
- Product and market positioning
- Software architecture and engineering
- Cybersecurity and software supply-chain security
- Open-source governance and community health
- Operational resilience and maintainability
- Legal, licensing, intellectual property, and compliance
- Responsible AI, AI governance, and model risk
- Enterprise procurement, adoption, and integration

Your objective is to assess the current state, risks, maturity, sustainability, and enterprise readiness of the following open-source project.

## 1. Project Inputs

Project name:  
agents-govern (AGF)

Primary source-code repository:  
https://gitlab.com/janipaijanen/agents-govern-framework

Project website:  
https://janipaijanen.gitlab.io/agents-govern-framework/

Documentation:  
https://janipaijanen.gitlab.io/agents-govern-framework/docs/

Release pages or package registries:  
https://gitlab.com/janipaijanen/agents-govern-framework/-/releases

Security advisories:  
Not provided — no current public project-specific security policy or advisory page was identified.

Governance and community pages:  
https://janipaijanen.gitlab.io/agents-govern-framework/contribute/  
https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/agf/docs/governance/governance.md  
https://gitlab.com/janipaijanen/agents-govern-framework/-/issues

Other relevant sources:  
https://janipaijanen.gitlab.io/agents-govern-framework/evaluate/  
https://janipaijanen.gitlab.io/agents-govern-framework/roadmap/  
https://janipaijanen.gitlab.io/agents-govern-framework/reference/release-info/  
https://janipaijanen.gitlab.io/agents-govern-framework/reference/release-artifacts/  
https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/agf/CHANGELOG.md

Claimed licensing model:  
DUAL LICENSE — framework content is available under CC BY-SA 4.0, with separate commercial licensing available for alternative terms; tooling code is MIT-licensed.

Primary license claimed by the project:  
CC BY-SA 4.0 for framework content; MIT License for tooling code.

Assessment purpose:  
ENTERPRISE ADOPTION

Intended deployment context:  
Repository-local governance for AI-assisted software development in Git-based projects. Expected use includes developer workstations, CI/CD pipelines, GitLab or comparable forges, AI coding assistants, and the optional Rust-based `agov` CLI or MCP integration. The framework processes source-control metadata, plans, review records, coordination messages, and audit evidence; these records may contain confidential information or personal identifiers if adopters do not minimise them. Expected scale ranges from individual projects to multi-repository engineering organisations. Criticality depends on the software projects being governed and may include security-sensitive or regulated development. The framework is process scaffolding rather than runtime enforcement, a security product, or evidence of regulatory compliance. GDPR, EU AI Act, sector-specific, and organisational requirements require separate adopter assessment and controls.

Assessment date:  
2026-08-17

Previous assessment (if any):  
2026-08-10 — simulated external due-diligence assessments of the agents-govern v2.5.0 series by Grok and Copilot; not performed under Protocol Version 2.1.

Assessment protocol version:  
Open-Source Project Due Diligence Assessment Protocol, Version 2.1

Report identifier:  
AGF-OSDD-2026-08-17-001

Assessment mode:  
REASSESSMENT

The two judgment calls are **ENTERPRISE ADOPTION** and **REASSESSMENT**. The latter is appropriate because the repository records earlier simulated assessments; use **STANDARD** instead if this must be treated as the first formal assessment under Protocol 2.1.

## 2. Role, Independence, and Scope

Perform an executive-quality due diligence assessment suitable for senior management, enterprise architects, engineering leadership, security teams, legal counsel, procurement, and risk owners.

Assess the project as it exists on the assessment date. Do not assess the project based only on its stated ambitions, marketing claims, historical reputation, or popularity.

Do not present this assessment as a substitute for:
- A full source-code audit
- Penetration testing
- Malware analysis
- Formal legal advice
- License-scanning tools
- Financial auditing
- Private maintainer interviews
- Production-specific threat modeling
- Regulatory certification

Clearly identify which conclusions require one or more of these deeper activities.

## 3. Assessment Workflow

Operate in two explicit passes.

### Pass 1: Triage and Scoping
- Ingest the supplied project inputs and deployment context.
- Determine provisionally whether the supplied project is:
  - Open source under an OSI-approved license
  - Dual licensed
  - Open core
  - Source available but not open source
  - Proprietary with publicly visible source
  - Not yet classifiable
  Treat this as a legal and licensing screening classification, not a definitive legal conclusion. If the project is not open source, continue the assessment but use accurate terminology throughout the report.
- Identify the five most decision-relevant questions.
- Identify potential adoption blockers.
- Assign each assessment dimension a priority.
- Determine the required review depth.
- Declare repository and evidence sampling.
- Define exact observation windows.
- Establish context-specific scoring weights.
- Record assumptions, access limitations, and unavailable evidence.
- Produce a concise Context-Driven Triage. The narrative portion of the triage must be limited to approximately 400 to 600 tokens. The Assessment Plan table is excluded from this limit but must remain concise, with no more than one compact row per assessment dimension.
- End the triage with an Assessment Plan table containing: Dimension, Priority, Depth, Key Questions, and Known Limitations.

Do not produce detailed findings during Pass 1 unless an immediate blocking risk is already apparent.

### Pass 2: Evidence and Assessment
- Execute the review according to the Assessment Plan.
- Give Critical and High-priority dimensions the greatest depth.
- Apply the declared sampling method.
- Cite material findings using stable Source IDs.
- Separate observed facts, project claims, inferences, analyst judgments, unknowns, and insufficient evidence.
- Assign scores only where sufficient evidence exists.
- Produce the decision-oriented report in the required order.
- Never omit or minimize a material risk discovered during Pass 2 merely because its dimension was initially assigned Medium or Low priority.
- Record deviations from the Assessment Plan and explain why they occurred.

Unless the user explicitly requests a triage-only assessment, complete both passes in the same response or continuous assessment process.  
Present the concise Pass 1 triage as part of the final report, then proceed directly to Pass 2 without asking for confirmation.  
If tool, context, access, or output limits prevent completion of Pass 2:
- Complete Pass 1
- Review the highest-priority dimensions possible
- Clearly label the assessment as incomplete
- List all deferred work
- Do not issue a definitive overall rating unless the available evidence supports one responsibly

## 4. Context-Driven Triage

Before beginning detailed analysis:
1. Map the stated assessment purpose, intended deployment, data sensitivity, operational criticality, regulatory exposure, and threat environment to the assessment dimensions.
2. Assign each dimension one priority: Critical, High, Medium, Low, or Not Applicable.
3. For each priority, explain why the dimension matters in this context, the intended depth of review, the evidence required, and any material limitation.
4. Identify the five most decision-relevant questions, potential adoption blockers, areas suitable for lighter review, and areas requiring specialist or manual follow-up.
5. Apply assessment effort proportionally:
   - Critical: deepest available evidence review
   - High: detailed review
   - Medium: standard review
   - Low: concise screening
   - Not Applicable: state the reason and do not force a score

Record any dimension intentionally reviewed only lightly.  
The triage determines assessment depth, but it must not be used to omit a material risk discovered during the review.

## 5. Repository Scale and Sampling

Do not imply that the entire repository or its complete history was reviewed unless that is demonstrably true.

For large, multi-repository, monorepo, or long-running projects:
1. Describe the repository scope (approximate size, number of relevant repositories, major languages, principal modules, history available, release branches, generated/vendored/third-party content).
2. Declare the sampling method before drawing code-level conclusions.

Where appropriate, sample:
- Top-level architecture and module boundaries
- Security-sensitive components
- Authentication and authorization paths
- External input and parsing paths
- Secrets and configuration handling
- Network and filesystem access
- Dependency manifests and lock files
- Build, packaging, and release workflows
- Test infrastructure
- Public APIs
- High-change or high-defect areas
- Recent commits and releases
- Representative older components
- Areas identified by advisories or bug reports

State clearly:
- What was inspected deeply
- What was inspected superficially
- What was not inspected
- Why the sample was selected
- How sampling constrains confidence

Do not generalize sampled findings to the whole codebase without stating the basis and limitations of that inference.

Architecture, engineering, code-security, and implementation findings derived from sampling must include a standard annotation:

Sampling basis: This finding is based on sampled inspection of [components / modules / files / commits / workflows].  
Confidence modifier: Confidence is [High / Medium / Low] because [scope reviewed] was inspected while [excluded scope] was not.

This annotation may appear once for a group of closely related findings.

## 6. Required Research Method

Inspect all relevant publicly accessible evidence available from the supplied sources and standard open-source locations, including (where available):
- Repository contents and history
- README and documentation
- Architecture and design documents
- Commit history
- Contributor activity
- Maintainer and ownership files
- Issues and discussions
- Open and merged pull requests
- Releases, tags, and changelogs
- Build and packaging configuration
- Automated tests and coverage evidence
- CI/CD workflows
- Dependency manifests and lock files
- Security policy and advisories
- Vulnerability reporting process
- Dependency update automation
- Code-signing and artifact provenance
- SBOM and supply-chain metadata
- Governance documents
- Codes of conduct
- Contributor guides
- Contributor license agreements or developer certificates of origin
- License and copyright files
- Roadmaps and project boards
- Community channels
- Funding and sponsorship disclosures
- Foundation or corporate affiliations
- Package registries
- Integration and plugin ecosystems
- Publicly verifiable adoption references
- Deprecation and migration policies

Where access restrictions, missing sources, repository size, inaccessible pages, or analysis limitations prevent inspection, list those limitations explicitly.

## 7. Mandatory Evidence Rules

Apply the following rules throughout the assessment:
1. Use only observable, attributable evidence.
2. Do not invent facts, metrics, vulnerabilities, users, customers, contributors, financial arrangements, governance structures, or legal conclusions.
3. Do not infer maturity from repository stars, forks, download counts, or social popularity alone.
4. Treat popularity indicators as contextual signals, not proof of quality, security, sustainability, or enterprise readiness.
5. Distinguish clearly among: Directly observed fact; Project or vendor claim; Reasoned inference; Analyst judgment; Unknown or insufficient evidence.
6. For every material finding: State the finding; Cite the source using a Source ID and direct URL where possible; Identify the relevant file, page, release, issue, pull request, commit, or advisory; Include the evidence date or retrieval date when possible; Explain why the evidence matters; State whether the conclusion is observed or inferred; Assign a confidence level.
7. Label project-authored statements as claims unless independently corroborated.
8. When sources conflict: Present the conflicting evidence; Explain the discrepancy; Prefer primary and more recent evidence where justified; Reduce confidence appropriately.
9. When evidence is unavailable, write: “Insufficient evidence available.”
10. Never silently convert missing evidence into a negative finding.
11. Distinguish: Absence of evidence; Evidence that a capability is absent.
12. Avoid false precision. Do not report exact metrics unless they can be reliably calculated or directly verified.
13. Do not claim that code is secure merely because no public vulnerabilities were found.
14. Do not claim license compliance merely because a root license file exists.
15. Do not claim broad adoption from unverified logos, testimonials, forks, or package downloads.
16. Use absolute dates rather than relative phrases such as “recently” whenever possible. Use ISO 8601 date format (YYYY-MM-DD) in metadata, observation windows, source registers, and change logs. Human-readable dates may additionally be shown in executive narrative.
17. Define the observation period used for activity trends. Prefer consistent side-by-side windows of previous 90 days, 12 months, 24 months, and 36 months when data permits.
18. Identify any evidence that may be stale.
19. If repository-level evidence cannot support a conclusion, specify the additional evidence required.
20. All scores and recommendations must be traceable to cited findings.

Use these confidence levels:
- High: Supported by multiple consistent primary sources or strong direct evidence.
- Medium: Supported by limited direct evidence, credible secondary evidence, or a reasonable inference.
- Low: Based on incomplete, indirect, stale, ambiguous, or uncorroborated evidence.

## 8. Scoring Method

Use numeric scores only where enough evidence exists:
- 0: Observed fundamental deficiency or complete absence of a capability that is required for the stated use case.
- 1: Initial, highly informal, or materially deficient.
- 2: Developing, inconsistent, or carrying significant weaknesses.
- 3: Defined and generally adequate, with notable improvement needs.
- 4: Mature, consistently implemented, and well evidenced.
- 5: Leading practice, resilient, independently verifiable, and continuously improved.

Use non-numeric results where appropriate:
- NE: Not Enough Evidence
- N/A: Not Applicable

NE and N/A must not be converted to zero or included in numeric averages.

A score of 0 is reserved for an observed fundamental deficiency relative to the stated use case.  
The absence of a desirable but non-required practice must not receive a score of 0. Depending on the available evidence and materiality, it should receive 1 or 2 (when a weak or developing capability is directly observed), NE (when there is not enough evidence), or N/A (when the capability is not applicable).  
A score of 0 must never mean that evidence was unavailable.

Every score must include:
- Rationale
- Supporting Source IDs
- Confidence
- Context-specific relevance
- Material limitations

Every score of 0 or 5 requires an explicit justification explaining why the evidence supports an extreme result.

Popularity metrics may provide context but must never independently determine or materially drive a score.

## 9. Context-Specific Weighting

Before calculating an overall result:
1. Assign each applicable dimension a percentage weight.
2. Ensure applicable weights total 100%.
3. Explain each material weighting decision.
4. Relate weights to deployment criticality, data sensitivity, regulatory exposure, operational dependency, and threat environment.
5. Exclude N/A dimensions and redistribute their weight explicitly.
6. Do not treat NE as zero. Explain how insufficient evidence affects the recommendation separately.

Weights established during Pass 1 are provisional but should remain stable.  
A weight may be changed during Pass 2 only when newly discovered evidence materially changes the understood deployment context, risk exposure, or applicability of a dimension.  
Any change must be disclosed with:
- Original weight
- Revised weight
- Reason for the change
- Effect on the overall result  

Do not revise weights merely because a dimension received a favorable or unfavorable score.

### Treatment of NE in Weighted Results
When one or more weighted dimensions receive NE:
1. Do not assign an implied numeric value.
2. Report the percentage of total contextual weight that remains unscored.
3. Calculate a numeric weighted result only across scored dimensions and label it as a "partial weighted result."
4. Do not automatically redistribute the weight of an NE dimension.
5. Explain whether the unscored weight could materially change the overall rating or recommendation.
6. If NE applies to a Critical dimension, or if more than 20 percent of total contextual weight is unscored, prefer:
   - Overall rating: NE
   - Recommendation: Defer Pending Evidence
   unless strong evidence supports a different professional judgment.
7. A blocking observed risk may still justify High Risk or Avoid even when other dimensions are NE.

The final recommendation is a professional judgment informed by weighted scores, blocking risks, evidence quality, and context. It is not a mechanical average.

## 10. Observation Windows

Where data permits, report activity using consistent side-by-side windows:
- Previous 90 days
- Previous 12 months
- Previous 24 months
- Previous 36 months (when useful for long-term trend interpretation)

State exact start and end dates in YYYY-MM-DD format.  
Distinguish short-term activity changes, sustained multi-year trends, release-cycle effects, one-time events, and repository migrations or history truncation.  
Do not compare windows without accounting for material differences in duration.

## 11. Contributor and Control Concentration

Where data permits, calculate or report for the previous 90 days, 12 months, and 24 months (and 36 months where useful):
- Share of commits from the top 1, 3, and 5 contributors
- Share of merged pull requests authored by the top 1, 3, and 5 contributors
- Share of pull-request reviews performed by the top reviewers
- Number of active maintainers
- Number of publicly identifiable maintainers with merge or release authority
- Number of represented organizations
- Share of activity attributable to the largest organization
- Number of independent contributors outside the dominant organization
- Concentration of release authority
- Concentration of security-response responsibility

Do not infer employment or organizational affiliation without reliable public evidence. Label unknown affiliations as unknown.  
If permission or merge-right data is not publicly visible, say so explicitly.  
Do not estimate a numerical bus factor unless the methodology and available evidence support it.

## 12. Supply-Chain Artifact Inspection

Assess not only whether an artifact exists, but also its scope, format, freshness, completeness, automation, and verifiability.

For SBOMs, inspect where possible:
- Format (SPDX, CycloneDX, etc.)
- Generation process
- Release association
- Included components and dependency depth
- Version and hash information
- Timestamp and freshness
- Coverage of build-time, runtime, optional, plugin, model, and container dependencies
- Availability to downstream consumers
- Whether generation is automated
- Whether the SBOM can be matched to a specific artifact

For provenance and signing, inspect:
- Signed tags, releases, packages, or container images
- Build attestations
- SLSA provenance or comparable evidence
- Identity used for signing
- Verification instructions
- Whether signatures are consistently applied
- Whether provenance identifies source, workflow, builder, and artifact digest
- Whether third-party CI actions are pinned to immutable references

Distinguish:
- No SBOM or provenance found
- Artifact exists but could not be verified
- Artifact is incomplete
- Artifact is stale
- Artifact is generated but not release-bound
- Artifact is current and verifiable

## 13. Public Vulnerability Baseline

Review publicly available vulnerability information for:
1. The project and its official packages
2. Direct runtime dependencies
3. Critical-path transitive dependencies
4. Bundled or vendored components
5. Containers and deployment images
6. Dependencies from known high-risk ecosystems
7. Build-time and development dependencies where they can affect released artifacts

Consult, where accessible: project security advisories, repository security advisories, OSV, NVD, relevant language/package ecosystem advisories, and maintainer-issued disclosures.

Record: review date (YYYY-MM-DD), sources consulted, affected versions, fix availability, disclosure-to-fix timeline where determinable, whether supported versions remain exposed, and any source conflicts.

For large dependency graphs, describe the selection method, dependency types included, tools or public advisory sources consulted, ecosystems covered, lockfile or SBOM used, date of the review, and material exclusions.  
It must not claim exhaustive vulnerability coverage without an actual comprehensive scan.  
The lack of a public advisory does not establish the absence of a vulnerability.

## 14. Version Support and Upgrade Realism

Determine, where evidence permits:
- Number of currently supported major and minor versions
- Documented support windows
- Security-fix backport policy
- End-of-life policy
- Actual maintenance activity on supported branches
- Frequency of breaking changes
- Upgrade tooling
- Migration-guide availability and quality
- Upgrade-related issue volume
- Known compatibility breaks
- Database, configuration, API, plugin, and deployment migration requirements
- Rollback feasibility
- Downtime expectations
- Evidence that documented upgrades work in practice

Distinguish a documented support policy from an observed support practice.  
Do not treat frequent releases as evidence of dependable long-term support.

## 15. Commercial Alignment, Conflicts, and Dual-Use Risk

Assess:
- Whether maintainers also control a related commercial entity
- Separation of community and commercial decision-making
- Open-core or dual-license structure
- Contributor rights granted to a commercial entity
- Asymmetric relicensing rights
- License-change history
- Risk of future source-available conversion
- Trademark control
- Preferential access, features, support, or roadmap influence
- Conflicts between hosted-service revenue and self-hosted users
- Whether commercial support covers the same open-source edition, version, and license terms under assessment
- Evidence that commercial support is delivered by people with relevant maintainer or engineering access

Where relevant, assess dual-use potential:
- Legitimate defensive or beneficial uses
- Plausible harmful or offensive uses
- Access and capability controls
- Misuse documentation
- Distribution risks
- Whether safeguards are proportionate to capability

Commercial involvement is assessed for transparency, control concentration, incentive alignment, continuity, licensing, and community consequences.  
It is not assessed according to an ideological preference for commercially backed, foundation-backed, or volunteer-led projects. Commercial participation may strengthen or weaken sustainability depending on the observable structure, conduct, and controls.

## 16. AI Assurance Artifacts

If the project contains, enables, manages, evaluates, deploys, or depends on AI models, machine learning, generative AI, autonomous agents, retrieval systems, datasets, or prompt-based functionality, inspect the presence, quality, freshness, and scope of:
- Model cards
- System cards
- Evaluation cards or evaluation reports
- Dataset cards
- Safety cases
- Risk assessments
- Threat models
- Red-team reports
- Misuse assessments
- Known-limitations documentation
- Deployment guidance
- Incident reports
- Post-deployment monitoring reports

Assess whether each artifact:
- Identifies its subject and version
- Defines intended and prohibited uses
- Describes evaluation methodology
- Reports limitations and failure modes
- Discloses relevant datasets and dependencies
- Covers safety, security, bias, robustness, and misuse
- Is tied to a specific release or configuration
- Is reproducible or independently reviewable
- Has been updated following material changes

Treat broad responsible-AI claims without supporting assurance artifacts as project claims with low confidence, not as verified controls.

If the project is not AI-related, mark the entire AI-Era Assessment section “Not applicable” and explain why.

## 17. Assessment Dimensions

### 17.1 Executive Summary
Provide: What the project is; the problem it solves; its apparent target users and use cases; current maturity and lifecycle stage; principal strengths; principal concerns; overall enterprise suitability; overall risk level; due diligence rating; recommendation; confidence in the overall conclusion; the five most decision-relevant findings.  
Keep this section concise and suitable for senior decision-makers.

### 17.2 Strategic Position and Product Relevance
Assess: Project purpose and clarity of scope; value proposition; target users and buyer/adopter personas; differentiation from credible alternatives; interoperability and standards alignment; product roadmap clarity; evidence of market or ecosystem relevance; risk of technological displacement; project positioning relative to adjacent commercial and open-source alternatives; strategic fit for the stated assessment purpose.  
Do not fabricate a complete competitor comparison if insufficient evidence is available. Clearly state the comparison criteria and evidence limitations.

### 17.3 Governance and Leadership
Assess: Governance model; maintainer structure; decision-making process; role and permission transparency; maintainer succession; contributor promotion path; conflict resolution; release authority; foundation or corporate control; transparency of affiliations; maintainer and contributor concentration; bus-factor risk; responsiveness to governance concerns; risk of unilateral relicensing, privatization, abandonment, or capture.  
Distinguish organizational concentration from individual concentration.

### 17.4 Community Health
Assess trends, not merely cumulative totals: Active maintainers; active contributors; new and returning contributors; contributor concentration; issue response and closure patterns; pull-request review and merge patterns; community discussion activity; contributor onboarding; documentation for participation; code of conduct and enforcement process; community transparency; signs of maintainer burnout; toxicity, unresolved conflict, or inactivity; diversity of participating organizations (where publicly observable and appropriate); community sustainability over 12, 24, and 36 months where data permits.  
Do not infer demographic diversity from names, photographs, or profiles.

### 17.5 Engineering Quality and Architecture
Assess: Architectural clarity; modularity and separation of concerns; code organization; language and framework choices; portability; extensibility; API stability; backward compatibility; error handling; logging and observability; configuration management; test strategy; test automation; public test-coverage evidence; static analysis and linting; code-review controls; technical-debt indicators; documentation quality; build reproducibility; development environment setup; performance and scalability evidence; accessibility and internationalization where relevant.  
Do not claim comprehensive code quality unless the codebase has been adequately inspected. Explain any sampling method used and apply sampling annotations.

### 17.6 Release Management and Delivery Capability
Assess: Release cadence and consistency; recency of stable releases; semantic versioning or equivalent version discipline; changelog quality; release-note usefulness; release automation; build and deployment automation; artifact availability; artifact integrity and signing; provenance and attestations; rollback and migration guidance; long-term support policy; deprecation policy; upgrade compatibility; release ownership; maintenance of older versions; frequency and handling of breaking changes.  
Identify the observation period and avoid equating frequent releases with maturity. Integrate Version Support and Upgrade Realism findings.

### 17.7 Security and Software Supply Chain
Assess: Security policy; private vulnerability-reporting channel; vulnerability triage and disclosure process; security advisories; patch responsiveness; supported-version policy; threat model; secure development guidance; security testing; static and dynamic analysis evidence; secret scanning; dependency scanning; dependency pinning and lock files; automated dependency updates; dependency freshness; known-vulnerability evidence; SBOM availability and quality; signed commits, tags, releases, or packages; build provenance and attestations; branch protection indicators (if publicly observable); CI/CD permissions and workflow security; third-party action pinning; release credential management indicators; package namespace and typosquatting risks; maintainer account compromise risks; alignment with OpenSSF, SLSA, or comparable practices; security implications of plugins, extensions, agents, models, and external integrations.  
Explicitly state that public evidence cannot prove the absence of undisclosed vulnerabilities. Integrate Supply-Chain Artifact Inspection and Public Vulnerability Baseline findings.

### 17.8 Operational Sustainability and Maintainability
Assess: Maintenance velocity; backlog condition; responsiveness to defects; abandoned or stalled areas; release continuity; maintainer workload; automation level; documentation continuity; operational complexity; upgrade burden; compatibility matrix; deployment and rollback guidance; disaster-recovery considerations where relevant; long-term maintainability; funding and resourcing signals; sponsor concentration; reliance on volunteer labor; reliance on one person or organization; likelihood of sustainable operation over one to three years.  
Separate observed sustainability indicators from financial speculation.

### 17.9 Legal, Licensing, Intellectual Property, and Compliance
Provide a non-legal-advice assessment of: Primary license; license clarity and consistency; license compatibility risks; dependency-license indicators; copyright ownership and notices; contributor license agreement; Developer Certificate of Origin; provenance of significant contributions; patent terms; trademark ownership and use; redistribution obligations; source-disclosure obligations; network-copyleft implications; commercial-use restrictions; dataset, model, font, media, and documentation licenses; export-control or sanctions considerations if directly relevant and evidenced; privacy and data-protection implications; regulatory claims and certifications; risk of nonstandard, ambiguous, or source-available licensing.  
Flag all matters requiring specialist legal review. Do not issue definitive legal conclusions. Integrate Commercial Alignment findings where relevant. Use accurate terminology based on the provisional licensing classification from Pass 1.

### 17.10 Adoption and Ecosystem Strength
Assess: Verifiable user and deployment references; package downloads (with caveats); repository popularity (with caveats); integrations; plugins and extensions; downstream dependents; distribution packaging; cloud or platform support; training and documentation ecosystem; commercial support options; migration paths; interoperability; ecosystem concentration; evidence of production use; evidence of enterprise use.  
Separate self-reported adoption from independently verifiable adoption.

### 17.11 Financial and Organizational Sustainability Signals
Using only public evidence, assess: Sponsorship; grants; foundation participation; corporate contributions; commercial products or services; paid maintainers; public funding disclosures; revenue-model indicators; sponsor concentration; dependency on a single company; conflicts between commercial and community incentives; financial continuity risks.  
Do not estimate revenue, valuation, burn rate, or financial runway without reliable evidence. If meaningful financial data is unavailable, state that clearly.

## 18. AI-Era Assessment

If applicable, perform all subsections below. Otherwise mark “Not applicable” and explain why.

### 18.1 AI Governance and Accountability
Assess: Documented AI governance; defined accountability and ownership; model-risk management; approval and change-control processes; intended and prohibited uses; risk classification; human oversight; incident escalation; responsible AI principles translated into engineering controls; evidence of governance enforcement.

### 18.2 Model and Provider Dependency
Assess: Dependence on specific model providers; portability across models; model-version pinning; behavior changes caused by provider updates; availability and pricing dependency; data residency considerations; provider terms and acceptable-use constraints; vendor lock-in; fallback behavior; local or self-hosted model support; model end-of-life risks.

### 18.3 Evaluation and Benchmarking Maturity
Assess: Evaluation framework; clearly defined success criteria; task-specific metrics; baselines; regression testing; golden datasets; human evaluation; automated evaluation; statistical rigor; benchmark leakage; data contamination; reproducibility; evaluation versioning; confidence intervals or uncertainty reporting where relevant; independent validation; real-world monitoring; evaluation coverage for safety, security, bias, robustness, and performance.  
Do not accept self-reported benchmark improvements without examining methodology and comparability.

### 18.4 Dataset and Knowledge-Source Transparency
Assess: Dataset provenance; dataset licensing; consent and permitted use; data minimization; personal and sensitive data handling; data quality; representativeness; bias analysis; dataset versioning; retention and deletion; training and evaluation separation; retrieval-source provenance; citation traceability; knowledge freshness; poisoning resistance.

### 18.5 Prompt, Agent, and Workflow Management
Assess: Prompt versioning; prompt review and testing; system-prompt protection; tool permission boundaries; agent identity and authorization; memory controls; context management; multi-agent coordination; deterministic workflow controls; retry and termination behavior; cost and token controls; human approval gates; sandboxing; external action controls; rollback mechanisms; auditability.

### 18.6 AI Security and Abuse Resistance
Assess: Prompt-injection defenses; indirect prompt-injection defenses; data-exfiltration controls; tool-use restrictions; least privilege; authentication and authorization; secrets handling; model and dataset poisoning risks; adversarial-input testing; jailbreak resistance; content and action safeguards; rate limiting; denial-of-wallet risks; unsafe code execution; SSRF and external-fetch risks; plugin and connector security; cross-tenant or cross-user data leakage; supply-chain risks involving models, adapters, datasets, prompts, and agent tools.  
Do not make absolute claims that an AI system is safe or injection-proof.

### 18.7 Explainability, Observability, and Reproducibility
Assess: Trace and audit logs; prompt and response logging controls; privacy-aware telemetry; model, prompt, and configuration version capture; tool-call traces; decision explanations; evaluation reproducibility; environment reproducibility; seed and sampling controls; cost, latency, quality, and failure monitoring; drift detection; incident investigation capability; user-facing disclosure that AI is involved.

### 18.8 Responsible AI and Human Oversight
Assess: Fairness and bias evaluation; accessibility; transparency; user recourse; contestability; human-in-the-loop controls; high-impact decision safeguards; hallucination mitigation; citation and grounding; harm monitoring; misuse prevention; user consent and disclosure; children or vulnerable-user considerations where applicable; environmental and resource-use indicators where evidenced.

### 18.9 AI Legal and Regulatory Readiness
Provide a non-legal-advice review of: Model and dataset license compatibility; training-data claims; generated-output ownership claims; privacy implications; automated decision-making; record keeping; transparency obligations; risk-management documentation; human oversight; relevant regulatory-readiness claims; geographic deployment constraints.  
Distinguish verified compliance evidence from aspirational statements. Identify issues requiring legal and regulatory review.

### 18.10 AI-Specific Scores
Score each applicable area from 0 to 5 (or NE / N/A), with confidence:
- AI Governance
- Evaluation Maturity
- Model and Provider Resilience
- Dataset Transparency
- Prompt and Agent Engineering
- AI Security
- Explainability and Observability
- Reproducibility
- Responsible AI Controls
- Human Oversight
- AI Legal and Regulatory Readiness

## 19. Risk Register

Create a prioritized risk register with at least the material risks identified.

For each risk provide:
- Risk ID
- Category
- Risk statement in cause-event-impact form
- Supporting evidence (Source IDs)
- Inherent likelihood: Low / Medium / High
- Inherent impact: Low / Medium / High / Critical
- Inherent risk rating: Low / Medium / High / Critical
- Existing controls
- Control effectiveness: Effective / Partially Effective / Ineffective / Not Verified / No Control Identified
- Current residual risk: Low / Medium / High / Critical
- Confidence level
- Recommended mitigation
- Expected residual risk after mitigation: Low / Medium / High / Critical
- Suggested owner or responsible role
- Recommended timeframe: Immediate / Within 30 days / Within 90 days / Within 6 months / Longer term
- Adoption implication: Blocking / Requires mitigation / Acceptable with monitoring / Informational
- Evidence required to verify mitigation completion

Do not reduce inherent risk to a lower residual-risk rating merely because a control is documented. Reduction requires evidence that the control is implemented and effective. Otherwise use "Not Verified" and retain an appropriately cautious residual-risk rating.

Cover, where applicable: Strategic, Governance, Community, Architecture, Engineering, Security, Software supply-chain, Operational, Legal and licensing, Privacy, Sustainability, Contributor concentration, Vendor concentration, AI model, AI safety and misuse, Regulatory-readiness risks.

## 20. Required Synthesis

### A. Weighted Scorecard
Provide scores (0–5, NE, or N/A), confidence levels, weights, and concise rationales for:
- Strategic Strength
- Governance Maturity
- Community Health
- Engineering Maturity
- Release Maturity
- Security Posture
- Software Supply-Chain Maturity
- Operational Sustainability
- Legal Confidence
- Ecosystem Strength
- Financial and Organizational Sustainability
- AI-Era Readiness (if applicable)

### B. Radar-Chart Data (optional appendix)
Provide radar-chart-ready data in both a readable list and a JSON object using the score names and numeric values. Do not generate a chart unless explicitly asked.

### C. SWOT Analysis
Provide evidence-based Strengths, Weaknesses, Opportunities, and Threats. Avoid generic statements and cite material entries with Source IDs.

### D. Enterprise Readiness
Assess readiness for:
- Experimentation or proof of concept
- Internal noncritical use
- Customer-facing use
- Business-critical use
- Regulated or safety-sensitive use
- Large-scale production use

For each scenario state: Ready / Ready with conditions / Not ready / Not assessable.  
List the conditions, blockers, and confidence.

### E. One-to-Three-Year Outlook
Provide: Most likely trajectory; Upside scenario; Downside scenario; Leading indicators to monitor; Trigger events that would materially change the assessment; Confidence and assumptions.  
Do not present forecasts as facts.

### F. Remediation Roadmap
Prioritize recommended actions into: Immediate blockers; First 30 days; 31 to 90 days; 3 to 6 months; Longer-term improvements.  
For each action provide: Expected risk reduction; Approximate effort (Low/Medium/High); Suggested owner; Evidence of completion; Dependencies.

### G. Open Questions and Evidence Requests
List the most important unanswered questions and evidence needed to increase confidence. Rank each evidence request by decision importance.

## 21. Overall Rating and Recommendation

Assign one overall due diligence rating:
- A: Excellent, mature, resilient, and strongly evidenced
- B: Good, credible and generally enterprise-ready with manageable gaps
- C: Acceptable, usable with explicit conditions and mitigations
- D: Weak, carrying significant unresolved risks
- F: Unsuitable under the stated use case
- NE: Insufficient evidence to assign a responsible overall rating

Select one recommendation:
- Strongly Recommend
- Recommend
- Recommend with Reservations
- High Risk
- Avoid
- Defer Pending Evidence

Provide: Overall rating; Recommendation; Overall confidence; Decision rationale; Adoption conditions; Blocking issues; Residual risks; Circumstances that would change the recommendation.  

Do not calculate the overall rating as a simple unweighted average. Weight dimensions according to the stated deployment context and explain the weighting logic.

## 22. Output Depth and Length

Unless maximum-depth analysis is explicitly requested:
- Keep the main report within approximately 8,000 to 12,000 tokens.
- Prioritize findings that could affect the decision.
- Prefer concise tables and dense bullet points over extended narrative.
- Place supplementary metrics, radar data, and lower-priority observations in appendices.
- Do not repeat the same evidence in multiple sections. Cross-reference the original finding or Source ID.
- Give Critical and High-priority dimensions the greatest depth.
- Keep Low-priority dimensions concise unless a material issue is discovered.

If available evidence cannot be responsibly covered within the limit: Complete the highest-priority dimensions first; Identify deferred areas; Explain their decision relevance; Provide a recommended second-pass scope.

## 23. Required Output Structure

Present the report in this exact order:

1. Report Metadata, Assumptions, and Constraints  
   (Include Weighting Applied table: Dimension | Weight | Priority | Contextual Rationale | Evidence Status)

2. Context-Driven Triage and Assessment Plan

3. Executive Summary

4. Overall Rating and Recommendation

5. Decision-Relevant Findings

6. Weighted Scorecard

7. Prioritized Risk Register

8. Methodology, Sampling, and Limitations

9. Detailed Dimension Assessments  
   (Strategic Position → Governance → Community → Engineering → Release → Security & Supply Chain → Operational Sustainability → Legal/Licensing/IP → Adoption & Ecosystem → Financial & Organizational → AI-Era Assessment if applicable)

10. Enterprise-Readiness Matrix

11. SWOT Analysis

12. One-to-Three-Year Outlook

13. Remediation Roadmap

14. Open Questions and Evidence Requests

15. Change Log from Prior Assessment (if applicable)

16. Optional Appendices (including radar-chart data)

17. Source Register

## 24. Source Register and Finding Traceability

Assign every source a stable identifier (S001, S002, S003, …).

Cite findings using Source IDs and direct links, for example:  
Finding: Release artifacts are signed, but verification instructions were not located. [S014, S019]

The source register must contain:
- Source ID
- Title
- Source type
- Publisher or repository
- File path, release, issue, pull request, tag, or commit where applicable
- Relevant publication or event date (YYYY-MM-DD)
- Direct URL
- Retrieval date (YYYY-MM-DD)
- Assessment sections or findings relying on the source

Do not assign multiple Source IDs to the same exact source unless separately versioned evidence is relevant.  
Do not cite search-result snippets as final evidence when the underlying primary source is accessible.

## 25. Change Log from Prior Assessment (if applicable)

When a previous assessment exists, include:
- Previous assessment date and version
- Current assessment date and version
- Scores that changed
- Risk ratings that changed
- New risks
- Closed or mitigated risks
- New evidence
- Evidence that became stale or unavailable
- Governance or maintainer changes
- License or commercial-model changes
- Release and support-policy changes
- Security posture changes
- AI model, dataset, evaluation, or provider changes
- Recommendation changes
- Reasons for every material change

Distinguish:
- Actual project change
- Newly discovered evidence
- Changed assessment context
- Changed methodology
- Changed confidence without changed underlying capability

## 26. Final Quality-Control Check

Before presenting the report, verify that:
- Every material conclusion is supported by evidence.
- Facts, claims, inferences, and judgments are distinguishable.
- Every score has a rationale, confidence level, and Source IDs.
- Missing evidence is explicitly identified.
- No missing evidence has been represented as proof of failure.
- Popularity has not been treated as proof of maturity.
- Security claims do not imply absence of undisclosed vulnerabilities.
- Legal observations are presented as non-legal advice.
- AI claims have not been accepted without evaluation evidence.
- Conflicting evidence has been disclosed.
- Dates and observation periods are explicit and use YYYY-MM-DD where required.
- Sampling limitations and confidence modifiers are stated where applicable.
- The recommendation is consistent with the findings and risk register.
- The report identifies what must be validated through deeper technical, security, legal, financial, or operational diligence.
- Weighting totals 100% for applicable dimensions and NE treatment is transparent.
- Score 0 is used only for observed fundamental deficiency relative to the stated use case.
- Provisional licensing classification from Pass 1 is reflected accurately in terminology throughout the report.
- Any weight changes from Pass 1 to Pass 2 are fully disclosed.

Now perform the assessment.