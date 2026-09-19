**Classification of the identified risks**

Here is a clean separation of the risks from the AGF due-diligence report (R01–R07), plus a short note on related themes that appeared in the assessment.

### Risks that are largely **specific to this project** (agents-govern / AGF)

These arise from observable characteristics of *this* repository, its maintainer structure, license choices, or recent operational history:

| Risk ID | Statement (short) | Why project-specific |
|---------|-------------------|----------------------|
| **R01** | Single human holds all gate and release authority; unavailability stalls the project | Extreme concentration (one named individual as sole human governor, copyright holder, and release authority) is a direct observation about AGF, not inherent to every AI-governance framework. |
| **R02** | Heavy reliance on maintainer self-use; independent validation largely absent | The project itself publicly acknowledges this as its strongest current weakness and actively solicits independent adopters. This is a current-state evidence gap of *this* corpus. |
| **R05** | Dual-license commercial path may create future friction or preferential terms | The explicit dual structure (CC BY-SA 4.0 framework content + commercial alternative; MIT tooling) is a concrete choice of this project. |
| **R06** | Recent incomplete publication (v2.6.0) indicates process brittleness | Concrete incident in AGF’s own release pipeline (tag/OCI published, GitLab Release + package failed; corrected in v2.6.1). |

These four are the clearest “this project, right now” risks. They would look different (or disappear) if the project added maintainers, accumulated independent evidence, changed licensing, or hardened its release automation.

### Risks that are **broader** (apply to many AI-governance frameworks, tools, or services)

These are structural or category-level issues that AGF shares with a large class of similar offerings (process frameworks, policy-as-code, agent-governance toolkits, audit-trail systems, human-in-the-loop scaffolds, etc.):

| Risk ID | Statement (short) | Why broader |
|---------|-------------------|-------------|
| **R03** (process-gate part) | Process gates can be bypassed by human error / lack of technical enforcement | Almost every non-runtime, non-cryptographic AI-governance framework ultimately relies on human discipline + CI evidence rather than a hard technical root of trust. The missing public SECURITY.md is project-specific, but the deeper “process-only enforcement” problem is category-wide. |
| **R04** | Append-only identity-bearing audit trail conflicts with erasure rights (e.g. GDPR Art. 17) | Any governance system that deliberately keeps an immutable, reconstructible decision log containing personal or role identifiers faces the same tension. This is a design-class issue, not unique to AGF. |
| **R07** | Technological displacement by forge-native AI governance features or stronger enforcement frameworks | Competitive/strategic risk for the whole niche of third-party AI-coding governance tools. Platform vendors (GitLab, GitHub, etc.) or more enforcement-oriented control-plane designs can absorb the value proposition. |

### Mixed / hybrid risks

- **R03 overall**: The *absence of a public vulnerability-reporting channel* is currently project-specific (no SECURITY.md located). The *possibility that human process gates fail* is broader.
- Early-stage open-source AI-governance projects commonly show high maintainer concentration and limited independent evidence (R01/R02 pattern). AGF is an especially clear instance, but the pattern itself is not unique.

### Summary view

- **Project-specific (AGF today)**: R01, R02, R05, R06, and the missing security-reporting channel portion of R03.  
- **Broader category risks** (AI governance frameworks / tools / services in general): the process-only enforcement problem, the append-only audit-trail vs. erasure-rights tension, and competitive displacement by platforms or stronger control-plane designs (R03 process part, R04, R07).

This distinction matters for remediation priority: the project-specific items can be addressed by AGF’s maintainers or by an adopter’s contractual/operational mitigations; the broader ones require either design changes that many frameworks struggle with, or explicit acceptance that the chosen architecture class carries those residual risks.