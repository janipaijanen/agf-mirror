# Evaluate agents-govern

*The framework that cannot govern itself has no business governing anything else.*

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
| **Current version** | See [Release information](reference/release-info.md), generated from `agf/VERSION` at docs build time. Also shown in the page header (top-right corner). |

This project **is not**:

- Microsoft's "Agent Governance Toolkit" (April 2026). AGF is repository-local
  governance loaded by your existing tools, not a Microsoft-hosted toolkit.
- An SDK for building agents (compare Claude Agent SDK, OpenAI Agents SDK). AGF
  defines roles, gates, and evidence; it does not provide agent APIs.
- An ML / model governance framework (compare MLflow, Evidently). AGF governs
  AI-assisted software work, not model training, experiment tracking, or model
  drift monitoring.
- A runtime governance or agent-monitoring platform. AGF uses pre-action plans,
  CI evidence, and review records; it does not intercept production agent
  behavior at runtime.
- An agent runtime, orchestrator, or hosted service.

See [AGF and agent runtimes](docs/concepts/position.md) for how AGF
relates to Claude Code, Codex CLI, Cursor, and orchestrator libraries.

agents-govern is a file-based process framework. The framework files live in
your repository; AI assistants load them as instructions; your existing CI runs
the validators. No accounts, no runtime, no new infrastructure.

## What this page covers

This page governs the review process. It is not a feature tour and does not
replace the adoption guide. Start with the [home page](index.md): it is the
three-minute entry for the problem, mechanism, cost, exit path, and next step.
Then use this page to avoid common review errors and cite the specific pages
and files that support your claims. For adoption costs, read
[Downstream impact](docs/adopt/downstream-impact.md). For the mental model,
read [Roles and gates](docs/concepts/roles-and-gates.md).

## Canonical reading list

Choose the track that matches your review goal.

| Track | Pages | Time | Outcome |
|---|---|---|---|
| First scan | [Home](index.md) | ~3 min | Continue / stop judgment |
| Quick assessment | [Should you adopt?](docs/start/should-you-adopt.md), [Roles and gates](docs/concepts/roles-and-gates.md), [Downstream impact](docs/adopt/downstream-impact.md) | ~20 min | Adopt / wait / skip judgment |
| Thorough assessment | Full canonical list below | ~40 min | Substantiated external review |

For a thorough assessment, read these in order.

| Page | Time | Covers |
|---|---|---|
| [Home](index.md) | 3 min | Problem, mechanism, cost, exit path, and next step |
| [Should you adopt?](docs/start/should-you-adopt.md) | 5 min | Adopt / wait / skip decision guide and commitments |
| [Roles and gates](docs/concepts/roles-and-gates.md) | 10 min | Role definitions, Gate 1 and Gate 2 |
| [Downstream impact](docs/adopt/downstream-impact.md) | 5 min | Costs itemised, blast radius |
| [Exit and decommission](docs/adopt/exit-and-decommission.md) | 3 min | Reversibility |
| [Evidence and learnings](docs/concepts/evidence-and-learnings.md) | 5 min | How the evidence base works, provisional rule status |
| [Maturity vocabulary](reference/maturity-vocabulary.md) | 3 min | Capability maturity, evidence status, compliance profile |
| [Tooling conformance](reference/tooling-conformance.md) | 4 min | Command surface and maturity levels |

For a deeper technical assessment, also read:

| Artifact | Location | Covers |
|---|---|---|
| Role definitions | `agf/agents/*.yaml` | Authoritative agent role specifications |
| Governance rules | `agf/rules/` | Rule library loaded by AI assistants |
| Coordination schemas | [`agf/schemas/`](https://gitlab.com/janipaijanen/agents-govern-framework/-/tree/main/agf/schemas) | Schema for coordination messages and learning records |
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
| Level 2 — Surface-defined | Interface, type, or schema surface exists only |
| Level 1 — Planned | Named in roadmap, not yet in schema |

Do not claim Level 5 for a feature listed at Level 1. The
[maturity vocabulary](reference/maturity-vocabulary.md), [home page](index.md),
and [tooling conformance](reference/tooling-conformance.md) pages list current
levels explicitly.

**Evidence status.** When describing a framework claim, principle, or
recommendation, state whether it is **Empirically validated**, **Empirically
informed**, or **Theoretical**. Capability maturity and evidence status are
separate: a command can be Level 5 Deployable while the broader claim about its
organizational benefit is only empirically informed.

**Evidence.** For every factual claim, name the specific source. "The
docs say X" is insufficient; "The [Downstream impact](docs/adopt/downstream-impact.md)
page states X" is auditable.

**Uncertainty.** State explicitly what you could not verify. "I did not
read the rule files; my assessment of the rule library is based on the
docs summary" is more useful than a confident summary of files not seen.

## Reviewer evidence index

Use this table when turning a review claim into an auditable finding.

| Claim type | Cite this source | Notes |
|---|---|---|
| Project identity and current docs version | [Release information](reference/release-info.md) | Generated from `agf/VERSION` at docs build time. |
| Published release artifacts | [Release artifacts](reference/release-artifacts.md) and the [GitLab Releases page](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases) | Use the release page for what is actually downloadable now. |
| Role and gate authority | [Roles and gates](docs/concepts/roles-and-gates.md), plus `agf/agents/MANIFEST.yaml` for this project | Distinguish framework roles from the current AGF-NG project role assignments. |
| Coordination message protocol | [Coordination protocol](docs/concepts/coordination-protocol.md) and `agf/schemas/coordination-message.schema.json` | Schema shape and process intent are separate evidence. |
| Gate 1 plan format | `agf/schemas/gate1-plan.schema.json` | Cite the schema when making field-level claims. |
| Learning records and evidence status | [Evidence and learnings](docs/concepts/evidence-and-learnings.md) and `agf/schemas/learning-record.schema.json` | Separate evidence-status claims from capability-maturity claims. |
| Tooling maturity | [Tooling conformance](reference/tooling-conformance.md), plus live `agov --help` / `agov validate --help` when reviewing a checkout | A documented target is not automatically a shipped command. |
| Contracts, validators, and release gates | [Contracts and validators](docs/advanced/contracts-and-validators.md) and [Review and release](docs/operate/review-and-release.md) | Current tooling validates shapes and selected release-range evidence; semantic review remains human. |
| Adoption cost and downstream impact | [Downstream impact](docs/adopt/downstream-impact.md) and [Exit and decommission](docs/adopt/exit-and-decommission.md) | Use these for adopter burden, reversibility, and upgrade-risk claims. |

## Schema claim checklist

Before stating what an AGF schema requires:

- Read the exact schema file, not only prose that summarizes it.
- Cite the schema path and, when useful, the field name.
- Distinguish schema shape from semantic validation. A field can be required
  by schema without proving that the referenced decision, plan, or review is
  substantively correct.
- Distinguish schema contract from shipped tooling. If a validator is not
  listed as shipped, describe it as a contract target or roadmap item.
- If you did not read the schema directly, mark the claim as docs-only or
  unverified.

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
format. A free GitLab account is required to create an issue.
