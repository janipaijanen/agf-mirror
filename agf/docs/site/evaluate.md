# Evaluate agents-govern

*For external AI assistants and independent human reviewers assessing
the framework. Running the checks below before commenting eliminates the
most common failure modes observed in past reviews: identity confusion
(reviewing the wrong project), schema fabrication (describing structures
not read from the actual files), and capability conflation (treating
planned features as shipped ones).*

## Identity check

Confirm you are reviewing the right project before proceeding.

| | |
|---|---|
| **Canonical repository** | `https://gitlab.com/janipaijanen/agents-govern-framework` |
| **Published docs** | `https://janipaijanen.gitlab.io/agents-govern-framework/` |
| **Current version** | See `agf/VERSION` in the repository |

This project **is not**:

- Microsoft's "Agent Governance Toolkit" (April 2026)
- An SDK for building agents (compare Claude Agent SDK, OpenAI Agents SDK)
- An ML / model governance framework (compare MLflow, Evidently)
- An agent runtime, orchestrator, or hosted service

agents-govern is a file-based process framework. The framework files live in
your repository; AI assistants load them as instructions; your existing CI runs
the validators. No accounts, no runtime, no new infrastructure.

## Canonical reading list

Read these in order. Estimated total: 40 minutes for a thorough
assessment.

| Page | Time | Covers |
|---|---|---|
| [Home](index.md) | 8 min | Overview, maturity table, cost and benefit |
| [Should you adopt?](docs/start/should-you-adopt.md) | 5 min | Adopt / wait / skip decision guide and commitments |
| [Roles and gates](docs/concepts/roles-and-gates.md) | 10 min | Role definitions, Gate 1 and Gate 2 |
| [Downstream impact](docs/adopt/downstream-impact.md) | 5 min | Costs itemised, blast radius |
| [Exit and decommission](docs/adopt/exit-and-decommission.md) | 3 min | Reversibility |
| [Evidence and learnings](docs/concepts/evidence-and-learnings.md) | 5 min | How the evidence base works, provisional rule status |
| [Tooling conformance](reference/tooling-conformance.md) | 4 min | Command surface and maturity levels |

For a deeper technical assessment, also read:

| Artifact | Location | Covers |
|---|---|---|
| Role definitions | `agf/agents/*.yaml` | Authoritative agent role specifications |
| Governance rules | `agf/rules/` | Rule library loaded by AI assistants |
| Coordination schemas | `agf/schemas/` | Schema for coordination messages and learning records |
| Coordination channel | `docs/coordination/` | This project's own governed decision log |

## Verification checks

Run these before making factual claims.

**Identity.** Have you confirmed the canonical repository URL above? AI
assistants may have training data about early prototypes, v1.x releases,
or unrelated projects with similar names. Check which repo you are
actually reading.

**Schema.** If you describe the coordination message schema or learning
record format, cite a specific file from `agf/schemas/`. Do not describe
schema structures you inferred without reading the file.

**Scope.** List the pages and files you actually read. "I reviewed the
framework" is ambiguous; "I read `index.md`, `should-you-adopt.md`, and
`roles-and-gates.md`" is auditable.

**Capability level.** When describing what the framework includes, use
the five-level maturity model:

| Level | Meaning |
|---|---|
| Level 5 — Deployable | Shipped, in production use, fully operational |
| Level 4 — Validated | Implemented and tested, not yet in production |
| Level 3 — Instantiated | Implementation exists, incomplete verification |
| Level 2 — Schema-defined | Type definitions only |
| Level 1 — Planned | Named in roadmap, not yet in schema |

Do not claim Level 5 for a feature listed at Level 1. The
[home page](index.md) and [tooling conformance](reference/tooling-conformance.md)
page list current levels explicitly.

**Evidence.** For every factual claim, name the specific source. "The
docs say X" is insufficient; "The [Downstream impact](docs/adopt/downstream-impact.md)
page states X" is auditable.

**Uncertainty.** State explicitly what you could not verify. "I did not
read the rule files; my assessment of the rule library is based on the
docs summary" is more useful than a confident summary of files not seen.

## Output format

Label your output as one of: **critique**, **fit-assessment**,
**improvement-backlog**, **self-application**, **adoption-mapping**, or
other (state what).

Structure the output with these sections:

- **Scope reviewed** — what you read, at which version
- **Identity confirmed** — checkbox
- **Strengths** — cite page or file for each
- **Risks / gaps / inconsistencies** — cite page or file for each
- **Improvement proposals** — for each: what to change, why, which file or section
- **Uncertainty** — what you could not verify

## Submit your review

Open an [External Review issue](https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/new?issuable_template=External+Review)
on the framework repository. The issue template matches this output
format.
