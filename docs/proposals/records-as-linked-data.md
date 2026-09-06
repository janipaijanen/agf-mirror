<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Proposal: AGF records as linked data (PROV-O / JSON-LD export + SKOS vocabularies)

Status: draft for planner-architect critique. Companion to
`per-release-agent-role-participation-map.md`; routed together via handoff
`ggss`. Not a Gate 1 plan.

## Outcome

Two additive interoperability layers over records AGF already keeps:

1. **A PROV-O / JSON-LD export of the coordination channel** — the immutable
   audit trail becomes a standards-based knowledge graph anyone can query with
   off-the-shelf tools.
2. **SKOS concept schemes for AGF's controlled vocabularies** — the enum
   strings scattered across schemas and prose get stable URIs and
   machine-readable definitions, stated once canonically.

## Why

- AGF argues for open, removable, non-proprietary formats. A linked-data view
  of the audit trail is the strongest possible expression of that: the record
  outlives the tool.
- It makes governance claims mechanically checkable. Reviewer independence
  (#47), gate-chain integrity, and human-accountability (#45) become SPARQL
  queries over the exported graph rather than hand audits.
- It gives the `academic-peer-reviewer` lane something citable and queryable.
- The per-release role-participation map becomes a thin aggregate query over
  the PROV-O view instead of a bespoke command — so this layer is the more
  foundational of the two proposals.

## Design — 1. PROV-O / JSON-LD coordination export

`agov coord export --format jsonld` (extends the existing `agov coord
graph-index` graph). Mapping:

| AGF concept | PROV-O |
|---|---|
| a coordination record (review / handoff / decision-record / …) | `prov:Activity` |
| the acting role + model | `prov:Agent` (role as `prov:actedOnBehalfOf` an org/project agent) |
| a plan / MR / branch / artifact named in `artifacts_owned` or `topic` | `prov:Entity` |
| `references-to` | `prov:wasInformedBy` |
| a Gate approval deriving from a plan + its reviews | `prov:wasDerivedFrom` / `prov:used` |
| `timestamp` | `prov:atTime` / `prov:startedAtTime` |

Ship a published `@context` defining the `agf:` terms that PROV-O does not
cover (`agf:kind`, `agf:gate`, `agf:requiresAck`, `agf:supersedes`). Output is
metadata only — identifiers, roles, models, timestamps, edges — never record
bodies, note text, names, or restricted quotations. Codename-only on any
export that crosses the codebase boundary (`external-anonymization-rule`).

## Design — 2. SKOS concept schemes

Publish these as `skos:ConceptScheme`s with `skos:prefLabel`,
`skos:definition`, `skos:scopeNote`, and `skos:broader`/`skos:related` where
an ordering or grouping exists:

- coordination `kind` (9 concepts)
- learning `category` (gap / process / validation / adaptation / tension /
  improvement / bug / boundary)
- learning `confidence.level`
- maturity levels (Planned → Deployable; `skos:broader` chain by level)
- evidence-status (Empirically validated / informed / Theoretical)
- impact classification (BREAKING / ADDITIVE / EDITORIAL / PREVENTIVE)
- agent status (active / suspended / skipped)
- gate vocabulary (Gate 1 / Gate 2)

Definitions are lifted from the existing canonical sources
(`maturity-vocabulary.md`, `downstream-impact-rule.md`, the coordination
schema, etc.), not re-authored — this proposal makes the single-source
discipline machine-enforceable, it does not restate the definitions.

## Governed decision required

A **URI namespace / persistence policy** for the concept URIs and the
`@context` (e.g. a `w3id.org` redirect, a GitLab Pages path, or a DOI-backed
namespace). This is a governor decision, not a planner default, because it is
a long-lived external commitment.

## Scope boundary

Both layers ship as **optional additive** exports and published vocabularies —
no schema change, no gate, no CI-blocking rule, no change to how records are
authored. Making a linked-data export a *required* release artifact later is a
separate gate-mechanic Gate 1 with its own negative test.

## Fit and lower-value parts

- **Good fit:** SKOS vocabularies (bounded, definitions exist, immediate
  interop win); PROV-O/JSON-LD channel export (graph already built, ~1:1
  mapping, enables the independence/gate queries).
- **Medium:** learning records as JSON-LD — export the metadata graph
  (`category`, `severity`, `confidence`, `agents_involved`, `evidence_refs` as
  links), not the prose bodies. Worth doing alongside the SKOS layer.
- **Not worth it:** full RDF-izing rule and plan prose — only the frontmatter
  is structured and query value is low against the cost.

## Affected files (after Gate 1)

- `tooling/rust/` — `agov coord export --format jsonld` + tests; optional
  `agov vocab export` for the SKOS schemes.
- `agf/docs/site/` — a published `@context`, the SKOS turtle/JSON-LD files, and
  a page explaining the vocabulary URIs and the export.
- `docs/reviews/rule-projection/<plan-id>.yaml` — six-row screen if the docs
  page carries normative "use these URIs" wording.
- a `GOVERNANCE`/ADR entry recording the URI namespace decision.

No schema or record-authoring change.

## Test strategy

- Deterministic export: same coordination range → byte-identical JSON-LD
  (modulo a documented canonical ordering).
- JSON-LD validates against a shape (SHACL or a JSON-LD playground round-trip);
  PROV-O terms resolve.
- Anonymization test: export of a range containing `model: human` records
  contains no names, no bodies, no credentials.
- SKOS validation: every concept has `prefLabel` + `definition`; `broader`
  chains are acyclic; concept URIs resolve.

## Risks and controls

- **URI rot / broken namespace.** Control: the governed persistence-policy
  decision up front; prefer a redirect service over a raw repo path.
- **Definition drift between the SKOS scheme and its canonical source.**
  Control: the scheme is generated from the canonical source, or a validator
  diffs them; the scheme never becomes a second definition.
- **Export leaks content.** Control: metadata-only by construction + the
  anonymization test; boundary-crossing exports are codename-only.
- **Scope creep into a mandated format.** Control: additive/optional only;
  mandating it is a separate gate-mechanic Gate 1.

## Rollback

Remove the export subcommand(s), the published `@context` and SKOS files, and
the vocabulary page. Retire the URI namespace per its own policy. No data or
schema migration.
