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

- **Anti-rubber-stamping gate design** — automation bias / Bainbridge predict approvers
  degrade into click-throughs; candidate countermeasures (forced justification, sampling,
  approval-rate telemetry) are themselves surveillance-flavored — controversial. review-by:
  2026-10

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
  principle (observation-only variant conceivable). Kept on radar because industry pressure
  toward runtime interposition keeps growing (ACS, Enforra, Tenure). review-by: 2027-01

- **External context graph vs canonical repository trail** — Piyaz represents tasks,
  dependencies, decisions, execution records, and activity in a live PostgreSQL-backed graph
  shared through MCP and a web UI. That could complement AGF's compact repository records and
  downstream-impact work, but it could also create split-brain authority when the graph and Git
  disagree. A bounded evaluation must test export and deterministic reconstruction, immutable
  history versus editable records, identity and authorization, concurrent claims and stale
  writes, exact commit/MR/plan linkage, hosted-data privacy and retention, migration, and clean
  removal. Any projection must leave AGF's repository trail and human gate decisions canonical;
  a Piyaz status, hint, or review verdict acquires no AGF authority. source: Piyaz v0.4.13 in
  `docs/sources.md` Batch 20 | status: evaluate, not adopt | review-by: 2026-11

- **Renewable work leases vs repository-native claims** — AetherGrid uses
  time-bounded renewable leases, explicit execution-start confirmation,
  heartbeats, immutable lifecycle events, and reconciliation to preserve
  exclusive job ownership across worker failure. AGF already has claims, stale
  detection, completion, and Git-backed history, but not a continuously renewed
  ownership lease. The candidate benefit is safer concurrent execution and
  deterministic recovery from interrupted agents; the counter-risk is turning
  a removable file-based framework into a scheduler-dependent runtime, plus
  false reassignment when a healthy agent temporarily cannot renew. Evaluate
  whether the useful invariants can remain repository-native and advisory, or
  whether this belongs only in external orchestrator integrations. Do not infer
  gate authority from lease ownership. source: AetherGrid in `docs/sources.md`
  | status: compare mechanics, do not implement | review-by: 2026-11

- **Repo-contained, model-replaceable agent package / deliberate self-cannibalization** —
  APSquared describes one self-contained marketing agent per repository: local instructions,
  activity logs, a human task queue, one activity per run, and Claude or Codex as a replaceable
  engine. Most mechanics converge with AGF, but making portable agent operation rather than
  governance the product could absorb the current framework and warrant a major version or a
  separately incubated repository. Controversial: useful generalisation versus scope dilution;
  the source is a self-reported marketing case, not independent evidence. source: APSquared in
  `docs/sources.md` | status: joint Claude/Codex assessment requested | review-by: 2026-10

- **Cross-harness session-digest layer vs. coordination-only continuity** — LLM Wiki's
  session rehydration writes default-on, redacted digests to a `.sessions/` directory
  (state JSON + Markdown, no full transcripts) that stay operational-only until explicitly
  promoted into durable evidence, readable uniformly across Claude Code, Codex, OpenCode.
  AGF handles the same cross-harness continuity problem differently: `agov session resume`
  covers within-harness continuity, and durable coordination records
  (`docs/coordination/*.md`) cover cross-agent state, but nothing sits between them as a
  lighter-weight, redacted "what was I doing" pointer for a fresh session on a different
  harness. Open question: does that gap cause real friction (a concrete cold-start failure
  mode `agent-cold-start-habits.md`'s existing protocol doesn't already cover), or would a
  digest layer just be a second, weaker memory channel competing with the coordination
  channel's own authority — the exact risk `coordination-message-rule.md` and
  `coordination-sequence-guard-rule.md` exist to prevent? Controversial: any answer other
  than "no gap, coordination records already suffice" risks re-legitimizing an informal
  parallel-authority channel, the same failure mode flagged when assessing AIHero's
  `/handoff` pattern (`docs/coordination/20260816T115701Z-planner-architect-hrd1.md`).
  source: LLM Wiki in `docs/sources.md` Batch 25 | status: evaluate, no implementation
  proposed | review-by: 2026-11

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

- **Agent-to-agent coordination protocols converging on immutable-record +
  claim-by-pattern** — Radia ("ask the table, like Go Fish, not a specific
  player"; immutable records agents discover/claim/build on; capability-scoped
  authorization) and Bridle ("Tailscale for agents"; typed A2A actions with
  dual consent and policy-before-agent) are independent 2026 designs solving
  the same problem AGF's coordination channel solves, and Radia's
  broadcast-to-pattern is a direct design answer to issue #121 (role- vs
  session-addressing). Also related: AetherGrid's renewable-lease claims
  (existing radar item). Open question, load-bearing for run-list item 2 (the
  #121-#123 coordination-protocol redesign): should AGF adopt claim-by-pattern
  / capability-scoped discovery from this prior art rather than inventing
  session addressing from first principles? Study before that Gate 1, not
  after. sources: Radia and Bridle in `docs/sources.md` Batch 34,
  cross-referencing AetherGrid | review-by: 2026-11

- **Cost / spend governance as a gate** — TokenOps (Batch 34) enforces per-run
  token budgets in-path before every model call, with stop/throttle/mutate
  policies. AGF has cost *measurement* (`agov token roi`) but no enforcement
  surface and no stated position on whether it should have one. Controversial:
  a spend gate is a runtime interposition, which the "Governance proxy /
  LLM-gateway layer" item rejected in 2026-07 under the removability
  principle — but an *advisory* spend signal (report, not block) might fit the
  same way the observation-only proxy variant was left conceivable. Compounds
  the outcome-metric-interpretation item. sources: TokenOps in
  `docs/sources.md` Batch 34 | review-by: 2026-12

## Watch sources (checked at each review)

- EC digital-strategy pages (AI Act, CRA guidance) and artificialintelligenceact.eu
- Traficom / NCSC-FI announcements (Finland)
- Microsoft ACS / Agent Governance Toolkit releases
- OpenSSF + Eclipse ORC WG (CRA for OSS)
- CycloneDX Agent-BOM proposal (#895); OWASP AI Exchange
- IETF AIMS/WIMSE drafts
- arXiv: multi-agent SE governance, LLM-as-judge reliability, provenance for agents
- Competitor repos: CORE, MAREF, agentops, Enforra, Cruxible; Covenant, FULCRUMAXE, Radia, Bridle (Batch 34)
- Machine Testimony census (agent record-keeping conformance assessments)

## Retired

(none yet — record drops here with one-line rationale, per the sunset rule)
