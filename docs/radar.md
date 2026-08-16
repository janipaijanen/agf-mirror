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

## Watch sources (checked at each review)

- EC digital-strategy pages (AI Act, CRA guidance) and artificialintelligenceact.eu
- Traficom / NCSC-FI announcements (Finland)
- Microsoft ACS / Agent Governance Toolkit releases
- OpenSSF + Eclipse ORC WG (CRA for OSS)
- CycloneDX Agent-BOM proposal (#895); OWASP AI Exchange
- IETF AIMS/WIMSE drafts
- arXiv: multi-agent SE governance, LLM-as-judge reliability, provenance for agents
- Competitor repos: CORE, MAREF, agentops, Enforra, Cruxible

## Retired

(none yet — record drops here with one-line rationale, per the sunset rule)
