---
kind: note
from: contribution-auditor
to: planner-architect
id: 20260908T194354Z-contribution-auditor-6s5d
timestamp: 2026-09-08T19:43:54Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260908T192135Z-coder-tester-3a5r.md
  - 20260908T183242Z-planner-architect-ds4k.md
  - 20260906T202341Z-contribution-auditor-ggss.md
topic: issue129-evidence-inventory
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Issue #129 — bounded read-only evidence inventory

Per `3a5r`, advisory preparation only — **not** the #129 assessment, not a
Gate 1 plan, no linked-data / namespace / privacy decision. Purpose: make a
later #129 Gate 1 smaller and better grounded. Read-only against `main`
`13fe9738`, `docs/sources.md`, `docs/radar.md`, the `!2460` proposal
`docs/proposals/records-as-linked-data.md`, and the `agov coord graph-index`
help surface.

## 1. Strongest existing sources (already recorded)

| Source | `sources.md` action | Bears on #129 question |
|---|---|---|
| **W3C PROV-DM** `www.w3.org/TR/prov-dm/` | `integrate` — "Describe the trail in PROV terms; PROV-AGENT extends it for agents" | Q2 (correct model), Q3 (relation fidelity) — the anchor model |
| **in-toto / SLSA** `slsa.dev/blog/2023/05/in-toto-and-slsa` | `integrate` — "AGF approval artifacts... are conceptually in-toto attestations; align to predicate types instead of inventing a format" | Q5 (release window / publication verification), Q9 (interoperability) |
| **GSN assurance cases** `scsc.uk/gsn` | `compare-in-docs, watch` — "AGF's trail is evidence without an argument structure" | Q1 (positioning — claim→strategy→evidence), Q9 |
| **CycloneDX Agent-BOM** CycloneDX spec issue #895 | `watch` | Q9 (interop with input-provenance formats) |
| **ISO 27001 A.5.3 SoD** | `integrate` — "AGF role model = segregation of duties" | Q1 (which claims become differentiated), the reviewer-independence query |
| **`agov coord graph-index`** (shipped) | internal prior art | Q9 ("prefer a small bridge over a parallel ontology"), Q10 (the baseline any linked-data value claim must beat) |

`radar.md` pre-frames #129's required failure analysis: governance-metric /
approval-rate telemetry is "surveillance-flavored"; outcome-metric
interpretation / count-gaming risk; verification-independence tiers
(different-session < different-model < different-org); "the trail records
humans too" psychological-safety-vs-surveillance. These map directly to
#129 Q1 and Q7 and to the required count-gaming analysis.

## 2. Missing primary-standards evidence (must be fetched, not inferred)

The `!2460` proposal summarises mappings from memory; a real assessment
must read the specs:

- **PROV-O** `www.w3.org/TR/prov-o/` — the OWL ontology (classes + property
  URIs + stated domain/range), distinct from PROV-DM (the model). Every
  mapping in Q3 must be checked against the actual property semantics
  (`prov:wasInformedBy`, `wasDerivedFrom`, `actedOnBehalfOf`,
  `wasRevisionOf`, `invalidated`, `qualifiedAssociation`).
- **PROV-AGENT** — cited nowhere with a URL; Q2's human-session vs
  software-session vs role vs model distinction needs its real definitions.
- **SKOS Reference** `www.w3.org/TR/skos-reference/` — specifically its
  guidance on ordered vs hierarchical concepts; this decides whether
  encoding maturity levels as `skos:broader` is a semantic error (Q4).
- **JSON-LD 1.1** `www.w3.org/TR/json-ld11/` — `@context` design and
  whether a published context is a durable external commitment (Q8).
- **w3id.org** actual process/cost/commitment for a permanent redirect
  namespace (Q8) — do not choose before governor approval.
- **CycloneDX Agent-BOM current state** — issue #895 is *open*; whether a
  usable draft exists materially changes Q9.
- **No AGF fixture graphs exist.** Q's "required evidence" needs two small
  fixture-derived candidate graphs incl. one adverse/superseded chain —
  the assessment must build them.

## 3. Semantic traps (plausible-but-wrong mappings — Q2/Q3/Q4 negative cases)

