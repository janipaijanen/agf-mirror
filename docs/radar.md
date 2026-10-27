# AGF Radar

Purpose: keep the framework's authors (and downstreamers) aware of new, unresolved, or
controversial ideas, viewpoints, regulations, and competing approaches — *before* they are
mature enough to become issues. Items here carry no commitment. Plain file by design
(removability principle); anyone may propose additions via an issue labeled ~radar.

Rules:
- Every entry has a `review-by` date (radar items obey the sunset principle too).
- At review: promote to an issue, keep with a new date + note, or drop with one line why.
- Entries state their controversy honestly — this list exists precisely for ideas we are
  not sure about.
- The full classified source registry lives in docs/sources.md; radar entries reference it.

Review cadence: quarterly, or at every minor release, whichever comes first.
Next review: 2026-10.

## Active

- **Buyer-side AGF** — controversial: inverts who adopts the framework; public buyers
  requiring AGF trails from vendors could be read as gatekeeping or as procurement empowerment.
  Enabled by MCC-AI audit rights. status: proposed as issue #10 | review-by: 2026-10

- **GDPR erasure vs append-only trail** — unresolved tension inside our own exit guarantee;
  identity minimisation vs audit value. Blocks final wording of issue #8. status: must
  resolve | review-by: 2026-09

- **CRA steward structure** — should AGF be stewarded by a legal entity at all? Solo natural
  person cannot be a steward (Art. 3(14)); forming an entity trades personal simplicity for
  regulatory standing. status: decision pending (issue #13) | review-by: 2026-09 (reporting
  duties start 2026-09-11 for in-scope entities)

- **Legibility-first vs gate-first** — philosophical fork (Scott's critique vs control
  imperative); the field is splitting into camps (agentops/Cruxible vs CORE/ACS). AGF must
  declare a stance. status: drafting | review-by: 2026-10

- **Anti-rubber-stamping and governance-signal gaming** — automation bias / Bainbridge
  predict approvers degrade into click-throughs, while Goodhart-style pressure lets agents or
  teams optimize the visible gate signal instead of the intended outcome. Practitioner reports
  now include evaluator mutation, benchmark shortcuts, and code that switches behavior between
  correctness and performance tests. Candidate countermeasures (forced justification,
  independent or isolated evaluation, outcome sampling, approval-rate telemetry, and adversarial
  test design) add cost and can become surveillance or another metric to game — controversial.
  sources: Core Automation and Tom's Hardware in `docs/sources.md` Batch 7 | review-by: 2026-10

- **Outcome-metric interpretation risk in governance dashboards** — a governance metric moving
  (e.g. Gate 2 BLOCK-finding count falling, coordination-trail volume rising) reveals nothing
  about mechanism without knowing what actually shifted: quality improving, review rigor
  relaxing, and submission volume changing can all produce the same directional reading.
  Directly compounds the anti-rubber-stamping item above and the existing signal-gaming radar
  work: a dashboard reader who infers cause from a metric's direction alone is exposed to the
  same fallacy regardless of whether the underlying change was benign drift or deliberate
  gaming. No AGF dashboard/metric currently states its own shift-vs-movement caveat explicitly.
  sources: shonczinner Substack in `docs/sources.md` Batch 14 | review-by: 2026-10

- **Verification independence tiers** — different-session < different-model < different-
  vendor; evidence-backed (LLM-as-judge unreliability) but raises cost and vendor-neutrality
  questions. review-by: 2026-10

- **Agent identity standards race** — IETF AIMS/WIMSE, SPIFFE, ID-JAG, Entra/Google Agent ID;
  immature, converging. Trigger to act: a stable draft with a real security section, or an
  adopter requiring signed provenance. review-by: 2027-01

- **Adjacent MCP server-admission governance vs AGF development governance** — the OWASP-hosted
  MCP Governance and Risk repository describes inventory, classification, scoring, evidence,
  and hard-gate mechanics for deciding whether an MCP server may operate in an organisation.
  AGF governs development roles, evidence, and human gate decisions, so the scopes overlap but
  are not substitutes. The open question is whether AGF should cite or map its `agov-mcp` and MCP
  adopter surfaces to such a rubric, or keep the layers explicitly separate. Organisation
  placement does not establish a formal OWASP project maturity category, and no conformance is
  claimed. Trigger to act: verified formal status plus a stable mapping, or an adopter asking AGF
  to satisfy it. source: OWASP MCP Governance and Risk in `docs/sources.md` Batch 33 | status:
  compare scopes, no conformance or implementation proposed | review-by: 2027-01

