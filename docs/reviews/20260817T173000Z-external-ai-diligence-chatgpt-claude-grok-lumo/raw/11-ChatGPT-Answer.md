# Open-Source Project Due Diligence Assessment

## 1. Report Metadata, Assumptions, and Constraints

| Field | Value |
|---|---|
| Report ID | AGF-OSDD-2026-08-17-001 |
| Project | Agents Govern Framework (AGF) |
| Assessment purpose | Enterprise adoption and procurement due diligence |
| Assessment mode | Reassessment against Protocol 2.1 |
| Assessment date | 2026-08-17 |
| Evidence cut-off | 2026-08-17, Europe/Helsinki |
| Stable version assessed | v2.6.1 |
| Immutable source commit | `f780ac0ea37061e0ad3ee6fb1f00c6958b996fc0` |
| Live documentation snapshot | Development documentation built 2026-08-17 at commit `fb6bee8e3912d362ec6444521d2007062d1d62b6` |
| Analyst posture | Independent, evidence-led, enterprise-risk oriented |
| Overall risk | **High** |
| Recommendation | **Defer enterprise production adoption; permit only a controlled, low-risk pilot after named blockers are remediated or contractually controlled.** |

### Assumptions and interpretation

1. “AGF” means the public GitLab project, its v2.6.1 release artifacts, and the public documentation site—not an unobserved private support offering.
2. The source tag is the authoritative basis for code and release assessment. The live site is assessed separately because it was rebuilt from a later development commit. The site’s [release information](https://janipaijanen.gitlab.io/agents-govern-framework/reference/release-info/) identifies v2.6.1 and the same release commit.
3. “Enterprise-ready” means suitable for governed production use with defensible licensing, continuity, security reporting, release integrity, operational support, and auditable controls. It does not mean “interesting,” “well documented,” or “popular.”
4. A project statement is recorded as a claim unless independently corroborated. Repository inspection, API responses, reproducible tool output, and verified artifact hashes are observations. Predictions and consequence statements are analyst inferences or judgments.
5. The project is only about 67 days old. The requested 90-day, 12-, 24-, and 36-month windows therefore contain essentially the same history. Long-horizon conclusions are necessarily low-confidence.
6. This is not legal advice, a penetration test, a formal source-code audit, or a certification. Licensing conclusions are screening judgments requiring counsel.

### Weighting model

The assessment uses a 0–5 ordinal scale: 0 absent or materially unsafe; 1 initial/critical weakness; 2 developing; 3 defined and substantially usable; 4 managed and independently evidenced; 5 leading, resilient, and repeatedly validated. “NE” means insufficient public evidence and is excluded from the denominator rather than silently scored zero.

| Dimension | Weight | Criticality rationale |
|---|---:|---|
| Strategic fit | 5% | Important, but compensable in a pilot |
| Governance | 15% | Critical to the framework’s central promise |
| Community | 6% | High continuity and independence relevance |
| Engineering | 14% | Critical because adopters run tooling in control workflows |
| Release management | 11% | High supply and upgrade-path relevance |
| Security | 15% | Critical for enterprise distribution and control evidence |
| Operational readiness | 10% | High relevance to supportability and audit operations |
| Legal and licensing | 10% | Critical procurement gate |
| Adoption and ecosystem evidence | 8% | High validation and product-risk relevance |
| Financial/organizational sustainability | 2% | Relevant, but no public evidence supports a score |
| AI-era readiness | 4% | Material to an AI-governance framework, but not a shipped-model assessment |

## 2. Context-Driven Triage and Assessment Plan

AGF is unusual: its core deliverable is a governance framework whose credibility depends on governance, evidence integrity, and enforceability more than conventional feature breadth. The public project is very young, extremely active, and controlled by one identifiable human with agent and bot assistance. Pass-one triage therefore treated governance, licensing, release provenance, control enforceability, historical ledger validity, and succession as the dominant risk paths. Popularity metrics were excluded from maturity scoring. The primary evidence anchor was the immutable v2.6.1 tag; live documentation was treated as a later development snapshot, preventing same-day documentation drift from being mistaken for release content.

The strongest early signals were real engineering investment—five Rust crates, schema validators, negative tests, pinned container base, CI scanning, checksummed artifacts, and explicit provenance records. Counter-signals were equally material: no independent human maintainer, no signed commits or tags, no SBOM, multiple incomplete publications, a current dependency advisory, advisory gates without authenticated identities, and hundreds of historical coordination and plan records rejected by current validators. A direct license contradiction between the public license summary and Cargo/OCI metadata became the primary procurement blocker. The assessment therefore deepened around independent artifact verification, Git history concentration, schema enforcement boundaries, licensing paths and contribution rights, issue/branch operational state, dependency exposure, and release recovery evidence.

| Workstream | Evidence selected | Purpose |
|---|---|---|
| Identity and scope | Tag, commit, `agf/VERSION`, live release page | Prevent version ambiguity |
| Governance and continuity | Git authors, protected branches, roles/gates, schemas | Test authority, independence, and enforceability |
| Engineering/security | Rust workspace, tests, CI, lockfile, OSV, container | Test implementation and supply-chain controls |
| Release integrity | Package downloads, hashes, provenance, binary self-report | Independently verify release claims |
| Operational evidence | Validators, pending acknowledgements, handoffs, rule drift, branches | Measure control debt, not just documented process |
| Legal | Root and package licenses, Cargo metadata, OCI labels, contribution terms | Establish procurement classification |
| Adoption | Learning records, published limitations, prior assessments | Separate dogfood and claims from independent use |

## 3. Executive Summary

AGF is a promising, unusually transparent early-stage governance framework with a credible technical nucleus. It defines human decision gates, role separation, coordination records, schemas, policy rules, and an `agov` Rust toolchain. The v2.6.1 file artifacts were independently downloaded, checksum-verified, matched to the tagged commit, and the released binary correctly reported its version and clean source identity. The project’s documentation is candid that AGF is not a root of trust, that some gates remain advisory, and that its strongest evidence currently comes from a single developer/governor environment. Those disclosures increase confidence in the project’s honesty, not its maturity.

The project is not ready for unrestricted enterprise production adoption. Four blockers drive that conclusion.

First, licensing is not reliably classifiable. The root [licence text](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/v2.6.1/LICENCE) says framework content is CC BY-SA 4.0 or commercial and tooling code is MIT. Yet the tagged Rust workspace [declares](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/v2.6.1/tooling/rust/Cargo.toml) `CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial`, and the container metadata repeats that expression. Source files generally lack SPDX headers, and current contribution guidance does not clearly grant rights needed for commercial relicensing. This is a critical procurement and redistribution ambiguity.

Second, governance and continuity remain single-person dependent. Normalizing the author aliases shows approximately 96.0% of all 4,180 commits are attributable to the owner and 4.0% to the project bot; no independent human committer was observed. The same human appears to hold governor, maintainer, and release authority. Main-branch protection exists, but unauthenticated public API access did not expose approval settings or protected-tag controls. There is no demonstrated succession, independent maintainer quorum, or separation of release authority.

Third, the framework’s strongest controls are not cryptographically or identity bound. Gate 1 and Gate 2 are described as human decisions, but public schemas largely validate record structure. A coordination message’s `from` value is a non-empty string, not an authenticated identity; Gate 1 approval is not constrained to a verified governor. The project itself records that local technical gate enforcement, authenticated authorship, and tamper-evident authenticity remain unresolved. This is acceptable for a social governance aid but not sufficient as an enterprise control system without external enforcement.

Fourth, operational evidence does not yet support end-to-end audit assurance. The released tool validated 2,309 coordination messages but rejected 333 other coordination files and rejected 12 of 163 plan files under current schemas. Many failures appear to be historical pre-contract records or documentation caught by broad globs; the project advises validating changed files. Even so, current tooling cannot machine-validate the complete retained history under a version-aware model, and operational commands ignore invalid records. At the review cut-off there were also 81 pending handoffs, eight unresolved required acknowledgements, 51 copied-rule drift findings, and 547 branches.

Security and release practices are developing rather than absent. CI includes Clippy with warnings denied, workspace tests, `cargo-deny`, and Trivy; the runtime container is minimal, digest-pinned, and non-root. But there is no current root security policy for the Rust project, no public security response SLA, no SBOM, no signed tag/commit/artifact, and one locked transitive package—`anyhow 1.0.102`—was affected by [RUSTSEC-2026-0190](https://rustsec.org/advisories/RUSTSEC-2026-0190.html). Reachability was not established. Custom provenance is useful but is self-issued metadata rather than an independent signed attestation.

The result is **2.1/5 on 98% scored weight**, with 2% not evaluated for lack of organizational/financial evidence. A bounded pilot could be justified after the licensing conflict, security intake, and dependency issue are resolved, provided the enterprise treats AGF as a workflow aid—not as a tamper-proof approval system—and supplies its own identity, branch, signing, retention, and review controls.

## 4. Overall Rating and Recommendation

**Overall rating: 2.1/5 — Developing, high-risk for enterprise production use.**

**Recommendation: Defer production adoption and procurement approval. Permit a time-boxed, non-regulated pilot only after the Critical legal issue and immediate security hygiene items are closed.**

A pilot should be approved only with all of these conditions:

1. Counsel or the copyright owner issues an internally consistent, path-specific licensing position for Rust tooling, images, documentation, schemas, examples, and contributions.
2. The dependency advisory is remediated or formally risk accepted after reachability analysis; a current `SECURITY.md`, reporting channel, supported-version statement, and response targets are published.
3. The adopter pins an immutable release, independently verifies hashes, produces its own SBOM, mirrors artifacts, signs the internal promotion, and scans source/container artifacts.
4. Forge identity, protected branches/tags, mandatory reviews, CODEOWNERS, and enterprise SSO provide the actual authority boundary. AGF text fields must not be treated as proof of identity.
5. The pilot excludes regulated approvals, safety-critical changes, or claims of non-repudiation. It measures false positives, missed controls, process latency, exception rates, and audit reconstruction.
6. An exit plan retains evidence in an enterprise-controlled store and does not depend on the upstream project remaining available.

Production reconsideration should require: resolved licensing and inbound rights; an independent maintainer/backup release authority; signed and SBOM-bearing releases; a version-aware validation story for historical records; at least five genuinely independent adopters and 50 governed changes with published outcome data; and a supported-version/EOL policy.

## 5. Decision-Relevant Findings

### F1 — Tooling licensing contradicts the public summary (Critical)

**Fact:** The root `LICENCE` says tooling code under `tooling/` is MIT, while the tagged Rust workspace and OCI label declare `CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial`. The package also carries both [CC BY-SA](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/v2.6.1/agf/LICENSE) and [MIT](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/v2.6.1/agf/LICENSE-MIT) texts. GitLab’s project API returned no detected license, plausibly because the root file is named `LICENCE` and because the mapping is non-standard.

**Inference:** A reasonable redistributor cannot determine solely from the release whether compiled Rust tooling is MIT, CC BY-SA/commercial, or governed by file-specific metadata. The container label points away from MIT.

**Judgment:** This is an enterprise procurement blocker until a consistent license map, package metadata, SPDX headers, and commercial-relicensing basis are published. CC recommends against applying CC licenses to software, and CC BY-SA is not on the [OSI approved license list](https://opensource.org/licenses); therefore AGF as distributed should not presently be represented as wholly OSI-approved open source. See the [Creative Commons software guidance](https://creativecommons.org/faq/#can-i-apply-a-creative-commons-license-to-software).

### F2 — Control and continuity are concentrated in one human (High)

**Fact:** The repository contains 4,180 commits over about 67 days. Author-string normalization attributes 4,014 (96.0%) to the owner’s names/email aliases and 166 (4.0%) to the project bot. No independent human committer was observed. Public learning data identifies a one-person team and the same human governor. Main is protected, but merge-approval and protected-tag evidence was unavailable without authorization.

**Inference:** Maintenance, governance interpretation, release approval, and commercial-license authority appear to share a single failure domain.

**Judgment:** An enterprise cannot rely on project continuity, conflict resolution, or independent challenge until a second accountable maintainer and succession/release procedures are demonstrated.

### F3 — Gates are structured social controls, not an authenticated control plane (High)

**Fact:** Public role documentation assigns Gate 1 and Gate 2 to a human governor. The [coordination schema](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/v2.6.1/agf/schemas/coordination-message.schema.json) accepts a string author, and the [Gate 1 plan schema](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/v2.6.1/agf/schemas/gate1-plan.schema.json) does not cryptographically bind approval to an authorized governor. The [changelog](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/v2.6.1/agf/CHANGELOG.md) records unresolved authorship, tamper-evidence, and local enforcement findings.

**Inference:** A malicious or mistaken contributor with repository write access can create structurally valid assertions that are not independently authenticated. Schema-valid is not authority-valid.

**Judgment:** AGF can organize governance evidence, but enterprises must anchor identity, review, and change authority in their forge/IAM/signing systems.

### F4 — Current validators do not establish whole-history validity (High)

**Fact:** Against the v2.6.1 tree, `agov coord validate docs/coordination/*.md` produced 2,309 valid and 333 invalid files; `agov plan validate docs/plans/*.md` produced 151 valid and 12 invalid files. Failures were mostly historical format/status differences or non-message documentation. The project’s [validator guidance](https://janipaijanen.gitlab.io/agents-govern-framework/docs/advanced/contracts-and-validators/) says historical repositories should validate changed files.

**Inference:** Incremental enforcement may prevent new drift, but the retained corpus lacks a single version-aware machine-verifiable interpretation. Operational commands omit invalid files, which can make an apparently clean status incomplete.

**Judgment:** Historical evidence should be schema-versioned, migrated with preserved hashes, or explicitly classified as legacy/unverified. Until then, AGF’s audit trail is readable but not wholly machine-assured.

### F5 — Release integrity is improving, but not enterprise-grade (High)

**Fact:** v2.6.1 framework and x86_64 artifacts were independently downloaded and matched their SHA-256 sidecars and provenance metadata; the binary self-reported the expected commit and clean state. The project publishes detailed [release-artifact guidance](https://janipaijanen.gitlab.io/agents-govern-framework/reference/release-artifacts/). However, 0 of 4,180 commits and 0 of 39 tags were cryptographically signed; 17 tags were lightweight; no SBOM was found. v2.6.0’s GitLab release/package publication failed and v2.6.1 repaired it. Earlier v2.4.x releases also required publication/fixture corrections.

**Inference:** Self-issued hashes and provenance detect accidental corruption when distributed independently, but same-channel metadata does not authenticate publisher identity or establish build provenance.

**Judgment:** Release consumption requires an enterprise-controlled verification/promotion pipeline until signed attestations, SBOMs, and reproducible-build evidence are standard.

### F6 — Security process is incomplete and a current advisory exists (High)

**Fact:** CI runs workspace tests, Clippy with warnings denied, `cargo-deny`, and Trivy. The container is digest-pinned and non-root. No current root `SECURITY.md` was found; a legacy file describes a pre-Rust threat surface. OSV/RustSec identified `anyhow 1.0.102` as affected by RUSTSEC-2026-0190, fixed in 1.0.103. It is transitive through `jsonschema`; direct reachability of the vulnerable method was not established.

**Inference:** Exploitability may be low or absent, but dependency monitoring failed to update a known affected lockfile by the cut-off, and external reporters lack a current project-specific path.

**Judgment:** Patch or document non-reachability immediately; publish a modern vulnerability-handling policy and advisories baseline.

### F7 — Operational debt reduces signal quality (Medium–High)

**Fact:** Tool output showed 81 pending handoffs, eight unresolved required acknowledgements after the reported cutoff, and 51 rule-copy drift findings (44 differing, five missing, two extra). GitLab exposed 547 branches, 164 marked merged. Issue #110 explicitly tracks cleanup of obsolete merge requests while preserving audit evidence.

**Inference:** The project’s extreme development cadence creates more governance records than one human can reliably close and review. Large queues make exceptions easier to overlook.

**Judgment:** Before enterprise claims expand, the project needs service-level objectives for acknowledgements, bounded branch lifetime, drift closure, and evidence-preserving archival.

### F8 — External adoption evidence is insufficient (Medium–High)

**Fact:** The primary foundry learning record describes a one-person project with agent assistance and mostly single-adopter confidence. A Harbor Workshop snapshot is described as externally submitted by email, but public material does not enable identity or outcome verification. The project’s [contribution page](https://janipaijanen.gitlab.io/agents-govern-framework/contribute/) openly acknowledges single-developer evidence as its strongest weakness.

**Inference:** The corpus demonstrates extensive dogfooding and rapid learning, but not independent repeatability across organizations, regulated contexts, or multi-governor teams.

**Judgment:** Adoption claims should remain bounded to single-project/single-governor workflows until independent cohorts and measured outcomes exist.

## 6. Weighted Scorecard

| Dimension | Weight | Score | Weighted points | Evidence confidence | Summary |
|---|---:|---:|---:|---|---|
| Strategic fit | 5% | 3 | 15 | Medium | Clear problem framing and differentiated governance focus; narrow and not independently validated |
| Governance | 15% | 2 | 30 | High | Defined roles/gates, but single-person authority and unauthenticated/advisory controls |
| Community | 6% | 1 | 6 | High | No independent human committer, current succession path, or demonstrated maintainer plurality |
| Engineering | 14% | 3 | 42 | Medium–High | Substantial Rust/tooling/test investment; monolithic CLI, no public coverage, very young codebase |
| Release management | 11% | 2 | 22 | High | Verifiable artifacts and transparent recovery; hyper-cadence, unsigned releases, no support lifecycle |
| Security | 15% | 2 | 30 | High | Useful CI checks and hardened container; no current policy/SBOM/signing and one advisory |
| Operational readiness | 10% | 2 | 20 | High | Strong instrumentation, but unresolved queues, drift, branch sprawl, invalid history |
| Legal and licensing | 10% | 1 | 10 | High | Direct license contradiction and uncertain inbound commercial rights |
| Adoption and ecosystem | 8% | 2 | 16 | Medium | Extensive dogfood plus one unverified external snapshot; no independent cohort evidence |
| Financial/organizational | 2% | NE | — | Low | No public entity, funding, SLA, insurance, escrow, or continuity evidence |
| AI-era readiness | 4% | 3 | 12 | Medium | AI assistance disclosed and human authority explicit; evaluations remain self-produced |
| **Total** | **100%** |  | **203/98 = 2.1/5** |  | **98% scored; 2% unscored** |

The arithmetic is `Σ(weight × score) / Σ(scored weight) = 203 / 98 = 2.071`, rounded to **2.1/5**. No popularity metric contributes to the score.

## 7. Prioritized Risk Register

| ID | Risk | Likelihood | Impact | Rating | Evidence/trigger | Required treatment |
|---|---|---|---|---|---|---|
| R1 | Contradictory tooling license and unclear inbound relicensing rights | Likely | Critical | **Critical** | Root summary conflicts with Cargo and OCI metadata | Freeze procurement; publish counsel-reviewed path/license map, SPDX metadata, and contribution grant |
| R2 | Single-person governance, maintenance, and release authority | Certain | High | **High** | 96% owner-attributed commits; no independent human committer | Add independent maintainer and backup release authority; publish succession/quorum rules |
| R3 | Forged or mistaken governance records accepted as structurally valid | Likely | High | **High** | String identities; approvals not identity-bound; gates acknowledged as advisory | Bind approvals to forge/IAM identity or signatures; enforce CODEOWNERS and protected refs |
| R4 | Incomplete audit reconstruction because current schemas reject history | Certain | High | **High** | 333 coordination and 12 plan failures; invalid records omitted operationally | Add schema versioning/migration and explicit legacy-evidence status; validate full corpus in release CI |
| R5 | Supply-chain compromise or unverifiable publisher identity | Possible | High | **High** | No signed commits/tags/artifacts, no SBOM, self-issued provenance | Sign releases/attestations, publish SBOMs, make provenance verifiable and rebuildable |
| R6 | Vulnerability handling failure or delayed dependency remediation | Possible | High | **High** | Current RustSec advisory; absent current security policy and automation evidence | Upgrade/assess advisory; publish security policy/SLA; automate dependency updates |
| R7 | Upgrade/support disruption from release volatility | Likely | Medium–High | **High** | 39 tags in ~66 days; median 13.1-hour interval; several corrective releases | Define release train, supported minors, backport and EOL policy; stabilize before LTS claim |
| R8 | Control signal buried by operational queues and drift | Likely | Medium | **Medium–High** | 81 handoffs, eight required acknowledgements, 51 drift findings, 547 branches | Establish closure SLOs, archive/retention procedure, dashboards, and escalation ownership |
| R9 | Adoption fails outside founder-led dogfood | Likely | Medium | **Medium–High** | Single-governor evidence; external snapshot not independently verified | Run independent pilots and publish predefined outcome measures and negative results |
| R10 | Append-only evidence conflicts with privacy/deletion duties | Possible | Medium | **Medium** | Persistent human/agent records; no demonstrated enterprise privacy design | Data classification, minimization, retention, redaction, DSAR and legal-hold design |
| R11 | Upstream interruption leaves adopters without support or legal counterparty | Possible | Medium | **Medium** | No public entity, SLA, funding, escrow, or continuity arrangement | Mirror/fork plan, internal ownership, support contract or qualified alternative supplier |

## 8. Methodology, Sampling, and Limitations

### Method

The assessment used four evidence layers, ranked by reliability:

1. **Immutable primary evidence:** the v2.6.1 tag and its peeled commit, source files, schemas, lockfile, CI definition, license metadata, and released artifacts.
2. **Reproducible observations:** local Git history analysis; released-binary execution; full-corpus validator runs; SHA-256 checks; archive inspection; OSV queries from `Cargo.lock`; public GitLab API counts.
3. **Current project claims:** live documentation, roadmap, contribution pages, learning records, changelog, and release narratives.
4. **Analyst judgments:** enterprise consequence, control sufficiency, risk priority, and forward outlook.

The assessed tree had 3,362 tracked files: approximately 3,140 Markdown files, 51 Rust files, 46 YAML files, 16 Python files, 12 shell files, 14 JSON files, ten schemas, 49 rule files, and 16 agent/manifest YAML files. Approximate code counts were 23,016 Rust, 3,617 Python, and 1,824 shell lines; YAML and Markdown dominate the policy/evidence corpus. The Rust workspace contains five crates and roughly 363 `#[test]` annotations across 18 test files. Counts are descriptive, not quality scores.

### Time windows

| Window | Inclusive period | Commits observed | Interpretation |
|---|---|---:|---|
| 90 days | 2026-05-19 to 2026-08-17 | 4,180 | Entire project history |
| 12 months | 2025-08-18 to 2026-08-17 | 4,180 | Same history; project did not exist for most of window |
| 24 months | 2024-08-18 to 2026-08-17 | 4,180 | Same history |
| 36 months | 2023-08-18 to 2026-08-17 | 4,180 | Same history |

This prevents false trend claims: there is no evidence yet for multi-year retention, maintenance, succession, or security response.

### Sampling

- All commits and tags were counted; author identities were normalized conservatively using names and email aliases visible in Git metadata.
- All tracked file types were enumerated; Rust workspace and dependency lockfile were reviewed.
- All coordination and plan files matching the project’s published command patterns were tested with the released binary.
- All 170 crates.io packages in the lockfile were queried through OSV; this is advisory matching, not reachability analysis.
- Framework and x86_64 tool release artifacts were independently downloaded and checksum-verified. The project’s post-publication checker verified all ten file assets. OCI pull verification could not complete because Podman was unavailable.
- Public issues and branches were paginated through GitLab’s API. Merge request approval settings and protected-tag rules were inaccessible anonymously.

### Limitations

1. Cargo was unavailable in the assessment environment, so the analyst did not independently compile the source or rerun `cargo test --workspace`. CI configuration and test inventory were inspected, while binary and artifact behavior were independently exercised.
2. No authenticated GitLab access was used. Private security reports, protected-tag rules, approval counts, runner controls, audit logs, and maintainer permissions were not available.
3. No maintainer or adopter interviews were performed. The Harbor Workshop record could not be independently authenticated.
4. No penetration testing, dynamic fuzzing, legal opinion, malware analysis, or reproducible-build comparison was performed.
5. Git author attribution does not prove natural-person identity; normalization is an evidence-based estimate.
6. Same-day live-doc drift was observed: the roadmap still described v2.6.1 publication as in progress before the later release-info build reported it released. The tag, not the moving site, controlled release conclusions.

## 9. Detailed Dimension Assessments

### 9.1 Strategic Fit — 3/5

AGF addresses a real and growing problem: coordinating human authority, specialized AI agents, review gates, evidence, and learning without pretending the orchestration layer itself is a security boundary. The [project home](https://janipaijanen.gitlab.io/agents-govern-framework/) clearly distinguishes the framework from a runtime, model, compliance guarantee, or tamper-proof ledger. That boundary is strategically healthy. The framework’s adoption and exit documentation encourages selective adoption and preserves an exit path rather than maximizing lock-in.

The project is differentiated by operational artifacts rather than abstract principles: role files, schemas, coordination records, plan contracts, validators, policy rules, evidence capture, and an improvement loop. The positioning is particularly relevant to organizations already using repository-native AI agents and Git-based delivery.

The constraint is evidence. The framework is optimized and validated mainly in a founder-led, single-project environment. Multi-team, multi-governor, regulated, unionized, or vendor-managed deployments are not demonstrated. The framework can also impose high process overhead: thousands of coordination messages and rapid policy evolution illustrate the cost. Strategic fit is therefore strongest for a controlled engineering pilot, not an organization-wide governance standard.

### 9.2 Governance — 2/5

The governance model is explicit. Three core roles distinguish human authority, planning/coordination, and implementation; Gate 1 approves plans and Gate 2 approves completion. The [roles and gates documentation](https://janipaijanen.gitlab.io/agents-govern-framework/docs/concepts/roles-and-gates/) is readable and appropriately keeps human accountability visible. Rules, plans, coordination messages, and learnings create an inspectable decision trail.

However, governance design and governance reality diverge. One human appears to own the repository, governor role, maintenance, and releases. Public evidence does not show an independent steering mechanism, conflict-of-interest process, appeal path, maintainer promotion, quorum, succession, or release separation. The project bot is automation, not independent oversight.

Enforcement is also bounded. Schemas validate document shape; they do not authenticate the person asserting authority, prove consultation occurred, or establish that a plan matches a diff. AGF’s own unresolved findings recognize these limitations. An enterprise deployment must therefore treat repository permissions and signed review events as authoritative, with AGF records as contextual evidence.

### 9.3 Community — 1/5

The project is publicly accessible, documents contribution routes, exposes issues, and publishes unusually detailed internal learning and recovery material. These are positive openness signals. The [contribution page](https://janipaijanen.gitlab.io/agents-govern-framework/contribute/) invites rules, agents, evidence snapshots, and tooling contributions.

But no independent human committer was observed in the full history. Issue authorship is also concentrated: 81 of 110 issues were opened by the owner, 27 by project bots, with only isolated other identities. Zero stars or forks were observed but are deliberately not scored; the meaningful problem is lack of demonstrated independent maintenance and review. A current root Code of Conduct and maintainer governance guide were not found, and the current contribution mechanism does not clearly address IP grants for dual/commercial licensing.

The community is best described as a public founder-led project, not yet a resilient open-source community.

### 9.4 Engineering — 3/5

The technical implementation is substantial for the project’s age. The Rust workspace separates core logic, coordination, validators, MCP integration, and CLI concerns across five crates. The lockfile is committed. Tests include negative and adversarial cases; the tree contains approximately 363 Rust test annotations. CI runs workspace tests, Clippy with `-D warnings`, dependency-policy checks, artifact builds, and vulnerability scanning. No Unsafe Rust block was found in the reviewed source. The release binary validated its own build identity.

Weaknesses remain. The principal CLI `main.rs` is approximately 6,552 lines and provenance logic exceeds 2,200 lines, creating concentration and review burden. No public line/branch coverage report or minimum threshold was found, and a rustfmt enforcement stage was not evident in the reviewed CI. The dependency graph contains 175 locked packages, increasing supply-chain surface. The analyst could not independently compile or rerun unit tests due environment limitations.

Most importantly, the framework’s data-contract evolution has outrun its corpus migration. A validator is valuable only if its validity domain is explicit; today, changed-file enforcement and whole-history assurance are easy to conflate.

### 9.5 Release Management — 2/5

Release engineering has several strong elements. v2.6.1 publishes framework, binary, checksum, and provenance artifacts. The independent download matched recorded bytes and commit, and `agov --version` returned `2.6.1`, the expected commit, and a clean state. The project documents failure modes and publishes recovery releases rather than hiding incidents.

The cadence is nevertheless unstable: 39 tags in roughly 66 days, with a median tag interval of about 13.1 hours, an average of 41.3 hours, and a minimum of 0.24 hours. Some days produced four to seven tags. Of 39 tags, 22 were annotated and 17 lightweight; none was signed. v2.6.0 built an OCI image but failed GitLab release/package publication because of a stale asset contract, requiring v2.6.1. Earlier v2.4.x versions also corrected incomplete publications and missing fixtures.

No clear enterprise support window, backport commitment, current-version matrix, LTS definition, or vendor EOL policy was found. A deprecation rule provides useful defaults, but it is not equivalent to a maintained-product support contract.

### 9.6 Security — 2/5

The project uses meaningful preventive controls: Rust memory safety by default, Clippy warnings as errors, `cargo-deny` advisory/license checks, Trivy high/critical scanning with blocking behavior for fixable findings, a locked dependency graph, a digest-pinned Alpine base, static binary packaging, non-root container execution, and artifact provenance. These practices exceed many projects of similar age.

Gaps prevent a higher score. The root/current project lacks a relevant security policy; the legacy `agents-govern/SECURITY.md` describes a Bash-centric project and is stale after the Rust toolchain. No private reporting route, response SLA, supported-version policy, public advisory process, threat model, SBOM, release signature, or independent security review was found. No automated dependency-update configuration was observed.

The OSV sweep identified `anyhow 1.0.102` affected by RUSTSEC-2026-0190. The issue concerns unsoundness in `Error::downcast_mut` and is fixed in 1.0.103. The package is transitive and direct vulnerable-code reachability was not shown, so this is not evidence of exploitability. It is evidence that the release lockfile contains a known affected version.

### 9.7 Operational Readiness — 2/5

AGF has strong operational observability for an early project. `agov` exposes validation, coordination status, pending acknowledgement, drift, release provenance, and post-publication checks. The project keeps failures and improvement records in the repository, which improves forensic review.

The current queues reveal scaling stress. There were 2,309 valid coordination messages, 81 pending handoffs, eight unresolved required acknowledgements, and 51 copied-rule drift findings. The repository had 547 branches, only 164 marked merged by the API. Full-corpus validation rejected 333 coordination-path files and 12 plan files, while operational tools ignore invalid records. These numbers do not prove that work or approval was improper, but they reduce the completeness and clarity of current status signals.

No public enterprise support channel, incident on-call model, service objective, backup operator, disaster-recovery exercise, or capacity/availability commitment was found. Because AGF is repository-native, adopters can self-host the artifacts, but that portability does not replace upstream operational continuity.

### 9.8 Legal and Licensing — 1/5

The public distribution mixes CC BY-SA 4.0, an unprovided/externally negotiated commercial license reference, and MIT. Mixed licensing can be legitimate when paths are precise. Here, the root prose assigns MIT to tooling while Cargo and OCI metadata assign CC BY-SA/commercial to the same Rust tooling. File-level SPDX coverage is sparse, so the root statement that file-specific SPDX markers control does not resolve most files.

The legal classification is consequently provisional: **mixed-license/dual-license public-source material, not presently demonstrable as a wholly OSI-approved open-source software distribution**. Documentation/content under CC BY-SA can be openly licensed without being OSI software. Rust tooling may have been intended as MIT, but the tagged package metadata contradicts that intent.

Inbound rights are also unclear. The live contribution guide does not visibly include a DCO, CLA, or explicit grant allowing the maintainer to issue third-party contributions under a commercial alternative. A legacy contribution document appears to contain such a grant, but its placement in a bootstrap/legacy subtree and references to an older project make applicability uncertain. Counsel should determine copyright ownership, contributor assent, notice obligations, database/content licensing, trademark rights, and commercial relicensing scope.

### 9.9 Adoption and Ecosystem Evidence — 2/5

AGF has an extensive self-observation corpus. The foundry record contains 54 learnings, with 37 marked single-adopter confidence and 17 older records without that field. It describes a one-human team using Claude/Codex-style agents. A Harbor Workshop snapshot describes an external upgrade from 2.2.4 to 2.5.0, submitted by email, and is marked single-adopter confidence.

This is useful formative evidence, but not independent market validation. No authenticated case study, public deployment reference, package-download history, paid production customer, independent benchmark, regulated adoption, or cross-organization survey was found. The project explicitly acknowledges the limitation. The validated core should therefore be described narrowly: founder-led, repository-native, single-project/single-governor governance with AI agent assistance.

### 9.10 Financial and Organizational Sustainability — NE

No public evidence established a legal operating entity, revenue, funding, runway, insurance, commercial support capacity, escrow, staffing plan, or contractual continuity. An open issue reportedly tracks a Cyber Resilience Act entity decision, but an issue is not an operating structure.

The dimension is marked **NE**, not scored low, because absence of public evidence is not proof of financial weakness. For procurement, the practical consequence is the same: the buyer lacks a counterparty, service commitment, and continuity assurance unless separately negotiated.

### 9.11 AI-Era Readiness — 3/5

AGF is unusually explicit about AI participation. Agent roles, model/tool use, human authority, limitations, consultation, and evidence are represented in project records. The framework resists anthropomorphizing agents into final authorities and keeps the human governor accountable for gates. AI-generated figures and model-assisted work are disclosed in the corpus.

AGF does not ship a foundation model, training dataset, or autonomous hosted agent, so conventional model/data cards are not directly applicable. The relevant question is whether AI-assisted governance is evaluated independently. Today, most evaluations and learning records are produced inside the same founder-led system. The framework needs independent red-team scenarios, model substitution tests, prompt-injection/record-forgery threat modeling, and outcome comparisons against non-AGF workflows.

## 10. Enterprise-Readiness Matrix

| Enterprise requirement | Current evidence | Readiness | Adoption condition |
|---|---|---|---|
| Unambiguous license and redistribution rights | Conflicting root/Cargo/OCI terms | **Blocked** | Counsel-reviewed path map, corrected metadata, SPDX coverage, inbound grant |
| Accountable supplier/continuity | No public entity, SLA, or backup maintainer | **Blocked for supported procurement** | Contracting entity or enterprise-owned support/fork plan |
| Authenticated approvals | Schema strings and advisory gates | **Blocked as system of record** | Enterprise IAM/forge identity, signed reviews, protected refs |
| Reproducible audit history | Human-readable corpus; current validators reject legacy records | **Partial** | Schema versioning, migration evidence, complete status semantics |
| Secure release consumption | Hashes/provenance; no signatures/SBOM | **Partial** | Internal mirroring/signing/SBOM now; upstream signed attestations later |
| Vulnerability management | CI scans; stale policy and one current advisory | **Partial/blocked pending fix** | Current policy, reporting channel, supported versions, patch/reachability decision |
| Engineering maintainability | Modular crates and tests; young, some large modules | **Pilot-ready** | Internal code review, pinned version, test/rebuild confirmation |
| Stable upgrades | Transparent but extremely rapid corrective cadence | **Partial** | Release train, compatibility/support policy, staged upgrades |
| Operational support | Useful CLI visibility; large unresolved queues | **Partial** | Named operators, SLOs, queue cleanup, enterprise runbook |
| Independent adoption proof | Dogfood and one unverified external snapshot | **Insufficient** | Independent pilots with published, predefined measures |
| Privacy/records governance | Append-only evidence orientation; limited enterprise privacy proof | **Partial** | Retention, minimization, redaction, legal hold, access controls |
| Exit and portability | Repository-native artifacts and exit guidance | **Good** | Mirror evidence and tooling; test decommission/export |

## 11. SWOT Analysis

| Strengths | Weaknesses |
|---|---|
| Clear human-authority model and bounded claims | Critical license contradiction |
| Substantial repository-native tooling and schemas | Single-person governance and bus factor |
| Transparent incident/recovery and learning records | Advisory, unauthenticated gates |
| Verifiable file artifacts and exact binary identity | No signed releases, SBOM, or current security policy |
| Useful tests, CI scanning, pinned non-root container | Current validators reject material historical corpus |
| Portable, inspectable, low infrastructure dependency | Hyper-cadence, support lifecycle absent, operational queues |

| Opportunities | Threats |
|---|---|
| Become a neutral evidence interchange for agent governance | A license dispute or ambiguity blocks enterprise/legal adoption |
| Integrate signed forge identities and attestations | Founder unavailability stalls governance, fixes, or releases |
| Establish independent adopter consortium and benchmarks | “Governance” claims are mistaken for security/compliance guarantees |
| Offer stable/LTS profiles and migration tooling | Rapid schema churn makes old evidence unusable |
| Build privacy-aware evidence retention and redaction | Better-funded runtimes/forges absorb equivalent controls |
| Convert transparency into auditable assurance | Supply-chain incident exploits unsigned same-channel releases |

## 12. One- to Three-Year Outlook

### Base case

Over 12 months, AGF is likely to remain a fast-moving, founder-led framework with improving validators and release mechanics. If licensing is corrected and security/release basics are added, it can become credible for controlled internal engineering pilots. Its repository-native design and transparent failure records are advantages. Without a slower compatibility promise, adopters will likely pin versions and selectively copy patterns rather than depend on the full framework.

At 24 months, the key discriminator will be institutionalization. Independent maintainers, signed releases, versioned evidence contracts, and several externally verified deployments would move AGF toward a durable project. Continued owner-only control with thousands of self-generated records would instead make it a sophisticated personal operating system with limited procurement value.

At 36 months, three scenarios are plausible:

- **Positive (25%):** license and supply-chain controls are resolved, governance diversifies, a stable/LTS profile emerges, and independent organizations publish outcome evidence. AGF becomes a credible niche standard or reference implementation.
- **Base (50%):** the project remains active and technically useful but founder-centric. Enterprises borrow its patterns or run internally controlled forks rather than procure upstream reliance.
- **Negative (25%):** cadence and evidence debt overwhelm maintenance, licensing inhibits contributions, or founder availability declines. Releases or documentation continue but assurance and adoption stagnate.

Probabilities are analyst judgments with low confidence because the project has only 67 days of observable history.

## 13. Remediation Roadmap

### 0–30 days: remove procurement and security blockers

| Action | Owner profile | Completion evidence |
|---|---|---|
| Decide the intended Rust/tooling license; align root license map, Cargo fields, OCI labels, package manifests, and SPDX headers | Copyright owner + counsel | Tagged corrective release and machine-readable license inventory |
| Establish inbound contribution terms compatible with every outbound license | Counsel + maintainer | CLA/DCO or explicit contribution grant with recorded assent |
| Replace legacy security policy with current scope, private channel, supported versions, response targets, and disclosure process | Security owner | Root `SECURITY.md`, tested reporting path, public support table |
| Upgrade `anyhow` to ≥1.0.103 or document reachability and time-bound acceptance | Engineering/security | Updated lockfile, CI evidence, advisory decision record |
| Generate release-bound CycloneDX or SPDX SBOMs | Release engineering | SBOM asset per binary/container/source release |
| Sign tags, artifacts, checksums, and provenance using an independently verifiable identity | Release engineering | Verification instructions and successful independent check |
| Fix glob classification and introduce schema-version-aware legacy validation | Tooling/data-contract owner | Full corpus classified valid under declared schema or explicitly legacy/unverified |
| Triage 81 handoffs, eight acknowledgements, 51 drift findings, and branch backlog without deleting required audit evidence | Governor/maintainer | Bounded queues, disposition records, branch-retention policy |

### 31–90 days: establish independent governance and stable consumption

| Action | Completion evidence |
|---|---|
| Appoint an independent maintainer and backup release authority | Public role/permission roster, two-person release exercise |
| Publish maintainer promotion, removal, conflict, succession, quorum, and appeal rules | Governance document exercised through one real decision |
| Implement CODEOWNERS, protected tags, required independent review, and forge-linked Gate records | Public configuration evidence and negative enforcement tests |
| Add identity-bound approval references and plan-to-diff/MR linkage checks | Validator tests showing forged/stale approval rejection |
| Define release train, compatibility policy, supported minors, backport criteria, and EOL notice | Published support matrix and first scheduled release |
| Add automated dependency updates and security-advisory release tests | Bot configuration and patch-latency metric |
| Recruit at least five independent pilots covering at least 50 governed changes | Predefined cohort protocol, anonymized raw metrics, negative findings |

### 91–365 days: build external assurance

| Action | Completion evidence |
|---|---|
| Commission independent security and architecture review | Public report and tracked remediation |
| Add reproducible-build or SLSA-aligned signed provenance | Independent rebuild match and signed builder identity |
| Validate multi-governor, multi-project, and regulated workflows | Published case studies with exceptions and failure rates |
| Establish legal entity or durable foundation/governance home | Public entity, asset/IP ownership, continuity plan |
| Offer documented support/SLA or certify third-party providers | Support terms and exercised incident response |
| Implement privacy-aware evidence retention/redaction/export | Data model, retention tests, deletion/legal-hold procedures |

## 14. Open Questions and Evidence Requests

1. What license is intended for each Rust crate and compiled binary: MIT or CC BY-SA/commercial? Which metadata is authoritative?
2. Where is `LicenseRef-AgentsGovern-Commercial` defined, and what rights are available to an enterprise evaluator?
3. What contributor assent mechanism permits commercial relicensing of third-party changes?
4. Who besides Jani Päijänen can merge, publish, revoke a release, administer CI secrets, or assume governor authority?
5. Are protected tags, required approvals, CODEOWNERS, and runner environments configured? Provide exported settings or screenshots.
6. Are the 4,180 commits intentionally unsigned? Is a signing/attestation migration planned?
7. What is the authoritative interpretation of the 333 coordination and 12 plan files rejected by v2.6.1? Can each record declare its schema version?
8. Why do operational commands ignore invalid records, and how are users warned that status may be incomplete?
9. What are the age and disposition of the 81 pending handoffs and eight required acknowledgements?
10. Has `anyhow 1.0.102` reachability been analyzed, and when will it be upgraded?
11. What versions receive security fixes and for how long? What is the private reporting channel and target response time?
12. Can the project publish signed SBOMs, builder identity, and a reproducible-build recipe for every release?
13. Who independently verified the Harbor Workshop record, and can anonymized outcome/exception data be shared?
14. What measured benefit does AGF deliver relative to ordinary protected-branch reviews: defect escape, rework, latency, audit effort, or policy adherence?
15. How will append-only records handle personal data, secrets, deletion requests, retention limits, and legal holds?
16. Is there a legal entity, trademark owner, continuity plan, commercial SLA, or escrow arrangement?
17. What is the supported migration path when schemas or rules change, and which compatibility guarantees apply?
18. Can an enterprise obtain a complete permission roster, incident history, and release-key recovery procedure under NDA?

## 15. Change Log from Prior Assessment

The repository includes a 2026-08-10 external-AI review using Grok and Copilot. It is not a Protocol 2.1 baseline, and its category scores are not numerically comparable with this weighted 0–5 assessment. The prior material characterized AGF as medium risk and suitable for a controlled pilot; Copilot reportedly scored strategic alignment 8.5/10, product maturity 7/10, governance 8/10, security 6.5/10, and enterprise readiness 6.5/10. This reassessment preserves the pilot-only direction but strengthens the production deferral because it independently tested release artifacts, validators, history concentration, and dependency advisories.

| Area | Change since prior review / newly established evidence | Effect |
|---|---|---|
| Release | v2.6.1 superseded v2.6.0 after failed GitLab package/release publication; file artifacts independently verified | Positive recovery evidence, but reinforces release-volatility risk |
| Governance | v2.5-era role/coordination visibility improvements are present | Positive design improvement; no independent authority added |
| Licensing | Cargo and OCI licensing found to contradict root MIT tooling statement | New Critical blocker |
| Validation | Full corpus run found 333 coordination-path and 12 plan failures | New High audit-assurance risk |
| Security | OSV matched `anyhow 1.0.102` to RUSTSEC-2026-0190; no current root security policy/SBOM/signatures | New High remediation need |
| Operations | 81 pending handoffs, eight required acknowledgements, 51 rule drift findings, 547 branches | Stronger evidence of scale/queue pressure |
| Adoption | Harbor Workshop snapshot adds one externally described experience | Small positive; still not independently verifiable |
| Method | Immutable-tag clone, artifact downloads, binary execution, Git/API enumeration, validator runs, OSV lockfile sweep | Higher evidence confidence than document-only review |

## 16. Optional Appendices

### Appendix A — Radar-chart data

Financial/organizational sustainability is omitted from radar plotting because it is NE.

| Dimension | Score (0–5) |
|---|---:|
| Strategic | 3 |
| Governance | 2 |
| Community | 1 |
| Engineering | 3 |
| Release | 2 |
| Security | 2 |
| Operational | 2 |
| Legal | 1 |
| Adoption | 2 |
| AI-era | 3 |

### Appendix B — Reproduction commands and observations

The following command classes reproduce the material observations from public evidence. Exact artifact URLs are in the Source Register.

```bash
git clone https://gitlab.com/janipaijanen/agents-govern-framework.git
git checkout --detach v2.6.1
git rev-parse HEAD
git rev-list --count HEAD
git log --format='%aN%x09%aE' HEAD
git tag --format='%(refname:short)%09%(objecttype)%09%(taggerdate:iso8601)'

./agov --version
./agov version --json
./agov validate
./agov coord validate docs/coordination/*.md
./agov plan validate docs/plans/*.md
./agov coord status
./agov coord pending
./agov dist post-publication-check --version 2.6.1

sha256sum -c agf-2.6.1.tar.gz.sha256
sha256sum -c agov-2.6.1-linux-x86_64.tar.gz.sha256
```

Observed stable identifiers:

| Artifact | Observation |
|---|---|
| Source tag | v2.6.1 → `f780ac0ea37061e0ad3ee6fb1f00c6958b996fc0` |
| `agf/VERSION` | `2.6.1` |
| Framework archive | 1,150,111 bytes; SHA-256 `44c71c...6928` |
| x86_64 archive | 3,445,001 bytes; SHA-256 `4bca7f...82ce` |
| OCI provenance digest | `sha256:2fcc8b4b1ab8735faf6336daec5898c39b32001f11a173f2be11411467e01366` |
| Binary self-report | `agov 2.6.1 (f780ac0... clean)` |
| Full coordination validation | 2,309 OK; 333 ERR; 272 WARN |
| Full plan validation | 151 OK; 12 ERR |
| Operational status | 81 pending handoffs; eight required acknowledgements; 51 rule-copy drift findings |

Full hashes should be taken from and checked against the signed or independently obtained release metadata, not copied from this abbreviated report.

### Appendix C — Compensating controls for a pilot

- Vendor the selected tag and all transitive dependencies in an enterprise-controlled registry.
- Produce and retain a complete SBOM and vulnerability snapshot at promotion time.
- Require two independent human reviewers for changes to rules, schemas, agents, CI, licenses, and release code.
- Map AGF roles to named corporate identities; prohibit free-form authority assertions.
- Store approval events in the forge/audit platform and link immutable event IDs into AGF records.
- Validate only declared schema versions, but report legacy/unvalidated evidence prominently rather than ignoring it.
- Prohibit secrets and unnecessary personal data in coordination records; apply retention and redaction rules.
- Define maximum open-handoff/acknowledgement ages and fail the pilot gate when exceeded.
- Maintain a tested export, fork, and decommission procedure.

## 17. Source Register

Retrieved 2026-08-17 unless otherwise noted.

| ID | Title | Type / publisher | Version, path, or date | URL | Used in |
|---|---|---|---|---|---|
| S001 | Release information | Live project documentation | Build 2026-08-17; v2.6.1 | [Link](https://janipaijanen.gitlab.io/agents-govern-framework/reference/release-info/) | Identity, scope, release |
| S002 | Version marker | GitLab repository | v2.6.1 `agf/VERSION` | [Link](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/v2.6.1/agf/VERSION) | Identity |
| S003 | Project README | GitLab repository | v2.6.1 `README.md` | [Link](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/v2.6.1/README.md) | Scope, claims, limitations |
| S004 | Root licence and path map | GitLab repository | v2.6.1 `LICENCE` | [Link](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/v2.6.1/LICENCE) | Legal |
| S005 | CC BY-SA license | GitLab repository | v2.6.1 `agf/LICENSE` | [Link](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/v2.6.1/agf/LICENSE) | Legal |
| S006 | MIT license text | GitLab repository | v2.6.1 `agf/LICENSE-MIT` | [Link](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/v2.6.1/agf/LICENSE-MIT) | Legal |
| S007 | Rust workspace manifest | GitLab repository | v2.6.1 `tooling/rust/Cargo.toml` | [Link](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/v2.6.1/tooling/rust/Cargo.toml) | Engineering, legal |
| S008 | agov Containerfile | GitLab repository | v2.6.1 | [Link](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/v2.6.1/tooling/rust/Containerfile.agov) | Security, legal |
| S009 | GitLab CI definition | GitLab repository | v2.6.1 `.gitlab-ci.yml` | [Link](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/v2.6.1/.gitlab-ci.yml) | Engineering, security, release |
| S010 | cargo-deny policy | GitLab repository | v2.6.1 `deny.toml` | [Link](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/v2.6.1/tooling/rust/deny.toml) | Security, legal |
| S011 | Release artifacts and verification | Live project documentation | Current 2026-08-17 | [Link](https://janipaijanen.gitlab.io/agents-govern-framework/reference/release-artifacts/) | Release, provenance |
| S012 | Roadmap | Live project documentation | Observed 2026-08-17 | [Link](https://janipaijanen.gitlab.io/agents-govern-framework/roadmap/) | Roadmap, doc drift |
| S013 | Contracts and validators | Live project documentation | Current 2026-08-17 | [Link](https://janipaijanen.gitlab.io/agents-govern-framework/docs/advanced/contracts-and-validators/) | Governance, validation limitations |
| S014 | Roles and gates | Live project documentation | Current 2026-08-17 | [Link](https://janipaijanen.gitlab.io/agents-govern-framework/docs/concepts/roles-and-gates/) | Governance |
| S015 | Coordination message schema | GitLab repository | v2.6.1 | [Link](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/v2.6.1/agf/schemas/coordination-message.schema.json) | Identity/enforcement |
| S016 | Gate 1 plan schema | GitLab repository | v2.6.1 | [Link](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/v2.6.1/agf/schemas/gate1-plan.schema.json) | Approval enforcement |
| S017 | Changelog | GitLab repository | v2.6.1 | [Link](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/v2.6.1/agf/CHANGELOG.md) | Releases, unresolved findings |
| S018 | Contribute | Live project documentation | Current 2026-08-17 | [Link](https://janipaijanen.gitlab.io/agents-govern-framework/contribute/) | Community, inbound rights, adoption caveat |
| S019 | Foundry learning record | GitLab repository | v2.6.1 `foundry.yaml` | [Link](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/v2.6.1/agf/learnings/foundry.yaml) | Adoption, AI, team evidence |
| S020 | Harbor Workshop learning record | GitLab repository | v2.6.1 `harbor-workshop.yaml` | [Link](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/v2.6.1/agf/learnings/harbor-workshop.yaml) | External adoption claim |
| S021 | AGENTS role instruction | GitLab repository | v2.6.1 `AGENTS.md` | [Link](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/v2.6.1/AGENTS.md) | Governance design |
| S022 | GitLab project API | GitLab | Observed 2026-08-17 | [Link](https://gitlab.com/api/v4/projects/janipaijanen%2Fagents-govern-framework) | Project age, public metadata, license detection |
| S023 | GitLab issues API | GitLab | All public issues at cutoff | [Link](https://gitlab.com/api/v4/projects/83321848/issues?scope=all&per_page=100) | Community, operational evidence |
| S024 | GitLab branches API | GitLab | All public branches at cutoff | [Link](https://gitlab.com/api/v4/projects/83321848/repository/branches?per_page=100) | Governance, operations |
| S025 | RUSTSEC-2026-0190 | RustSec Advisory Database | Published 2026-06-25 | [Link](https://rustsec.org/advisories/RUSTSEC-2026-0190.html) | Dependency security |
| S026 | Framework provenance record | GitLab generic package | v2.6.1 | [Link](https://gitlab.com/api/v4/projects/janipaijanen%2Fagents-govern-framework/packages/generic/agf/v2.6.1/agf-2.6.1.tar.gz-provenance.json) | Artifact verification |
| S027 | x86_64 tool provenance record | GitLab generic package | v2.6.1 | [Link](https://gitlab.com/api/v4/projects/janipaijanen%2Fagents-govern-framework/packages/generic/agf/v2.6.1/agov-2.6.1-linux-x86_64.tar.gz-provenance.json) | Artifact verification |
| S028 | OCI provenance record | GitLab generic package | v2.6.1 | [Link](https://gitlab.com/api/v4/projects/janipaijanen%2Fagents-govern-framework/packages/generic/agf/v2.6.1/agov-oci-2.6.1-provenance.json) | Container release |
| S029 | Tooling and tokens | Live project documentation | Current 2026-08-17 | [Link](https://janipaijanen.gitlab.io/agents-govern-framework/docs/adopt/tooling-and-tokens/) | Tool scope and adoption |
| S030 | Prior external-AI review | GitLab repository | 2026-08-10 review bundle | [Link](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/v2.6.1/docs/reviews/20260810T134228Z-external-ai-diligence-grok-copilot/README.md) | Change log |
| S031 | Rust crates and tests | GitLab repository | v2.6.1 | [Link](https://gitlab.com/janipaijanen/agents-govern-framework/-/tree/v2.6.1/tooling/rust/crates) | Engineering |
| S032 | Adoption and exit guidance | Live project documentation | Current 2026-08-17 | [Adoption](https://janipaijanen.gitlab.io/agents-govern-framework/docs/start/should-you-adopt/) / [Exit](https://janipaijanen.gitlab.io/agents-govern-framework/docs/adopt/exit-and-decommission/) | Strategic, portability |
| S033 | Deprecation policy rule | GitLab repository | v2.6.1 | [Link](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/v2.6.1/agf/rules/workflow/deprecation-policy-rule.md) | Release support |
| S034 | Project documentation home | Live project documentation | Current 2026-08-17 | [Link](https://janipaijanen.gitlab.io/agents-govern-framework/) | Product identity and boundaries |
| S035 | Approved licenses | Open Source Initiative | Current 2026-08-17 | [Link](https://opensource.org/licenses) | Legal classification |
| S036 | CC licenses and software FAQ | Creative Commons | Current 2026-08-17 | [Link](https://creativecommons.org/faq/#can-i-apply-a-creative-commons-license-to-software) | Legal classification |
| S037 | Independent reproduction log | Analyst observation from public tag/artifacts | 2026-08-17; commands in Appendix B | N/A | Counts, hashes, validator and binary outputs |

---

**Final decision statement:** AGF is suitable for continued evaluation and a carefully controlled pilot after immediate legal and security remediation. It is **not currently suitable as an enterprise production control system, procurement-approved dependency, or source of authenticated/non-repudiable governance evidence** without substantial compensating controls.