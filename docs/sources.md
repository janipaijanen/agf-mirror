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

# Batch 7 (2026-08-02) — general web search, lighter verification pass

**Verification caveat, stated up front:** unlike Batches 1-6, these entries were found via
general WebSearch queries plus targeted WebFetch on two of them (agentic-os, AI-Governor-
Framework), not the systematic per-repo star/license/commit-history verification applied
throughout the prior batches. Treat star counts, license terms, and feature claims below as
unverified pending a closer pass — the classification itself (type/relation/action) is a
first read, not a confirmed one. Motivation: a request to scan for AGF siblings/competitors
during idle time while waiting on an unrelated Gate 2 pipeline; conceptual queries only, no
external destination received AGF's own name or specifics (external-anonymization-rule).

## Classified (light verification — see caveat above)

- https://github.com/KbWen/agentic-os
  type: framework | relation: competitor, prior-art | action: compare-in-docs
  note: Closest single match found to AGF's own shape: drop-in rules for Claude Code/Codex/
  Cursor/Copilot/Antigravity via AGENTS.md, risk-scaled phase workflows (tiny-fix through
  feature/hotfix), evidence-gated commits (credential scan, real test-suite run, phase-chain
  validation via `validate.sh`), shared coordination state file
  (`.agentcortex/context/current_state.md` + per-branch work logs) with single-writer locking.
  Verified via WebFetch (2026-08-02): explicitly **machine-enforced only** — no mandated human
  approval step; git hooks and CI substitute for human judgment on compliance. That is the
  central axis distinguishing it from AGF's Gate 1/Gate 2 human-decision model. MIT-licensed
  per the fetched page.

- https://github.com/alecnielsen/adversarial-review
  type: tooling | relation: competitor(narrow), prior-art | action: compare-in-docs
  note: Claude + GPT Codex independently review code, then critique each other across
  independent-review / cross-review / meta-review / synthesis rounds until consensus or a
  round limit. Verified via WebFetch (2026-08-02): directly the same two-named-AI-model
  adversarial pattern this project's own contribution-auditor practice uses (Codex reviewing
  Claude's implementation independently), but with no human approval gate, no persistent
  append-only audit trail, and no distinct named roles beyond the two reviewer identities —
  Claude self-synthesizes the final call. Worth a compare-in-docs specifically contrasting
  "debate-to-consensus, self-synthesized" against AGF's "independent review recommends,
  named human governor decides."

- https://www.augmentcode.com/guides/bmad-method-ai-development (BMAD-METHOD)
  type: framework | relation: competitor, prior-art | action: compare-in-docs
  note: "Breakthrough Method for Agile AI-Driven Development" — full named-role virtual team
  (Analyst, PM, Architect, Developer, QA agents), structured handoffs, human-vetted planning
  gate before implementation ("Agentic Planning" + "Context-Engineered Development"),
  auditable artifacts per phase. Closest match found on the named-role-plus-human-gate axis;
  not independently verified whether coordination between roles is an append-only file
  channel or a different state-passing mechanism, or whether an independent adversarial
  review role exists distinct from QA. One of the most frequently cited spec-driven/agentic
  frameworks in 2026 roundups alongside GitHub Spec Kit, Kiro, and OpenSpec — treat as a
  well-known reference point in this space regardless of the verification gap.

- https://github.com/Fr-e-d/AI-Governor-Framework
  type: framework | relation: competitor(partial), prior-art | action: watch
  note: Rules directory (`/rules`, `/dev-workflow`, `.cursor/`/`.claude/`) plus a sequential
  4-stage workflow (Bootstrap, Define, Plan, Implement & Review) with a "7-layer quality
  audit protocol" (layers not detailed in what was fetched). Verified via WebFetch
  (2026-08-02): single-agent-facing — no multi-agent coordination, no independent reviewer
  role, no coordination channel described. Narrower overlap with AGF than agentic-os or
  BMAD-METHOD; the rules-directory distribution pattern is the closest point of contact.

- https://github.com/systempromptio (systemprompt-core / systemprompt-template)
  type: tooling | relation: complement, competitor(runtime layer) | action: watch
  note: Self-hosted governance layer for Claude Code and MCP agents in a compiled Rust
  binary — authentication, authorisation, audit trail, cost controls, policy enforcement;
  systemprompt-core described elsewhere as "the MCP governance runtime," a 30-crate Rust
  workspace. Notable point of contact: like AGF's own `agov`/`agov-mcp`, this is a Rust
  CLI+MCP-server pairing wrapping Claude Code — but oriented at runtime access-control and
  cost governance, not repo-process review gates. Same crowded "runtime tool-call governance
  layer" category Batch 6's synthesis note already flagged (sovereign-agentops, Gategraph,
  enforra, TaG, sentience-governor, etc.) — file alongside that cluster rather than as a
  standalone finding. Not independently verified beyond the search-result description.

- https://github.com/FutureAtoms/agentic-control-framework
  type: tooling | relation: complement | action: watch
  note: CLI + MCP-based task management for agentic workflows. Same tooling-pattern choice
  as `agov`/`agov-mcp` (CLI-first, MCP as a thin wrapper) and as the already-logged
  loadlore.dev entry, but for task management rather than governance gates or an audit
  trail — a distribution/tooling-pattern reference, not a governance competitor. Not
  independently verified beyond the search-result description.

- https://medium.com/@visrow/comprehensive-guide-to-spec-driven-development-kiro-github-spec-kit-and-bmad-method-5d28ff61b9b1 (GitHub Spec Kit)
  type: framework | relation: complement, prior-art | action: watch
  note: Constitution-first, spec-driven pipeline (AI respects stated architectural intent
  across interactions); templates for multiple coding assistants. Adjacent axis — spec-as-
  source-of-truth rather than gate/role governance — lower priority than BMAD-METHOD or
  agentic-os for a direct AGF comparison. Not independently verified beyond the search-result
  description.