- **Agent-BOM / input provenance** — CycloneDX #895 open proposal; governs what agents
  *consume*. Novel territory, format unsettled. review-by: 2027-01

- **Accountability sink risk in AGF itself** — "the gate approved it" as blame absorber;
  Davies' test (accountable = able to change the decision) as design invariant. Standing
  self-critique item; never fully resolvable. review-by: each release

- **Psychological safety vs surveillance** — the trail records humans too; what is
  deliberately NOT trailed needs a stated answer. Controversial by nature. review-by: 2026-10

- **Non-code domains** — AGF for documents/data/research workflows; scope question only for
  now. review-by: 2027-01

- **Governance proxy / LLM-gateway layer** — rejected 2026-07 under the removability
  principle (observation-only variant conceivable). Runtime interposition keeps growing:
  TokenOps applies run-scoped spend policy before model calls, while Actradeck holds selected
  agent actions for operator review. The unresolved AGF question is whether an advisory spend
  or activity signal adds value without becoming a required runtime, policy authority, or
  second gate. Their enforcement and savings claims remain project-authored. sources: TokenOps
  and Actradeck in `docs/sources.md` Batch 34 | review-by: 2027-01

- **External operator/context planes vs canonical repository trail** — Piyaz represents tasks,
  dependencies, decisions, execution records, and activity in a live graph; AgentConnect adds
  the adjacent operator/runtime case, connecting people and agents through chat and forge
  surfaces while sessions, schedules, permissions, knowledge, daemon state, and control-plane
  orchestration live outside AGF's repository trail. Either could complement compact repository
  records or create split-brain authority when external state and Git disagree. A bounded
  comparison must test export and deterministic reconstruction, immutable history versus
  editable records, identity and authorization, concurrent ownership and stale writes, exact
  commit/MR/plan linkage, hosted-data privacy and retention, migration, and removal. AGF's Git
  trail and human gates remain canonical; external status, schedules, and verdicts acquire no
  authority. sources: Piyaz in `docs/sources.md` Batch 20 and AgentConnect in Batch 32 |
  status: compare, not adopt | review-by: 2026-11

- **Renewable work leases and content-routed claims vs repository-native claims** — AetherGrid
  uses time-bounded renewable leases, execution-start confirmation, heartbeats, immutable
  lifecycle events, and reconciliation. Radia describes workers discovering records by content
  pattern and claiming them under fenced renewable leases rather than addressing a named player.
  AGF already has claims, stale detection, completion, and Git-backed history, but not a
  continuously renewed ownership lease or runtime content router. Candidate benefits are safer
  concurrent execution, capability-based discovery, and deterministic recovery; counter-risks
  are scheduler dependence, false reassignment, hidden matching policy, and a second authority
  channel. Evaluate which invariants can remain repository-native and advisory. Neither lease
  ownership nor pattern matching grants gate authority, and Radia is not load-bearing for a
  future coordination change. sources: AetherGrid and Radia in `docs/sources.md` (Radia Batch
  34) | status: compare mechanics, do not implement | review-by: 2026-11

- **Repo-contained, model-replaceable agent package / deliberate self-cannibalization** —
  APSquared describes one self-contained marketing agent per repository: local instructions,
  activity logs, a human task queue, one activity per run, and Claude or Codex as a replaceable
  engine. Most mechanics converge with AGF, but making portable agent operation rather than
  governance the product could absorb the current framework and warrant a major version or a
  separately incubated repository. Controversial: useful generalisation versus scope dilution;
  the source is a self-reported marketing case, not independent evidence. source: APSquared in
  `docs/sources.md` | status: joint Claude/Codex assessment requested | review-by: 2026-10

- **Advisory memory and session-digest layers vs. coordination-only continuity** — LLM Wiki's
  session rehydration writes default-on, redacted digests to a `.sessions/` directory
  (state JSON + Markdown, no full transcripts) that stay operational-only until explicitly
  promoted into durable evidence, readable uniformly across Claude Code, Codex, OpenCode.
  OpenContext and CRBRO add local file-resident advisory memory with Git-readable or otherwise
  inspectable storage, retrieval, and correction mechanics. AGF handles continuity differently:
  `agov session resume` covers within-harness continuity, and durable coordination records
  (`docs/coordination/*.md`) cover cross-agent state, but nothing sits between them as a
  lighter-weight, redacted operational pointer or memory layer. Open question: does that gap
  cause measured friction beyond the existing cold-start protocol, or would another mutable
  channel compete with coordination authority, retain sensitive material, and serve stale
  facts? Any evaluation must separate private advisory recall from facts that require canonical
  publication and test correction, deletion, provenance, promotion, and removal. sources: LLM
  Wiki in `docs/sources.md` Batch 25; OpenContext and CRBRO in Batch 33 | status: evaluate, no
  memory format or implementation proposed | review-by: 2026-11