1. **Role-as-agent.** The AGF role is a hat, not the acting party. Acting
   party = a human/software session; `prov:actedOnBehalfOf` chains session →
   role → project. The role is not itself `prov:Agent`. (Q2 names this.)
2. **`topic` → entities.** `topic:` is free text. Only `references-to`,
   `artifacts_owned`, and explicit ID fields are safe entity sources;
   inferring entities from `topic` tokens manufactures structure from prose.
   The `!2460` table's "…named in … `topic`" hint is a trap.
3. **`references-to` → blanket `prov:wasInformedBy`.** `references-to`
   carries several relations (answering a handoff; superseding a prior
   record; citing decision inputs). Blanket-mapping overstates; supersession
   needs `wasRevisionOf` / `invalidated` + a reason, approvals need
   `wasGeneratedBy` / `used`.
4. **Supersession = `prov:wasRevisionOf`.** AGF supersession spans "replaced
   by a better record" and "overtaken / moot" (cf. `emaq`/`3qn8`). The
   second is not a revision of the first; it needs an `agf:` term or
   `invalidatedAtTime` + reason.
5. **Ordinal order as `skos:broader`.** Planned→…→Deployable and Gate
   1→Gate 2 are sequences, not subsumption. `skos:broader` = "is a kind
   of"; the ladder is not. Use `skos:related` + an ordering property. (Q4
   names this.)
6. **"The graph shows what led to the release."** Only true if the range
   provably covers commit inclusion + event time + decision time + tag
   ancestry + post-tag verification (Q5). Otherwise it is an unsupported
   causation claim.
7. **Export-as-authority.** If any consumer treats the JSON-LD graph as the
   source of truth rather than a derived view over Git + coordination
   files, it becomes a second authority store (Q6) — the exact
   anti-pattern the roadmap-owner model just shipped in B1 exists to
   prevent.
8. **Count/participation as effectiveness.** A graph that makes "N reviews
   / M approvals / K independent sessions" trivially queryable invites
   inferring rigor from volume. `radar.md` flags this; #129's failure
   analysis is mandatory.

## 4. Smallest credible Gate 1 shape

The full 10-question assessment is a research programme, not one Gate 1.
The only **freeze-blocking** part is:

**Slice 1 — "does linked data constrain the `qbka` provenance contract?"**
- Scope: only the records `qbka` Slice A touches — plan lifecycle states,
  supersession provenance (replacement id + governor decision + timestamp),
  transitions.
- Answer Q3 (relation fidelity) and Q4 (vocabulary fidelity) for *those*
  relations only, against the real PROV-O and SKOS specs.
- Output: a one-page compatibility note — should `qbka`'s provenance field
  *shape* be chosen to remain PROV-O/SKOS-projectable later (cheap now), or
  is that premature coupling (defer)? Plus the two required fixture graphs,
  one adverse/superseded chain.
- ~1 review-sized Gate 1 plan. It is the only #129 output that must land
  before the v2.11 lifecycle/provenance freeze.

**Everything else** (full `agov coord export --format jsonld`, `@context`,
SKOS schemes, URI namespace, in-toto/SLSA bridge, adopter-query
cost/benefit, privacy projections) is **not freeze-blocking** and is its
own later Gate 1 — exactly as #129's own "Release disposition" already
says (v2.11 compatibility decisions only; v2.13 for implementation).

## 5. Recommendation to the #129 planner / governor

1. **Split #129:** a small freeze-blocking compatibility slice + the full
   assessment as a separate, later, non-blocking Gate 1.
2. The freeze-blocking slice's binding input into `qbka` is **field-shape
   choices only, not an ontology commitment**. `qbka` must not wait on the
   full assessment.
3. **Fetch the primary specs** (section 2) before trusting any mapping
   table; the `!2460` table has at least three of the section-3 traps baked
   in — a starting point to challenge, not adopt.
4. The Q10 value bar: beat `agov coord graph-index` on a **named real
   adopter/reviewer query**. If none can be named, the honest
   recommendation is `defer` or `narrow to the role-map only`.

Advisory input only. Authors no Gate 1 plan; makes no linked-data,
namespace, or privacy decision.
