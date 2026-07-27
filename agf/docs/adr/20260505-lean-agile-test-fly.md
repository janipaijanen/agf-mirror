<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Architecture Decision Record (ADR): Lean-Agile mapping profile + Test-Fly principle

## Status

Proposed (Provisional)

## Date

2026-05-04 (proposed); committed 2026-05-05

## Target framework version

agents-govern 1.1.46

## Supersedes

The earlier inline draft titled *"ADR-001: Adopt JavaScript Object Notation (JSON) for Linked Data (JSON-LD) layered ontology with Lean-Agile-Skunkworks philosophy"* (rejected — see Decision §1–3 below).

## Context

agents-govern 1.1.46 is a mature, Yet Another Markup Language (YAML)-based, evidence-driven Software Development Lifecycle (SDLC)-governance framework:

- 215 learnings across 15 (single-developer-correlated) projects.
- Process-oriented, explicitly *not* runtime enforcement (see [README.md "What this is, what this is not"](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/agents-govern/README.md)).
- Validators (`agov-validate-*`) consume YAML directly.
- Audit trail = git history (see [GOVERNANCE.md "Audit Trail"](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/agents-govern/GOVERNANCE.md)).
- Adoption surface optimized for `cp -r` + grep `<!-- CUSTOMIZE -->`.
- Explicit anti-pattern: theoretical additions without evidence (see [README.md "What does not belong in the framework"](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/agents-govern/README.md)).
- Strongest stated need: **independent adopters**, not new framework features (see [README.md "Independent adopters wanted"](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/agents-govern/README.md)).

An initial proposal sought:
- (a) JavaScript Object Notation for Linked Data (JSON-LD) ontology rewrite with three-layer Core/Shared/Project composition,
- (b) Provenance (PROV) Ontology (PROV-O) provenance,
- (c) Lean + Agile + Skunkworks philosophical scaffolding.

Iterative review pruned (a), (b), and the Skunkworks element of (c). What remained is a documentation extension.

## Decision

1. **Reject JSON-LD migration.** No format change. YAML remains ground truth. Reasoning: no learning record cites a YAML-caused failure mode; YAML-LD/JSON-LD-in-YAML is available if linked-data semantics ever become evidence-backed (and is partially adopted as Provisional artifact shape per [`20260505-yaml-ld-provisional-artifact-shape.md`](20260505-yaml-ld-provisional-artifact-shape.md)); PROV-O duplicates the git audit trail; Shapes Constraint Language (SHACL)/pyld tooling regresses adopter ergonomics; round-tripping is a permanent maintenance tax during a phase when adopter acquisition is the bottleneck.

2. **Reject Skunkworks framing.** Out of bounds. Kelly Johnson's Rule 1 ("the program manager has practically complete control") and the elite-team-with-one-boss model conflict with the framework's Verb-Test, single-decision-point gates, and Escalation-over-Improvisation principles.

3. **Reject ontology-layer rewrite.** The existing layout (`rules/`, `project-specific-examples/`, adopter profiles) already provides Core/Shared/Project composition without IRIs or contexts.

4. **Accept a Lean-Agile mapping profile.** Descriptive cross-reference of existing framework mechanisms to Lean (Toyota Production System) and Agile-Manifesto vocabulary. No new prescriptions.

5. **Accept Test-Fly as a named principle.** Elevate the existing implicit "we eat our own dog food" stance ([GOVERNANCE.md eats-our-own-dogfood line](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/agents-govern/GOVERNANCE.md)) into a citable, enforceable gate-criterion.

6. **Process the addition through the framework's own evidence cycle.** Submit a learning record under a research codename. Sentinel review applies the standard promotion rules.

## Consequences

**Positive**
- Zero tooling debt.
- Zero adopter regression.
- Lean/Agile alignment becomes citable without inventing new assertions.
- Test-Fly gives existing dogfood discipline a name and a reference.
- The framework's own evolution rules are honored (no theory-driven changes; corroboration required for promotion).

**Negative**
- Gives up the speculative AI-interop wins that JSON-LD would have provided. Accepted: those wins had no learning-record demand.
- Profile is a vector for future theoretical additions if not policed. Mitigation: profile is descriptive only; new assertions must arrive via separate learning records.

**Neutral**
- Adds three small artifacts to the framework. None of them block, alter, or replace anything that exists.

## Deliverables

### D1. Lean-Agile mapping profile

A single mapping table with three columns — `lean_or_agile_concept`, `framework_mechanism`, `cite_path` (file:line into the framework). Required entries (all map to existing mechanisms; no new ones):

| Concept | Framework mechanism |
|---|---|
| Jidoka — build quality in, stop the line | Gate-2 review (no commit without review) |
| Kaizen — continuous improvement | Sentinel review log + evidence-driven evolution |
| Muda — waste elimination | `overkill` learning category |
| Genchi Genbutsu — go and see | "Empirically validated" tier |
| Respect for People | Escalation over Improvisation; Human as ultimate authority |
| Agile: working software over comprehensive documentation | Anti-theoretical-addition rule |
| Agile: responding to change | Versioning + `superseded_by` in learning records |
| Agile: individuals and interactions | Verb Test (no two agents share a primary verb) |

**Status:** Provisional (single-conversation evidence). Cannot be promoted higher without an independent adopter learning record.

**Constraint:** every row MUST cite a framework section, `GOVERNANCE.md`, `LEARNINGS.md`, or an `agents/*.yaml` field. Rows without citations are invalid.

### D2. Test-Fly principle in framework.md

Tenet 1 of `MANIFESTO.md`. The principle is also referenced from the AI-Consumption appendix in framework.md.

### D3. Learning record

Filed at [`learnings/linked-data-probe.yaml`](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/agents-govern/learnings/linked-data-probe.yaml).

### D4. No tooling changes

At the time of this ADR, `pyproject.toml`, `src/agents_govern/`, and
`agov-validate-*` were untouched. This remains historical context; current v2
validator guidance lives in the Elixir Mix tasks under `tooling/elixir/`.

### D5. CHANGELOG entry

Single line under the next minor version (when bumped): `Added: MANIFESTO.md (Provisional) — Lean-Agile alignment + Test-Fly principle. ADRs 20260505-lean-agile-test-fly and 20260505-yaml-ld-provisional-artifact-shape. Rejected: JSON-LD migration; Skunkworks framing.`

## Non-Deliverables (must NOT be produced)

- Any `*.jsonld` file, `@context` parallel artifact, or SHACL shape.
- Any new agent definition under `agents/`.
- Any rewording of existing principles or agent jurisdictions.
- Any addition to `docs/adr/` referencing the rejected JSON-LD proposal — this ADR is the canonical record.
- Any changes to the distribution archive's adopter-facing copy/customize flow.

## Promotion Criteria

This ADR's outputs leave **Provisional** status when:
- At least one independent adopter project records a `validation` or `adaptation` learning citing the Lean-Agile profile, MANIFESTO, or Test-Fly principle, **and**
- A Sentinel review confirms corroboration per the rules in [`LEARNINGS.md`](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/agents-govern/LEARNINGS.md).

Per the framework's 5-record-staleness rule, if neither D1 nor D2 is exercised by any independent adopter within five Sentinel review cycles, both are reviewed for removal.

## See also

- [`20260505-yaml-ld-provisional-artifact-shape.md`](20260505-yaml-ld-provisional-artifact-shape.md) — companion ADR grounding Tenet 5.
- `MANIFESTO.md` — Provisional manifesto produced by this ADR's deliverables.
- [`learnings/linked-data-probe.yaml`](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/agents-govern/learnings/linked-data-probe.yaml) — research codename + evidence record.
