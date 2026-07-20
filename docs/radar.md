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