- **Stale-authorization / policy-state serializability at Gate 1→merge** — "Stateful
  Governance for Concurrent Agentic Systems" formalizes the requirement that a committed
  effect must be authorized against the policy state immediately before it occurs, not
  merely at initial-request time. `coordination-publication-rule.md` already documents an
  accepted, governed gap of exactly this shape: `only_allow_merge_if_pipeline_succeeds`
  blocks a failed/skipped pipeline but does not force revalidation when the *target*
  branch advances after a candidate's pipeline last passed, and GitLab merge-trains
  (the feature that would close it) are confirmed absent from this project's plan tier.
  The paper's prototype (MasuGate) is evaluated only on a scripted, LLM-free procurement
  workflow — not evidence its specific mechanism transfers to a Git/CI setting — but the
  formal correctness condition itself (policy-state serializability) is a candidate
  vocabulary for naming and eventually closing this gap properly, rather than leaving it
  as a documented narrowing. Controversial only in scope: is this worth a real mechanism
  (e.g. binding a Gate 1 approval to an exact plan hash + forced re-check at merge time),
  or does `agov coord publication-check`'s existing fresh-fetch-at-run-time behavior
  already provide equivalent protection when actually invoked pre-merge, making this a
  documentation gap rather than a mechanism gap? source: arXiv 2608.02764 in
  `docs/sources.md` Batch 28 | status: evaluate against existing gap, no implementation
  proposed | review-by: 2026-11

- **Named-standard prompt compression vs explicit instruction fidelity** —
  SimplerEnglish reports that a one-line instruction naming ASD-STE100 can
  approach a much longer skill's writing behavior on some Claude models while
  using less prompt context. Its aggregate pre-registered criterion was not
  met, however, and its one-run, Claude-only evaluation did not test human
  comprehension or preservation of governed meaning. The open AGF question is
  whether a model's latent knowledge of a named writing standard can safely
  reduce repeated instruction text, or whether it imports an unknown,
  model-specific and changing approximation that can erase modality,
  negation, authority boundaries, exact identifiers, citations, or necessary
  caveats. A future separately governed evaluation would need repeated
  no-skill/compact/explicit arms across Claude and Codex, deterministic
  semantic checks, blinded human readers from AGF's target audience, and
  token/cost reporting separated from fidelity. source: SimplerEnglish in
  `docs/sources.md` Batch 30 | status: testable hypothesis only; no skill
  adoption or trial authorized | review-by: 2026-11

- **Untrusted inbound content as an agent-context boundary** — AGF governs what
  leaves the codebase (`external-anonymization-rule.md`) but has no rule for what
  *enters* an agent's context from a source the project does not control: a fetched
  web page, a downloaded archive, a dependency's files, or the contents of an
  untrusted branch or merge request under review. Rehberger's demonstrated chain
  against Claude Code shows the gap is exploitable rather than hypothetical — asking
  an agent to summarise a website led to remote code execution via Python module
  shadowing, at a reported 60–80% success rate — and the vendor's reported position
  that the affected mode is "not a security guarantee" places the mitigation burden
  at the adopter and framework layer, which is AGF's layer. Controversial in shape,
  not in existence: candidate responses range from an advisory handling rule (treat
  external content as data, never instruction; state the boundary before fetching)
  to mechanical measures AGF may have no authority or means to enforce inside a
  third-party harness, and an over-broad rule risks forbidding ordinary research
  work this very file depends on. The open question is what an assistant-neutral,
  removable framework can legitimately require here, given that the enforcement
  point sits inside the harness rather than the repository. Note also that any
  actual rule is a normative change requiring full Gate 1 — it is not eligible for
  the research-maintenance fast path that created this entry. source: The Register /
  embracethered in `docs/sources.md` Batch 31 | status: open question, no rule
  proposed | review-by: 2026-10

