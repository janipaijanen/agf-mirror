---
provenance: verified-agf-v2
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Implementation Honesty: Domain Maturity Levels
#
# Framework principle: Honest capability communication
# See: framework.md — Appendix: Validation Status
# Evidence: frost-quill learning (2026-04-06, critical), iron-gate tension (schema breadth vs depth)

This rule applies to all documentation, presentations, publications, and
external-facing materials that describe what the project can do.

## The problem

A project may define types, schemas, or models for a domain without having
full operational support. Claiming "supports X" when only type definitions
exist is overclaiming. Overclaiming damages credibility with reviewers,
customers, and partners.

## Five-level domain maturity model

When describing the maturity of any domain, feature area, or capability,
use this five-level classification:

| Level | Name | Definition | What exists |
|-------|------|------------|-------------|
| 5 | **Deployable** | Full operational support | Schema + scenarios/instances + validation + deployment/generation |
| 4 | **Validated** | Verified but not deployable | Schema + scenarios/instances + validation (no deployment) |
| 3 | **Instantiated** | Used but incompletely verified | Schema + scenarios/instances (incomplete validation) |
| 2 | **Schema-defined** | Types exist only | Schema types defined but not used in scenarios, or orphaned |
| 1 | **Planned** | Identified only | Named in roadmap or design docs but not yet in schema |

## Rules

- **Never claim a capability without stating its maturity level.** "The
  framework models physical security" must be qualified: "physical security
  is Schema-defined (Level 2)."

- **Binary yes/no capability tables are forbidden.** Replace "Yes" / "No"
  columns with the maturity level. A reader can then distinguish between
  a domain that is fully deployable and one that exists only as type
  definitions.

- **Maturity levels must be verifiable.** For each claimed level, a reviewer
  should be able to point to the artifacts that justify it:
  - Level 5: deployment output or generated artifacts
  - Level 4: validation rules/shapes/tests that pass
  - Level 3: instance data or scenario files
  - Level 2: schema/type definitions
  - Level 1: roadmap or design document entry

- **Update maturity levels when the implementation changes.** A domain that
  gains validation shapes moves from Level 3 to Level 4. A domain whose
  only scenario is removed drops from Level 3 to Level 2. Keep the
  classification current.

## Adaptation

This model was developed for a schema-driven project (scenario definition
language) but the levels apply to any project with layered capabilities:

| Generic equivalent | Schema-driven project | Web application | CLI tool |
|--------------------|----------------------|-----------------|----------|
| Level 5: Deployable | Schema + scenario + validation + deployment | Feature complete, tested, documented, deployed | Command implemented, tested, documented, in release |
| Level 4: Validated | Schema + scenario + validation | Feature complete and tested, not yet deployed | Command implemented and tested, not yet released |
| Level 3: Instantiated | Schema + scenario instances | Feature partially implemented, some tests | Command partially implemented |
| Level 2: Schema-defined | Types defined only | API route or model exists, no logic | Flag/subcommand defined, not yet wired |
| Level 1: Planned | In roadmap | In backlog or design doc | In README "planned features" |

Adapt the level names and definitions to your domain, but preserve the
five-level granularity to prevent overclaiming.