## Suggested follow-up

agentic-os is the strongest single candidate for a `docs/radar.md` Active entry alongside
the existing "Repo-contained, model-replaceable agent package" item, given how closely its
shape (rules + AGENTS.md landing + phase gates + evidence requirements) converges with AGF's
own while diverging sharply on the human-gate question. Not added to the Active radar list
here — that list's precedent (see `docs/plans/radar-governance-signal-gaming.md`) treats
live radar entries as needing their own Gate 1 authorization; this batch is a source-registry
addition only, per the registry's own stated first-pass/additive purpose.

# Batch 8 (2026-08-02) — third-party AI research, independently cross-verified

**Provenance and verification method, stated up front:** the candidate list below originated
from a sibling/competitor scan the human operator ran against a general-purpose third-party AI
assistant (Mistral's Lumo), given this project's public GitLab URL directly — a different
research method than Batches 1-7 (which used this project's own search/fetch tooling).
Treated as an unverified external claim on arrival, per this project's own discipline for
externally-supplied research: every named project below was independently re-searched via
this session's own WebSearch tool before being classified here, specifically to catch
plausible-sounding but fabricated project names (a known LLM failure mode). Every entry that
survived is confirmed to exist and to roughly match its described characteristics; none were
found to be fabricated. Feature-level detail (star counts, exact license terms, architecture
specifics) reflects the third-party assistant's original description, re-confirmed only at
the level of "this repository/product exists and does approximately this," not independently
re-derived line-by-line the way Batches 1-6 verified their entries — a real, disclosed gap in
depth, not full parity with those batches.

Corroboration note: the third-party scan independently surfaced `maref-org/maref`, already
classified in this registry's first "Classified" section above — cross-source agreement on
that entry, not a new finding.

## Classified (third-party-sourced, existence/description independently confirmed)

- https://github.com/SpillwaveSolutions/agent_rulez (Agent RuleZ)
  type: tooling | relation: competitor(partial), prior-art | action: compare-in-docs
  note: Rust binary + hook system for Claude Code — deterministic, sub-10ms YAML policy
  evaluation, contextual convention-file injection, blocks dangerous operations (e.g. `git
  push --force`) before execution, governance/provenance logging for compliance. Two points
  of contact with AGF specifically worth a compare-in-docs: same "convention files as
  governance" framing as AGF's rules directory, and the same choice of Rust for the
  enforcement binary as `agov`. Difference: runtime pre-tool-call hook enforcement, not a
  repo-process Gate 1/Gate 2 review-then-human-decides workflow — same axis distinction
  already drawn for agentic-os in Batch 7.

- https://github.com/Nikita-Filonov/ai-review
  type: tooling | relation: complement | action: watch
  note: AI-powered PR/MR review tool for GitHub/GitLab/Bitbucket Cloud+Server/Azure DevOps/
  Gitea; "Agent Mode" runs a ReAct-style loop exploring the repo (`ls`/`cat`/`rg`/`git`)
  before producing a review, and can reply within existing review threads. Confirmed via
  direct repo fetch. Different category from AGF: this is CI-triggered automated review
  commentary (like CodeRabbit, already referenced elsewhere in AGF's own docs), not a named
  independent-AI-reviewer role inside a human-gated approval chain — closer to a review
  *tool* than a governance *framework*. Worth watching as a possible ingredient (an AGF
  contribution-auditor implementation could plausibly use a tool like this for the mechanical
  repo-exploration step) rather than a direct competitor.

- https://github.com/IgorGanapolsky/ThumbGate
  type: tooling | relation: competitor(runtime layer) | action: watch
  note: PreToolUse hook for Claude Code/Cursor/Codex/Gemini/Amp/Cline/OpenCode — hard-blocks
  secret exfiltration, supply-chain installs, and destructive deletes by default; feedback
  ("thumbs up/down") becomes durable, shared "lessons" and org-visible pre-action checks.
  Same crowded runtime-tool-call-governance-layer category Batch 6's synthesis note already
  named (sovereign-agentops, Gategraph, enforra, TaG, sentience-governor) — file alongside
  that cluster.

- https://github.com/vijaym2k6/SteerPlane
  type: tooling | relation: competitor(runtime layer, narrow) | action: watch
  note: Runtime control plane for autonomous agents — per-run cost ceilings (local
  enforcement if the control API is unreachable), loop/step-cap detection, telemetry
  dashboard, single-decorator integration (Python/TypeScript). Narrower scope (cost and
  loop safety specifically) than the general policy-enforcement entries in this cluster;
  same runtime layer, same "watch only" treatment as the similar narrow-scope entries
  already logged in Batch 6 (irenee28/akm-clr-governance-layer, simonhansedasi/ai_glue).

- https://github.com/aeoess/agent-passport-system
  type: framework | relation: complement, prior-art | action: watch
  note: Cryptographic agent identity, scoped delegation, and signed accountability receipts
  (Ed25519), framework/transport-agnostic (CrewAI, LangChain, Google ADK, A2A, MCP, Cedar
  variant). Different axis from AGF's process/review governance — identity and delegation,
  not repo-change review — adjacent to this registry's already-logged "Agent identity
  standards race" entry (IETF AIMS/WIMSE, SPIFFE, ID-JAG) rather than a new category. A
  same-named but apparently distinct product exists at agentpassportai.com (OAuth-style
  consent layer) — not independently disambiguated further; treat the two as possibly
  unrelated projects sharing a generic name until confirmed otherwise.

- https://github.com/agentlas-ai/Agentlas-OS
  type: tooling | relation: complement | action: watch
  note: Local-first "agent OS" — keeps specialist agents in a hub, spins up a temporary
  orchestrator per task, package contracts (routing cards, anti-scopes, memory boundaries,
  verification shims) with auditable receipts, two-boundary governed memory. Governs agent
  *composition and routing*, not change review or approval gates — a different problem than
  AGF's, similar distinction already drawn for FutureAtoms/agentic-control-framework in
  Batch 7.

- https://github.com/Firma-AI/openfirma
  type: tooling | relation: competitor(runtime layer) | action: watch
  note: Local sidecar gating every outbound agent call against Cedar policies, deterministic
  ALLOW/DENY/HITL verdicts, capability tokens issued by a separate "Authority" trust root, no
  model on the enforcement hot path. Same runtime-tool-call-governance-layer category as
  ThumbGate/sentience-governor/enforra above and in Batch 6 — the Cedar-policy-engine
  approach specifically parallels AWS's Cedar-for-agents pattern (surfaced in this same
  verification pass, not independently classified here).

- https://certiv.ai/
  type: tooling | relation: competitor(runtime layer) | action: watch
  note: "AI Agent Assurance for Endpoints" — shadow-agent discovery, real-time monitoring,
  pre-execution policy enforcement at the endpoint, continuous queryable record of every
  agent action, policy decision, and outcome. Same runtime layer category as the rest of
  this batch; endpoint/fleet-visibility framing (rather than single-repo coding-agent
  framing) distinguishes it slightly from the others but the enforcement-layer classification
  is the same.

## Noted but not independently deep-verified (well-known industry names, existence not in doubt)

- Credo AI, OneTrust AI Governance, Lumenova AI — established enterprise AI governance/GRC
  (governance-risk-compliance) platforms: org-wide AI risk inventory, compliance-framework
  mapping (EU AI Act, NIST AI RMF, ISO 42001), model cards, explainability monitoring.
  type: practice | relation: competitor(org-governance layer) | action: watch. Operate at
  the org/portfolio level (which AI systems exist, what risk tier, what compliance evidence)
  rather than the per-commit/per-change level AGF gates — same "upstream org-governance vs.
  AGF's per-change gate" distinction already drawn for the AI Council Toolkit entry in
  Batch 6 and the Harvard Corporate Governance Forum entry in Batch 3.

- ARMO (armosec.io) — confirmed via independent search: Kubescape's parent company,
  eBPF/kernel-level runtime enforcement for AI agent tool calls and cloud workloads,
  behavioral baselining with progressive Observe-to-Enforce mode. type: tooling | relation:
  competitor(runtime layer) | action: watch. Same runtime-call-layer category as the rest of
  this batch, at the deepest (kernel/eBPF) enforcement point found across all eight batches.

- GitLab Duo Agent Platform — confirmed via independent search: reached general availability
  January 2026 (GitLab 18.8); group-based access control, LDAP/SAML integration, model
  selection with namespace inheritance, usage visibility. type: tooling | relation:
  complement | action: watch. Notable because AGF is itself hosted on GitLab — a plausible
  future integration surface (Duo's access-control layer governing *who* may invoke agents;
  AGF governing *whether an agent's change may merge*) rather than a competing product, but
  not evaluated further here.

- Endor Labs Agent Governance — confirmed via independent search: hooks-based policy
  enforcement across Cursor/Claude Code/Gemini CLI, blocks shell commands/file access/MCP
  tool calls, searchable audit log of every agent action. type: tooling | relation:
  competitor(runtime layer) | action: watch. Same runtime layer as the rest of this batch;
  positioned as part of a broader "Agentic AppSec Platform" (application security), a
  different starting discipline than AGF's process-governance framing.

# Batch 9 (2026-08-03) — governor-requested addition + radar-to-source crosswalk,
# routed via contribution-auditor (20260802T213418Z-contribution-auditor-js3j.md)

Contribution-auditor fetched/verified all six directly (per
`20260802T213418Z-contribution-auditor-js3j.md`). Claude independently
re-fetched four of the six (Gavin Ray, GDPR, Eclipse ORC, IETF WIMSE) before
adding — no discrepancies found. The EC AI Act page and the IETF AIMS slides
were not independently re-fetched; their entries below rely on
contribution-auditor's stated verification alone.

## Classified

- https://gavinray97.github.io/blog/design-by-contract-and-effects-for-llms
  type: discussion, practice | relation: prior-art, complement | action: cite, watch
  note: Gavin Ray (2026-08-02) argues that LLM-generated code increases the value of
  machine-checkable preconditions, postconditions, invariants, and explicit effect
  declarations — verified: author, date, and content match. Particularly relevant to
  AGF's contract-core, capability-boundary, and downstream-impact work: proposed semantic
  PR reports would expose weakened postconditions and newly introduced effects. The
  Rust-like syntax is illustrative, not an implemented feature; no accompanying tool or
  empirical evaluation. Design/opinion argument, not validation evidence.

- https://eur-lex.europa.eu/eli/reg/2016/679/oj/eng
  type: regulation | relation: threat-model, demand-driver | action: cite, integrate
  note: Official EUR-Lex text of Regulation (EU) 2016/679 (GDPR), CELEX 32016R0679, OJ L
  119, 4.5.2016. Grounds the append-only-trail-vs-Art.-17-erasure tension (already tracked
  in `docs/radar.md` and `docs/plans/ai-regulation-requirements-gap-analysis.md` AI-5) in
  the primary regulation text itself, alongside the existing hoop.dev secondary-source
  essay — the registry should not rest that claim on the vendor essay alone.

- https://digital-strategy.ec.europa.eu/en/policies/regulatory-framework-ai
  type: regulation | relation: demand-driver | action: cite, watch
  note: Official European Commission page for the AI Act regulatory framework. Adds the
  second half of `docs/radar.md`'s "EC digital-strategy pages (AI Act, CRA guidance)" watch
  instruction to the registry alongside the existing CRA open-source-steward page (Batch 3).

- https://orcwg.org/
  type: practice | relation: complement, demand-driver | action: watch, cite
  note: Eclipse Foundation's Open Regulatory Compliance (ORC) Working Group — verified
  live: facilitates collaboration between industry, open-source communities, and regulators
  on supply-chain/AI-policy compliance. `docs/radar.md`'s "OpenSSF + Eclipse ORC WG" watch
  entry previously named ORC without a direct registry source; this is that source.

- https://datatracker.ietf.org/wg/wimse/about/
  type: standard (emerging) | relation: prior-art, complement | action: watch
  note: IETF WIMSE (Workload Identity in Multi-System Environments) — verified active
  working group (chairs Justin Richer, Pieter Kasselman; Applications and Real-Time Area).
  Charter covers workload identity/access-management propagation, representation, and
  processing, including a JOSE-based token solution — the concrete standards-track home for
  part of `docs/radar.md`'s "IETF AIMS/WIMSE drafts" entry.

- https://datatracker.ietf.org/doc/slides-126-wimse-aims/
  type: discussion, standard (proposal) | relation: prior-art, complement | action: watch
  note: IETF 126 presentation "AI Agent Authentication and Authorization (AIMS)" — the
  IETF Datatracker has no working group with acronym AIMS; AIMS currently exists only as
  this presentation/proposal, distinct from the active WIMSE WG above. `docs/radar.md`'s
  current "IETF AIMS/WIMSE drafts" wording reads as if both were established WG/draft
  series to the same degree; they are not — flagged here as a radar wording precision item
  for a separate governed correction (this source-only batch does not itself edit the
  radar Active list).

# Batch 10 (2026-08-02 UTC) — Coldcard entropy incident and creation-time evidence

## Classified

- https://808bits.com/articles/coldcard-entropy-failure-lessons/
  type: incident analysis, practice | relation: evidence, prior-art, complement |
  action: cite, integrate
  note: Secondary technical synthesis, published 2026-08-02 without a clearly named author.
  Uses the Coldcard entropy failure to make three governance-relevant distinctions:
  security code being present does not prove the executed path reaches it; redundant
  components sharing one vendor/firmware lineage do not provide independent assurance;
  and a property that cannot be reconstructed afterward needs evidence at creation time.
  These are useful adjacent-domain examples for AGF's verification-independence tiers,
  input provenance, end-to-end validation, and capability-maturity discipline. Evidence
  status: Empirically informed by one adjacent-domain incident, not independent validation
  of AGF. Treat the article's financial totals, victim counts, attribution, and claims about
  which configurations survived as secondary reporting; do not cite those figures from this
  page alone.

- https://blog.coinkite.com/coldcard-mk3-seed-generation-warning/
  type: incident primary source, practice | relation: evidence, threat-model |
  action: cite, integrate
  note: Coinkite's security advisory, published 2026-07-30 and updated 2026-08-01,
  identifies affected Coldcard models/firmware, the independent-dice-entropy exception,
  the partial passphrase mitigation, fixed releases, and the requirement to generate a new
  seed and migrate because updating firmware cannot repair an existing weak seed. Primary
  source for the vendor's disclosure and remediation instructions, but an interested party
  rather than independent evidence of incident scale or attribution. Pair with the technical
  backgrounder below when citing the failure mechanism.

- https://blog.coinkite.com/entropy-technical-backgrounder/
  type: incident primary source, technical analysis | relation: evidence, prior-art,
  threat-model | action: cite, integrate
  note: Coinkite's technical backgrounder for the same incident. Documents the 2021
  integration regression, reduced effective entropy, later-model partial reseeding, and the
  failure of review to verify end-to-end symbol resolution and call reachability. Strong
  primary evidence for what the vendor says its code and review process did; not independent
  proof of the broader forensic narrative. The AGF-relevant lesson is to validate the actual
  security-critical path and its provenance, not merely the presence of a nominal control.

# Batch 11 (2026-08-02 UTC) — Kota multi-agent workspace

## Classified

- https://www.kota.place/
  type: tooling | relation: sibling, complement, competitor(operator layer),
  potential-partner | action: evaluate, watch, contact
  note: Kota is a macOS/Apple-Silicon desktop workspace for project-scoped multi-agent
  rooms. Its primary product pages claim shared timelines, agent/human chat, editable agent
  templates, centralized skills, account/project rules, per-agent worktrees, conflict
  handoffs, and persistent searchable project memory across Claude Code, Codex, Gemini,
  OpenCode, and Pi CLIs. This substantially overlaps AGF's operator and coordination surface,
  while AGF remains the governance/authority/evidence layer rather than the desktop runtime.
  Potential partnership: Kota hosts agents, worktrees, memory, and human interaction while
  AGF supplies governed roles, immutable coordination, deterministic validation, and gates.
  Integration risks must be resolved first: Kota's advertised automatic synchronization can
  publish clean worktree changes toward main, conflicting with AGF's feature-branch/MR/Gate-2
  discipline unless disabled or redirected; Kota's emergent-generalist role model also
  differs from AGF's explicit governed role authority. Evidence status: self-reported product
  capability; independently verify behavior before making effectiveness claims.

- https://www.kota.place/docs/research/long-term-teammates/v1/
  type: research, discussion | relation: sibling, prior-art, competitor(organization model),
  complement | action: cite, compare-in-docs, watch
  note: Nan Wu's versioned whitepaper "Kota: AI Agents Collaborating as Long-Term
  Teammates," v1.0 (2026-07-14). Proposes persistent inspectable agent identity, shared
  project memory, human-readable handoffs, conflict-safe worktree integration, and emergent
  specialization rather than pre-cast roles. The versioned HTML/PDF records publish hashes,
  which makes the artifact unusually suitable for stable comparison. Evidence status:
  Theoretical/self-authored design argument; its architectural descriptions are primary for
  Kota's intent, not independent validation that the collaboration model works.

- https://github.com/stabruriss/kota-app
  type: tooling, source | relation: sibling, complement, potential-partner |
  action: evaluate, watch
  note: Public Apache-2.0 source and release home for Kota's desktop app, website, docs,
  privacy/security notices, and citation metadata. Primary evidence that an inspectable
  implementation and release path exist; it does not by itself validate the feature or
  collaboration-effectiveness claims on the product page and whitepaper. A partnership
  evaluation should test whether AGF coordination records remain immutable and visible,
  whether `agov` obligations can be surfaced as required work rather than optional chat,
  and whether synchronization can preserve protected-main and governor Gate-2 authority.

# Batch 12 (2026-08-02 UTC) — Y Combinator QM and independent source audit

Both sources belong in the registry, but they serve different evidentiary roles. QM's
pinned repository is the primary source for the implementation and stated threat model;
RuntimeWire is a secondary, independent source-level audit of that snapshot. Feature and
security claims below are tied to reviewed commit
`7f2c916360f1797a8ff2a77ce2ce40c5fabab087` / npm package
`@yc-software/qm@0.1.4`; do not silently project them onto later QM revisions.

## Classified

- https://github.com/yc-software/qm/tree/7f2c916360f1797a8ff2a77ce2ce40c5fabab087
  type: tooling, source | relation: sibling, complement,
  competitor(operator/runtime layer), potential-partner |
  action: evaluate, compare-in-docs, watch
  note: Primary source for Y Combinator's MIT-licensed "multiplayer agent harness for
  work." QM gives people and shared rooms scoped durable sandboxes, memory, files,
  credentials, permissions, background jobs, and apps; its central core resolves identity
  and audience, applies policy, persists an audit/run ledger, and can route Pi, OpenCode,
  Codex, or Claude Code through the same control plane. This is a close sibling on the
  multi-agent operator surface and a partial competitor at the runtime-policy layer, while
  AGF remains the repo/process authority, independent-review, and binding-human-gate layer.
  Potential partnership: QM could host execution, scopes, credentials, and background work
  while AGF supplies governed roles, immutable coordination, plan validation, and Gate 1/
  Gate 2 authority. Treat this only as an integration hypothesis pending a threat model and
  hands-on evaluation; QM's own security document limits the intended deployment to one
  trusted organization and records material sandbox, credential, screening, egress, and
  retention caveats.

- https://runtimewire.com/article/inside-qm-we-read-y-combinator-s-company-wide-agent-runtime
  type: research, technical analysis | relation: evidence, prior-art, complement |
  action: cite, compare-in-docs, watch
  note: Ryan Merket / RuntimeWire source audit published 2026-08-02. The analysis pins QM
  source and npm package to the same revision, reads runtime and deployment code, generates
  a deployment directory, and runs the package CLI tests. It is unusually useful secondary
  evidence because it separates verified implementation behavior from README/security-doc
  claims and identifies at least one stale security warning at the reviewed head. Strong
  method reference for AGF's verifiable-claims discipline: review the deployment artifact
  and runtime together, pin source/package/image provenance, test the claimed boundary, and
  preserve contradictions rather than smoothing them over. It is not independent proof of
  QM's effectiveness in production; time-sensitive popularity counts and all findings must
  remain attributed to the pinned snapshot.

## First AGF takeaways

1. **The durable control plane around the model is the relevant sibling surface.** QM spends
   its architectural effort on scope resolution, audience filtering, ACLs, credentials,
   policy, audit records, scheduling, and sandbox lifecycle; the harness is replaceable.
   AGF should keep its runtime/process boundary explicit, but any future operator integration
   should bind governance decisions to this control-plane identity and scope rather than to
   a model name.
2. **Information-flow rules should be executable where possible.** QM's audience filter
   retains context only when every audience member is entitled to it; allowed egress is
   intersected and denials are unioned. This is useful prior art for AGF's cross-record leak
   controls and future scoped coordination views, while remaining runtime-layer enforcement
   rather than evidence that AGF should absorb a secrets or identity store.
3. **Mutable memory is not a governance record.** QM's inspectable plain-text memory and
   revision history are attractive, but model-assisted extraction/consolidation can add,
   rewrite, or delete facts asynchronously. AGF should preserve the distinction between
   helpful mutable operational memory and immutable decision/coordination evidence.
4. **Human approval must be classified by layer.** QM Strict posture can pause every
   effectful tool call; AGF gates plans and merge decisions. Combining them naively would
   create approval fatigue without improving accountability. An integration should record
   whether an approval authorizes a runtime effect, a repository change, or a binding
   governance decision, and must keep the named governor's authority distinct.
5. **Policy friction is not isolation.** QM candidly states that command policy can be
   bypassed, materialized credentials are readable inside a sandbox, screening coverage is
   incomplete, egress enforcement differs by backend, and durable state may persist
   indefinitely. AGF should reuse this honesty pattern: name the trusted operator/core,
   deployment-specific controls, retention boundary, and unsupported guarantees rather
   than turning a strict posture into a confinement claim.
6. **Independent agent review is convergent practice worth comparing.** QM's `AGENTS.md`
   forbids self-review and calls for an independent agent, while its contribution policy
   asks outsiders for human-written proposals that maintainers' agents implement. This is
   close prior art for AGF's planner/implementer/auditor separation; AGF's differentiator is
   the durable role authority and binding human governor decision, not merely using two
   agents.
7. **Release provenance is a concrete benchmark.** QM's reviewed release path pins container
   images by digest, signs images with Sigstore, and publishes npm provenance. AGF should
   compare that end-to-end artifact chain with `agov` checksums/provenance and avoid treating
   an unsigned coordination trail as equivalent to a cryptographic software attestation.

## Evaluation boundary

Do not add QM directly to the governed radar or pursue contact/integration from this source
entry alone. A separate evaluation should test role/identity mapping, whether `agov`
obligations can be surfaced as required work, whether QM's mutable memory can remain outside
the immutable coordination channel, whether private-to-shared audience transitions prevent
record leakage, and whether runtime approvals can coexist with protected-main and governor
Gate-2 authority without bypass or approval fatigue.

# Batch 13 (2026-08-02 UTC) — micro/mu agent tool server

## Classified

- https://github.com/micro/mu/tree/f2ffccc99cefa5e429fa431887584b1584387e96
  type: tooling, source | relation: complement, prior-art |
  action: compare-in-docs, watch
  note: Primary source for μ at the pinned 2026-08-02 revision: an AGPL-3.0,
  self-hostable single Go binary exposing first-party services (mail, search, storage,
  wallet, apps, and others) to agents through MCP, a built-in agent, CLI, app SDK, and
  human-facing web surfaces. It is not a close AGF sibling or governance competitor: μ
  supplies runtime tools and server-side authorization, while AGF governs plans, changes,
  review independence, and binding human decisions. Its strongest relevance is architectural
  and security prior art. Evidence status: primary implementation/design evidence for μ,
  not empirical validation that its security approach is sufficient in deployment.

## First AGF takeaways

1. **One declaration should derive every supported surface.** μ's typed service registry
   drives its native agent tools, picker, CLI, and app SDK. That is useful prior art for
   `agov`'s parser-authority rule and CLI/MCP parity. Its own architecture document also
   records a revealing exception: MCP entries remain hand-written at this revision, so a
   new service can reach the agent before it reaches MCP. The lesson is to test generated
   surface parity rather than infer it from a single-source-of-truth design claim.
2. **Identity is context, not model input.** Account-scoped μ handlers bind the authenticated
   account server-side and remove identity fields from tool arguments. This is a strong
   least-authority pattern for any future AGF/MCP integration: agents may select an action,
   but must not select whose authority, records, or funds the action uses.
3. **Capability boundaries belong at method granularity.** μ withholds selected irreversible
   methods while leaving read/non-destructive methods from the same service available. AGF
   should compare this with coarse tool allowlists and high-risk-operation rules; granting an
   MCP server should not automatically grant every effect exposed by that server.
4. **Treat retrieved content and the model as mutually untrusted.** μ's security charter
   explicitly assumes email, web, news, social, and app content can carry prompt injection,
   and says prompts are only a backstop to server-side controls. This supports AGF's declared
   threat-model and input-provenance work, while remaining runtime evidence rather than a
   substitute for process gates.
5. **Economic effects need governed limits.** μ exposes wallet/payment and other paid tools,
   binds the source wallet to the session, and documents caps plus remaining hardening gaps.
   This is a concrete example for capability-sensitive approvals: cost-bearing, publishing,
   messaging, deletion, and data-access effects should be distinguishable in tool authority
   and audit evidence.
6. **Self-hostability aids removability but expands operator duty.** A single inspectable
   binary and AGPL service-source obligation align with AGF's preference for replaceable,
   adopter-controlled infrastructure. They do not remove the need to govern secrets, SSRF,
   app sandboxing, retention, upgrades, and human review of identity/wallet changes.

## Evaluation boundary

Keep μ as compare/watch prior art rather than a radar or partnership candidate for now. A
future technical comparison should test registry-to-MCP parity, per-method capability
metadata, server-bound identity propagation through MCP, destructive/economic effect
classification, and whether μ's audit evidence can be referenced without importing mutable
runtime state into AGF's immutable coordination channel.

# Batch 14 (2026-08-03) — user-supplied, economics/epistemics analogy

- https://shonczinner.substack.com/p/supply-and-demand-is-not-what-most
  type: discussion | relation: analogy, evidence | action: cite, watch
  note: Argues the supply-and-demand model predicts equilibrium outcomes from *shifts* of the
  supply/demand curves, not from movements along them — price and quantity moving together
  (or apart) reveals nothing about mechanism without knowing which curve shifted and why; the
  same directional co-movement can come from a demand shock or a supply shock. Cites three
  documented misreadings as evidence this is a widespread, not niche, confusion: a Substack
  economics writer, economist Noah Smith, and ChatGPT scoring 33% on textbook supply-and-demand
  problems. AGF relevance is by analogy, not economics content: the same reasoning trap applies
  to governance outcome metrics (e.g. a drop in Gate 2 BLOCK-finding count reveals nothing about
  whether quality improved, review rigor relaxed, or submission volume fell, without knowing
  which underlying factor actually shifted) — see the new radar item below. The ChatGPT
  33%-accuracy data point is a secondary, weaker transfer: one more data point that LLMs get
  precise, textbook-structured domain reasoning wrong at a non-trivial rate even in mature,
  well-taught fields, consistent with this project's existing verify-before-trust posture. This
  is a single opinion essay, not a controlled study — treat the underlying economics claim as
  the author's argument, not as independently verified pedagogy research, and preserve that
  distinction whenever citing it. (accessed 2026-08-03)

# Batch 15 (2026-08-03) — intelligence-product tradecraft and Aurora prototype

The user supplied Luntta's article and Aurora as a possible post-v2.5.0 learning source.
They belong in the registry, but not as operational validation: the article is a secondary
design synthesis and Aurora uses simulated data and heuristic/deterministic assessments.
The official analytic standards discovered through the article are the primary sources for
any eventual AGF claim.

## Classified

- https://luntta.fi/notes/what-makes-an-intelligence-product/
  type: discussion, design synthesis | relation: prior-art, complement, evidence-discovery |
  action: cite, compare-in-docs, research-further
  note: Raine Luntta's 2026-07-14 account of redesigning a public-data maritime radar into
  the Aurora intelligence-product prototype after reading public US and UK analytic
  standards. Its strongest AGF relevance is the product discipline it synthesizes:
  start with a named customer requirement and intended decision; distinguish observation,
  assumption, and analytical judgment; separate likelihood from confidence in the
  evidentiary basis; expose alternatives and what would change the assessment; treat source
  limitations as provenance rather than merely attaching links; and optimize dissemination
  for timeliness, access, and use. The article also treats AI-assisted analysis as a sourcing
  and reproducibility problem, naming model/system version, settings, prompt/parameters,
  training/performance information, and date of use as potentially relevant metadata.
  Evidence status: Empirically informed design reflection, not validated intelligence
  tradecraft or evidence that Aurora is effective. Preserve the author's explicit caveats:
  no intelligence-practitioner involvement, no real customer requirement, and no claim that
  public doctrine describes Finnish operational practice.

- https://luntta.fi/aurora
  type: prototype, interactive design artifact | relation: prior-art, complement |
  action: evaluate, watch
  note: Primary artifact for Aurora's design claims: simulated SAR/AIS/passive-radio inputs,
  observation-versus-judgment separation, distinct likelihood and confidence, source grades
  and measurement uncertainty, a leading hypothesis plus benign alternatives, assessment
  change notes, and an AI assistant with citations, caveats, abstention, and limited query
  provenance. It is useful as a UI/protocol thought experiment for AGF review products, not
  as empirical or operational evidence. The author states that all data are simulated, the
  confidence model is heuristic, hypotheses are deterministic, the audit trail is not
  persistent/reproducible, and the prototype lacks a real customer, raw-input lineage,
  versioned assessments, handling controls, accreditation, and institutional validation.

- https://www.dni.gov/files/documents/ICD/ICD-203.pdf
  type: standard, primary source | relation: prior-art, evidence |
  action: cite, compare-in-docs
  note: US Intelligence Community Directive 203, Analytic Standards. Primary basis for
  objectivity, timeliness, source-quality description, explicit uncertainty, separation of
  underlying information/assumptions/judgments, analysis of alternatives, customer
  relevance, clear argumentation, changed-judgment explanation, and pertinent visual
  information. Applicable to the US Intelligence Community, not directly to AGF; use it as
  mature adjacent-domain prior art. A post-v2.5.0 comparison should test whether AGF review
  artifacts can distinguish evidence from inference and surface contrary information without
  importing intelligence authority or compliance claims.

- https://www.dni.gov/files/documents/ICD/ICD-206.pdf
  type: standard, primary source | relation: prior-art, evidence |
  action: cite, compare-in-docs
  note: US Intelligence Community Directive 206, Sourcing Requirements for Disseminated
  Analytic Products. Primary source for source locatability, quality/scope description, and
  source-summary expectations referenced by ICD 203 and the Luntta synthesis. Relevant to
  AGF's provenance and verifiable-claims work: a URL or record identifier alone does not
  state age, completeness, bias, access, validation, technical limitations, or potential
  deception. Applicability remains limited to its own institutional scope.

- https://www.dni.gov/files/documents/ICD/ICD-208-Maximizing-the-Utility-of-Analytic-Products-2017-01-09.pdf
  type: standard, primary source | relation: prior-art, evidence |
  action: cite, compare-in-docs
  note: US Intelligence Community Directive 208, Maximizing the Utility of Analytic
  Products. Primary source for knowing the customer, timeliness/classification/dissemination
  needs, discoverability/accessibility, feedback, tailored reuse, and keeping facts,
  judgments, confidence, and probability congruent across sanitized or alternate versions.
  Particularly relevant to AGF's distinction between a valid record existing somewhere and
  a decision product being visible to the right recipient in time to act.

- https://www.dni.gov/files/documents/ICD/ICS-206-01.pdf
  type: standard, primary source | relation: prior-art, evidence |
  action: cite, compare-in-docs
  note: US Intelligence Community Standard 206-01 on citing public/commercial information,
  OSINT, and AI-assisted services. Candidate primary source for a bounded post-v2.5.0 audit
  of AGF's AI-contribution provenance fields. Do not copy its metadata requirements blindly:
  compare model/version/settings/prompt/date and source-discovery expectations against AGF's
  privacy boundary, immutable coordination model, reproducibility limits, and the fact that
  AGF governs software-change decisions rather than intelligence products.

- https://www.gov.uk/government/publications/explaining-uncertainty-in-uk-intelligence-assessment/explaining-uncertainty-in-uk-intelligence-assessment
  type: guidance, primary source | relation: prior-art, evidence |
  action: cite, compare-in-docs
  note: UK Professional Head of Intelligence Assessment guidance, published 2025-03-24.
  Separates probability (likelihood a statement is true or event occurs) from analytical
  confidence (soundness and stability of its foundations), with confidence informed by the
  information base, analytical rigour, and complexity/volatility. This is directly useful
  for checking whether AGF's capability maturity and evidence-status labels leave a separate
  gap for uncertainty about a particular review judgment. The frameworks are related but not
  interchangeable and should not be collapsed into a single score.

- https://www.gov.uk/government/publications/jdp-2-00-understanding-and-intelligence-support-to-joint-operations
  type: doctrine, primary source | relation: prior-art, evidence |
  action: cite, research-further
  note: UK Ministry of Defence JDP 2-00 (fourth edition, 2023) describes the intelligence
  cycle and the need for accurate, timely intelligence supporting decision makers who
  understand their own role. Use only as adjacent-domain process prior art after reading the
  relevant sections directly; the Luntta article's design interpretation is not a substitute
  for the doctrine, and neither source establishes requirements for AGF adopters.

## First AGF takeaways

1. **A governed artifact should name the decision it serves.** A valid plan, review, or
   coordination record can still be useless if its audience, question, deadline, authority,
   and intended next action are unclear.
2. **Evidence and judgment need different fields or unmistakable language.** AGF already
   distinguishes advisory evidence from binding decisions, but a post-v2.5.0 audit should
   test whether reviews consistently separate observations, assumptions, inferences,
   alternatives, and recommendations inside the advisory layer.
3. **Likelihood, analytical confidence, evidence status, and capability maturity answer
   different questions.** Any proposal must preserve those axes rather than create a seductive
   composite score.
4. **Provenance includes limitations.** Source identity is insufficient without currency,
   scope, quality, access, validation, independence, and known failure/deception modes. AI
   provenance must also balance reproducibility metadata against privacy and secret leakage.
5. **Changed judgments deserve first-class explanation.** Corrections and renewed reviews
   should identify what new evidence or reasoning changed the conclusion, not merely publish
   a new verdict.
6. **Visibility and utility are part of correctness.** Publication to the intended recipient,
   discoverability, timeliness, and consistent alternate/sanitized versions are useful lenses
   for the branch-only coordination and bridge-note work already planned.
7. **Aurora's self-limitation is itself good evidence practice.** Simulated inputs,
   heuristics, missing lineage, and lack of a real customer are stated plainly; AGF should
   retain that implementation-honesty pattern while avoiding claims that the prototype
   validates the proposed mechanisms.

# Batch 16 (2026-08-03) — Agent Detective multi-agent fault attribution

## Classified

- https://github.com/Thomeras/agent_detective/tree/1f536cccd285e2d2fc3d207b9fad5978081fbf97
  type: tooling, source, evaluation framework | relation: complement,
  competitor(evaluation/evidence layer), potential-partner |
  action: evaluate, compare-in-docs, watch
  note: Primary source for Agent Detective v0.4.2 at the pinned 2026-07-31
  revision. The Apache-2.0 project ingests OpenTelemetry traces, reconstructs a
  multi-agent execution graph, and attempts to distinguish where quality first
  broke (origin) from its propagation path and terminal manifestation. It has a
  deterministic evidence channel for contract and propagation checks plus an
  optional LLM-judged channel; reports retain supporting, refuting, and context
  evidence, separate observation confidence from attribution confidence, cap
  confidence for weaker verdict types, and render absent trace evidence as
  `unverified` rather than silently `ok`. Local CLI/CI use can run without the
  full persistence stack or an LLM; the full stack stores raw trace payloads
  and analysis state across ClickHouse/Postgres/MinIO/Redis and currently relies
  on external authentication if exposed beyond localhost.

  This is adjacent to AGF's evidence and contribution-auditor surface, not its
  binding governance authority. Agent Detective analyzes what a runtime trace
  contains and can mechanically fail CI; AGF defines roles, plans, independent
  review, coordination visibility, and the human governor's Gate 1/Gate 2
  decisions. A plausible integration would attach a pinned Detective report as
  advisory Gate 2 evidence, bind trace nodes to governed role/task/commit IDs,
  and preserve the distinction between a mechanical incident signal and a
  binding review finding. Do not allow its `culprit` terminology, exit code, or
  confidence number to acquire governor authority.

  Strong AGF-relevant design patterns: origin-versus-manifestation separation;
  typed evidence polarity; no defect without supporting evidence; deterministic
  checks counterbalancing judge failure; explicit observability boundaries;
  report-version/prompt-hash provenance; human ground-truth feedback; and the
  rule that missing evidence cannot be upgraded to success. These connect
  directly to AGF's verification-independence, evidence-status, tool-authority,
  correction, and post-v2.5.0 analytic-product work.

  Evidence boundary: the README labels the product Beta and warns that report
  classifications still change between minor versions. The repository reports
  hundreds of hermetic tests and golden fixtures, but its capability document
  also says the labelled corpus covers one harness, one linear topology, and
  one injection; fan-out, retry-loop, and A2A claims still await validating
  traces. Treat all effectiveness, calibration, and root-cause-accuracy claims
  as self-reported until reproduced on independent multi-agent traces. Trace
  payloads may include verbatim prompts, outputs, artifacts, costs, and model
  metadata, so any AGF evaluation must test minimization, redaction, retention,
  access control, prompt-injection exposure, and whether incomplete
  instrumentation produces honest `unverified` results.

## First AGF takeaways

1. **Separate fault origin, propagation, and discovery.** A reviewer finding a
   bad terminal artifact is not necessarily the actor or step that introduced
   the defect. Coordination and review evidence should preserve that causal
   distinction rather than assigning blame from the last visible failure.
2. **Use deterministic and judged evidence as distinct channels.** Contract
   violations, hashes, graph edges, and propagation checks should not be mixed
   into one opaque LLM score. Disagreement between channels is useful evidence,
   not noise to average away.
3. **Missing observability is its own result.** If the trace, coordination
   record, or test artifact cannot support a claim, report `unverified` or an
   observability gap—never infer `CLEAR` from silence.
4. **Confidence needs a subject.** Confidence that an observation occurred and
   confidence that one node caused it are different. Neither is equivalent to
   AGF evidence status, capability maturity, likelihood, or governor approval.
5. **Automated blame is high-risk language.** Prefer evidence-linked origin or
   attribution candidates, expose counter-evidence and caps, and keep a human
   correction path. A CI exit code can block under an authorized rule, but it
   cannot make the binding disposition.

## Evaluation boundary

Do not add Agent Detective directly to blocking AGF CI from this source entry.
A later evaluation should use pinned synthetic and real-consented traces;
measure incomplete-instrumentation behavior, deterministic replay, judge
variance, false attribution, parallel/cyclic graphs, privacy/redaction, and
report-version stability; and decide whether the useful integration is a Gate
2 evidence attachment, an `agov` import/summary adapter, or documentation-only
comparison. Contact/partnership is reasonable only after that bounded test.