- **Machine-enforced write permission vs declared artifact ownership** —
  EngineeringSpec proposes that a coding agent's writable and protected paths come
  from a reviewed, version-controlled contract and are enforced at write time, with
  boundary failures routed to human review. AGF already carries the same data:
  `artifacts_owned` is a declared field on coordination records, and plans name
  affected files. The difference is enforcement — AGF's is by review, not by
  mechanism. The tempting inference is that AGF should mechanically block writes
  outside `artifacts_owned`, and this project's own history is the strongest
  argument against doing that naively: handoff `20260827T100251Z-coder-tester-7bkm`
  records four approved-criterion owner paths that were absent from the plan's
  affected-file estimate and had to be added mid-implementation because omitting
  them would have left an approved criterion incomplete. A hard write gate would
  have blocked legitimate, correct work; the actual defect was an estimate derived
  from the wrong basis, which learning `20260827T091844Z-as7k` already addresses.
  So the open question is narrower than "adopt path enforcement": is there a useful
  advisory or reconciliation form — for example, reporting the divergence between
  declared `artifacts_owned` and an MR's actual changed paths as review evidence
  rather than as a block — and does that add anything Gate 2 review does not already
  do? Treat the source as unevidenced: it is a release-candidate tool whose own
  authors state they have no evidence it improves safety or correctness. source:
  EngineeringSpec in `docs/sources.md` Batch 31 | status: evaluate mechanics, no
  implementation proposed | review-by: 2026-11

- **`agf/skills/` naming overlap with the cross-vendor Agent Plugins
  specification** — Kiro's announcement of "Agent Plugins 1.0.0" (a
  vendor-stated cross-vendor specification with a named Technical Steering
  Committee: AWS, Cursor, Microsoft, OpenAI, Vercel) describes a packaging
  shape for agent skills that uses the identical `SKILL.md` filename this
  project's own `agf/skills/<name>/SKILL.md` convention already uses,
  independently arrived at. The open question is narrow and genuinely
  unresolved: does aligning `agf/skills/`'s directory/manifest shape with
  Agent Plugins improve portability for adopters who also use
  Plugins-compatible clients, or is the naming overlap coincidental and
  not worth coupling AGF's own skill format to an external, single-vendor-
  announced specification whose actual multi-vendor adoption and stability
  are not independently verified? Treat the source as vendor-stated,
  unverified: the steering-committee claim and the spec's real-world
  compatibility surface come from Kiro's own blog post, not the
  specification document or a neutral survey. No alignment, adoption, or
  `agf/skills/` shape change is proposed here — this is a real, checkable
  design question worth revisiting once the specification itself (not
  just a vendor announcement about it) can be read directly. source: Kiro
  blog in `docs/sources.md` Batch 39 | status: open question, no
  alignment proposed | review-by: 2026-12

- **Operational checkpoint substrate: gitignored planning files vs local WIP
  commits vs the canonical coordination trail** — Planning with Files restores
  active phases, findings, and progress from gitignored Markdown and can
  mechanically hold completion while work remains; gstack instead offers
  opt-in local `WIP:` commits with structured context, push disabled by default,
  context restoration from Git, and filter-squashing before review. Both address
  context loss and interrupted work, but neither mechanism automatically has
  AGF authority. The open question is whether either substrate closes a measured
  gap beyond `agov session resume`, governed handoffs, and local telemetry, or
  creates another mutable state channel that can disagree with Git-visible
  coordination. A bounded comparison would test crash recovery, compaction,
  dirty and untracked work, concurrent worktrees, stale-worker revival,
  exact-head review
  invalidation, CI duplication, cleanup, and promotion of durable facts into
  canonical records. It must also measure steady-state token and latency cost:
  per-turn reinjection can preserve intent while repeatedly consuming context,
  and frequent Git checkpoints can preserve state while polluting history or
  triggering automation if pushed. Treat both projects' performance claims as
  self-report. No hook, checkpoint convention, stop authority, schema, or trial
  is proposed here. sources: Planning with Files and gstack in
  `docs/sources.md` Batch 40 | status: compare mechanics only | review-by:
  2026-12

- **Periodic Internet / AI-ecosystem landscape scan** — Which siblings,
  complements, substitutes, and prior art materially change AGF's positioning
  or warrant a separately governed experiment? The retained research contract
  defines stable comparison axes, evidence classes, output limits, and the
  boundary against outreach, adoption, integration, and public comparative
  claims. Promote only when a concrete positioning or integration decision
  needs the comparison. Full contract:
  `docs/proposals/radar/965-ecosystem-landscape.md`; source: MR !965 | status:
  parked question, no scan or implementation commitment | review-by:
  2026-10-10

