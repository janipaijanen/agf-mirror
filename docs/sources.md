# AGF source registry — first-pass classification (2026-07)

Scheme: `type` (regulation | standard | framework | tooling | research | practice | discussion)
· `relation` (competitor | complement | prior-art | demand-driver | evidence | threat-model | none, multiple allowed)
· `action` (cite | integrate | compare-in-docs | watch | verify | ignore)

Entries marked **[unverified]** could not be confirmed by search — content classification is
withheld, not guessed. Verify before relying on them.

## Classified

- https://apsquared.co/posts/marketing-automation-claude-codex
  type: practice, discussion | relation: prior-art, complement, demand-driver | action: watch,
  compare-in-docs
  note: Self-reported portfolio-marketing architecture: each repository carries its own
  `AGENT.md`, activity logs, and human task queue; each run performs one activity; Claude and
  Codex are treated as replaceable engines. The potentially fresh AGF question is whether a
  portable repo-contained *agent package* should sit above governance, deliberately replace
  part of it in a major version, or incubate separately. Treat the claimed economics and
  quality as unverified: the article excludes the unspecified Post Bridge fee and human review
  time from its headline, and supplies no comparative outcome data. (accessed 2026-07-26)

- https://www.thoughtworks.com/insights/blog/open-source/zero-cost-fallacy-open-source-agentic-era
  type: practice | relation: demand-driver, evidence | action: cite
  note: OSS maintainers drowning in AI slop PRs; trust signals (stars, commit history) degraded.
  Directly motivates gated, auditable AI contribution flows — background for issues #3/#7 and
  for AGF's own positioning as a solo-maintained OSS project. (Thoughtworks, 2026-07)

- https://github.com/mmccalla/coding-agent-skill-library
  type: tooling | relation: complement, competitor(partial) | action: watch
  note: 113-skill library + safety policy docs (AGENTS.md, global safety files) + optional Neo4j
  GraphRAG, Apache-2.0. Governs *how* agents work via skills, not *whether* work passes gates —
  mostly orthogonal; an "AGF workflow as a skill" packaging could ride on this pattern.

- https://www.blackhc.net/essays/trust_is_not_governance/
  type: discussion | relation: prior-art(thematic) | action: cite
  note: Kirsch (DeepMind/UTAW) essay: safety culture and trusted leadership are not governance —
  structure is. Org-level, not coding-level, but the thesis is exactly AGF's advisory-gates
  argument one abstraction up; useful for positioning language. (2026-06)

- https://thenewstack.io/mcp-enterprise-agent-governance/
  type: practice | relation: complement, evidence | action: cite (with caveat)
  note: Argues MCP's tool-call-level context (which server, credentials, scope) is what makes
  agent actions auditable; "allowlists feel like governance but aren't". Supports the agov-mcp
  framing. Caveat: GitLab-sponsored content.

- https://github.com/cruxible-ai/cruxible
  type: tooling | relation: complement, prior-art | action: compare-in-docs
  note: Link appears stale — active repo is cruxible-ai/cruxible-core. Deterministic decision
  engine with DAG "receipts" proving how each answer was derived; MCP + CLI mirror; Python.
  Receipt-based provenance is a sibling idea to AGF's audit trail; correct-by-construction
  writes parallel to issue #1.

- https://dolphindb.com/blogs/48
  type: practice | relation: demand-driver, competitor(domain-adjacent) | action: verify, watch
  note: Could not confirm which post #48 is; DolphinDB launched "DolphinX — enterprise-grade
  platform for agent development and governance" 2026-07-13, likely the subject. Fintech/
  time-series domain, hosted platform — opposite end of the lock-in spectrum from AGF.

- https://www.cyera.com/blog/the-mcp-governance-illusion-why-governing-tools-is-not-enough
  type: practice | relation: evidence | action: verify, then cite
  note: Vendor confirmed (Cyera, data security); article body not read. Title thesis —
  tool-level governance is insufficient — matches AGF's process-level positioning and the
  agreed scoping that agov-mcp is ergonomics, not enforcement.

- https://github.com/DariuszNewecki/CORE
  type: framework | relation: competitor, prior-art, evidence | action: compare-in-docs, cite
  note: The closest true competitor found. "Constitutional governance runtime" hard-blocking
  agent mutations against machine-readable rules (92+), full audit trail; Python, MIT, solo
  maintainer, explicitly targets regulated environments. Philosophical opposite on one axis:
  CORE is a runtime the agent lives inside (high coupling), AGF is files + advisory/CI gates
  (removable). Their published lesson — "the constitution is only as strong as its enforcement
  mechanisms" — is independent support for issue #3. A comparison section is warranted.

- https://www.theregister.com/security/2026/06/20/why-amazon-hates-human-in-the-loop-ai-governance/5258639
  type: practice | relation: threat-model, demand-driver | action: verify, then cite
  note: Article body not read (headline + date only). Industry pushback against human-in-the-loop
  governance bears directly on AGF's gate philosophy; classify fully after reading.

- https://github.com/maref-org/maref
  type: framework | relation: competitor | action: compare-in-docs
  note: "Open-source agent governance operating system" (maref.cc) — quality-control layer
  between AI-generated code and production. Marketing leans on slop-crisis statistics.
  Overlapping adoption decision with AGF; depth/architecture not yet assessed.

- https://tenureai.dev/writing/ai-governance-cannot-be-a-tool-call/
  type: practice | relation: evidence, competitor(adjacent) | action: cite
  note: Tenure = governed context/memory proxy for agents (policy-enforced injection, per-turn
  audit). Essay thesis per title: governance must sit outside the agent's tool surface —
  independently reaches the same conclusion as AGF's agov-mcp scoping (wrapper ≠ gate).
  Adjacent competitor for the word "governance", different object (memory vs process).

All entries formerly listed here as [unverified] were fetched and classified 2026-07-21 —
see Batch 6 below.

## Suggested additions (surfaced during verification; fetched and confirmed 2026-07-21)

