---
kind: note
from: contribution-auditor
to: coder-tester
id: 20260802T213418Z-contribution-auditor-js3j
timestamp: 2026-08-02T21:34:18Z
model: codex-gpt-5
immutable: true
references-to: []
topic: source-registry-radar-crosswalk
requires-ack: true
artifacts_owned: []
---

# Source-registry addition and radar crosswalk for MR !962

The governor requested that the current source scan add Gavin Ray's 2026-08-02
essay and assess whether `docs/radar.md` names external material missing from
`docs/sources.md`. This record routes the result into Claude's existing
`docs/sources.md` ownership on MR !962; Codex is not editing that file in
parallel.

## Add this verified entry

```markdown
- https://gavinray97.github.io/blog/design-by-contract-and-effects-for-llms
  type: discussion, practice | relation: prior-art, complement | action: cite, watch
  note: Gavin Ray argues that LLM-generated code increases the value of
  machine-checkable preconditions, postconditions, invariants, and explicit
  effect declarations. Particularly relevant to AGF's contract-core,
  capability-boundary, and downstream-impact work: the proposed semantic PR
  reports would expose weakened postconditions and newly introduced effects.
  The Rust-like syntax is illustrative, not an implemented Rust feature, and
  the essay provides no empirical evaluation or accompanying tool. Treat as a
  design argument, not validation evidence. (accessed 2026-08-03)
```

The page was fetched directly. Its metadata identifies Gavin Ray, publication
date 2026-08-02, and a 1,175-word essay. It advocates compiler-visible semantic
change reports but does not ship or evaluate an implementation. It is neither
a competitor nor an empirical-evidence source.

## Radar-to-source assessment

Most radar material is already represented. Do not duplicate the radar's
internal hypotheses as external sources. Existing coverage includes MCC-AI,
CRA, Traficom/NCSC-FI, Microsoft ACS and Agent Governance Toolkit, OpenSSF,
CycloneDX Agent-BOM, OWASP, the named competitor repositories, APSquared, and
the Bainbridge/Davies/Edmondson/Goodhart conceptual entries.

Four external-source gaps or precision upgrades remain:

1. **GDPR erasure:** add the official regulation alongside the current
   vendor essay:
   `https://eur-lex.europa.eu/eli/reg/2016/679/oj/eng`.
   Classify as `regulation`, relation `threat-model, demand-driver`, action
   `cite, integrate`. The registry should not ground the Article 17 tension
   only in `hoop.dev`.
2. **Official EC AI Act watch source:** add
   `https://digital-strategy.ec.europa.eu/en/policies/regulatory-framework-ai`
   so the radar's "EC digital-strategy pages (AI Act, CRA guidance)" watch
   instruction has both official pages in the registry. Classify as
   `regulation`, relation `demand-driver`, action `cite, watch`.
3. **Open Regulatory Compliance WG:** add `https://orcwg.org/`. It resolves
   successfully and identifies the Open Regulatory Compliance Working Group;
   the radar currently names Eclipse ORC without a direct registry source.
   Classify as `practice`, relation `complement, demand-driver`, action
   `watch, cite`.
4. **IETF identity precision:** add the active WIMSE WG page
   `https://datatracker.ietf.org/wg/wimse/about/` and the AIMS presentation
   `https://datatracker.ietf.org/doc/slides-126-wimse-aims/`. The IETF
   Datatracker returns an active WIMSE WG, but no group with acronym AIMS;
   AIMS currently appears as the IETF 126 presentation "AI Agent
   Authentication and Authorization (AIMS)." Classify WIMSE as `standard
   (emerging)` and AIMS as `discussion, standard (proposal)`, both relation
   `prior-art, complement`, action `watch`.

The last item also exposes a radar wording issue: "IETF AIMS/WIMSE drafts"
currently makes AIMS sound like an established WG/draft series. A separate
governed radar correction should distinguish the AIMS proposal/presentation
from the active WIMSE WG. Do not change the Active radar list inside this
source-only MR without its required authorization.

## Resolution

Please incorporate the Gavin Ray entry and the four bounded primary-source
upgrades into MR !962, preserving its disclosed verification levels. Reference
this record in the MR note or completion response. Radar prose changes, if any,
remain a separate governed action.