- **Regulatory / AI-governance requirements landscape re-assessment** — Does
  a concrete adopter request or compliance-evidence need justify a new,
  bounded assessment against a governor-selected released baseline? The
  retained contract preserves the required fact-checking, capability evidence,
  applicability uncertainty, and structural-exclusion boundaries. Its dated
  source material is historical and unverified; this entry makes no current
  legal, regulatory, or compliance finding. Full contract:
  `docs/proposals/reconstruct/973-regulatory-reassessment.md`; source: MR !973
  | status: parked question, no assessment or legal conclusion | review-by:
  2026-10-10

- **Semantic-palette accessibility assessment** — Does a concrete visual or
  accessibility defect justify a bounded assessment of AGF's existing semantic
  palette across contrast, colour-vision-deficiency simulation, pairwise
  distinguishability, greyscale, print, and representative rendered assets?
  Swatch supplies relevant project-authored tooling prior art, but it has not
  been installed, security-reviewed, or reproduced against AGF fixtures, and
  its outputs do not replace primary standards or AGF's governed contrast
  authority. Promote only when a measured defect or approved visual-system
  change needs the evidence; any tool adoption, palette migration, validator,
  dependency, or blocking-threshold change requires separate Gate 1 scope.
  source: Swatch in `docs/sources.md` Batch 41; historical source MR !1061 |
  status: parked question, no assessment or implementation commitment |
  review-by: 2026-11

- **External durable Work/Attempt/settlement control plane vs repository-native
  governance trail** — Kungfu combines several questions that the Piyaz context-
  graph, AetherGrid renewable-lease, and LLM Wiki cross-harness-digest items
  currently expose separately: one durable Work identity continues through
  multiple Agent Sessions and failed Attempts, a live owner excludes a second
  writer, evidence remains attached to the Work, and governed review plus
  settlement—not the executing Agent—decide completion. That combination could
  complement AGF's coordination and evidence trail, but it could also become a
  competing canonical state machine. A bounded comparison must map Project,
  Work, Attempt, ownership, evidence, review, and settlement onto AGF claims,
  handoffs, reviews, completion records, plans, and human gates; force conflicts
  between `.kungfu/` and Git; test crash recovery, stale ownership, concurrent
  Agents, exact commit/MR linkage, deterministic export and reconstruction,
  secrets and local-data handling, migration, and clean removal. AGF's
  repository trail and authorized human decisions remain canonical throughout:
  no Kungfu state or actor acquires Gate 1, Gate 2, merge, release, or governor
  authority. source: Kungfu at reviewed commit in `docs/sources.md` Batch 42 |
  status: compare as adjacent control plane, do not integrate | maturity: Level
  1 Planned; evidence: Empirically informed at most | review-by: 2026-11