- https://github.com/boshu2/agentops
  type: framework | relation: competitor, prior-art | action: compare-in-docs
  note: Independent verification for coding agents — "no verdict = not done", verdicts recorded
  as plain markdown in-repo, Apache-2.0, explicit portability pitch ("your .agents/ folder comes
  with you"). Strikingly convergent with AGF's plain-file + removability stance; the closest
  competitor to AGF's exit-guarantee positioning specifically.
  Verified 2026-07-21: 414 stars, Apache-2.0 confirmed, v3.2.0 (2026-07-03), install via
  `npx skills@latest add boshu2/agentops`. Two corrections to the note above: verdicts are
  **plain JSON** (`.agents/ao/verdicts/sha256/`), not markdown; and the verdict model is a
  three-state `PASS`/`FAIL`/`NOT_PROVEN`, not a binary done/not-done. Core loop: "Plan →
  Implement → fresh Validate (separate context from the author session) → durable verdict →
  report and stop" — the fresh-context requirement is itself a form of verification
  independence (concept #4).

- https://microsoft.github.io/agent-governance-toolkit/
  type: tooling | relation: competitor(runtime layer), complement | action: compare-in-docs
  note: Microsoft OSS — YAML policy engine wrapping agent tool calls (deny / require_approval),
  framework-agnostic. Runtime-call governance, not repo-process governance; useful contrast case.
  Verified 2026-07-21: broader than originally scoped — also includes identity/trust (SPIFFE,
  DID, mTLS), tamper-evident audit logging, execution sandboxing with "four privilege rings",
  cost governance, and compliance mappings to OWASP/NIST/EU AI Act. Answers three questions:
  "Is this action allowed? Which agent did this? Can you prove what happened?" — the third is
  exactly AGF's audit-trail territory, enforced at the tool-call layer instead of the repo/
  process layer. Action upgraded from `watch` to `compare-in-docs` given the maturity found.

- https://arxiv.org/pdf/2603.26487
  type: research | relation: evidence, prior-art | action: cite
  note: "Beyond Banning AI: A First Look at GenAI Governance in Open Source Software Communities" —
  empirically studies repo-level AI governance (AGENTS.md etc.) incl. its dependence on tool
  compliance; academic support for the enforcement-gap argument and for issue #7's framing.
  Verified 2026-07-21: authors Wenhao Yang, Runzhi He, Minghui Zhou. Analyzed 67 OSS projects,
  identified 12 governance strategies spanning accountability, verification, review capacity,
  code provenance, and platform infrastructure. Correction: the abstract frames venues more
  broadly as "contribution guidelines, security policies, and repository instructions" rather
  than specifically "AGENTS.md"; examines governance *patterns*, not tool-specific compliance
  architecture in detail.

- https://arxiv.org/pdf/2604.16870
  type: research | relation: prior-art, threat-model | action: watch
  note: "Governed MCP" — treats agent tool calls as syscalls needing kernel-level governance;
  relevant to the enforcement-locus debate (userspace/prompt-level controls are bypassable).
  Verified 2026-07-21: full title "Governed MCP: Kernel-Level Tool Governance for AI Agents via
  Logit-Based Safety Primitives" (Daeyeon Son). Six-layer governance pipeline in a bare-metal
  OS kernel (schema validation, trust tiers, rate limiting, adversarial filtering, logit-based
  semantic gate). Notable result: removing the semantic-classification layer drops F1 from
  0.789 to 0.357 — concrete evidence that rule-based/hand-coded filtering alone is insufficient,
  useful ammunition for issue #3's advisory-vs-structural argument.

# Batch 2 (2026-07-15)

## Classified

- https://commandline.microsoft.com/agent-control-specification-runtime-governance/
  type: standard (emerging) | relation: complement, prior-art, competitor(runtime layer) | action: compare-in-docs, watch closely
  note: **The most strategically important link in either batch.** ACS = open, Apache-2.0,
  community-governed spec (Build 2026) for deterministic runtime agent controls — validation
  checkpoints across input/LLM/state/tool-execution/output, portable YAML policies, adapters
  for LangChain/OpenAI/Anthropic SDKs/CrewAI/MCP; Microsoft's Agent Governance Toolkit adopts
  it as policy language; IBM/KPMG/Arize/Obsidian endorsements. Uses AGF's exact argument
  ("advisory system prompts don't scale; enforcement must be structural"). Critically, third-
  party analysis flags what ACS explicitly does NOT cover: who authors the policy, who is
  accountable for the authorization decision, where that decision is recorded — i.e. the
  process/accountability layer upstream of enforcement. That upstream layer is AGF's territory.
  Positioning implication: AGF is not competing with ACS; a future where governor-approved AGF
  gate decisions compile down to ACS policy manifests is a plausible complement story.

- https://arxiv.org/pdf/2603.16586
  type: research | relation: prior-art, evidence | action: cite
  note: "Runtime Governance for AI Agents: Policies on Paths" (Kaptein et al., TU Eindhoven,
  2026-03). Formalizes compliance policies over execution paths; proves prompt-level
  instructions and static access control are weak special cases. Academic backing for issue
  #3's advisory-vs-structural framing.

- https://github.com/enforra/enforra
  type: tooling | relation: competitor(runtime layer), complement | action: watch
  note: OSS action-governance SDK (Node+Python, Apache-2.0): YAML policies →
  allow/block/require_approval/log_only before tool callbacks; local hash-chained JSONL audit,
  no telemetry; observe mode; MCP wrapping; hosted cloud tier for approvals/dashboards.
  Runtime-call layer, not repo-process layer. Two AGF-relevant data points: (a) local-file
  audit + no-telemetry as an explicit selling point validates the removability positioning;
  (b) OSS-core + hosted-approvals is a business-model reference.

- https://www.indiehackers.com/post/is-system-prompting-enough-for-production-why-i-m-building-a-runtime-governance-layer-for-ai-agents-2f57424547
  type: discussion | relation: demand-driver | action: verify
  note: Thesis and wording closely match Enforra's Show HN ("system prompts are not a security
  boundary") — plausibly the same founder's build-in-public post, unconfirmed. If so, merge
  with the Enforra entry rather than tracking separately.

- https://www.tigera.io/blog/five-principles-of-an-accountable-ai-agent-network-how-to-evaluate-any-governance-platform/
  type: practice | relation: complement, demand-driver | action: verify, then cite
  note: Vendor confirmed (Tigera/Calico, container networking-security); body not read.
  Title promises evaluation criteria for governance platforms — if sound, useful as an
  external yardstick to assess AGF against; expect network-layer bias.

- https://corpgov.law.harvard.edu/2026/05/11/ai-corporate-governance-and-ben-jerrys-risk/
  type: practice | relation: likely none | action: verify, probably ignore
  note: Venue confirmed (Harvard Law Forum on Corporate Governance); body not read. Board-level
  corporate governance of AI — different meaning of "governance" than AGF's; keep only if the
  EU public-sector audience angle needs board-level citations.

- https://thezvi.substack.com/p/cyber-lack-of-security-and-ai-governance
  type: discussion | relation: threat-model, demand-driver | action: verify, then cite
  note: Venue confirmed (Zvi Mowshowitz's newsletter — high-signal AI commentary); body not
  read. Cyber(in)security + AI governance survey; likely threat-model source material.

- https://rootcx.com/blog/ai-agent-governance-implementation
  type: practice | relation: demand-driver | action: verify
  note: Vendor blog, body not read. Implementation guidance angle; assess for overlap.

- https://rootcx.com/blog/agentic-ai-vs-ai-agents
  type: practice | relation: none (terminology explainer) | action: ignore unless
  vocabulary alignment becomes a docs concern.

- https://www.riddhimohan.com/blog/architecture-policy-compiling-governance-into-ai-stack
  type: practice | relation: prior-art(possible) | action: verify
  note: Personal essay, body not read. Title thesis — compiling governance into architecture —
  is adjacent to correct-by-construction (issue #1); read before classifying further.

## Unverified — do not classify yet

- https://lospino.so/blog/sunday-field-notes/the-workshop-has-changed/ — [unverified]. Fetch
  attempted twice (2026-07-21), both returned HTTP 403 (site blocks automated fetching). Cannot
  classify without a human-provided copy (paste, screenshot, or saved page).

All other entries formerly listed here were fetched and classified 2026-07-21 — see Batch 6
below.

# Batch 3 (2026-07-16) — standards, legal, cross-discipline, philosophies
Source: deep-research pass. Verification status per entry; regulatory dates move — recheck before citing.

## Regulations & legal (EU/Finland-first)

- https://artificialintelligenceact.eu/article/26/
  type: regulation | relation: demand-driver, complement | action: cite, integrate
  note: AI Act Art. 26 deployer duties: named-human oversight, monitoring with duty to
  suspend, and keeping auto-generated logs ≥6 months (Art. 26(6)). Direct match to governor
  role + git audit trail. High-risk applicability 2026-08-02; a provisional "Omnibus VII"
  deferral (to 2027-12/2028-08) is agreed but not yet law. [dates: verify before citing]

- https://public-buyers-community.ec.europa.eu/communities/procurement-ai/resources/updated-eu-ai-model-contractual-clauses
  type: regulation | relation: demand-driver, complement | action: integrate, cite
  note: EU Model Contractual Clauses for AI procurement (MCC-AI, upd. 2025-03-05): public
  buyers get rights to technical documentation, logs, record-keeping, audits. **Strongest
  demand-driver found** — AGF artifacts ≈ the required vendor deliverables. Motivates
  compliance-export (#9) and buyer-side AGF (#10). [clause structure via secondary source]

- https://digital-strategy.ec.europa.eu/en/policies/cra-open-source
  type: regulation | relation: demand-driver, threat-model | action: verify, cite
  note: CRA "open-source steward" (Art. 24): cybersecurity policy + vuln reporting, no fines;
  reporting from 2026-09-11, full applicability 2027-12-11. Key correction: Art. 3(14)
  requires a *legal person* — a solo natural-person maintainer cannot be a steward; the
  question only arises if AGF is stewarded via an entity. → new issue #13. [partially
  unverified — based on draft Commission guidance 2026-03-03]

- https://digital-strategy.ec.europa.eu/en/policies/nis2-directive
  type: regulation | relation: demand-driver | action: cite, compare-in-docs
  note: NIS2 Art. 21: supply-chain security + compliance documentation, flowed down to
  suppliers by contract. In-scope public-sector adopters will demand auditable supply-chain
  records — AGF's trail is such documentation.

- https://www.traficom.fi/en/news/cybersecurity-act-passed-parliament-obligations-under-nis-2-directive-enter-force-8-april-2025
  type: regulation | relation: demand-driver | action: cite
  note: Finland: Kyberturvallisuuslaki 124/2025 (in force 2025-04-08), Traficom/NCSC-FI as
  SPOC+CSIRT. Home-jurisdiction reference for docs.

- https://valtioneuvosto.fi/en/-/1410877/
  type: regulation | relation: demand-driver | action: cite
  note: Finland's decentralised AI Act supervision: sectoral authorities, Traficom SPOC +
  sandbox, powers from 2026-01-01 (HE 46/2025), sanctions board at Traficom.

- https://www.freshfields.com/en/our-thinking/blogs/risk-and-compliance/product-risks-today-how-the-new-product-liability-directive-turns-ai-act-complia-102mpu2
  type: regulation | relation: demand-driver, threat-model | action: cite
  note: New PLD (EU) 2024/2853 (transposition by 2026-12-09): software/AI as products, strict
  liability, rebuttable defectiveness presumption, mandatory evidence disclosure. Auditable
  process records become a *defensive legal asset*. Non-commercial OSS excluded; monetised
  OSS in scope.

- https://hoop.dev/blog/why-gdpr-compliance-lives-in-your-git-history
  type: regulation | relation: threat-model | action: integrate
  note: **Overrides earlier assumption in issue #8.** Git-stored audit trail with author
  identities = GDPR personal-data processing; append-only history resists erasure (Art. 17)
  — a real tension with the exit-guarantee/append-only design. Must be resolved (lawful
  basis, identity minimisation, documented erasure story) before issue #8 ships.

## Standards

- https://www.iso.org/standard/42001
  type: standard | relation: complement, demand-driver | action: integrate, compare-in-docs
  note: ISO/IEC 42001 AI management systems; AGF produces the operational evidence a 42001
  audit consumes. Primary compliance-export target. Ecosystem: 42005/42006/23894/5338.

- https://csrc.nist.gov/pubs/sp/800/218/a/final
  type: standard | relation: complement, prior-art | action: cite, compare-in-docs
  note: NIST SP 800-218A — GenAI profile of the SSDF; explicitly addresses producers,
  integrators AND acquirers. Acquirer framing supports buyer-side AGF (#10).

- https://www.nist.gov/itl/ai-risk-management-framework
  type: standard | relation: complement, demand-driver | action: cite
  note: NIST AI RMF + GenAI profile; Govern/Map/Measure/Manage mapping target. [profile
  identifier unverified]

- https://slsa.dev/blog/2023/05/in-toto-and-slsa
  type: standard | relation: complement, prior-art | action: integrate
  note: in-toto attestations / SLSA provenance. AGF approval artifacts and sender identity
  (#4) are conceptually in-toto attestations — align schema to in-toto predicate types
  instead of inventing a format; enables existing verifiers to consume AGF output.

- https://www.w3.org/TR/prov-dm/
  type: standard | relation: complement, prior-art | action: integrate
  note: W3C PROV entities/activities/agents = the mature model for AGF's audit-trail
  semantics; PROV-AGENT extends it for agents. Describe the trail in PROV terms.

- https://github.com/CycloneDX/specification/issues/895
  type: standard | relation: complement, demand-driver | action: watch
  note: CycloneDX ML-BOM + open "Agent BOM" proposal (MCP servers, tools, authorisations).
  Target format for input-provenance governance (concept #6). OWASP AIBOM adjacent.

- https://openssf.org/blog/2024/11/25/understanding-the-cra-openssfs-role-in-the-cyber-resilience-act-implementation-part-1/
  type: standard | relation: complement | action: watch
  note: OpenSSF Scorecard / Best Practices Badge = cheap external legibility signals for the
  AGF repo itself; ORC WG = CRA guidance reference.

- https://www.ietf.org/archive/id/draft-gudlab-agentid-protocol-00.html
  type: standard | relation: prior-art, competitor | action: watch
  note: Agent-identity standards race (IETF AIMS/WIMSE, SPIFFE, ID-JAG, Entra/Google Agent
  ID) — immature (AIMS security section: "TODO"). Constrains #4: keep identity abstract,
  bind later to SPIFFE-style IDs. Do not integrate yet.

- https://owasp.org/www-project-top-10-for-large-language-model-applications/
  type: standard | relation: threat-model | action: cite
  note: OWASP LLM Top 10 / AI Exchange = threat vocabulary for the declared threat model
  (#11). [URL slug unverified]

## Frameworks & concepts from other disciplines

- https://books.google.com/books?vid=ISBN0201001748
  type: research | relation: prior-art | action: cite
  note: Chris Argyris and Donald A. Schön, *Organizational Learning: A Theory
  of Action Perspective* (Addison-Wesley, 1978, ISBN 0201001748). Establishes
  the single-loop/double-loop distinction: correcting action within existing
  governing assumptions versus questioning policies, objectives, and norms.
  AGF's rule and authority amendments are a double-loop analogue; this is a
  conceptual relationship, not empirical validation. Verified 2026-07-21 via
  publisher bibliographic metadata indexed by Google Books.

- https://deming.org/explore/pdsa/
  type: framework | relation: prior-art | action: cite
  note: W. Edwards Deming Institute description of Plan–Do–Study–Act as a
  repeated learning and improvement process in which predictions are tested
  against observed results and the theory or method is adjusted. Supports
  AGF's explicit implementation-to-observed-use feedback step. Verified
  2026-07-21 from the Deming Institute.

- https://sunnyday.mit.edu/CAST-Handbook.pdf
  type: framework | relation: prior-art, threat-model | action: cite, integrate
  note: Nancy Leveson's CAST Handbook describes system-theoretic causal
  analysis of adverse events, focusing on why designed controls did not prevent
  loss and what control changes prevent recurrence rather than reducing the
  event to one root cause. Supports an explicit causal/control-analysis stage
  between AGF learning records and governance proposals. Verified 2026-07-21
  from MIT PSASS materials.

- https://www.iso.org/standard/42001
  type: standard | relation: prior-art, demand-driver | action: cite
  note: ISO/IEC 42001:2023 specifies establishing, implementing, maintaining,
  and continually improving an AI management system. ISO's public explanation
  describes its continual-improvement cycle; AGF artifacts may supply evidence
  to such a process but do not establish conformity or certification. Verified
  2026-07-21 from ISO.

- maker-checker / four-eyes principle (banking)
  type: framework | relation: prior-art | action: integrate
  note: AGF's implementer/reviewer split IS maker-checker: no self-approval, atomic pending
  state, logged maker+checker. Adopt the vocabulary → instant legibility to auditors;
  precise semantics for #3.

- IIA Three Lines (of Defense) Model
  type: framework | relation: prior-art | action: compare-in-docs
  note: Implementer=1st line, reviewer/governor=2nd, audit-trail-enabled independent
  review=3rd. Canonical accountability taxonomy; clarifies where the governor sits.

- ISO 27001 Annex A 5.3 segregation of duties
  type: framework | relation: prior-art, demand-driver | action: integrate
  note: AGF role model = SoD. Gap exposed: emergency-bypass ("break glass") + post-incident
  reconciliation — AGF has no single-operator/P1 story yet.

- GSN assurance cases (scsc.uk/gsn)
  type: framework | relation: prior-art | action: compare-in-docs, watch
  note: AGF's trail is evidence without an argument structure; a GSN-style "governance case"
  (claim→strategy→AGF artifacts as evidence) would connect AGF to formal assurance practice.

- STAMP/STPA (Leveson; arxiv.org/html/2506.01782v1 for AI application)
  type: framework | relation: prior-art, threat-model | action: compare-in-docs
  note: Safety as control problem; STPA = rigorous method to derive gate placement and the
  threat model (#11); CAST = post-incident analysis for the incident loop (concept #3).

- Ostrom commons principles (sustainoss.pubpub.org/pub/jqngsp5u)
  type: framework | relation: prior-art | action: cite
  note: Eight design principles (collective choice, graduated sanctions, nested enterprises)
  = design language for the meta-governance/amendment process (concept #1) and minimalism (#12).

- policy-as-code lineage (OPA/Rego, Cedar, OpenID AuthZEN)
  type: framework | relation: prior-art, complement | action: compare-in-docs
  note: Mature allow/deny engines. AGF is *process*-governance-as-code (sequencing +
  accountability), a different layer — state the contrast explicitly; consider
  AuthZEN-compatible decision logs. [AuthZEN finalisation date unverified]

- capability-based security; reproducible builds; GitOps declarative philosophy
  type: concept | relation: prior-art | action: compare-in-docs
  note: Capabilities > ACLs for ephemeral delegating agents (informs #1/#4 thinking).
  Reproducible-builds + GitOps "repo as single source of truth" = direct prior art for the
  removability principle (#8) — cite them.

- checklist philosophy (Gawande / WHO surgical checklist)
  type: concept | relation: prior-art, evidence | action: cite
  note: Evidence that mandatory pre-action gates reduce error — AND documents checklist
  fatigue/box-ticking, the failure mode of both advisory gates and gate proliferation (#12).

## AI-specific emerging work

- https://arxiv.org/html/2606.19544v1 (+ JudgeBiasBench, RAND Judge Reliability Harness)
  type: research | relation: evidence, threat-model | action: cite, integrate
  note: LLM-as-judge unreliability: >50% error on advanced bias tests; no judge uniformly
  reliable; self-enhancement bias. Direct evidence that a model-performed reviewer role
  cannot be trusted alone → verification independence (concept #4) is evidence-backed.

- https://arxiv.org/pdf/2603.21692 (reasoning provenance / PROV-AGENT)
  type: research | relation: prior-art, evidence | action: watch
  note: Observability traces ≠ tamper-evident audit trails. Validates AGF's durable
  version-controlled Markdown over ephemeral logs; artifact-centric coordination grounding.

- Sigstore/Rekor keyless signing (+ attested-build research cluster)
  type: research | relation: prior-art | action: watch
  note: OIDC-tied keyless signing could give gate decisions cryptographic non-repudiation on
  top of plain files, without a key registry (removability-compatible). [trend evidence]

- https://atlan.com/know/ai-agent/ai-agent-identity/ ("know your agent", NIST NCCoE concept paper)
  type: discussion | relation: prior-art, demand-driver | action: watch
  note: Enterprise agent-identity market converging on workload identity + per-agent policy +
  trails. #4 should *bind to* enterprise identities, never own an identity store.

## Philosophies

- Scott, *Seeing Like a State* — legibility critique
  type: concept | relation: threat-model | action: integrate
  note: Making work legible to governance can distort the work and destroy tacit knowledge.
  The central philosophical risk of AGF itself; basis for the legibility-first vs gate-first
  declaration (concept #10). Epistemic status: theoretical.

- Davies, *The Unaccountability Machine* — accountability sink
  type: concept | relation: threat-model | action: integrate
  note: "The process approved it" absorbs responsibility and scrambles feedback. Design test:
  every gate must resolve to a named human who could have decided otherwise → grounds
  concept #2.

- Bainbridge, "Ironies of Automation" (1983) + automation-bias literature
  type: concept | relation: threat-model, evidence | action: integrate, cite
  note: Human approvers de-skill and rubber-stamp; residual tasks get harder. Strongest
  argument that #3's gates need anti-rubber-stamping design (forced justification,
  sampling, independence) — a click-recorder is not a control.

- Goodhart's law
  type: concept | relation: threat-model | action: integrate
  note: Any enforced gate metric will be gamed. Gate metrics are diagnostic, never targets;
  structural defence = minimalism + sunsets (#12).

- psychological safety vs surveillance (Edmondson)
  type: concept | relation: threat-model | action: watch
  note: An all-activity trail implicitly surveils humans too, suppressing candour. Bound
  explicitly what is trailed and why; ties to the GDPR entry. [theoretical]

- proceduralism vs outcome-based regulation
  type: concept | relation: none | action: compare-in-docs
  note: AGF is procedural; EU regulation trends outcome/risk-based. Position AGF as producing
  *evidence for* outcome accountability, never as guaranteeing outcomes.

## Pre-identified concepts (from 2026-07-16 discussion, now filed)

- concept #1 meta-governance/amendment process — type: concept | relation: prior-art |
  action: integrate | note: Auditable workflow for changing AGF's own rules (Ostrom:
  collective choice). Without it the governor is an unaccountable rule-maker. → new issue #12.
- concept #2 human accountability as first-class object — type: concept | relation:
  prior-art, complement | action: integrate | note: Named human owner per gate/rule in the
  trail (Davies' test). The gap ACS explicitly leaves open — AGF's differentiator.
- concept #3 incident workflow + governance evolution loop — type: concept | relation:
  prior-art | action: integrate | note: incident → CAST-style control-failure analysis →
  rule amendment. Also NIS2/CRA incident-duty adjacent.
- concept #4 verification independence — type: concept | relation: prior-art, evidence |
  action: integrate, cite | note: Reviewer must be demonstrably independent (session/model/
  vendor), recorded in the artifact. Evidence-backed by LLM-as-judge research.
- concept #5 time/budget as governance primitives — type: concept | relation: prior-art |
  action: integrate | note: Cooling-off, rate limits, token budgets as gate conditions —
  semantics-free throttles against machine speed.
- concept #6 input provenance governance — type: concept | relation: prior-art,
  threat-model | action: integrate | note: Instruction files/skills/MCP servers as governed
  supply chain; record versions in effect per action; Agent-BOM target format.
- concept #7 compliance-export generator — type: concept | relation: demand-driver,
  complement | action: integrate | note: AGF artifacts → MCC-AI / AI Act Art. 26 / ISO
  42001 / SSDF evidence packs. Highest-leverage roadmap item. → new issue #9.
- concept #8 buyer-side AGF — type: concept | relation: demand-driver | action: integrate |
  note: Profile + verification tooling for public buyers requiring an AGF trail from
  vendors. Mirror of #7; enabled by MCC-AI audit rights. → new issue #10.
- concept #9 non-code domain applicability — type: concept | relation: none | action:
  watch | note: State the design boundary; keep schema code-agnostic; not near-term.
- concept #10 legibility-first vs gate-first declaration — type: concept | relation:
  threat-model | action: integrate | note: Foundational stance, stated with epistemic
  status; informs how aggressively advisory gates become enforced ones.
- concept #11 declared threat model — type: concept | relation: threat-model | action:
  integrate | note: Cooperative-but-fallible vs adversarial, built with STPA + OWASP
  vocabularies. Prerequisite for honest guarantees. → new issue #11.
- concept #12 governance minimalism + sunset clauses — type: concept | relation: prior-art |
  action: integrate | note: Every rule carries expiry/review date; defence against Goodhart,
  checklist fatigue, and AGF becoming the bureaucracy it replaces.

## New issue candidates from batch 3

- #9 compliance-export generator (MCC-AI first, then ISO 42001, SSDF)
- #10 buyer-side AGF profile + verification tooling
- #11 declared threat model (STPA-derived)
- #12 governance meta-process: amendment workflow + sunset clauses
- #13 CRA entity-structure decision (legal person or not; if entity + in scope: policy
  published before 2026-09-11)
- amendment to #8: resolve GDPR erasure vs append-only tension before publishing the
  exit guarantee

# Batch 4 (2026-07-21) — vektorgeist.com/blog, fetched and classified

Vendor: VektorGeist, promoting their product "Aviary" (local-first agent runtime). All
six posts carry agent bylines ("Vanta", "Elif") rather than named human authors — treat
as AI-drafted marketing copy; verify specific claims before citing per
verifiable-claims-rule. Common thread: governance/privacy enforced by keeping model,
orchestration, and logs on the user's own machine rather than vendor infrastructure.

## Classified

- https://vektorgeist.com/blog/local-first-agent-governance (2026-06-28)
  type: tooling | relation: competitor(runtime layer), complement | action: compare-in-docs
  note: The directly relevant entry. Argues governance should concentrate strictness at
  the moment an agent acts on something external to itself (irreversible actions), via
  least-privilege default-deny, explicit approval gates, credential-scrubbed logs,
  tamper-proof audit records, and a kill switch — all enforced locally ("governance you
  don't control isn't governance"). Same category as the Microsoft ACS and enforra
  entries above: a runtime tool-call enforcement layer, not a repo/process layer —
  complements rather than competes with AGF directly. Worth a compare-in-docs mention
  alongside ACS/enforra to show AGF's process layer vs. this client-enforcement layer.

- https://vektorgeist.com/blog/run-ai-agents-offline (2026-06-30)
  type: tooling | relation: competitor(runtime layer), complement | action: compare-in-docs
  note: Companion post to the above. Splits "offline agent" into model execution,
  orchestration, and memory/logs, arguing governance (scope control, outbound-action
  gating, secret hygiene, audit trails) is the hard part, not the model. Independent
  restatement of "verify system state directly, don't trust agent summaries" — corroborates
  AGF's audit-trail-over-self-report stance from an unrelated vendor.

- https://vektorgeist.com/blog/what-local-first-actually-means (2026-07-15)
  type: practice | relation: prior-art | action: watch
  note: Defines "local-first" against diluted industry usage — architectural (not
  policy-dependent) privacy via local execution; briefly mentions "agent guardrails"
  functioning locally being structurally stronger than network-dependent ones. Tangential
  to AGF's removability/plain-files stance but focused on runtime data locality, not
  process governance.

- https://vektorgeist.com/blog/design-agent-context-icm (2026-07-06)
  type: practice | relation: prior-art | action: watch
  note: "ICM" — folder-based, layered (L0 root router … L4 per-run scratch) context
  architecture replacing orchestration code with markdown files, explicitly for human
  readability/auditability over prompt bloat. Orthogonal to AGF (context-window
  engineering, not gates) but shares the "readable files over opaque logic" philosophy —
  same instinct as AGF's plain-markdown coordination trail.

- https://vektorgeist.com/blog/search-claude-code-transcripts-locally (2026-06-26)
  type: tooling | relation: none | action: ignore
  note: Introduces magpie-search, a local keyword+semantic index over Claude Code
  transcripts. Useful utility, no governance content; out of scope for AGF.

- https://vektorgeist.com/blog/why-we-build-local-first (2026-06-24)
  type: practice | relation: none | action: ignore
  note: Company philosophy/build-log post (privacy and ownership rationale for
  local-first architecture). Mentions "governing an autonomous agent" only as a passing
  example with no detail. Background only.

# Batch 5 (2026-07-21) — twmrg.substack.com, full text fetched (user-supplied, tmp/papers/twmrg.md)

- https://twmrg.substack.com/p/the-governance-gap ("The Governance Gap", Alfino Hatta, 2026-07-08)
  type: discussion | relation: demand-driver, evidence, threat-model | action: cite (with caveat)
  note: Secondary-source essay dissecting a vendor conference talk (vendor and speaker not
  named in the piece). Central claim — ~80% of PRs at some orgs get no human review — is
  **unverified per the essay's own account**: one vendor's internal analysis of its own
  product's usage, paired with a self-selected survey (~617 companies, all existing
  customers of that vendor). Per verifiable-claims-rule, do not cite the 80%/70%/47%/88%
  figures as established fact — attribute explicitly as "reported at a vendor conference
  talk, methodology unpublished" if cited at all, and prefer the more concrete two: 70% of
  scanned PRs had ≥1 high-severity issue (operational scan data, not self-report), and only
  ~60% of review conversations stay inside auditable tooling (GitHub/GitLab/Bitbucket) vs.
  ~40% leaking into Slack/Teams outside any audit trail.
  Independent of the disputed number, the essay's *structural* argument is directly useful
  and doesn't depend on the 80% figure being true: compliance controls (SOC2, ISO 27001)
  can be satisfied on paper by an AI-only reviewer with no human involved; insurance
  underwriting, board reporting, vendor/supplier attestations, and litigation discovery all
  currently have no mechanism to distinguish "AI approved" from "human approved" — direct
  support for concept #2 (named human accountability as first-class object) and the
  compliance-export item (issue #9). Also independently corroborates the framework's
  advisory-vs-structural argument via a historical parallel (audit standards lagging
  CI/CD adoption; financial straight-through-processing requiring documented control
  points and kill switches only after regulatory pressure post-adoption) — same shape as
  the Kaptein et al. and ACS entries above (Batch 2/3).
  Also flags a genuinely new gap not otherwise present in this registry: multi-agent
  **disagreement arbitration** (coding agent vs. testing/planning agent reaching different
  verdicts, no defined resolver) — candidate addition to concept #3/#4 or a new concept.
  Caveat: essay author's own methodology is a close reading of one transcript, not
  independent verification; treat as opinion/analysis, not a primary data source.

# Batch 6 (2026-07-21) — remaining [unverified] entries from Batches 1 and 2, fetched

Resolves every previously-unverified entry except `lospino.so` (blocked by the site, see the
`## Unverified` note under Batch 2 above). Headline finding: the "runtime tool-call governance
layer for AI agents" space is considerably more crowded than Batches 1-3 suggested — six
separate, independently-built projects doing local/self-hosted pre-execution gating +
audit-trail recording turned up just from following named-but-unverified links. None compete
with AGF directly (all are runtime/tool-call layer; AGF is repo/process layer, same distinction
already drawn for ACS, enforra, and the VektorGeist/Aviary entries) — but the crowding itself is
worth a line in AGF's own positioning docs: the market has converged hard on "wrap the tool
call," which is exactly the layer AGF and the "Governed MCP" paper (Batch 3 suggested additions)
argue is necessary-but-insufficient without the accountability layer above it.

## Classified

- https://github.com/geludobre/sovereign-agentops
  type: tooling | relation: competitor(runtime layer) | action: compare-in-docs
  note: Self-hosted MCP governance server: "constitutional governance," Ed25519-signed
  execution receipts for the audit trail, workspace path confinement, local model routing,
  offline/air-gapped support, EU AI Act-aligned compliance tooling claimed. 7 MCP tools
  (Community, Apache-2.0) vs. 91 tools (paid Enterprise). Early (1 star, v2.4.1 as of
  2026-07-12) but functional and released. The signed-receipt approach parallels the
  Sigstore/Rekor entry in Batch 3 — worth grouping in a "cryptographic non-repudiation for
  gate decisions" comparison alongside it.

- https://github.com/irenee28/akm-clr-governance-layer
  type: tooling | relation: competitor(runtime layer, narrow) | action: watch
  note: Multi-tenant control layer above vLLM-style OpenAI-compatible inference engines —
  tenant authorization, domain routing, pre-inference blocking, output guards, audit logging.
  Governs *inference-request tenancy*, not coding-agent or repo-process workflows — different
  problem than AGF's. Prototype stage (8 commits, 1 star); watch only.

- https://coderadius.ai/
  type: tooling | relation: complement | action: watch
  note: Cross-repo architecture dependency graph + "blast radius" diff analysis, exposed to
  coding agents via MCP; also a CI gate blocking PRs that violate architecture policy. Local
  analysis, no source upload, Apache-2.0, public alpha 2026. Different axis (architectural
  awareness for agents) than AGF's process governance — complementary; a coding agent informed
  by CodeRadius could still go through an AGF-gated review.

- https://github.com/MAVS-RESEARCH
  type: research | relation: prior-art | action: watch
  note: Academic/research GitHub org, 9 Python/MIT repos under a "MAVS-GC" (governance +
  consensus) research program — inference-time safety governance for multi-model systems,
  certified updates/rollback, counterexample-driven diagnostic learning, false-rejection-rate
  reduction. No external validation or publication venue visible from the repos themselves;
  treat as an unreviewed research program, not citable evidence, until a paper or independent
  reproduction surfaces. Thematically adjacent to concept #4 (verification independence) and
  the LLM-as-judge-unreliability entry in Batch 3.

  **2026-07-21 deep dive — https://github.com/MAVS-RESEARCH/MAVS-Chapter-9:** MAVS-GC is a
  *governed aggregation pipeline* for combining outputs from multiple specialist agents:
  per-specialist bounded scores [0,1], diagnostics for disagreement/corruption/overconfidence/
  inconsistency, weighted aggregation with contextual rebalancing, a hard-veto path, and a
  final governed-consensus decision. This is directly the same problem twmrg's essay (Batch 5)
  flagged as unsolved industry-wide — "who or what resolves it when a coding agent and a
  testing agent disagree" — MAVS-GC is a candidate mechanism for that arbitration step,
  relevant to concept #3/#4.
  Caveats, and they're significant: 0 stars, 0 forks, **2 commits**, MIT, no individual authors
  credited, no linked paper/dataset/external publication. The repo's own disclaimer, quoted
  verbatim: "This repository does not claim performance superiority" — it benchmarks against
  synthetic, deterministically-generated cases only (mean ensemble / static weighted ensemble /
  "Veto MAVS" baselines), not real agent transcripts or real disagreement data. Chapter-9 is
  one of an announced "Foundation Arc" (chapters 1–8 referenced but not linked from this repo).
  Read as: an interesting *design pattern* to track, not yet evidence of anything — action
  stays `watch`, not `cite`, until a chapter ships with real (non-synthetic) validation or an
  external paper appears.

  **2026-07-21 deep dive — https://github.com/MAVS-RESEARCH/MAVS-Self-Learning-1A:** The most
  substantial single artifact found in this org — 64 commits, a frozen/pinned release
  (`perception_closure_v04_phase10_20260718_r3`), extends the Chapter-10D foundation through
  Phases 0-10. Mechanisms of direct AGF interest: **"blind certification and anti-gaming
  controls"** — a concrete attempt at the exact problem named in Batch 3's Goodhart's-law entry
  (gate metrics get gamed once they become targets); a **"Perception Closure runtime"** that
  requires explicit hypotheses and targeted evidence acquisition before a claim is accepted,
  relevant to concept #4 (verification independence) and the LLM-as-judge-unreliability entry;
  and **immutable, hash-bound, Ed25519-signed audit trails** — same cryptographic-non-repudiation
  idea as the Sigstore/Rekor entry (Batch 3) and sovereign-agentops' signed receipts (this
  batch), now a third independent instance of the pattern.
  Claimed results (self-reported, not externally audited): Phase 10 internal audit passed all
  25 WorkPlan clauses with zero findings; 411 regression tests (203 in a clean pinned
  environment); "600 certification gates and 6,624 Phase 9 metric values independently
  recompute with zero mismatch." Take these as self-reported until reproduced independently.
  What earns real credit: the authors' own scope disclaimer, quoted verbatim — **"Universal
  zero-error, zero-escalation outside the covered class, production deployment readiness, and
  general-domain performance are unsupported."** That is precisely the discipline this project's
  own implementation-honesty-rule and verifiable-claims-rule ask for, and it's rare to see a
  research repo state it this plainly. Still: 0 stars/forks, no external paper, single-org
  self-publication — treat the mechanism ideas as worth studying, the results as unverified.

- https://splabs.io/compliance
  type: tooling | relation: complement, competitor(evidence layer) | action: compare-in-docs
  note: Silicon Psyche Labs' "Posture Sequence Analysis" (PSA) — a behavioral-evidence layer
  mapped against 12 in-force 2026 regulatory frameworks (EU AI Act, NIST AI RMF, ISO, etc.),
  six evidence primitives (event logging, tamper-evident integrity, adversarial robustness
  testing, human-AI interaction risk, continuous monitoring, behavioral transparency).
  Explicitly scopes itself as reading "what a model does, from its output text, from the
  outside," and explicitly disclaims procedural/organizational governance and bias detection.
  That explicit disclaimer is the same shape as the ACS gap noted in Batch 3 — PSA measures
  model *output behavior*; AGF records *process/accountability*. The two are complementary,
  not competing, and the contrast is worth stating directly in AGF's docs.

- https://omega-lang.org/
  type: framework | relation: prior-art, competitor(policy-language layer) | action: compare-in-docs
  note: A formal governance *language* — typed `.omega` specs, load-time checked, parsed by a
  "Safety Kernel" that returns ALLOW/DENY/UNKNOWN before an action executes; 13 "constitutional
  primitives" (context, temporal, resource-bound, environment-interface, data-schema,
  state-transition, trust, governance, mutation rules); inspired by TLA+ and Cedar. This is a
  new entrant in the policy-as-code lineage already logged in Batch 3 (OPA/Rego, Cedar,
  AuthZEN) — file alongside those rather than as a standalone category; the load-time
  ALLOW/DENY/UNKNOWN verdict model is a close cousin of the runtime-gate pattern seen across
  this whole batch.

- https://github.com/jacobthomasmichael/GatekeeperAI
  type: tooling | relation: competitor(adjacent), complement | action: compare-in-docs
  note: Resolves the Batch 2 speculation — the real repo is confirmed at this URL, but it is
  **not** the CI-policy-as-code linter guessed at (the `ai-governance`/`eu-ai-act`/
  `nist-ai-rmf`/`iso-42001` topic tags are not present). Actual project: a self-hosted platform
  for scanning, approving, and deploying internal AI *applications* — five-stage pipeline
  (submit → concurrent scanners for secrets/vuln/egress/PII/LLM-code-review → human approval →
  isolated Docker deploy → secret injection). MIT, FastAPI/Next.js/Celery, 2 stars, v0.2 as of
  2026-06-16. Adjacent to AGF's gate concept (mandatory human approval before deploy) but scoped
  to AI-application deployment, not coding-agent change review.

- https://loadlore.dev/
  type: tooling | relation: complement, competitor(practice-layer) | action: compare-in-docs
  note: Registry/control-plane for capturing engineering practices into versioned "harnesses"
  (skills, tools, hooks, configs) and distributing them across multiple coding agents (Claude
  Code, Cursor, Copilot, custom); tracks which teams run which version and flags drift.
  Tagline: "Capture once. Distribute everywhere. Govern continuously." This governs *which
  standards/rules are installed and in sync*, not *whether a given change passed a gate* — a
  distribution-layer complement to AGF's enforcement-layer, not a competitor. Directly relevant
  to AGF's own "adopters copy `.claude/rules/` into their tool's directory" distribution
  problem — worth watching as a potential distribution mechanism for AGF's own rule sets.

- https://www.aicounciltoolkit.com/
  type: practice | relation: prior-art, complement | action: compare-in-docs
  note: Open-source, implementation-focused toolkit for standing up an org-level AI governance
  *council* — six-stage cycle (intake, triage, review, approval, monitoring, refresh) with
  templates for executive sponsors, legal/compliance, product/engineering, security, and
  program-management roles. Operates at the org-committee level (which AI use cases get
  approved to exist at all), not the per-commit/per-change level AGF gates. Same "upstream
  org-governance vs. AGF's per-change gate" distinction already drawn for the Harvard Corporate
  Governance Forum entry in Batch 3 — complementary layer above AGF, cite as the org-level
  counterpart when positioning AGF as the change-level layer.

- https://trust.aos-1.com/whitepapers/aos-1-governance
  type: practice | relation: demand-driver | action: verify further
  note: Vendor whitepaper, title confirmed as "AOS-1 — Is your AI ready for the EU AI Act?"
  (product: AOS-1). WebFetch could retrieve only the title/framing, not the body — the page
  likely gates full content behind a form or renders it client-side. Confirmed as EU AI Act
  compliance-readiness content, directly relevant to the same demand-driver cluster as the
  AI Act Art. 26 and MCC-AI entries in Batch 3, but no specific claims can be verified yet.
  Needs a direct PDF fetch or human-supplied copy before citing.

- https://pypi.org/project/sentience-governor/
  type: tooling | relation: competitor(runtime layer) | action: compare-in-docs
  note: Local governance wrapper for AI agents (Crescere Labs, Inc.) — intercepts execution at
  system/tool-call boundaries, evaluates against 5 default policy rules (POL-001–005), tracks
  intent-declared-vs-actual scope and token usage, produces local audit traces, optional Claude
  Code MCP integration, LangChain support. Apache-2.0, actively developed (13 releases,
  2026-04 to 2026-07, latest 0.3.0). Framing — "hold each agent to the intent and scope it
  declares, keep a verifiable record of what it did, and flag anything out of bounds" — is
  close in spirit to AGF's audit trail, but again at the tool-call layer, not the
  review/approval layer.

- https://github.com/AIObuilt/TaG
  type: tooling | relation: competitor(runtime layer) | action: compare-in-docs
  note: "Trust and governance framework" for AI coding agents — 21 pre-execution governance
  hooks blocking dangerous actions (payment API calls, credential leaks, unvalidated
  deployments), cost-aware local-first model routing, local audit logging with no external
  dependencies, platform-agnostic (Claude/GPT/Ollama). Business Source License 1.1 (converts
  to Apache-2.0 after a change date) — an eventual-open-source model worth noting as a
  business-model reference point alongside the enforra entry in Batch 3. 11 stars, v0.1.0
  (2026-04-29).

- https://github.com/humancoreai/Gategraph
  type: tooling | relation: competitor(runtime layer) | action: compare-in-docs
  note: The most substantial find in this batch — 2,627 commits, v0.28.2_STABLE, and a
  dedicated governance-documentation set (GOVERNANCE.md, SECURITY_MODEL.md, ARCHITECTURE.md,
  INVARIANTS.md, OWASP_AGENTIC_AI_MAPPING.md). "Fail closed" deterministic governance layer:
  evaluates whether an agent action may proceed *before* execution, issues capability tokens
  from policy evaluation, records evidence-first for audit, supports deterministic replay and
  explanation of past decisions. Local, single-node. The explicit OWASP Agentic AI mapping and
  the evidence-first/replay design make this the closest runtime-layer analog to AGF's own
  audit-trail philosophy found across all six batches — recommend a dedicated compare-in-docs
  section (alongside CORE and boshu2/agentops as the "closest competitors" cluster), specifically
  contrasting "deterministic replay of a single agent's actions" (Gategraph) against "a
  human-reviewed, multi-agent coordination trail" (AGF).

- https://github.com/simonhansedasi/ai_glue
  type: tooling | relation: competitor(runtime layer, narrow) | action: watch
  note: Observability/governance proxy sitting between applications and OpenAI/Anthropic APIs —
  call logging (provider, model, tokens, cost, latency), spend visibility by project/team, PII
  detection with review workflow, hard rules (model allowlists, cost caps, rate limits),
  multi-instance aggregation, MIT. Governs LLM-API cost/spend and PII exposure, not
  coding-agent workflow or code review — narrower and more tangential to AGF than the other
  entries in this batch; watch only.
