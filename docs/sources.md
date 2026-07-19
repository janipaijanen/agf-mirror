# AGF source registry — first-pass classification (2026-07)

Scheme: `type` (regulation | standard | framework | tooling | research | practice | discussion)
· `relation` (competitor | complement | prior-art | demand-driver | evidence | threat-model | none, multiple allowed)
· `action` (cite | integrate | compare-in-docs | watch | verify | ignore)

Entries marked **[unverified]** could not be confirmed by search — content classification is
withheld, not guessed. Verify before relying on them.

## Classified

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

## Unverified — do not classify yet

- https://github.com/geludobre/sovereign-agentops — [unverified] not found via search.
- https://vektorgeist.com/blog — [unverified]
- https://twmrg.substack.com/p/the-governance-gap — [unverified] (title suggests demand-driver essay)
- https://coderadius.ai/ — [unverified]
- https://github.com/irenee28/akm-clr-governance-layer — [unverified] ("governance layer" in name;
  potential competitor — verify first)
- https://github.com/MAVS-RESEARCH — [unverified] (org link, no specific repo)
- https://splabs.io/compliance — [unverified]
- https://omega-lang.org/ — [unverified]
- https://github.com/jacobthomasmichael/GatekeeperAI/blob/main/README.md — [unverified] repo not
  found directly; a matching-tagged project exists (CI policy-as-code linter: ai-governance,
  eu-ai-act, nist-ai-rmf, iso-42001, github-action) but identity unconfirmed. If it is that:
  type tooling, relation competitor/complement to issue #3's CI enforcement.

## Suggested additions (surfaced during verification)

- https://github.com/boshu2/agentops
  type: framework | relation: competitor, prior-art | action: compare-in-docs
  note: Independent verification for coding agents — "no verdict = not done", verdicts recorded
  as plain markdown in-repo, Apache-2.0, explicit portability pitch ("your .agents/ folder comes
  with you"). Strikingly convergent with AGF's plain-file + removability stance; the closest
  competitor to AGF's exit-guarantee positioning specifically.

- https://microsoft.github.io/agent-governance-toolkit/
  type: tooling | relation: competitor(runtime layer), complement | action: watch
  note: Microsoft OSS — YAML policy engine wrapping agent tool calls (deny / require_approval),
  framework-agnostic. Runtime-call governance, not repo-process governance; useful contrast case.

- https://arxiv.org/pdf/2603.26487
  type: research | relation: evidence, prior-art | action: cite
  note: "Beyond Banning AI: A First Look at GenAI Governance in Open Source Software Communities" —
  empirically studies repo-level AI governance (AGENTS.md etc.) incl. its dependence on tool
  compliance; academic support for the enforcement-gap argument and for issue #7's framing.

- https://arxiv.org/pdf/2604.16870
  type: research | relation: prior-art, threat-model | action: watch
  note: "Governed MCP" — treats agent tool calls as syscalls needing kernel-level governance;
  relevant to the enforcement-locus debate (userspace/prompt-level controls are bypassable).

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

- https://loadlore.dev/ — [unverified]
- https://www.aicounciltoolkit.com/ — [unverified] (name suggests org-level AI council/
  compliance templates; if so, relation likely none)
- https://trust.aos-1.com/whitepapers/aos-1-governance — [unverified] ("AOS-1" governance
  whitepaper; vendor trust page — potential competitor claim, verify)
- https://lospino.so/blog/sunday-field-notes/the-workshop-has-changed/ — [unverified]
- https://pypi.org/project/sentience-governor/0.2.5.1/ — [unverified] (name suggests an agent
  governor package; verify maintainer/substance — very low version number)
- https://github.com/AIObuilt/TaG — [unverified]
- https://github.com/humancoreai/Gategraph — [unverified] (name suggests gate/graph governance —
  potential competitor, verify first)
- https://github.com/simonhansedasi/ai_glue — [unverified]

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