- **Release feedback latency (issue #53)** — Is the delay between a change and
  usable pipeline/review feedback large enough, and measured reliably enough, to
  justify a bounded re-measurement? The retained plan and its 2026-07-24 evidence
  preserve the method and the normalized data; the evidence is historical and has
  not been re-verified. Promote only when feedback delay is reported as a concrete
  problem or a release-cadence review asks for the numbers. Carrier files:
  `docs/proposals/radar/459-issue-53-feedback-latency-measurement.md`,
  `docs/proposals/radar/459-issue-53-feedback-latency-evidence-2026-07-24.md`,
  `docs/proposals/radar/data/459-issue-53-feedback-latency-normalized-2026-07-24.json`;
  source: MR !459 (the fourteen coordination records of that branch stay only on
  the retained branch) | status: parked question, no measurement commitment |
  review-by: 2027-01-31

- **Expectation signals for material caveats (issue #100)** — Does AGF need a
  small convention so that a load-bearing caveat is not technically true but
  practically buried? The retained proposal reached consensus on the foundation
  and narrowed the mechanism, but at Level 1 Planned and Theoretical; it grants no
  rule, template or validator authority and warns against warning decoration and
  AI-hedging misuse. Promote only on a second concrete incident of a buried caveat
  or a named sponsor. Carrier: `docs/proposals/radar/923-expectation-signals-consensus.md`;
  source: MR !923, issue #100 stays open | status: parked question | review-by:
  2027-01-31

- **Learning-record enum enforcement** — Should `category`, `severity` and
  `scope` move from documented conventions to enforced values, and should the top
  `confidence.level` tier have an evidence threshold? The retained audit
  reproduces the live schema and corpus state as of 2026-08-03 and leaves the
  decision open, so its counts are historical. Revisit before any new
  classification field is added to the learning schema. Carrier:
  `docs/proposals/radar/998-learning-field-enforcement-audit.md`; source: MR !998
  | status: parked question | review-by: 2027-01-31

- **Deferred LLIS comparison learning candidates** — Which of the candidate
  learnings drawn from the NASA LLIS comparison and the schema audit should be
  recorded once the learning schema is stable? The retained list is a deferred
  TODO gated on the supersession-field and enforcement-audit work, and nothing in
  it was recorded as a learning. Carrier:
  `docs/proposals/radar/1000-llis-deferred-learning-candidates.md`; source: MR
  !1000 | status: parked, gated on learning-schema stability | review-by:
  2027-01-31

- **Strategic candidate comparison and release mapping** — Is a scored
  comparison of the deferred strategic candidates, mapped to releases, worth doing
  after the backlog triage? The retained document defines the exercise and says
  explicitly not to start it. Promote only when the Human Governor asks for the
  comparison or a release-cadence review needs it. Carrier:
  `docs/proposals/radar/1451-strategic-candidate-comparison.md`; source: MR !1451
  | status: parked question, no commitment | review-by: 2027-01-31

- **Release provenance: commit/tag signing and SBOM** — Should AGF releases carry
  independently verifiable provenance, meaning signed tags or commits and a
  dependency inventory (SBOM), beyond same-channel checksums and self-issued
  metadata? The retained learning entry and its handoff record a 2026-08-17
  observation of zero signed commits, zero signed tags and no SBOM, taken from an
  external due-diligence simulation and counted at that date, so the numbers are
  historical and not re-verified. The independent read recommended separating an
  SBOM slice from a signing-identity proof of concept (keyless CI identity versus
  a hardware-backed fallback) and warned against implementing from the entry.
  This is security-relevant: revisit at the next release-engineering or
  deprecation/supply-chain review, or on an adopter request for verifiable
  provenance. Carriers: `docs/proposals/radar/1722-supply-chain-signing-learning-entry.txt`,
  `docs/proposals/radar/1722-supply-chain-signing-handoff.md`; source: MR !1722 |
  status: parked question, no signing or SBOM commitment | review-by: 2027-01-31

- **Operational-debt backlog bounds** — Does the queue itself need closure SLOs,
  bounded branch lifetime and an ownership view, given pending handoffs, branch
  sprawl and rule-copy drift? The retained Phase-0 plan was approved, but only
  Snapshot 1 was taken (2026-08-17) and Snapshot 2 never was, so the baseline is
  one dated snapshot and the proposed SLO response never found a sponsor. Promote
  only with a named sponsor and a fresh snapshot. Carriers:
  `docs/proposals/radar/1718-operational-debt-phase0-measurement.md`,
  `docs/proposals/radar/1718-operational-debt-learning-entry.txt`,
  `docs/proposals/radar/1718-snapshot-1-README.md` and the data files under
  `docs/proposals/radar/data/1718-snapshot-1-*`; source: MR !1718 (its 554-line
  branch inventory stays only on the retained branch) | status: parked question,
  no SLO commitment | review-by: 2027-01-31

- **AI-regulation requirements gap analysis (historical)** — The retained analysis
  and its fact-check history (bounded renewed fact-check CLEAR at the MR head on
  2026-08-03) preserve how AGF maps to AI regulation and governance requirements as
  of that date. It is dated source material and makes no current legal, regulatory
  or compliance finding; the live question is the re-assessment entry above.
  Carrier: `docs/proposals/radar/967-ai-regulation-requirements-gap-analysis.md`;
  source: MR !967 (its `docs/sources.md` edit stays only on the retained branch) |
  status: parked, folded under the re-assessment question | review-by: 2027-01-31

## Watch sources (checked at each review)

- EC digital-strategy pages (AI Act, CRA guidance) and artificialintelligenceact.eu
- Traficom / NCSC-FI announcements (Finland)
- Microsoft ACS / Agent Governance Toolkit releases
- OpenSSF + Eclipse ORC WG (CRA for OSS)
- CycloneDX Agent-BOM proposal (#895); OWASP AI Exchange, MCP Top 10, and MCP Governance and Risk
- IETF AIMS/WIMSE drafts
- arXiv: multi-agent SE governance, LLM-as-judge reliability, provenance for agents
- Competitor repos: CORE, MAREF, agentops, Enforra, Cruxible; Covenant and FULCRUMAXE (Batch 34)
- Machine Testimony census (pinned agent-record evidence assessments; Batch 34)

## Retired

(none yet — record drops here with one-line rationale, per the sunset rule)
