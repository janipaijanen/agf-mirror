<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Architecture Decision Record (ADR): YAML Ain't Markup Language (YAML)-LD-Compatible Source Records as Provisional Artifact Shape

## Status

Proposed (Provisional)

## Date

2026-05-05

## Subject

`MANIFESTO.md` Tenet 5

## Supersedes

None.

## Related

- [`20260505-lean-agile-test-fly.md`](20260505-lean-agile-test-fly.md)
- `MANIFESTO.md`
- [`docs/process/manifesto-self-test.md`](../process/manifesto-self-test.md)
- [`learnings/linked-data-probe.yaml`](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/agents-govern/learnings/linked-data-probe.yaml)

## Context

The manifesto consensus thread (2026-05-04 to 2026-05-05) explored linked-data governance for agents-govern. Earlier drafts overcommitted by placing `YAML-LD records over parallel format migrations` in the value list. The self-test run identified a defect: YAML-LD is a format/profile direction, not itself a Lean, Agile, or existing agents-govern mechanism.

`MANIFESTO.md` v3 corrected this by changing the top-level value to:

> **Authoritative source records** over parallel format migrations.

YAML-LD-compatible YAML remains in Tenet 5 as a Provisional artifact-shape hypothesis:

> YAML remains the human-facing source of truth. Linked-data semantics may live in YAML-LD-compatible records.

This ADR decides whether that narrower YAML-LD position is acceptable as a Provisional direction.

## Decision

Adopt **YAML-LD-compatible YAML as the Provisional artifact shape** for linked-data governance experiments.

This means:

- YAML remains the authoritative, human-facing governance artifact.
- Linked-data semantics may be expressed directly in YAML-compatible records.
- Any graph, index, JavaScript Object Notation (JSON) view, or serialized representation is derived.
- No `.jsonld` file, graph database, Shapes Constraint Language (SHACL) layer, or parallel canonical representation is required by this decision.
- YAML-LD is not promoted to a universal manifesto value. It is an implementation hypothesis under the broader value of authoritative source records.

## Rationale

YAML is already the framework's authoring and validation surface. The framework's existing adoption model depends on files that humans can copy, read, review, diff, and customize.

Linked-data semantics are still desirable because governance artifacts benefit from:

- stable identifiers;
- typed records;
- explicit provenance;
- machine-checkable references;
- reusable profiles;
- cross-record navigation.

YAML-LD-compatible YAML is the smallest plausible experiment that preserves the current authoring surface while testing those benefits.

## Mapping

This ADR grounds the YAML-LD tenet by mapping it to existing mechanisms and lineage:

| Source | Mapping |
|---|---|
| agents-govern Artifact Ownership | The YAML record is the owned source artifact; derived views cannot override it. |
| agents-govern append-only learning records | Learning history remains record-shaped and auditable. |
| agents-govern provenance fields | Source path, framework version, and review history can become explicit linked fields. |
| Lean / standardized work | The source record is the standard; derived views stop when they diverge. |
| Lean / Jidoka | Validation can stop the line when derived views disagree with the source record. |
| Agile / working artifact | The reviewable artifact is preferred over detached format machinery. |

## Consequences

**Positive:**

- Preserves human-readable YAML governance.
- Avoids a parallel format migration.
- Allows controlled linked-data experimentation.
- Gives AI agents stable identifiers and references without making graph tooling mandatory.
- Keeps source-of-truth discipline explicit.

**Negative:**

- YAML-LD skill and tooling familiarity may be limited.
- YAML-LD may be misread as a hidden JSON-LD migration.
- Ordinary YAML parsers may not validate linked-data semantics.
- The framework may need future validators if the experiment succeeds.

## Risks And Mitigations

| Risk | Mitigation |
|---|---|
| AI-safety scope creep | State repeatedly that this is Software Development Lifecycle (SDLC) governance for human-agent software work, not general AI safety, model governance, or runtime containment. |
| Sentinel review misread as every-change approval | Define Sentinel review as framework promotion/evolution review, not mandatory project-task review. |
| YAML-LD tooling complexity | Require ordinary YAML readability and parser compatibility. Use lowest-common-denominator YAML. |
| Privacy controls overclaimed as compliance | State that codenames and anonymization are privacy controls, not legal or regulatory certification. |
| Provenance overclaimed as full causality | State that provenance records evidence and authorization, not every causal factor. |
| External commentary lost outside admissible evidence | Add a commentary-triage bridge so useful external signals can be admitted or discarded with reasons (see [`docs/process/commentary-triage.md`](../process/commentary-triage.md)). |

## Non-Goals

- No JSON-LD migration.
- No `.jsonld` canonical artifacts.
- No `.ld` extension parallel artifacts.
- No runtime enforcement.
- No tool-call authorization.
- No model, prompt, dataset, or ML lifecycle governance.
- No requirement that adopters run semantic-web infrastructure.
- No promotion above Provisional.

## Acceptance Criteria

This ADR remains Provisional until:

- one existing framework artifact is converted to YAML-LD-compatible YAML;
- the converted artifact remains understandable in plain Markdown/YAML review;
- existing validators are not broken;
- any new validation is additive and optional during the experiment;
- a Test-Fly record documents what worked, what failed, and what overhead was introduced.

## Rejection Criteria

Retire or revise this direction if:

- YAML-LD makes governance artifacts harder for humans to review;
- derived views begin to compete with YAML as source of truth;
- ordinary adopters cannot use the artifacts without specialized tooling;
- no real framework change benefits from the linked-data fields after the review window;
- external adopter evidence shows the added semantics increase maintenance overhead without improving traceability.

## Review Notes

This ADR does not make YAML-LD mandatory for adopters. It allows a controlled experiment and gives Tenet 5 of `MANIFESTO.md` an explicit recorded basis.

The manifesto value remains format-neutral:

> **Authoritative source records** over parallel format migrations.
